//! Edge-case tests for the ported d3d12core stub model.
//!
//! These cover the export table, its lookup, the helper records and the constants. None of the
//! expectations here come from the oracle; each test says so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the two lines of dlls/d3d12core/d3d12core.spec, in order.
    assert_eq!(EXPORTS, ["D3D12GetInterface", "D3D12SDKVersion"]);
}

#[test]
fn every_export_raises_its_own_stub() {
    // Not oracle-derived: each spec entry resolves to the helper that names it.
    assert_eq!(
        export_stub("D3D12GetInterface"),
        Some(d3d12_get_interface())
    );
    assert_eq!(export_stub("D3D12SDKVersion"), Some(d3d12_sdk_version()));
    for name in EXPORTS {
        let record = export_stub(name).expect("every spec export has a stub record");
        assert_eq!(record.module, "d3d12core.dll");
        assert_eq!(record.function, name);
    }
}

#[test]
fn unexported_names_have_no_stub() {
    // Not oracle-derived: names the spec does not export, including near misses. Case, a
    // trailing space and the empty name must not match.
    for name in [
        "",
        "d3d12getinterface",
        "D3D12GetInterface ",
        "D3D12CreateDevice",
        "d3d12core.dll",
    ] {
        assert_eq!(export_stub(name), None, "{name:?}");
    }
}

#[test]
fn helpers_differ_only_in_function_name() {
    // Not oracle-derived: both helpers share the module, code, flags and parameter count. The
    // oracle confirms the same values for both exports (rows 01 and 02).
    let get = d3d12_get_interface();
    let version = d3d12_sdk_version();
    assert_eq!(get.module, version.module);
    assert_eq!(get.code, version.code);
    assert_eq!(get.flags, version.flags);
    assert_eq!(get.number_parameters, version.number_parameters);
    assert_ne!(get.function, version.function);
}
