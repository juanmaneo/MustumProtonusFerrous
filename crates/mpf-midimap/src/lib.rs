//! Port of the deterministic logic of Wine's `dlls/midimap/midimap.c`: the mapper's table of
//! midi out ports, the registry settings that route each of the 16 channels to a port, the
//! channel routing of messages, and the `MIDIHDR` state that prepare and long data check.
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - `DRVM_INIT` and `DRVM_EXIT`: the port table is built once, and kept after exit.
//! - [`ports`]: the port table build and `MIDIMAP_FindPort`, including the `#n` form.
//! - [`settings`]: `MIDIMAP_LoadSettings`, `MIDIMAP_LoadSettingsDefault` and
//!   `MIDIMAP_LoadSettingsScheme`.
//! - [`Driver`]: `MODM_OPEN`, `MODM_CLOSE`, `MODM_DATA`, `MODM_LONGDATA`, `MODM_RESET` and
//!   `MODM_GETDEVCAPS`.
//! - [`mapper`]: `MODM_PREPARE`, `MODM_UNPREPARE` and `MODM_GETVOLUME`.
//!
//! Divergences, all on paths the oracle cannot run or where C has undefined behaviour:
//! - The host boundary is the [`MidiOut`] and [`settings::Registry`] traits. The
//!   `midiOut*`, `DriverCallback` and `RegOpen*`/`RegQuery*` calls are scripted results.
//! - A port whose capability query failed has no name, so it never matches by name. The
//!   `#n` form still reaches it. C leaves that name uninitialised, so C's result depends on
//!   the heap.
//! - Port names are clipped to 31 code units, as `lstrcpyW` into `MAXPNAMELEN` does.
//! - A string value longer than 512 bytes, or a DWORD longer than 4 bytes, reads as absent.
//!   A shorter DWORD is zero-extended. C reads into a fixed buffer and keeps stale bytes.
//! - A subkey name of 256 code units or more ends the scheme walk, as the `RegEnumKeyW`
//!   buffer does.
//! - A later scheme entry overwrites an earlier one for the same channel. C logs an error
//!   and does the same.
//! - `DRVM_EXIT` keeps the port table. An instance opened before the exit keeps its port
//!   indices, and a stale index is ignored. C frees the table and dereferences the index.
//! - Channels that are not mapped are skipped where C dereferences a null map entry: the
//!   `modOpen` loop and the system-message fan-out of `modData`.
//! - `MODM_GETDEVCAPS` writes at most `min(size, 84, buf.len())` bytes. C does not check
//!   the buffer length.
//! - `modLongData` passes its header to the host by shared reference, so the host cannot
//!   write back to the copy. The `MHDR_DONE` check on that copy (C's "wait until MHDR_DONE"
//!   FIXME) is not modelled.
//! - `MOM_DONE` reports the header's address, as C does. The value is the address of the
//!   Rust reference, not a stable handle.
//! - `TRACE`, `FIXME`, `ERR` and `winediag` logging is not modelled.
//!
//! Skipped:
//! - `DriverProc` and the `DRV_*` messages, including the `drvOpen` patch maps and the
//!   identity `aChn` table.
//! - `DRVM_ENABLE` and `DRVM_DISABLE` (a constant 0), and `MODM_GETNUMDEVS` (a constant 1).
//! - `modSetVolume`, which is a FIXME stub, and the `MIDIMAP_modMessage` dispatch and its
//!   `MMSYSERR_NOTSUPPORTED` default.
//! - `MIDIMAP_IsBadData`: `Option` models the null-pointer case. The memory probe is not
//!   needed.
//! - `HeapAlloc` and `MMSYSERR_NOMEM`, the allocation of instances.

pub mod caps;
pub mod mapper;
pub mod ports;
pub mod settings;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
#[cfg(test)]
mod stub;

pub use settings::{Registry, Root};

use crate::caps::{mapper_caps, MIDIOUTCAPSW_SIZE};
use crate::ports::{OutPort, PortState};
use crate::settings::ChannelMap;

/// `MMSYSERR_NOERROR`.
pub const MMSYSERR_NOERROR: u32 = 0;
/// `MMSYSERR_ERROR`: also returned for a missing driver instance.
pub const MMSYSERR_ERROR: u32 = 1;
/// `MMSYSERR_BADDEVICEID`: returned by `MODM_GETDEVCAPS` when there are no ports.
pub const MMSYSERR_BADDEVICEID: u32 = 2;
/// `MMSYSERR_INVALPARAM`.
pub const MMSYSERR_INVALPARAM: u32 = 11;
/// `MIDIERR_UNPREPARED`.
pub const MIDIERR_UNPREPARED: u32 = 64;
/// `MIDIERR_STILLPLAYING`.
pub const MIDIERR_STILLPLAYING: u32 = 65;
/// `MIDIERR_INVALIDSETUP`: returned when the registry settings cannot be loaded.
pub const MIDIERR_INVALIDSETUP: u32 = 69;

/// `MHDR_DONE`.
pub const MHDR_DONE: u32 = 1;
/// `MHDR_PREPARED`.
pub const MHDR_PREPARED: u32 = 2;
/// `MHDR_INQUEUE`.
pub const MHDR_INQUEUE: u32 = 4;

/// `MOM_OPEN`.
pub const MOM_OPEN: u32 = 0x3C7;
/// `MOM_CLOSE`.
pub const MOM_CLOSE: u32 = 0x3C8;
/// `MOM_DONE`.
pub const MOM_DONE: u32 = 0x3C9;
/// `CALLBACK_TYPEMASK`: the bits of the open flags that select the callback type.
pub const CALLBACK_TYPEMASK: u32 = 0x0007_0000;

/// `MIDIOPENDESC`: the client's handle, callback and instance data.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MidiOpenDesc {
    /// `hMidi`, the client's handle, passed back to the callback.
    pub handle: usize,
    /// `dwCallback`.
    pub callback: usize,
    /// `dwInstance`.
    pub instance: usize,
}

/// The fields of `MIDIHDR` that the mapper reads and writes.
pub struct MidiHdr {
    /// `dwFlags`.
    pub flags: u32,
    /// Whether `lpData` is set. The mapper only checks for a null pointer.
    pub has_data: bool,
}

/// The `midiOut*` calls the mapper makes on each port, and `DriverCallback`.
///
/// A port is identified by its index in the host device table. A handle is what the
/// host returns from `open`. The mapper only compares handles for equality.
pub trait MidiOut {
    /// `midiOutGetNumDevs`.
    fn num_devs(&self) -> usize;
    /// `midiOutGetDevCapsW` for device `dev`: the full product name, or `None` if the
    /// query fails.
    fn dev_name(&self, dev: usize) -> Option<Vec<u16>>;
    /// `midiOutOpen` on device `dev`: the handle, or `None` on failure.
    fn open(&mut self, dev: usize) -> Option<usize>;
    /// `midiOutClose`.
    fn close(&mut self, handle: usize) -> u32;
    /// `midiOutShortMsg`.
    fn short_msg(&mut self, handle: usize, msg: u32) -> u32;
    /// `midiOutLongMsg` on a private copy of the header.
    fn long_msg(&mut self, handle: usize, hdr: &MidiHdr) -> u32;
    /// `midiOutPrepareHeader` on a private copy of the header. The result is unused.
    fn prepare(&mut self, handle: usize, hdr: &MidiHdr);
    /// `midiOutUnprepareHeader` on a private copy of the header. The result is unused.
    fn unprepare(&mut self, handle: usize, hdr: &MidiHdr);
    /// `midiOutReset`.
    fn reset(&mut self, handle: usize) -> u32;
    /// `DriverCallback` with the instance's flags and descriptor.
    fn callback(&self, desc: &MidiOpenDesc, flags: u16, msg: u32, p1: usize, p2: usize);
}

/// One open instance: the channel map chosen at open, the client's descriptor, and the
/// running status byte.
pub struct Instance {
    map: ChannelMap,
    desc: MidiOpenDesc,
    cb_flags: u16,
    running_status: u8,
}

/// The driver state: the host, the registry, and the port table that `DRVM_INIT` builds.
pub struct Driver<M, R> {
    midi: M,
    registry: R,
    ports: Vec<OutPort>,
    initialized: bool,
}

impl<M: MidiOut, R: Registry> Driver<M, R> {
    pub fn new(midi: M, registry: R) -> Self {
        Self {
            midi,
            registry,
            ports: Vec::new(),
            initialized: false,
        }
    }

    /// `DRVM_INIT`: builds the port table. Returns 1 when it was built and 0 when it already was.
    pub fn init(&mut self) -> u32 {
        if self.initialized {
            return 0;
        }
        self.ports = ports::enumerate(&self.midi);
        self.initialized = true;
        1
    }

    /// `DRVM_EXIT`: returns 1 when a table was built, else 0. The table is kept, because C
    /// frees it while open instances still hold port indices into it.
    pub fn exit(&mut self) -> u32 {
        u32::from(std::mem::take(&mut self.initialized))
    }

    /// `modOpen`: loads the channel map and opens each port it uses that is not open yet.
    /// Then it reports `MOM_OPEN`.
    pub fn open(&mut self, desc: Option<&MidiOpenDesc>, flags: u32) -> Result<Instance, u32> {
        let Some(desc) = desc else {
            return Err(MMSYSERR_INVALPARAM);
        };
        let map = settings::load(&self.registry, &self.ports).ok_or(MIDIERR_INVALIDSETUP)?;
        for dev in map.iter().flatten() {
            let port = &mut self.ports[*dev];
            if port.state == PortState::Idle {
                port.state = match self.midi.open(*dev) {
                    Some(handle) => PortState::Open { handle },
                    None => PortState::Failed,
                };
            }
        }
        let cb_flags = ((flags & CALLBACK_TYPEMASK) >> 16) as u16;
        self.midi.callback(desc, cb_flags, MOM_OPEN, 0, 0);
        Ok(Instance {
            map,
            desc: *desc,
            cb_flags,
            running_status: 0,
        })
    }

    /// `modClose`: closes the open ports the instance uses. Reports `MOM_CLOSE` only when
    /// every close succeeds, and the caller drops the instance on success.
    pub fn close(&mut self, inst: Option<&mut Instance>) -> u32 {
        let Some(inst) = inst else {
            return MMSYSERR_ERROR;
        };
        let mut ret = MMSYSERR_NOERROR;
        for dev in inst.map.iter().flatten() {
            let Some(port) = self.ports.get_mut(*dev) else {
                continue;
            };
            let PortState::Open { handle } = port.state else {
                continue;
            };
            let result = self.midi.close(handle);
            if result == MMSYSERR_NOERROR {
                port.state = PortState::Idle;
            } else if ret == MMSYSERR_NOERROR {
                ret = result;
            }
        }
        if ret == MMSYSERR_NOERROR {
            self.midi
                .callback(&inst.desc, inst.cb_flags, MOM_CLOSE, 0, 0);
        }
        ret
    }

    /// `modData`: sends a packed short message. A data byte below 0x80 reuses the running
    /// status, and a system message below 0xF8 clears it.
    pub fn data(&mut self, inst: Option<&mut Instance>, mut param: usize) -> u32 {
        let Some(inst) = inst else {
            return MMSYSERR_ERROR;
        };
        let mut status = (param & 0xFF) as u8;
        if status < 0x80 {
            if inst.running_status == 0 {
                return MMSYSERR_NOERROR;
            }
            status = inst.running_status;
            param = ((param & 0xFFFF) << 8) | usize::from(status);
        }
        let chn = usize::from(status & 0x0F);
        if inst.map[chn].is_none() {
            return MMSYSERR_NOERROR;
        }
        let msg = param as u32;
        let mut ret = MMSYSERR_NOERROR;
        if status < 0xF0 {
            if let Some(handle) = inst.map[chn].and_then(|dev| self.open_handle(dev)) {
                ret = self.midi.short_msg(handle, msg);
            }
            inst.running_status = status;
        } else {
            for dev in inst.map.iter().flatten() {
                if let Some(handle) = self.open_handle(*dev) {
                    ret = self.midi.short_msg(handle, msg);
                }
            }
            if status <= 0xF7 {
                inst.running_status = 0;
            }
        }
        ret
    }

    /// `modLongData`: sends a buffer to each open port the instance uses. The header is
    /// `MHDR_INQUEUE` while the sends run, then `MHDR_DONE`, and `MOM_DONE` reports its address.
    pub fn long_data(&mut self, inst: Option<&mut Instance>, hdr: &mut MidiHdr) -> u32 {
        let Some(inst) = inst else {
            return MMSYSERR_ERROR;
        };
        if (hdr.flags & MHDR_PREPARED) == 0 {
            return MIDIERR_UNPREPARED;
        }
        if (hdr.flags & MHDR_INQUEUE) != 0 {
            return MIDIERR_STILLPLAYING;
        }
        hdr.flags = (hdr.flags & !MHDR_DONE) | MHDR_INQUEUE;
        let mut ret = MMSYSERR_NOERROR;
        for dev in inst.map.iter().flatten() {
            let Some(handle) = self.open_handle(*dev) else {
                continue;
            };
            let copy = MidiHdr {
                flags: 0,
                has_data: hdr.has_data,
            };
            self.midi.prepare(handle, &copy);
            ret = self.midi.long_msg(handle, &copy);
            self.midi.unprepare(handle, &copy);
            if ret != MMSYSERR_NOERROR {
                break;
            }
        }
        inst.running_status = 0;
        hdr.flags = (hdr.flags & !MHDR_INQUEUE) | MHDR_DONE;
        let hdr_addr = std::ptr::from_ref(&*hdr) as usize;
        self.midi
            .callback(&inst.desc, inst.cb_flags, MOM_DONE, hdr_addr, 0);
        ret
    }

    /// `modReset`: resets each open port the instance uses, stopping at the first failure.
    pub fn reset(&mut self, inst: Option<&mut Instance>) -> u32 {
        let Some(inst) = inst else {
            return MMSYSERR_ERROR;
        };
        let mut ret = MMSYSERR_NOERROR;
        for dev in inst.map.iter().flatten() {
            let Some(handle) = self.open_handle(*dev) else {
                continue;
            };
            ret = self.midi.reset(handle);
            if ret != MMSYSERR_NOERROR {
                break;
            }
        }
        inst.running_status = 0;
        ret
    }

    /// `MODM_GETDEVCAPS`: writes the mapper's `MIDIOUTCAPSW` into `buf`, up to `size` bytes.
    /// The write also stops at the end of `buf`, which C does not check.
    pub fn dev_caps(&self, size: usize, buf: Option<&mut [u8]>) -> u32 {
        let Some(buf) = buf else {
            return MMSYSERR_INVALPARAM;
        };
        if self.ports.is_empty() {
            return MMSYSERR_BADDEVICEID;
        }
        let caps = mapper_caps();
        let len = size.min(MIDIOUTCAPSW_SIZE).min(buf.len());
        buf[..len].copy_from_slice(&caps[..len]);
        MMSYSERR_NOERROR
    }

    fn open_handle(&self, dev: usize) -> Option<usize> {
        match self.ports.get(dev)?.state {
            PortState::Open { handle } => Some(handle),
            PortState::Idle | PortState::Failed => None,
        }
    }
}
