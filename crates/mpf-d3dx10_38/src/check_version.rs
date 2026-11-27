/// `D3D10_SDK_VERSION`, from `include/d3d10.idl`.
const D3D10_SDK_VERSION: u32 = 29;

/// `D3DX10CheckVersion` from `dlls/d3dx10_38/d3dx10_38_main.c`.
///
/// The function succeeds only when both the D3D SDK version and this DLL's D3DX SDK version
/// match the versions it was built against.
pub fn d3dx10_check_version(d3d_sdk_version: u32, d3dx_sdk_version: u32) -> bool {
    d3d_sdk_version == D3D10_SDK_VERSION && d3dx_sdk_version == 38
}
