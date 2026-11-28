/// `D3D10_SDK_VERSION` from `include/d3d10.idl`.
const D3D10_SDK_VERSION: u32 = 29;

/// The `D3DX10_SDK_VERSION` that `dlls/d3dx10_33/d3dx10_33_main.c` compares against.
const D3DX10_SDK_VERSION: u32 = 33;

/// `D3DX10CheckVersion` from `dlls/d3dx10_33/d3dx10_33_main.c`.
pub fn d3dx10_check_version(d3dsdkvers: u32, d3dxsdkvers: u32) -> bool {
    d3dsdkvers == D3D10_SDK_VERSION && d3dxsdkvers == D3DX10_SDK_VERSION
}
