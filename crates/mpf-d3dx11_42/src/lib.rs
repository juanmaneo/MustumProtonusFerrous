//! Port of the portable logic in Wine's `dlls/d3dx11_42/d3dx11_42_main.c`.
//!
//! Ported:
//! - `D3DX11CheckVersion`: [`check_version`], with the SDK versions from
//!   `d3d11.h` ([`D3D11_SDK_VERSION`]) and the `-DD3DX11_SDK_VERSION=42`
//!   build define from `Makefile.in` ([`D3DX11_SDK_VERSION`]).
//!
//! Skipped:
//! - The `-import` exports (`D3DX11CompileFromFile*`, `D3DX11CreateTextureFromFile*`,
//!   `D3DX11GetImageInfoFromFile*`, `D3DX11SaveTextureTo*`, and similar): forwarders
//!   to `d3dcompiler` and `d3dx11`, which are other DLLs.
//! - Every `@ stub` export (`D3DX11ComputeNormalMap`, `D3DX11SHProjectCubeMap`,
//!   `D3DX11CreateThreadPump`, and similar): not implemented in Wine.

/// `D3D11_SDK_VERSION` from `d3d11.h`.
pub const D3D11_SDK_VERSION: u32 = 7;

/// `D3DX11_SDK_VERSION`, set by `-DD3DX11_SDK_VERSION=42` in `Makefile.in`.
pub const D3DX11_SDK_VERSION: u32 = 42;

/// `D3DX11CheckVersion`: true only when both SDK versions match exactly.
pub fn check_version(d3d_sdk_ver: u32, d3dx_sdk_ver: u32) -> bool {
    d3d_sdk_ver == D3D11_SDK_VERSION && d3dx_sdk_ver == D3DX11_SDK_VERSION
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
