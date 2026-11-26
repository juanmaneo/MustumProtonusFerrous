//! Oracle-derived test vectors for the ported `d3dim.spec` stubs.
//!
//! Vectors were captured by compiling the verbatim body of `__wine_spec_unimplemented_stub`
//! from `dlls/ntdll/exception.c` with `gcc -fsanitize=address,undefined -g`, under
//! `target/canon/d3dim/stub_harness.c`. The harness called the stub once for each of the 13
//! `@ stub` exports in `dlls/d3dim/d3dim.spec`, with the module name from
//! `dlls/d3dim/Makefile.in`, and three more times with names that are not exports (rows 14-16),
//! to check the pass-through. It printed 16 rows. ASan and UBSan flagged no row, so none is
//! excluded.

use super::*;

#[test]
fn oracle_export_stub_rows() {
    // row 01 D3DFree -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=D3DFree
    assert_eq!(
        d3d_free(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "D3DFree",
        }
    );

    // row 02 D3DMalloc -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=D3DMalloc
    assert_eq!(
        d3d_malloc(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "D3DMalloc",
        }
    );

    // row 03 D3DRealloc -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=D3DRealloc
    assert_eq!(
        d3d_realloc(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "D3DRealloc",
        }
    );

    // row 04 Direct3DCreate -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=Direct3DCreate
    assert_eq!(
        direct3d_create(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "Direct3DCreate",
        }
    );

    // row 05 Direct3DCreateDevice -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=Direct3DCreateDevice
    assert_eq!(
        direct3d_create_device(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "Direct3DCreateDevice",
        }
    );

    // row 06 Direct3DCreateTexture -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=Direct3DCreateTexture
    assert_eq!(
        direct3d_create_texture(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "Direct3DCreateTexture",
        }
    );

    // row 07 Direct3DGetSWRastZPixFmts -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=Direct3DGetSWRastZPixFmts
    assert_eq!(
        direct3d_get_sw_rast_zpix_fmts(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "Direct3DGetSWRastZPixFmts",
        }
    );

    // row 08 Direct3D_HALCleanUp -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=Direct3D_HALCleanUp
    assert_eq!(
        direct3d_hal_clean_up(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "Direct3D_HALCleanUp",
        }
    );

    // row 09 FlushD3DDevices2 -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=FlushD3DDevices2
    assert_eq!(
        flush_d3d_devices2(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "FlushD3DDevices2",
        }
    );

    // row 10 FlushD3DDevices -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=FlushD3DDevices
    assert_eq!(
        flush_d3d_devices(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "FlushD3DDevices",
        }
    );

    // row 11 PaletteAssociateNotify -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=PaletteAssociateNotify
    assert_eq!(
        palette_associate_notify(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "PaletteAssociateNotify",
        }
    );

    // row 12 PaletteUpdateNotify -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=PaletteUpdateNotify
    assert_eq!(
        palette_update_notify(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "PaletteUpdateNotify",
        }
    );

    // row 13 SurfaceFlipNotify -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=SurfaceFlipNotify
    assert_eq!(
        surface_flip_notify(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "SurfaceFlipNotify",
        }
    );
}

#[test]
fn oracle_pass_through_rows() {
    // row 14 p0=d3dim.dll p1="" -> code=0x80000100 flags=0x01 nparams=2 p0=d3dim.dll p1=
    assert_eq!(
        wine_spec_unimplemented_stub("d3dim.dll", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dim.dll",
            function: "",
        }
    );

    // row 15 p0="" p1="" -> code=0x80000100 flags=0x01 nparams=2 p0= p1=
    assert_eq!(
        wine_spec_unimplemented_stub("", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "",
            function: "",
        }
    );

    // row 16 p0=other.dll p1=NotAnExport -> code=0x80000100 flags=0x01 nparams=2 p0=other.dll p1=NotAnExport
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "NotAnExport"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "NotAnExport",
        }
    );
}
