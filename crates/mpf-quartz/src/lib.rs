//! Headless port of the deterministic parts of Wine's `dlls/quartz`, the DirectShow runtime.
//!
//! The C reference is a DirectShow filter library. Its filter graph, pins, COM objects, threads,
//! DirectSound, VMR and AVI calls need a live process. The helpers below run here as pure functions
//! over values. The integrator supplies each buffer, media type field and time as a value.
//!
//! Ported (names in `dlls/quartz`):
//! - `array_reserve` in `main.c`: [array::reserve_plan].
//! - `copy_plane` and `adjust_surface_size` in `vmr9.c`: [plane::copy_plane],
//!   [surface::adjust_surface_size].
//! - `get_bitmap_info_size`, `AVIDec_DropSample`, `is_nontrivial_rect`, `needs_decompressex` and
//!   the `biSizeImage` formula in `avidec.c`: [avi::bitmap_info_size], [avi::drop_sample],
//!   [avi::is_nontrivial_rect], [avi::needs_decompressex], [size_image].
//! - `time_from_pos` and `pos_from_time` in `dsoundrender.c`: [dsound::time_from_pos],
//!   [dsound::pos_from_time].
//! - `MatchTypes` in `filtermapper.c`: [pin_types::match_types].
//!
//! Skipped: `DllMain`, the registry and class-factory code in `main.c`; the COM interfaces, the
//! filter and pin callbacks, the threads and the DirectSound buffers; the VMR surface and
//! presenter calls; the AVI codec and `ICDecompressEx` calls; the file and I/O code of
//! `filesource.c`; the colour converter's libswscale calls; and all `TRACE`, `FIXME`, `WARN` and
//! `ERR` logging.
//!
//! Deviations from Wine, where Wine has undefined behaviour:
//! - [array::reserve_plan] returns `Fail` for a zero element size, where C divides by zero, and
//!   when the byte count overflows, where C wraps it.
//! - [plane::copy_plane] returns `None` when a buffer is too short, when the row count exceeds
//!   `dst_height`, or when a length overflows. C writes past the surface in those cases. It also
//!   returns `None` for a height of `i32::MIN`, where C's `-src_height` overflows.
//! - [surface::adjust_surface_size] returns `None` when the power of two above a dimension does
//!   not fit in 32 bits. C loops forever there.
//! - [size_image] and [avi::is_nontrivial_rect] wrap signed overflow, and `abs` of
//!   `i32::MIN` stays `i32::MIN`, as the C `int` code does in practice.
//! - [dsound::time_from_pos] and [dsound::pos_from_time] return `None` for a zero rate or
//!   alignment, and for a product that overflows. C divides by zero or overflows there.

/// A 128-bit identifier, stored in the byte order of a `GUID`.
pub type Guid = [u8; 16];

/// `GUID_NULL`.
pub const GUID_NULL: Guid = [0; 16];

/// The fields of `RECT` that the ported functions read.
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// The fields of `BITMAPINFOHEADER` that the ported functions read.
pub struct BitmapInfoHeader {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub compression: u32,
    pub clr_used: u32,
}

pub mod array;
pub mod avi;
pub mod dsound;
pub mod pin_types;
pub mod plane;
pub mod surface;

/// The `biSizeImage` formula of `avi_decompressor_source_get_media_type`. Each row is padded to
/// four bytes. A negative height gives a negative size, which is returned as the `DWORD` it is.
pub fn size_image(width: i32, height: i32, bit_count: u16) -> u32 {
    let row = (width.wrapping_mul(i32::from(bit_count)).wrapping_add(31) / 8) & !3;
    height.wrapping_mul(row) as u32
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
