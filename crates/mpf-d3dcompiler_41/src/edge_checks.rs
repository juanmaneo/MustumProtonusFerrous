//! Edge-case tests for the ported d3dcompiler_41 logic.
//!
//! These cover the spec table shape, the lookup misses, and the stub dispatch. None of
//! them come from the oracle. The vectors in `oracle_checks.rs` cover the stub records.

use super::*;

#[test]
fn spec_has_thirteen_entries_in_order() {
    assert_eq!(EXPORTS.len(), 13);
    assert_eq!(EXPORTS[0].name, "D3DAssemble");
    assert_eq!(EXPORTS[12].name, "D3DStripShader");
}

#[test]
fn kinds_match_the_spec() {
    let count = |kind: Kind| EXPORTS.iter().filter(|e| e.kind == kind).count();
    assert_eq!(count(Kind::Stub), 3);
    assert_eq!(count(Kind::StdcallPrivate), 1);
    assert_eq!(count(Kind::Stdcall), 9);
}

#[test]
fn private_export_is_d3d_assemble() {
    assert_eq!(
        export("D3DAssemble"),
        Some(&Export {
            name: "D3DAssemble",
            kind: Kind::StdcallPrivate,
        })
    );
}

#[test]
fn stdcall_export_is_found() {
    assert_eq!(export("D3DCompile").map(|e| e.kind), Some(Kind::Stdcall));
}

#[test]
fn unknown_export_is_none() {
    assert_eq!(export("D3DCompress"), None);
    assert_eq!(stub_exception("D3DCompile"), None);
    assert_eq!(stub_exception(""), None);
}

#[test]
fn every_stub_in_the_table_has_a_record() {
    for entry in EXPORTS.iter().filter(|e| e.kind == Kind::Stub) {
        let rec = stub_exception(entry.name).expect("stub record");
        assert_eq!(rec.function, entry.name);
    }
}

#[test]
fn wine_stub_uses_the_given_names() {
    let rec = wine_spec_unimplemented_stub("mod.dll", "fn");
    assert_eq!(rec.module, "mod.dll");
    assert_eq!(rec.function, "fn");
    assert_eq!(rec.code, EXCEPTION_WINE_STUB);
    assert_eq!(rec.flags, EXCEPTION_NONCONTINUABLE);
}
