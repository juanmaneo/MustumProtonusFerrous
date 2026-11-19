//! Headless port of the deterministic logic in Wine's `dlls/windowscodecs`, the Windows Imaging
//! Component.
//!
//! The C reference is a set of COM objects that read through `IStream` and report `HRESULT`s.
//! The logic that touches no COM runs here on byte slices and plain values, and follows the C
//! control flow, so its results match Wine.
//!
//! Ported (names in the Wine sources):
//! - The checks `DdsDecoder_Initialize` makes on the file header: [parse_header].
//! - `get_dds_info`: [dds_info]. `get_dimension`: [dimension]. `get_alpha_mode_from_fourcc`:
//!   [alpha_mode_from_fourcc].
//! - `get_frame_count`: [frame_count]. `get_frame_dds_index`: [frame_dds_index].
//! - The `dds_format_table` lookup of `get_dds_format`: [dds_info]. `dxgi_format_to_wic_format`:
//!   [dxgi_to_wic].
//! - `get_bytes_per_block_from_format`: [bytes_per_block_from_format]. `is_compressed`:
//!   [is_compressed]. `rgb565_to_argb`: [rgb565_to_argb].
//! - `PaletteImpl_InitializePredefined`: [predefined_palette] and [PaletteType::from_wic]. Wine
//!   returns `E_INVALIDARG` for every type outside the eleven predefined ones, including Custom
//!   and MedianCut, and the port does the same.
//! - `float_16_to_32` and `rgb_to_palette_index` in `converter.c`: [float_16_to_32] and
//!   [rgb_to_palette_index]. Half-float exponent 31 decodes like any other exponent, as in Wine,
//!   so no infinity or NaN is produced.
//! - `copy_palette` and `copy_interlaced_pixels` in `gifformat.c`: [copy_palette] and
//!   [copy_interlaced_pixels].
//!
//! Skipped: the COM objects, vtables and factories; stream reads and decoder state; frame
//! decoding; `decode_block`, whose block reads are unaligned and flagged by UBSan; the flip,
//! rotate, scale, clip and bitmap converters; colour contexts; thumbnails; metadata readers; the
//! DDS encoder; and `median_cut`, which is portable but was not ported in this round.
//!
//! Deviations from Wine, where Wine reads past its input, wraps or divides by zero:
//! - [copy_palette] leaves colour-map entries past the end of the map unchanged. Without a map it
//!   writes only the entries below `count`, where Wine writes entries 0 and 1 regardless. A
//!   transparency index past the end of `colors` is ignored. Writes stop at `colors.len()`.
//! - [copy_interlaced_pixels] rejects a negative `Width` or `Height` with `E_INVALIDARG`. Wine
//!   rejects only a negative `X` or `Y`, and then reads past its source. Here every source row is
//!   bounds-checked before the first write. Sizes are exact `u64` arithmetic, so a zero `Height`
//!   returns `S_OK`, where Wine's `UINT` arithmetic wraps and fails.
//! - [frame_dds_index] returns `None` for a zero array size, a zero per-texture frame count or a
//!   zero depth. Wine divides by zero or loops forever on those.
//! - [dds_info] reports BGRA for every compressed DX10 format. Wine's premultiplied branch cannot
//!   be reached, because the DX10 alpha bit is 0 or 8, and `WICDdsAlphaModePremultiplied` is 2.

mod convert;
mod dds;
mod dxgi;
mod gif;
mod palette;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use convert::{float_16_to_32, rgb_to_palette_index};
pub use dds::*;
pub use dxgi::*;
pub use gif::{copy_interlaced_pixels, copy_palette, ExtensionBlock, Rgb, GRAPHICS_EXT_FUNC_CODE};
pub use palette::{predefined_palette, PaletteType};

/// An `HRESULT`.
pub type HResult = u32;

/// `S_OK`.
pub const S_OK: HResult = 0;
/// `E_INVALIDARG`.
pub const E_INVALIDARG: HResult = 0x8007_0057;
/// `WINCODEC_ERR_UNKNOWNIMAGEFORMAT`.
pub const WINCODEC_ERR_UNKNOWNIMAGEFORMAT: HResult = 0x8898_2F07;
/// `WINCODEC_ERR_BADHEADER`.
pub const WINCODEC_ERR_BADHEADER: HResult = 0x8898_2F61;
/// `WINCODEC_ERR_STREAMREAD`.
pub const WINCODEC_ERR_STREAMREAD: HResult = 0x8898_2F72;

/// A `WICRect`: an origin and a size, in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WicRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl WicRect {
    /// The origin and size as unsigned values, or `E_INVALIDARG` if any field is negative.
    pub(crate) fn unsigned(self) -> Result<[u64; 4], HResult> {
        let fields = [self.x, self.y, self.width, self.height];
        if fields.iter().any(|&field| field < 0) {
            return Err(E_INVALIDARG);
        }
        Ok(fields.map(|field| u64::from(field.unsigned_abs())))
    }
}
