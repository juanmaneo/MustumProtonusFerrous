/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = d3dim700.dll` in `dlls/d3dim700/Makefile.in`.
const D3DIM700_DLL_NAME: &str = "d3dim700.dll";

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

/// `@ stub D3DFree` in `dlls/d3dim700/d3dim700.spec`.
pub fn d3d_free() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "D3DFree")
}

/// `@ stub D3DMalloc` in `dlls/d3dim700/d3dim700.spec`.
pub fn d3d_malloc() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "D3DMalloc")
}

/// `@ stub D3DRealloc` in `dlls/d3dim700/d3dim700.spec`.
pub fn d3d_realloc() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "D3DRealloc")
}

/// `@ stub Direct3DCreateDevice` in `dlls/d3dim700/d3dim700.spec`.
pub fn direct3d_create_device() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "Direct3DCreateDevice")
}

/// `@ stub CreateTexture` in `dlls/d3dim700/d3dim700.spec`.
pub fn create_texture() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "CreateTexture")
}

/// `@ stub D3DBreakVBLock` in `dlls/d3dim700/d3dim700.spec`.
pub fn d3d_break_vb_lock() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "D3DBreakVBLock")
}

/// `@ stub D3DTextureUpdate` in `dlls/d3dim700/d3dim700.spec`.
pub fn d3d_texture_update() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "D3DTextureUpdate")
}

/// `@ stub DestroyTexture` in `dlls/d3dim700/d3dim700.spec`.
pub fn destroy_texture() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "DestroyTexture")
}

/// `@ stub Direct3DCreate` in `dlls/d3dim700/d3dim700.spec`.
pub fn direct3d_create() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "Direct3DCreate")
}

/// `@ stub Direct3D_HALCleanUp` in `dlls/d3dim700/d3dim700.spec`.
pub fn direct3d_hal_clean_up() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "Direct3D_HALCleanUp")
}

/// `@ stub FlushD3DDevices` in `dlls/d3dim700/d3dim700.spec`.
pub fn flush_d3d_devices() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "FlushD3DDevices")
}

/// `@ stub GetLOD` in `dlls/d3dim700/d3dim700.spec`.
pub fn get_lod() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "GetLOD")
}

/// `@ stub GetPriority` in `dlls/d3dim700/d3dim700.spec`.
pub fn get_priority() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "GetPriority")
}

/// `@ stub PaletteAssociateNotify` in `dlls/d3dim700/d3dim700.spec`.
pub fn palette_associate_notify() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "PaletteAssociateNotify")
}

/// `@ stub PaletteUpdateNotify` in `dlls/d3dim700/d3dim700.spec`.
pub fn palette_update_notify() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "PaletteUpdateNotify")
}

/// `@ stub SetLOD` in `dlls/d3dim700/d3dim700.spec`.
pub fn set_lod() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "SetLOD")
}

/// `@ stub SetPriority` in `dlls/d3dim700/d3dim700.spec`.
pub fn set_priority() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "SetPriority")
}

/// `@ stub SurfaceFlipNotify` in `dlls/d3dim700/d3dim700.spec`.
pub fn surface_flip_notify() -> StubException {
    wine_spec_unimplemented_stub(D3DIM700_DLL_NAME, "SurfaceFlipNotify")
}
