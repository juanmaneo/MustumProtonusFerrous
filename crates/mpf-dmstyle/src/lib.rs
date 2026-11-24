//! dmstyle subset for MPF: the argument checks and GUID dispatch of the track objects in
//! `dlls/dmstyle`, and the motif enumeration of `style_EnumMotif`.
//!
//! Reference Wine's `dlls/dmstyle` implements the DirectMusic style and track COM objects. Most of
//! it works on interfaces, lists, streams and the registry. This crate ports the pure functions
//! that map a parameter GUID to a status in `auditiontrack.c`, `chordtrack.c`, `commandtrack.c`,
//! `motiftrack.c`, `mutetrack.c` and `styletrack.c`, and `style_EnumMotif` from `style.c`. It works
//! on integers, slices and arrays, with no COM objects, lists, handles or I/O.
//!
//! Ported (pure logic, verified against the verbatim C by oracle rows and edge tests):
//! - `track`: `IsParamSupported`, `GetParam` and `SetParam` for the audition, chord, command,
//!   motif, mute and style tracks. The NULL check, the GUID dispatch and the status of each
//!   operation are ported.
//! - `style`: `style_EnumMotif`, which copies a motif name into a 64-unit buffer.
//! - The crate root holds the `HRESULT` codes and the `GUID` type.
//!
//! Deviations from the C:
//! - The `time`, `next` and `param` arguments of `GetParam` and `SetParam` are not modeled, so the
//!   port returns only the status.
//! - `style_track_GetParam` for `GUID_IDirectMusicStyle` writes the first style item to `*param`
//!   and takes a reference on it. The port returns the status for `has_items` and does not model
//!   the write or the reference.
//! - The C null pointers become `Option`. A missing GUID returns `E_POINTER`, and a missing name
//!   buffer does too.
//! - The name buffer of `style_EnumMotif` is an array of `DMUS_MAX_NAME` units, and a pattern name
//!   is a slice. The copy stops at the slice end or at the NUL, and it never copies more than 63
//!   units.
//! - The `TRACE` and `FIXME` messages are not ported.
//!
//! Skipped (reason per item):
//! - The COM objects, vtables and class factories of the six track files and `style.c`, along with
//!   their `IUnknown` and `IDirectMusic*` methods: they need reference counts, lists, streams and
//!   the registry.
//! - The `GUID_*` handlers that only print `FIXME` and return `S_OK`: they have no body to port,
//!   so the port keeps their status and drops the empty body.
//! - The `style_track` item list, which holds COM pointers, and its `LIST_FOR_EACH_ENTRY` walk.
//! - Style loading from `IStream` and the other `IDirectMusicStyle8` methods.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub mod style;
pub mod track;

/// `HRESULT`.
pub type Hresult = i32;

/// `S_OK`.
pub const S_OK: Hresult = 0;

/// `S_FALSE`.
pub const S_FALSE: Hresult = 1;

/// `E_POINTER`.
pub const E_POINTER: Hresult = 0x8000_4003_u32 as Hresult;

/// `FACILITY_DIRECTMUSIC`.
const FACILITY_DIRECTMUSIC: u32 = 2168;

/// `DMUS_ERRBASE`: the offset of the DirectMusic error codes.
const DMUS_ERRBASE: u32 = 0x1000;

/// `MAKE_DMHRESULTERROR(code)`: `MAKE_HRESULT(1, FACILITY_DIRECTMUSIC, DMUS_ERRBASE + code)`.
pub const fn make_dmhresult_error(code: u32) -> Hresult {
    (0x8000_0000 | (FACILITY_DIRECTMUSIC << 16) | (DMUS_ERRBASE + code)) as Hresult
}

/// `DMUS_E_SET_UNSUPPORTED`: `MAKE_DMHRESULTERROR(0x0123)`.
pub const DMUS_E_SET_UNSUPPORTED: Hresult = make_dmhresult_error(0x0123);

/// `DMUS_E_GET_UNSUPPORTED`: `MAKE_DMHRESULTERROR(0x0124)`.
pub const DMUS_E_GET_UNSUPPORTED: Hresult = make_dmhresult_error(0x0124);

/// `DMUS_E_NOT_FOUND`: `MAKE_DMHRESULTERROR(0x0161)`.
pub const DMUS_E_NOT_FOUND: Hresult = make_dmhresult_error(0x0161);

/// `DMUS_E_TYPE_UNSUPPORTED`: `MAKE_DMHRESULTERROR(0x0164)`.
pub const DMUS_E_TYPE_UNSUPPORTED: Hresult = make_dmhresult_error(0x0164);

/// A COM `GUID`, laid out as in the Windows header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
