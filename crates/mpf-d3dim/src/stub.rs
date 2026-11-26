/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = d3dim.dll` in `dlls/d3dim/Makefile.in`.
const D3DIM_DLL_NAME: &str = "d3dim.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `D3DFree`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn d3d_free() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "D3DFree")
}

/// `D3DMalloc`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn d3d_malloc() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "D3DMalloc")
}

/// `D3DRealloc`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn d3d_realloc() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "D3DRealloc")
}

/// `Direct3DCreate`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn direct3d_create() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "Direct3DCreate")
}

/// `Direct3DCreateDevice`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn direct3d_create_device() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "Direct3DCreateDevice")
}

/// `Direct3DCreateTexture`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn direct3d_create_texture() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "Direct3DCreateTexture")
}

/// `Direct3DGetSWRastZPixFmts`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn direct3d_get_sw_rast_zpix_fmts() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "Direct3DGetSWRastZPixFmts")
}

/// `Direct3D_HALCleanUp`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn direct3d_hal_clean_up() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "Direct3D_HALCleanUp")
}

/// `FlushD3DDevices2`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn flush_d3d_devices2() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "FlushD3DDevices2")
}

/// `FlushD3DDevices`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn flush_d3d_devices() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "FlushD3DDevices")
}

/// `PaletteAssociateNotify`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn palette_associate_notify() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "PaletteAssociateNotify")
}

/// `PaletteUpdateNotify`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn palette_update_notify() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "PaletteUpdateNotify")
}

/// `SurfaceFlipNotify`, exported as `@ stub` in `dlls/d3dim/d3dim.spec`.
pub fn surface_flip_notify() -> StubException {
    wine_spec_unimplemented_stub(D3DIM_DLL_NAME, "SurfaceFlipNotify")
}
