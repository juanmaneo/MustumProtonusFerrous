use crate::stub::{
    create_texture, d3d_break_vb_lock, d3d_free, d3d_malloc, d3d_realloc, d3d_texture_update,
    destroy_texture, direct3d_create, direct3d_create_device, direct3d_hal_clean_up,
    flush_d3d_devices, get_lod, get_priority, palette_associate_notify, palette_update_notify,
    set_lod, set_priority, surface_flip_notify, StubException,
};

type StubHelper = fn() -> StubException;

/// The `dlls/d3dim700/d3dim700.spec` export table, in spec order. Every entry is `@ stub`, so each
/// name maps to the helper that returns its `__wine_spec_unimplemented_stub` record.
pub const EXPORTS: [(&str, StubHelper); 18] = [
    ("D3DFree", d3d_free),
    ("D3DMalloc", d3d_malloc),
    ("D3DRealloc", d3d_realloc),
    ("Direct3DCreateDevice", direct3d_create_device),
    ("CreateTexture", create_texture),
    ("D3DBreakVBLock", d3d_break_vb_lock),
    ("D3DTextureUpdate", d3d_texture_update),
    ("DestroyTexture", destroy_texture),
    ("Direct3DCreate", direct3d_create),
    ("Direct3D_HALCleanUp", direct3d_hal_clean_up),
    ("FlushD3DDevices", flush_d3d_devices),
    ("GetLOD", get_lod),
    ("GetPriority", get_priority),
    ("PaletteAssociateNotify", palette_associate_notify),
    ("PaletteUpdateNotify", palette_update_notify),
    ("SetLOD", set_lod),
    ("SetPriority", set_priority),
    ("SurfaceFlipNotify", surface_flip_notify),
];

/// The stub record for `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, stub)| stub())
}
