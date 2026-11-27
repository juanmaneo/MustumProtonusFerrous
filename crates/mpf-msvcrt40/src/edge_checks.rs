//! Edge-case tests for the ported msvcrt40 export table and stub records.
//!
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h, include/winnt.h and the MODULE line of
    // dlls/msvcrt40/Makefile.in.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(MSVCRT40_DLL_NAME, "msvcrt40.dll");
}

#[test]
fn stub_record_passes_module_and_function_through() {
    // Not oracle-derived: the C function stores its two arguments unchanged as parameters 0 and 1.
    // The code, flags and parameter count are the ones the oracle rows show.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Function"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Function",
        }
    );
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: a loader resolves a name to one entry, so a repeated name would hide one.
    let mut names: Vec<&str> = EXPORTS.iter().map(|export| export.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn lookup_is_case_sensitive_and_exact() {
    // Not oracle-derived: the lookup is an exact string match, so near misses and the empty name
    // return None.
    assert!(find_export("atexit").is_some());
    for name in [
        "ATEXIT",
        "Atexit",
        "atexit ",
        " atexit",
        "msvcrt.atexit",
        "",
    ] {
        assert_eq!(find_export(name), None, "{name:?}");
    }
}

#[test]
fn forward_is_none_for_stubs_and_unknown_names() {
    // Not oracle-derived: a stub has no forward, and a name the spec does not export has none.
    assert_eq!(forward("??_EIostream_init@@QAEPAXI@Z"), None);
    assert_eq!(forward("not_in_the_spec"), None);
}

#[test]
fn forwards_name_msvcrt_or_msvcirt_under_the_same_name() {
    // Not oracle-derived: the two modules the spec names, and the rule that a forward targets the
    // export of the same name. Only stubs have no forward.
    for export in EXPORTS.iter() {
        match forward(export.name) {
            Some(Forward { module, function }) => {
                assert!(matches!(module, "msvcrt" | "msvcirt"), "{}", export.name);
                assert_eq!(function, export.name);
            }
            None => assert_eq!(export.kind, ExportKind::Stub, "{}", export.name),
        }
    }
}

#[test]
fn every_stub_helper_names_a_stub_export() {
    // Not oracle-derived: each helper's function name resolves to a `stub` entry, so the record
    // matches the table.
    for stub in [
        iostream_init_vector_deleting_dtor,
        iostream_init_scalar_deleting_dtor,
    ] {
        let name = stub().function;
        assert_eq!(
            find_export(name).map(|export| export.kind),
            Some(ExportKind::Stub),
            "{name}"
        );
        assert_eq!(stub().module, MSVCRT40_DLL_NAME);
    }
}
