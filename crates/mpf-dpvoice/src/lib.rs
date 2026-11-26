//! Port of the portable logic in Wine's `dlls/dpvoice/server.c`.
//!
//! Ported:
//! - `DPVOICE_GetCompressionTypes`, which the client and server
//!   `GetCompressionTypes` methods forward to: [`get_compression_types`].
//!   It reports the single `MS-PCM 64 kbit/s` entry and writes it into the caller's
//!   buffer, using the `DVCOMPRESSIONINFO` layout selected by [`Abi`].
//!
//! Skipped:
//! - `DPVOICE_CreateDirectPlayVoice{Client,Server,Test}`, `DirectPlayVoiceCreate`,
//!   `DllGetClassObject`, the class factory, and all `IUnknown` and vtable code:
//!   COM object and allocation plumbing.
//! - The other `dpvclient_*`, `dpvserver_*` and `dpvtest_*` methods: `FIXME` stubs
//!   that return `E_NOTIMPL`.

mod abi;

use std::iter::once;

pub use abi::Abi;

pub type HRESULT = i32;

pub const DV_OK: HRESULT = 0;
/// `E_POINTER`, which is also `DVERR_INVALIDPOINTER`.
pub const DVERR_INVALIDPOINTER: HRESULT = 0x8000_4003u32 as HRESULT;
pub const DVERR_INVALIDFLAGS: HRESULT = make_dvhresult(0x0078);
pub const DVERR_BUFFERTOOSMALL: HRESULT = make_dvhresult(0x001E);

/// `MAKE_DVHRESULT(code)`: `MAKE_HRESULT(1, _FACDPV, code)` with `_FACDPV` 0x15.
const fn make_dvhresult(code: u32) -> HRESULT {
    (0x8000_0000 | (0x15 << 16) | code) as HRESULT
}

/// `dwSize` as written in Wine's initializer. It is a literal, not `sizeof`.
const PCM_DW_SIZE: u32 = 80;
const PCM_MAX_BITS_PER_SECOND: u32 = 64000;
const PCM_NAME: &str = "MS-PCM 64 kbit/s";
/// The name is ASCII, so its byte length equals its UTF-16 unit count. Includes the NUL.
const PCM_NAME_BYTES: usize = (PCM_NAME.len() + 1) * 2;

/// `DPVOICE_GetCompressionTypes`.
///
/// `data` is `pData` (`None` models NULL). `data_size` and `num_elements` are the
/// out-pointers (`None` models NULL). `*data_size` is read on entry as the buffer
/// size, and when `data` is present it must hold at least that many bytes.
/// `data_addr` is the address of `data`, used for the `lpszName` pointer. Pass 0
/// to get the name's offset within the buffer.
pub fn get_compression_types(
    data: Option<&mut [u8]>,
    data_size: Option<&mut u32>,
    num_elements: Option<&mut u32>,
    flags: u32,
    data_addr: u64,
    abi: Abi,
) -> HRESULT {
    let (Some(data_size), Some(num_elements)) = (data_size, num_elements) else {
        return DVERR_INVALIDPOINTER;
    };
    if flags != 0 {
        return DVERR_INVALIDFLAGS;
    }
    *num_elements = 1;

    let required = abi.struct_size() + PCM_NAME_BYTES;
    let hr = if (*data_size as usize) < required {
        DVERR_BUFFERTOOSMALL
    } else if let Some(data) = data {
        write_pcm_entry(&mut data[..required], data_addr, abi);
        DV_OK
    } else {
        DVERR_INVALIDPOINTER
    };
    *data_size = required as u32;
    hr
}

/// Writes the `pcm_type` struct followed by its name. `out` is exactly `required` bytes.
fn write_pcm_entry(out: &mut [u8], data_addr: u64, abi: Abi) {
    let size = abi.struct_size();
    let pointer = abi.pointer_size();
    out[..size].fill(0);
    out[0..4].copy_from_slice(&PCM_DW_SIZE.to_le_bytes());
    out[4..8].copy_from_slice(&0x8de1_2fd4u32.to_le_bytes());
    out[8..10].copy_from_slice(&0x7cb3u16.to_le_bytes());
    out[10..12].copy_from_slice(&0x48ceu16.to_le_bytes());
    out[12..20].copy_from_slice(&[0xa7, 0xe8, 0x9c, 0x47, 0xa2, 0x2e, 0x8a, 0xc5]);
    let name_at = abi.name_offset();
    let name_addr = data_addr.wrapping_add(size as u64);
    out[name_at..name_at + pointer].copy_from_slice(&name_addr.to_le_bytes()[..pointer]);
    let max_bits_at = abi.max_bits_offset();
    out[max_bits_at..max_bits_at + 4].copy_from_slice(&PCM_MAX_BITS_PER_SECOND.to_le_bytes());
    let name_bytes = PCM_NAME
        .encode_utf16()
        .chain(once(0))
        .flat_map(u16::to_le_bytes);
    for (slot, byte) in out[size..].iter_mut().zip(name_bytes) {
        *slot = byte;
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
