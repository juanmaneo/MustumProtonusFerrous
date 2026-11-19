//! Deterministic parts of Wine's `dlls/d3dx10_43`: the pixel and image file format mappings to
//! DXGI and D3DX10 values, the `D3DX10_IMAGE_INFO` derived from a parsed image header, the
//! default-filling of `D3DX10_IMAGE_LOAD_INFO`, the `D3DX10CheckVersion` argument check, the
//! capacity growth of `d3dx_array_reserve` and the thread count rules of the thread pump.
//!
//! Ported:
//! - `dxgi_format_from_d3dx_pixel_format_id` and `dxgi_format_from_legacy_dds_d3dx_pixel_format_id`
//!   from `texture.c`: pixel format id to `DXGI_FORMAT`, for DDS_DXT10 and for legacy DDS files.
//! - `d3dx10_image_file_format_from_d3dx_image_file_format` from `texture.c`. Wine returns
//!   `D3DX10_IFF_FORCE_DWORD` for unmapped formats, which is `None` here.
//! - `d3dx10_image_info_from_d3dx_image` from `texture.c`: image info derivation and its failure
//!   rules.
//! - `init_load_info` from `texture.c`: the default or copied `D3DX10_IMAGE_LOAD_INFO`.
//! - `D3DX10CheckVersion` from `d3dx10_43_main.c`.
//! - `d3dx_array_reserve` from `sprite.c`, as `array_reserve_capacity`. It returns the capacity
//!   that the C would grow to, and leaves the allocation to the caller.
//! - The thread count rules of `D3DX10CreateThreadPump` from `async.c`, with the processor count
//!   as an input in place of `GetSystemInfo`.
//!
//! Skipped, not portable without OS, COM, file or decoder state:
//! - `texture.c`: `get_image_info` and the `D3DX10GetImageInfoFrom*` exports (they read files and
//!   decode headers), `load_texture_data` and `create_d3d_texture` (decoding and D3D10 device
//!   calls), `D3DX10Create*` and `D3DX10Save*`, the trace helpers and the `FIXME` stubs.
//! - `async.c`: the `ID3DX10DataLoader` and `ID3DX10DataProcessor` objects, the thread pump
//!   object (heap allocation and worker threads), and the file and resource loaders.
//! - `sprite.c` and the other COM objects of the DLL.
//!
//! Deviations from the C:
//! - `array_reserve_capacity` clamps the first step to the largest capacity that fits in
//!   `SIZE_MAX` bytes. For elements larger than `SIZE_MAX / 4`, Wine's `max(4, capacity)` exceeds
//!   that, `new_capacity * size` wraps, and the C allocates fewer bytes than the capacity it
//!   reports.
//! - `array_reserve_capacity` returns `None` for a zero element size when it has to grow. The C
//!   divides by zero there.
//!
//! Oracle provenance: the ported functions are copied verbatim from Wine's `dlls/d3dx10_43`
//! sources and `dlls/d3dx9_36/d3dx_helpers.h`, and the enums and structs come from the `include/`
//! headers. The extract compiles under `gcc -fsanitize=address,undefined` against Win32 scalar
//! shims in `target/canon/d3dx10_43/` (not tracked in git).

mod array;
mod format;
mod image;
mod load_info;
mod thread_pump;
mod version;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use array::array_reserve_capacity;
pub use format::{
    dxgi_format_from_d3dx_pixel_format_id, dxgi_format_from_legacy_dds_d3dx_pixel_format_id,
    D3dxPixelFormatId, DxgiFormat,
};
pub use image::{
    d3dx10_image_file_format_from_d3dx_image_file_format, d3dx10_image_info_from_d3dx_image,
    D3dx10ImageFileFormat, D3dxImage, D3dxImageFileFormat, D3dxResourceType, ImageInfo,
    ResourceDimension,
};
pub use load_info::{init_load_info, ImageLoadInfo};
pub use thread_pump::thread_pump_thread_counts;
pub use version::d3dx10_check_version;

/// `HRESULT`.
pub type HResult = i32;

/// `E_FAIL`.
pub const E_FAIL: HResult = 0x8000_4005_u32 as HResult;
