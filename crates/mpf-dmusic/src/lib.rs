//! dmusic subset for MPF: the value logic of `dlls/dmusic` for MIDI locale and patch conversion,
//! the event buffer that DirectMusic packs events into, and the SoundFont modulator and generator
//! parsing in `instrument.c`.
//!
//! Reference Wine's `dlls/dmusic` implements the DirectMusic COM objects. Most of it works on
//! interfaces, segments, tracks, ports, streams and the registry. This crate ports the pure
//! functions that `dmusic_main.c`, `buffer.c` and `instrument.c` apply to their inputs. It works on
//! integers, slices and vectors, with no COM objects, threads, handles or I/O.
//!
//! Ported (pure logic, verified against the verbatim C by oracle rows and edge tests):
//! - `patch`: `MIDILOCALE2Patch` and `Patch2MIDILOCALE`, which pack a bank, a patch number and the
//!   drum bit.
//! - `buffer`: `DMUS_EVENT_SIZE`, the buffer size rounding of `create_format_and_size`,
//!   `PackStructured`, `PackUnstructured`, `SetUsedBytes`, `GetStartTime`, `Flush` and the getters
//!   of `IDirectMusicBufferImpl`.
//! - `soundfont`: `mod_src_to_conn_src` and `mod_src_to_conn_transform`, which map SoundFont
//!   modulator sources to DLS connection values, and `parse_modulators` and
//!   `parse_generators`, which collect the modulators and generators of one bag.
//! - The crate root holds the `HRESULT` codes, the `GUID` type and the two GUIDs that `buffer` uses.
//!
//! Deviations from the C:
//! - Event positions and sizes are computed in 64 bits. The C computes `new_write_pos` in a `DWORD`,
//!   so a large length or a write position near 2^32 can wrap and pass the fullness check. This
//!   crate returns `DMUS_E_BUFFER_FULL` for such an event instead.
//! - `rtDelta` is `ref_time - start_time`, which overflows in C when the two differ by more than
//!   `INT64_MAX`. This crate wraps, which gives the two's-complement difference.
//! - Bytes that no event writes are zero. Wine allocates the buffer data with `malloc`, so those
//!   bytes are unspecified there.
//! - Generators with an operator of `SF_GEN_END_OPER` or more are ignored. The C writes past the end
//!   of the generator table for them.
//! - Bag and table indices are not checked. A malformed bag table panics here, where the C reads
//!   out of bounds. A bag range whose end precedes its start also panics, where the C runs no
//!   iterations.
//! - `add_modulator` returns early when `realloc` fails. A `Vec` aborts on allocation failure.
//! - The `WARN` and `TRACE` messages in the C are not ported.
//!
//! Skipped (reason per item):
//! - The COM objects, vtables and class factories of `dmusic.c`, `dmusic_main.c`, `band.c`,
//!   `bandtrack.c`, `clock.c`, `collection.c`, `dmobject.c`, `download.c`, `port.c` and `wave.c`,
//!   along with the `IUnknown` methods of `buffer.c`: they need reference counts, handles, threads
//!   and `DllMain`.
//! - `IDirectMusicBufferImpl_Create` and `IDirectMusicBufferImpl_Release`: they need `calloc`,
//!   `malloc` and COM lifetime.
//! - SoundFont and collection loading in `instrument.c` and `collection.c`: they read RIFF chunks
//!   from `IStream`, so they cannot run without that input.
//! - The `debugstr_*` helpers: they feed only `WARN` and `TRACE`, which compile to nothing here.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub mod buffer;
pub mod patch;
pub mod soundfont;

/// `HRESULT`.
pub type Hresult = i32;

/// `FACILITY_DIRECTMUSIC`.
const FACILITY_DIRECTMUSIC: u32 = 2168;

/// `DMUS_ERRBASE`: the offset of the DirectMusic error codes.
const DMUS_ERRBASE: u32 = 0x1000;

/// `MAKE_DMHRESULTERROR(code)`: `MAKE_HRESULT(1, FACILITY_DIRECTMUSIC, DMUS_ERRBASE + code)`.
pub const fn make_dmhresult_error(code: u32) -> Hresult {
    (0x8000_0000 | (FACILITY_DIRECTMUSIC << 16) | (DMUS_ERRBASE + code)) as Hresult
}

/// `DMUS_E_BUFFER_EMPTY`: `MAKE_DMHRESULTERROR(0x0135)`.
pub const DMUS_E_BUFFER_EMPTY: Hresult = make_dmhresult_error(0x0135);

/// `DMUS_E_BUFFER_FULL`: `MAKE_DMHRESULTERROR(0x0136)`.
pub const DMUS_E_BUFFER_FULL: Hresult = make_dmhresult_error(0x0136);

/// `DMUS_E_INVALID_EVENT`: `MAKE_DMHRESULTERROR(0x013F)`.
pub const DMUS_E_INVALID_EVENT: Hresult = make_dmhresult_error(0x013F);

/// A COM `GUID`, laid out as in the Windows header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `GUID_NULL`.
pub const GUID_NULL: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// `KSDATAFORMAT_SUBTYPE_MIDI`: the format of a buffer created with `GUID_NULL`.
pub const KSDATAFORMAT_SUBTYPE_MIDI: Guid = Guid {
    data1: 0x1d26_2760,
    data2: 0xe957,
    data3: 0x11cf,
    data4: [0xa5, 0xd6, 0x28, 0xdb, 0x04, 0xc1, 0x00, 0x00],
};
