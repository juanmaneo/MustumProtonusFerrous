//! Edge-case tests for the d3dim700 export table and stub records.
//!
//! The DLL has no code to exercise beyond the stub records, so these tests check the table, the
//! lookup, and the record shape. Not every expectation is oracle-derived; the ones that aren't say
//! so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of d3dim700.spec. The code and
    // flags are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 18 `@ stub` lines of dlls/d3dim700/d3dim700.spec, in order.
    const SPEC_NAMES: [&str; 18] = [
        "D3DFree",
        "D3DMalloc",
        "D3DRealloc",
        "Direct3DCreateDevice",
        "CreateTexture",
        "D3DBreakVBLock",
        "D3DTextureUpdate",
        "DestroyTexture",
        "Direct3DCreate",
        "Direct3D_HALCleanUp",
        "FlushD3DDevices",
        "GetLOD",
        "GetPriority",
        "PaletteAssociateNotify",
        "PaletteUpdateNotify",
        "SetLOD",
        "SetPriority",
        "SurfaceFlipNotify",
    ];
    assert_eq!(EXPORTS.map(|(name, _)| name), SPEC_NAMES);
}

#[test]
fn export_stub_rejects_names_the_spec_does_not_export() {
    // Not oracle-derived: lookups that miss the table. Names are case-sensitive, and an empty
    // name or a trailing space is not an export.
    assert_eq!(export_stub("d3dfree"), None);
    assert_eq!(export_stub("D3DFree "), None);
    assert_eq!(export_stub(""), None);
    assert_eq!(export_stub("Direct3DCreate9"), None);
}

#[test]
fn export_stub_returns_a_record_for_every_export() {
    // Not oracle-derived: every table entry gives the stub record, never a plain value. The
    // function name in the record is the table name, and the module is the spec's DLL.
    for (name, _) in EXPORTS {
        let record = export_stub(name).expect("every table entry is an export");
        assert_eq!(record.function, name);
        assert_eq!(record.module, "d3dim700.dll");
        assert_eq!(record.code, EXCEPTION_WINE_STUB);
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(record.number_parameters, 2);
    }
}
