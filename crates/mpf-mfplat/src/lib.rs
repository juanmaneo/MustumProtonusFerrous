//! mfplat subset for MPF: the value logic of `dlls/mfplat` for frame rates, uncompressed video
//! formats, DXVA colour packing and GUID strings.
//!
//! Reference Wine's `dlls/mfplat` implements the Media Foundation runtime. Most of it works on
//! attribute stores, media types, samples, queues, buffers and the registry. This crate ports the
//! pure functions that `mediatype.c` and `main.c` apply to their inputs. It works on integers,
//! slices and fixed tables, with no COM objects, allocation, threads or registry access.
//!
//! Ported (pure logic, verified against the verbatim C by oracle rows and edge tests):
//! - `frame_rate`: `MFFrameRateToAverageTimePerFrame` and `MFAverageTimePerFrameToFrameRate`, with
//!   the known-rate table, the 30-unit tolerance, the clamp and the gcd reduction.
//! - `video_format`: the uncompressed video format table, `MFGetStrideForBitmapInfoHeader`,
//!   `MFCalculateImageSize` and `MFGetPlaneSize`. The C looks subtypes up with `bsearch` over a
//!   sorted table. This crate searches the table linearly, which finds the same row because the
//!   subtypes are distinct.
//! - `dxva`: `MFConvertColorInfoToDXVA`, the seven-field bit pack.
//! - `guid`: `GUIDFromString`, the parser for `XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX`.
//! - The crate root holds the `HRESULT` codes and the `DEFINE_*` GUID bases that the format table and
//!   its tests share.
//!
//! Deviations from the C:
//! - Stride and size arithmetic wraps at 32 bits, as the C `unsigned int` arithmetic does. Wine
//!   returns `S_OK` with the wrapped value, and so does `image_size` here. A caller that allocates
//!   from the result must check for overflow itself.
//! - `stride_for_bitmap_info_header` returns `Err(MF_E_INVALIDMEDIATYPE)` where the C also sets
//!   `*stride = 0`, which the `Result` makes unobservable.
//! - The C negates a bottom-up stride with `*stride *= -1`, which is undefined for `INT_MIN`. This
//!   crate uses `wrapping_neg`, which leaves `INT_MIN` unchanged.
//! - `parse_guid` returns `None` where the C returns `FALSE`. The C may already have written the
//!   parsed parts into its output `GUID`, and that partial state is not exposed.
//! - `parse_guid` reads a position past the end of the slice as NUL, as the C reads a NUL-terminated
//!   string. A slice of exactly 36 units is therefore a complete GUID.
//!
//! Skipped (reason per item):
//! - `MFCopyImage` (`main.c`): copies rows through raw pointers and strides with no bounds, so a
//!   safe port needs an out-of-bounds policy that Wine does not define.
//! - `buffer_get_aligned_length` (`buffer.c`): divides by the alignment, which is a division by zero
//!   in C for an alignment of zero.
//! - `guid_to_string` (`main.c`): formats through `swprintf` into a caller buffer, and feeds only the
//!   registry and debug output.
//! - `get_am_subtype_for_mf_subtype` (`mediatype.c`): an if-chain of about 25 GUID mappings. It is
//!   deterministic, but it needs the `MEDIASUBTYPE_*` GUIDs from `uuids.h`, which this pass does not
//!   extract.
//! - Attribute stores, media type creation and comparison, samples, buffers, queues, transform
//!   activation, the registry, COM vtables, class factories and `DllMain`: they need COM objects,
//!   allocation, threads, handles or the registry.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub mod dxva;
pub mod frame_rate;
pub mod guid;
pub mod video_format;

use crate::guid::Guid;

/// `HRESULT`.
pub type Hresult = i32;

/// `E_INVALIDARG`.
pub const E_INVALIDARG: Hresult = 0x8007_0057u32 as i32;

/// `MF_E_INVALIDMEDIATYPE`.
pub const MF_E_INVALIDMEDIATYPE: Hresult = 0xC00D_36B4u32 as i32;

/// The `MFVideoFormat_*` GUID of `data1`, a FourCC or `D3DFMT_*` value: the `DEFINE_MEDIATYPE_GUID`
/// base with `data1` replaced.
pub const fn video_format_guid(data1: u32) -> Guid {
    Guid {
        data1,
        data2: 0x0000,
        data3: 0x0010,
        data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
    }
}

/// The `MEDIASUBTYPE_RGB*` GUID of `data1`: the DirectShow base with `data1` replaced.
pub const fn media_subtype_guid(data1: u32) -> Guid {
    Guid {
        data1,
        data2: 0x524f,
        data3: 0x11ce,
        data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
    }
}
