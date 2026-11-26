use crate::stub::{
    d3d_free, d3d_malloc, d3d_realloc, direct3d_create, direct3d_create_device,
    direct3d_create_texture, direct3d_get_sw_rast_zpix_fmts, direct3d_hal_clean_up,
    flush_d3d_devices, flush_d3d_devices2, palette_associate_notify, palette_update_notify,
    surface_flip_notify, StubException,
};

/// The `d3dim.spec` export table, in spec order. Every entry is `@ stub`.
pub const EXPORTS: [&str; 13] = [
    "D3DFree",
    "D3DMalloc",
    "D3DRealloc",
    "Direct3DCreate",
    "Direct3DCreateDevice",
    "Direct3DCreateTexture",
    "Direct3DGetSWRastZPixFmts",
    "Direct3D_HALCleanUp",
    "FlushD3DDevices2",
    "FlushD3DDevices",
    "PaletteAssociateNotify",
    "PaletteUpdateNotify",
    "SurfaceFlipNotify",
];

/// The stub record for the export `name`, or `None` when the spec does not export it.
/// Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    Some(match name {
        "D3DFree" => d3d_free(),
        "D3DMalloc" => d3d_malloc(),
        "D3DRealloc" => d3d_realloc(),
        "Direct3DCreate" => direct3d_create(),
        "Direct3DCreateDevice" => direct3d_create_device(),
        "Direct3DCreateTexture" => direct3d_create_texture(),
        "Direct3DGetSWRastZPixFmts" => direct3d_get_sw_rast_zpix_fmts(),
        "Direct3D_HALCleanUp" => direct3d_hal_clean_up(),
        "FlushD3DDevices2" => flush_d3d_devices2(),
        "FlushD3DDevices" => flush_d3d_devices(),
        "PaletteAssociateNotify" => palette_associate_notify(),
        "PaletteUpdateNotify" => palette_update_notify(),
        "SurfaceFlipNotify" => surface_flip_notify(),
        _ => return None,
    })
}
