//! dxva2 subset for MPF: the deterministic logic of Wine's `dlls/dxva2/main.c`.
//!
//! The C file is a COM implementation whose objects, handles, locks and surfaces are backed by
//! `IDirect3DDevice9`. The logic that does not touch those objects runs here on plain values,
//! and follows the C control flow. Functions that return an `HRESULT` in C return
//! `Result<_, HRESULT>` here, and `Ok(())` stands for `S_OK`.
//!
//! Ported (names in `dlls/dxva2/main.c`; oracle rows in the `oracle_checks` tests):
//! - `intersect_rect` and `IsRectEmpty`: [`intersect_rect`], [`is_rect_empty`].
//! - `video_processor_get_background_color`: [`background_color`].
//! - `video_processor_VideoProcessBlt`, without the surface and device calls: [`plan_blt`].
//! - `dxva_is_supported_stream_format` and the format tables of the processor service:
//!   [`device_guids`], [`render_target_formats`].
//! - The fixed `GetVideoProcessorCaps` replies: [`service_caps`], [`processor_caps`].
//! - `device_manager_processor_service_CreateSurface`: [`surface_count`], [`create_surfaces`].
//! - The substream limit in `CreateVideoProcessor`: [`check_substreams`].
//! - The handle table of `ResetDevice`, `OpenDeviceHandle`, `CloseDeviceHandle`, `TestDevice`
//!   and the handle check in `GetVideoService`: [`DeviceHandles`].
//!
//! Divergences from the C bodies:
//! - The C `(BYTE)` conversions in `video_processor_get_background_color` are undefined when the
//!   value is out of range, and the sanitizer flags them. The port uses Rust's `as u8`, which
//!   saturates. The oracle excludes the vectors that reach that path.
//! - `dxva_array_reserve` is replaced by `Vec`. Its allocation-failure path is not modelled.
//! - `OpenDeviceHandle` sets the open flag of the first closed slot, then appends a new slot and
//!   returns the new handle. The port keeps this as it is in C.
//!
//! Skipped:
//! - `LockDevice` and `UnlockDevice`: they wait on a condition variable and a critical section,
//!   and `UnlockDevice` creates a state block through the device.
//! - The COM plumbing: `QueryInterface`, `AddRef`, `Release`, the vtables, and the `calloc` of
//!   the processor object.
//! - The decoder service, `GetProcAmpRange`, `GetFilterPropertyRange`,
//!   `RegisterVideoProcessorSoftwareDevice` and `GetVideoProcessorSubStreamFormats`: each only
//!   logs `FIXME` and returns `E_NOTIMPL`.
//! - The monitor, OPM and DXVAHD exports, and `DXVA2CreateDirect3DDeviceManager9` and
//!   `DXVA2CreateVideoService`, which need a live device.

mod blt;
mod color;
mod geometry;
mod handles;
mod service;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use blt::{plan_blt, BltPlan, Sample, Stretch};
pub use color::{background_color, Ayuv16};
pub use geometry::{intersect_rect, is_rect_empty, Rect};
pub use handles::{DeviceHandles, Handle};
pub use service::{
    create_surfaces, device_guids, fourcc, processor_caps, render_target_formats, service_caps,
    surface_count, D3dFormat, Device, ProcessorCaps, A8R8G8B8, NV12, X8R8G8B8, YUY2,
};

/// A Win32 `HRESULT`.
pub type HRESULT = i32;

/// `E_FAIL`.
pub const E_FAIL: HRESULT = 0x8000_4005u32 as i32;
/// `E_NOTIMPL`.
pub const E_NOTIMPL: HRESULT = 0x8000_4001u32 as i32;
/// `E_HANDLE`.
pub const E_HANDLE: HRESULT = 0x8007_0006u32 as i32;
/// `E_INVALIDARG`.
pub const E_INVALIDARG: HRESULT = 0x8007_0057u32 as i32;
/// `DXVA2_E_NOT_INITIALIZED`.
pub const DXVA2_E_NOT_INITIALIZED: HRESULT = 0x8004_1000u32 as i32;
/// `DXVA2_E_NEW_VIDEO_DEVICE`.
pub const DXVA2_E_NEW_VIDEO_DEVICE: HRESULT = 0x8004_1001u32 as i32;

/// The substream limit in `CreateVideoProcessor`. The C body does not check the device GUID,
/// so neither does this check.
pub fn check_substreams(max_substreams: u32) -> Result<(), HRESULT> {
    if max_substreams >= 16 {
        Err(E_INVALIDARG)
    } else {
        Ok(())
    }
}
