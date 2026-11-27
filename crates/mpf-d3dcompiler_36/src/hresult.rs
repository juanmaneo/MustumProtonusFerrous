/// `S_OK` from `include/winerror.h`.
pub const S_OK: u32 = 0x0000_0000;
/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: u32 = 0x8000_4001;
/// `E_FAIL` from `include/winerror.h`.
pub const E_FAIL: u32 = 0x8000_4005;
/// `E_OUTOFMEMORY` from `include/winerror.h`.
pub const E_OUTOFMEMORY: u32 = 0x8007_000E;
/// `E_INVALIDARG` from `include/winerror.h`.
pub const E_INVALIDARG: u32 = 0x8007_0057;
/// `D3DERR_INVALIDCALL`: `MAKE_D3DHRESULT(2156)` from `include/d3d9.h`.
pub const D3DERR_INVALIDCALL: u32 = 0x8876_086C;

/// `VKD3D_OK` from `libs/vkd3d/include/vkd3d_types.h`.
pub const VKD3D_OK: i32 = 0;
/// `VKD3D_ERROR` from `libs/vkd3d/include/vkd3d_types.h`.
pub const VKD3D_ERROR: i32 = -1;
/// `VKD3D_ERROR_OUT_OF_MEMORY` from `libs/vkd3d/include/vkd3d_types.h`.
pub const VKD3D_ERROR_OUT_OF_MEMORY: i32 = -2;
/// `VKD3D_ERROR_INVALID_ARGUMENT` from `libs/vkd3d/include/vkd3d_types.h`.
pub const VKD3D_ERROR_INVALID_ARGUMENT: i32 = -3;
/// `VKD3D_ERROR_INVALID_SHADER` from `libs/vkd3d/include/vkd3d_types.h`.
pub const VKD3D_ERROR_INVALID_SHADER: i32 = -4;
/// `VKD3D_ERROR_NOT_IMPLEMENTED` from `libs/vkd3d/include/vkd3d_types.h`.
pub const VKD3D_ERROR_NOT_IMPLEMENTED: i32 = -5;

/// `hresult_from_vkd3d_result` from `dlls/d3dcompiler_43/compiler.c`.
///
/// `VKD3D_ERROR_INVALID_SHADER` falls through to `E_FAIL`. Any other code, which the C
/// `default` arm reports with `FIXME`, also gives `E_FAIL`.
pub fn hresult_from_vkd3d_result(vkd3d_result: i32) -> u32 {
    match vkd3d_result {
        VKD3D_OK => S_OK,
        VKD3D_ERROR | VKD3D_ERROR_INVALID_SHADER => E_FAIL,
        VKD3D_ERROR_OUT_OF_MEMORY => E_OUTOFMEMORY,
        VKD3D_ERROR_INVALID_ARGUMENT => E_INVALIDARG,
        VKD3D_ERROR_NOT_IMPLEMENTED => E_NOTIMPL,
        _ => E_FAIL,
    }
}
