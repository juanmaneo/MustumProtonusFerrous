//! Port of the portable logic in Wine's `dlls/d3d10core/d3d10core_main.c`.
//!
//! Ported:
//! - `D3D10CoreCreateDevice` control flow: create through the DXGI path with a one-element
//!   feature level array, then query `ID3D10Device` only if creation succeeded:
//!   [`d3d10_core_create_device`]
//! - `FAILED` and the `HRESULT` values the file returns: [`failed`], [`S_OK`], [`E_FAIL`],
//!   [`E_NOTIMPL`]
//! - `D3D10CoreRegisterLayers`, a stub that returns `E_NOTIMPL`: [`d3d10_core_register_layers`]
//!
//! Skipped:
//! - `LoadLibraryA("d3d11.dll")` and `FreeLibrary`: OS module loading. The C code passes the
//!   handle to `DXGID3D10CreateDevice` without checking it, so the model has no handle argument.
//! - `IUnknown_Release(dxgi_device)`: COM reference counting, no effect on the returned HRESULT.
//! - `factory`, `adapter` and the `device` out-pointer: opaque COM pointers.
//! - Bodies of `DXGID3D10CreateDevice` and `IUnknown_QueryInterface`: they live in d3d11 and
//!   COM, so the caller supplies them as closures.
//! - `TRACE`, `WARN` and `ERR` logging.

mod device;

pub use device::d3d10_core_create_device;

/// `HRESULT` as a signed 32-bit value, so `FAILED` is a sign test.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const E_NOTIMPL: Hresult = 0x8000_4001_u32 as i32;
pub const E_FAIL: Hresult = 0x8000_4005_u32 as i32;

/// `FAILED(hr)` from `winerror.h`.
pub fn failed(hr: Hresult) -> bool {
    hr < 0
}

/// `D3D10CoreRegisterLayers`.
pub fn d3d10_core_register_layers() -> Hresult {
    E_NOTIMPL
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
