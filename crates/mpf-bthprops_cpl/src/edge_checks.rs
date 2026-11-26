//! Boundary and branch checks for the export table and the stub records.

use super::*;

/// Not oracle-derived: the values are copied from `include/winternl.h`, `include/winnt.h` and
/// `dlls/bthprops.cpl/Makefile.in`.
#[test]
fn constants_are_pinned() {
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(BTHPROPS_DLL_NAME, "bthprops.cpl");
}

/// Not oracle-derived: the arguments are passed through unchanged, which the harness does not
/// vary, so these cases come from the port's contract.
#[test]
fn stub_record_carries_the_arguments() {
    let record = wine_spec_unimplemented_stub("other.dll", "Other");
    assert_eq!(record.code, EXCEPTION_WINE_STUB);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(record.number_parameters, 2);
    assert_eq!(record.module, "other.dll");
    assert_eq!(record.function, "Other");
    assert_eq!(
        wine_spec_unimplemented_stub("", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "",
            function: "",
        }
    );
}

/// Not oracle-derived: the counts are the `@ stub`, `@ stdcall -import`, `@ stdcall` and
/// `@ stdcall -private` lines of `bthprops.cpl.spec`.
#[test]
fn export_kinds_are_counted_as_in_the_spec() {
    let count = |kind| EXPORTS.iter().filter(|(_, k)| *k == kind).count();
    assert_eq!(EXPORTS.len(), 64);
    assert_eq!(count(ExportKind::Stub), 42);
    assert_eq!(count(ExportKind::Import), 20);
    assert_eq!(count(ExportKind::Stdcall), 1);
    assert_eq!(count(ExportKind::Private), 1);
}

/// Not oracle-derived: the kinds are read from `bthprops.cpl.spec`.
#[test]
fn export_kind_finds_each_kind() {
    assert_eq!(
        export_kind("BluetoothAddressToString"),
        Some(ExportKind::Stub)
    );
    assert_eq!(
        export_kind("BluetoothEnableDiscovery"),
        Some(ExportKind::Import)
    );
    assert_eq!(
        export_kind("BluetoothGetDeviceInfo"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Private));
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Stub));
}

/// Not oracle-derived: `CPlApplet` is commented out in the spec, and names are case-sensitive.
#[test]
fn unknown_names_are_not_exports() {
    assert_eq!(export_kind("CPlApplet"), None);
    assert_eq!(export_kind("bluetoothaddresstostring"), None);
    assert_eq!(export_kind(""), None);
}

/// Not oracle-derived: a duplicate name would make [`export_kind`] ambiguous.
#[test]
fn export_names_are_unique() {
    for (i, (name, _)) in EXPORTS.iter().enumerate() {
        assert!(
            EXPORTS[i + 1..].iter().all(|(other, _)| other != name),
            "{name}"
        );
    }
}

/// Not oracle-derived: checks that [`STUB_EXPORTS`] pairs each helper with the `Stub` names of
/// [`EXPORTS`], in the same order.
#[test]
fn stub_table_matches_the_export_table() {
    let stub_names: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(STUB_EXPORTS.len(), stub_names.len());
    for (&(name, helper), expected) in STUB_EXPORTS.iter().zip(stub_names) {
        assert_eq!(name, expected);
        assert_eq!(
            helper(),
            wine_spec_unimplemented_stub(BTHPROPS_DLL_NAME, name)
        );
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
    }
}
