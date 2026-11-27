//! Edge-case tests for the ported photometadatahandler spec, stub and makefile models.
//!
//! The oracle covers the stub record. The export table and the makefile settings have no C body,
//! so their expectations come from the spec and `Makefile.in` lines and are not oracle-derived.

use super::*;

#[test]
fn stub_constants_match_headers() {
    // Not oracle-derived: the values from include/winternl.h and include/winnt.h. The harness
    // shims define the same numbers, so the oracle rows confirm the shim, not the header.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the four lines of dlls/photometadatahandler/photometadatahandler.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("DllCanUnloadNow", ExportKind::StdcallPrivate),
            ("DllGetClassObject", ExportKind::Stub),
            ("DllRegisterServer", ExportKind::StdcallPrivate),
            ("DllUnregisterServer", ExportKind::StdcallPrivate),
        ]
    );
}

#[test]
fn export_kind_is_case_sensitive() {
    // Not oracle-derived: lookups over the spec names.
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("DllCanUnloadNow"),
        Some(ExportKind::StdcallPrivate)
    );
    assert_eq!(export_kind("dllgetclassobject"), None);
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("MergeFontPackage"), None);
}

#[test]
fn only_the_stub_is_in_the_export_table() {
    // Not oracle-derived: winebuild skips FLAG_PRIVATE functions when it writes the export table
    // (tools/winebuild/import.c and tools/winebuild/spec32.c).
    assert!(ExportKind::Stub.in_export_table());
    assert!(!ExportKind::StdcallPrivate.in_export_table());
    assert!(is_exported("DllGetClassObject"));
    assert!(!is_exported("DllCanUnloadNow"));
    assert!(!is_exported("DllRegisterServer"));
    assert!(!is_exported("DllUnregisterServer"));
    assert!(!is_exported("NotAnExport"));
}

#[test]
fn makefile_settings_match_makefile_in() {
    // Not oracle-derived: the MODULE, EXTRADLLFLAGS and VER_* lines of dlls/photometadatahandler/Makefile.in.
    let settings = makefile_settings();
    assert_eq!(settings.module, "photometadatahandler.dll");
    assert_eq!(settings.extra_dll_flags, "-Wb,--prefer-native");
    assert_eq!(settings.file_description, "Wine photometadatahandler dll");
    assert_eq!(settings.product_version, [7, 0, 0, 0]);
}

#[test]
fn stub_module_name_comes_from_makefile() {
    // Not oracle-derived: the stub's module must be MODULE from Makefile.in. Oracle row 01 pins the value.
    assert_eq!(dll_get_class_object().module, makefile_settings().module);
}
