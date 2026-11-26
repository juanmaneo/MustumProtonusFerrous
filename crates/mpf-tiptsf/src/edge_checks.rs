//! Edge-case tests for the ported tiptsf logic.
//!
//! These cover the constants, the reference-count boundaries, the GUID comparison and the export
//! table. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(S_OK, 0x0000_0000);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_OUTOFMEMORY, 0x8007_000E);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn guids_are_pinned() {
    // Not oracle-derived: values from include/guiddef.h and include/peninputpanel.idl. The oracle
    // harness uses the same bytes in its shim.
    assert_eq!(
        IID_UNKNOWN,
        Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
    assert_eq!(IID_CLASS_FACTORY.data1, 1);
    assert_eq!(
        IID_TEXT_INPUT_PANEL,
        Guid {
            data1: 0x6b6a_65a5,
            data2: 0x6af3,
            data3: 0x46c2,
            data4: [0xb6, 0xea, 0x56, 0xcd, 0x1f, 0x80, 0xdf, 0x71],
        }
    );
    assert_eq!(
        CLSID_TEXT_INPUT_PANEL,
        Guid {
            data1: 0xf9b1_89d7,
            data2: 0x228b,
            data3: 0x4f2b,
            data4: [0x86, 0x50, 0xb9, 0x7f, 0x59, 0xe0, 0x2c, 0x8c],
        }
    );
}

#[test]
fn guid_equality_compares_every_byte() {
    // Not oracle-derived: IsEqualGUID compares the whole structure, so a change to the last byte
    // alone must make the GUIDs differ.
    let mut near = IID_TEXT_INPUT_PANEL;
    assert!(is_equal_guid(&IID_TEXT_INPUT_PANEL, &near.clone()));
    near.data4[7] ^= 1;
    assert!(!is_equal_guid(&IID_TEXT_INPUT_PANEL, &near));
    assert!(!is_equal_guid(&IID_UNKNOWN, &IID_CLASS_FACTORY));
}

#[test]
fn add_ref_wraps_like_interlocked_increment() {
    // Not oracle-derived: signed overflow is undefined in the C code, so the port wraps. This pins
    // the chosen two's-complement result.
    let mut p = TextInputPanel {
        ref_count: i32::MAX,
    };
    assert_eq!(p.add_ref(), 0x8000_0000);
    assert_eq!(p.ref_count, i32::MIN);
}

#[test]
fn release_below_one_returns_unsigned_count() {
    // Not oracle-derived: a count of zero is already freed in C. Releasing it again yields -1,
    // which the ULONG return value shows as 0xffffffff.
    let mut p = TextInputPanel { ref_count: 0 };
    assert_eq!(p.release(), 0xffff_ffff);
}

#[test]
fn failed_query_interface_leaves_count_alone() {
    // Not oracle-derived: the C code calls AddRef only after a successful match. Checked for every
    // rejected IID, not just the ones the oracle printed.
    let mut p = TextInputPanel { ref_count: 3 };
    assert_eq!(p.query_interface(&IID_CLASS_FACTORY), E_NOINTERFACE);
    assert_eq!(p.query_interface(&CLSID_TEXT_INPUT_PANEL), E_NOINTERFACE);
    assert_eq!(p.ref_count, 3);
}

#[test]
fn class_factory_rejects_the_panel_iid() {
    // Not oracle-derived: the factory's QueryInterface accepts only IUnknown and IClassFactory,
    // not the panel's own interface ID.
    assert_eq!(
        class_factory_query_interface(&IID_TEXT_INPUT_PANEL),
        E_NOINTERFACE
    );
    assert_eq!(
        class_factory_query_interface(&CLSID_TEXT_INPUT_PANEL),
        E_NOINTERFACE
    );
}

#[test]
fn dll_get_class_object_checks_only_the_clsid() {
    // Not oracle-derived: an unknown CLSID returns CLASS_E_CLASSNOTAVAILABLE before the IID is
    // examined, so even an IID the factory accepts gets no object.
    assert_eq!(
        dll_get_class_object(&IID_UNKNOWN, &IID_UNKNOWN),
        CLASS_E_CLASSNOTAVAILABLE
    );
}

#[test]
fn create_instance_with_malloc_failure_has_no_panel() {
    // Not oracle-derived: restates the failure branch in the Rust API. The oracle row 02 covers
    // the same result.
    assert!(panelinput_create_instance(false).is_err());
}

#[test]
fn export_table_has_every_spec_name_once() {
    // Not oracle-derived: the names and order come from dlls/tiptsf/tiptsf.spec.
    let names: Vec<&str> = EXPORTS.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        names,
        [
            "AdviseHook",
            "EndCaretTracking",
            "ProcessCaretEvents",
            "ProcessCiceroCaretEvent",
            "RestrictedModeMsgWndProc",
            "StartCaretTracking",
            "UnadviseHook",
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer",
        ]
    );
}

#[test]
fn export_kind_maps_each_name() {
    // Not oracle-derived: the kinds come from the `@ stub` and `@ stdcall -private` markers in
    // dlls/tiptsf/tiptsf.spec.
    assert_eq!(export_kind("AdviseHook"), Some(ExportKind::Stub));
    assert_eq!(export_kind("UnadviseHook"), Some(ExportKind::Stub));
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Stdcall));
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: names are matched exactly, as the spec file's names are.
    assert_eq!(export_kind("adviseHook"), None);
    assert_eq!(export_kind("DllGetClassObjectX"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_helpers_use_the_tiptsf_module_name() {
    // Not oracle-derived: every helper passes MODULE = tiptsf.dll and its own export name.
    assert_eq!(end_caret_tracking().module, "tiptsf.dll");
    assert_eq!(process_caret_events().function, "ProcessCaretEvents");
    assert_eq!(
        process_cicero_caret_event().function,
        "ProcessCiceroCaretEvent"
    );
    assert_eq!(
        restricted_mode_msg_wnd_proc().function,
        "RestrictedModeMsgWndProc"
    );
    assert_eq!(start_caret_tracking().function, "StartCaretTracking");
}
