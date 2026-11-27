/// `D3D10_SDK_VERSION` from `include/d3d10.idl`.
pub const D3D10_SDK_VERSION: u32 = 29;

/// The D3DX10 SDK version that `D3DX10CheckVersion` compares against. The C body uses the
/// literal 41, not the `-DD3DX10_SDK_VERSION=41` define from `Makefile.in`.
pub const D3DX10_SDK_VERSION: u32 = 41;

/// `D3DX10CheckVersion`: true only when both SDK versions match exactly.
pub fn check_version(d3d_sdk_ver: u32, d3dx_sdk_ver: u32) -> bool {
    d3d_sdk_ver == D3D10_SDK_VERSION && d3dx_sdk_ver == D3DX10_SDK_VERSION
}
