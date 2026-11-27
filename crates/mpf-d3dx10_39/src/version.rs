/// `D3D10_SDK_VERSION`, from Wine's `include/d3d10.idl`.
const D3D10_SDK_VERSION: u32 = 29;

/// `D3DX10CheckVersion`: checks this D3D10 SDK version and D3DX10 SDK version 39.
pub fn d3dx10_check_version(d3d_sdk_version: u32, d3dx_sdk_version: u32) -> bool {
    d3d_sdk_version == D3D10_SDK_VERSION && d3dx_sdk_version == 39
}
