//! Edge-case tests for the ported `d3dim.spec` logic.
//!
//! Not every expectation is oracle-derived. The ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the thirteen `@ stub` lines of dlls/d3dim/d3dim.spec, in order.
    assert_eq!(
        EXPORTS,
        [
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
        ]
    );
}

#[test]
fn every_export_has_a_stub_record_for_itself() {
    // Not oracle-derived: each name in the table dispatches to a record whose function is that
    // name, with the d3dim.dll module and the same code, flags and parameter count as the C stub.
    for name in EXPORTS {
        let stub = export_stub(name).expect("every name in EXPORTS is a stub export");
        assert_eq!(stub.module, "d3dim.dll");
        assert_eq!(stub.function, name);
        assert_eq!(stub.code, EXCEPTION_WINE_STUB);
        assert_eq!(stub.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(stub.number_parameters, 2);
    }
}

#[test]
fn export_stub_rejects_names_outside_the_spec() {
    // Not oracle-derived: lookups are exact and case-sensitive. Each name below is close to an
    // export but is not one, so each one must take the `None` arm.
    for name in [
        "",
        "d3dfree",
        "D3DFree ",
        "D3DMalloc2",
        "Direct3DCreate9",
        "Direct3D_HALCleanup",
    ] {
        assert_eq!(export_stub(name), None);
    }
}
