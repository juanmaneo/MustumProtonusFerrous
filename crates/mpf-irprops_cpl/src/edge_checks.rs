//! Edge-case tests for the ported irprops.cpl logic.
//!
//! These cover the stub record pass-through, the export table counts and the lookup. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the names of irprops.cpl.spec. The code and
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
fn export_kinds_match_spec_line_shapes() {
    // Not oracle-derived: counts of the spec lines by shape: 33 `@ stub`, 4 `@ stdcall`
    // forwarders, 13 `@ stdcall -import` and 1 `@ stdcall` with no target.
    let count = |kind: ExportKind| EXPORTS.iter().filter(|(_, k)| *k == kind).count();
    assert_eq!(EXPORTS.len(), 51);
    assert_eq!(count(ExportKind::Stub), 33);
    assert_eq!(count(ExportKind::Import), 13);
    assert_eq!(count(ExportKind::StdcallNoBody), 1);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, k)| matches!(k, ExportKind::Forward(_)))
            .count(),
        4
    );
}

#[test]
fn forward_targets_name_the_same_export_in_bthprops() {
    // Not oracle-derived: the four forwarders in irprops.cpl.spec point at bthprops.cpl with the
    // same export name.
    for (name, kind) in EXPORTS {
        if let ExportKind::Forward(target) = kind {
            assert_eq!(target, format!("bthprops.cpl.{name}"));
        }
    }
}

#[test]
fn lookup_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: `#@ stub CPlApplet` is commented out in irprops.cpl.spec, so the DLL
    // does not export it.
    assert_eq!(
        export_kind("BluetoothIsConnectable"),
        Some(ExportKind::Forward("bthprops.cpl.BluetoothIsConnectable"))
    );
    assert_eq!(
        export_kind("BluetoothGetDeviceInfo"),
        Some(ExportKind::StdcallNoBody)
    );
    assert_eq!(export_kind("bluetoothisconnectable"), None);
    assert_eq!(export_kind("CPlApplet"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: a duplicate name would make export_kind ambiguous.
    let mut names: Vec<&str> = EXPORTS.iter().map(|(n, _)| *n).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EXPORTS.len());
}
