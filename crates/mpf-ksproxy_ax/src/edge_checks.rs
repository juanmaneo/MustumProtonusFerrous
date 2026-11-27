//! Edge-case tests for the ported ksproxy.ax logic.
//!
//! These cover the stub record constants, the stub helpers' names, and the export table. Not
//! every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Oracle-derived: the harness `#define`s these values and prints them in every row.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived beyond rows 08-17: the code, flags and count are the same for any names.
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
fn every_stub_helper_names_its_spec_export() {
    // Not oracle-derived: checks each helper against the export table. Each helper must name a
    // `@ stub` export and use the module name from `dlls/ksproxy.ax/Makefile.in`.
    let helpers = [
        ("DllGetClassObject", dll_get_class_object()),
        ("KsGetMediaType", ks_get_media_type()),
        ("KsGetMediaTypeCount", ks_get_media_type_count()),
        (
            "KsGetMultiplePinFactoryItems",
            ks_get_multiple_pin_factory_items(),
        ),
        ("KsOpenDefaultDevice", ks_open_default_device()),
        (
            "KsResolveRequiredAttributes",
            ks_resolve_required_attributes(),
        ),
        (
            "KsSynchronousDeviceControl",
            ks_synchronous_device_control(),
        ),
    ];
    for (name, record) in helpers {
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
        assert_eq!(record.module, "ksproxy.ax");
        assert_eq!(record.function, name);
        assert_eq!(record.code, EXCEPTION_WINE_STUB);
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(record.number_parameters, 2);
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the ten lines of dlls/ksproxy.ax/ksproxy.ax.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("DllCanUnloadNow", ExportKind::PrivateWithoutBody),
            ("DllGetClassObject", ExportKind::Stub),
            ("DllRegisterServer", ExportKind::PrivateWithoutBody),
            ("DllUnregisterServer", ExportKind::PrivateWithoutBody),
            ("KsGetMediaType", ExportKind::Stub),
            ("KsGetMediaTypeCount", ExportKind::Stub),
            ("KsGetMultiplePinFactoryItems", ExportKind::Stub),
            ("KsOpenDefaultDevice", ExportKind::Stub),
            ("KsResolveRequiredAttributes", ExportKind::Stub),
            ("KsSynchronousDeviceControl", ExportKind::Stub),
        ]
    );
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    assert_eq!(stubs, 7);
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: name lookup behaviour of the spec table.
    assert_eq!(export_kind("KsGetMediaType"), Some(ExportKind::Stub));
    assert_eq!(export_kind("ksgetmediatype"), None);
    assert_eq!(
        export_kind("DllCanUnloadNow"),
        Some(ExportKind::PrivateWithoutBody)
    );
    assert_eq!(export_kind("ksproxy.ax"), None);
    assert_eq!(export_kind(""), None);
}
