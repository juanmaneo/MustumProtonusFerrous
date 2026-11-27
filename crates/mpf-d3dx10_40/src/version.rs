/// `D3D10_SDK_VERSION`, from Wine's `include/d3d10.idl`.
const D3D10_SDK_VERSION: u32 = 29;

/// `D3DX10CheckVersion`: true only for the D3D10 SDK and D3DX10 SDK versions used by this DLL.
pub fn d3dx10_check_version(d3d_sdk_version: u32, d3dx_sdk_version: u32) -> bool {
    d3d_sdk_version == D3D10_SDK_VERSION && d3dx_sdk_version == 40
}
