//! Edge-case tests for the ported initpki logic.
//!
//! These cover the export table, the stub record's pass-through of its names, and the stub
//! constants. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: names the harness did not pass. The code, flags and count do not depend
    // on the names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);
}

#[test]
fn export_helpers_use_the_module_from_makefile() {
    // Not oracle-derived: the module name is MODULE = initpki.dll from dlls/initpki/Makefile.in.
    assert_eq!(dll_install().module, "initpki.dll");
    assert_eq!(initialize_pki().module, "initpki.dll");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the four lines of dlls/initpki/initpki.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("DllInstall", ExportKind::Stub),
            ("DllRegisterServer", ExportKind::StdcallPrivate),
            ("DllUnregisterServer", ExportKind::StdcallPrivate),
            ("InitializePKI", ExportKind::Stub),
        ]
    );
    assert_eq!(export_kind("DllInstall"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::StdcallPrivate)
    );
    assert_eq!(
        export_kind("DllUnregisterServer"),
        Some(ExportKind::StdcallPrivate)
    );
    assert_eq!(export_kind("InitializePKI"), Some(ExportKind::Stub));
    assert_eq!(export_kind("dllinstall"), None);
    assert_eq!(export_kind("DllInstall "), None);
    assert_eq!(export_kind(""), None);
}
