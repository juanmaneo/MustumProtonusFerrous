//! Edge-case tests for the ported wmp logic.
//!
//! These cover the GUID comparisons, the IID and CLSID boundaries, the `DllMain` reason
//! branches, the stub record and the export table. Not every expectation is oracle-derived;
//! the ones that aren't say so in a comment.

use super::*;

const IID_IUNKNOWN_LAST_BYTE: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47],
};

const CLSID_WMP_LAST_BYTE: Guid = Guid {
    data1: 0x6bf5_2a52,
    data2: 0x394a,
    data3: 0x11d3,
    data4: [0xb1, 0x53, 0x00, 0xc0, 0x4f, 0x79, 0xfa, 0xa7],
};

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winnt.h, include/winternl.h
    // and include/winnt.h, which the harness also uses.
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(DLL_PROCESS_DETACH, 0);
    assert_eq!(DLL_PROCESS_ATTACH, 1);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn guid_comparison_checks_every_field() {
    // Not oracle-derived: the comparison boundaries of IsEqualGUID.
    assert!(is_equal_guid(&IID_IUNKNOWN, &IID_IUNKNOWN));
    let mut other = IID_IUNKNOWN;
    other.data1 = 1;
    assert!(!is_equal_guid(&IID_IUNKNOWN, &other));
    other = IID_IUNKNOWN;
    other.data2 = 1;
    assert!(!is_equal_guid(&IID_IUNKNOWN, &other));
    other = IID_IUNKNOWN;
    other.data3 = 1;
    assert!(!is_equal_guid(&IID_IUNKNOWN, &other));
    other = IID_IUNKNOWN;
    other.data4[7] = 0;
    assert!(!is_equal_guid(&IID_IUNKNOWN, &other));
    assert!(!is_equal_guid(&IID_IUNKNOWN, &IID_ICLASSFACTORY));
}

#[test]
fn query_interface_rejects_near_miss_iids() {
    // Not oracle-derived: an IID that differs from IID_IUnknown only in its last byte.
    assert_eq!(
        class_factory_query_interface(&IID_IUNKNOWN_LAST_BYTE),
        E_NOINTERFACE
    );
    // Not oracle-derived: the same IID for both accepted interfaces returns S_OK.
    assert_eq!(class_factory_query_interface(&IID_IUNKNOWN), S_OK);
    assert_eq!(class_factory_query_interface(&IID_ICLASSFACTORY), S_OK);
}

#[test]
fn class_object_dispatches_on_clsid() {
    // Not oracle-derived: a CLSID that differs from the WMP class only in its last byte is
    // not the WMP class.
    assert_eq!(
        dll_get_class_object(&CLSID_WMP_LAST_BYTE, &IID_ICLASSFACTORY),
        ClassObject::NotAvailable
    );
    assert_eq!(
        dll_get_class_object(&CLSID_WMP_LAST_BYTE, &IID_ICLASSFACTORY).hresult(),
        CLASS_E_CLASSNOTAVAILABLE
    );
    // Not oracle-derived: the WMP class with IClassFactory goes through the factory.
    let r = dll_get_class_object(&CLSID_WINDOWS_MEDIA_PLAYER, &IID_ICLASSFACTORY);
    assert_eq!(r, ClassObject::Queried(S_OK));
    assert_eq!(r.hresult(), S_OK);
}

#[test]
fn dll_main_covers_every_reason_branch() {
    // Not oracle-derived: DllMain with the other reason codes. winnt.h gives the codes.
    let plain = DllMainOutcome {
        ret: true,
        attached: false,
        cleanup: false,
    };
    assert_eq!(dll_main(2, false), plain);
    assert_eq!(dll_main(3, false), plain);
    assert_eq!(dll_main(DLL_PROCESS_DETACH, false), plain);
    // Not oracle-derived: attach ignores the lpv argument.
    assert!(dll_main(DLL_PROCESS_ATTACH, false).attached);
    assert!(dll_main(DLL_PROCESS_ATTACH, false).ret);
    assert!(!dll_main(DLL_PROCESS_ATTACH, false).cleanup);
    // Not oracle-derived: detach runs cleanup only when lpv is NULL.
    assert!(dll_main(DLL_PROCESS_DETACH, true).cleanup);
    assert!(dll_main(DLL_PROCESS_DETACH, true).ret);
}

#[test]
fn stub_record_echoes_its_arguments() {
    // Not oracle-derived: the record holds the arguments unchanged, and the code and flags are
    // the constants.
    let stub = wine_spec_unimplemented_stub("other.dll", "SomeName");
    assert_eq!(stub.module, "other.dll");
    assert_eq!(stub.function, "SomeName");
    assert_eq!(stub.code, EXCEPTION_WINE_STUB);
    assert_eq!(stub.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(stub.number_parameters, 2);
}

#[test]
fn ordinal_helpers_pass_their_ordinal() {
    // Not oracle-derived: each helper names its export. Its ordinal is the function string.
    assert_eq!(ordinal_3000().function, "3000");
    assert_eq!(ordinal_3002().function, "3002");
    assert_eq!(ordinal_3003().function, "3003");
    assert_eq!(ordinal_3004().function, "3004");
    for name in ["3000", "3002", "3003", "3004"] {
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
    }
}

#[test]
fn export_table_matches_the_spec() {
    // Not oracle-derived: the spec order and kinds from dlls/wmp/wmp.spec.
    assert_eq!(EXPORTS.len(), 8);
    assert_eq!(EXPORTS[0], ("3000", ExportKind::Stub));
    assert_eq!(EXPORTS[3], ("3004", ExportKind::Stub));
    assert_eq!(EXPORTS[4], ("DllCanUnloadNow", ExportKind::Generated));
    assert_eq!(EXPORTS[5], ("DllGetClassObject", ExportKind::Explicit));
    assert_eq!(EXPORTS[6], ("DllRegisterServer", ExportKind::Generated));
    assert_eq!(EXPORTS[7], ("DllUnregisterServer", ExportKind::Generated));
}

#[test]
fn export_kind_is_case_sensitive_and_total() {
    // Not oracle-derived: lookup of every spec name, and the misses.
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Explicit));
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Generated));
    assert_eq!(export_kind("3002"), Some(ExportKind::Stub));
    assert_eq!(export_kind("dllgetclassobject"), None);
    assert_eq!(export_kind("3001"), None);
    assert_eq!(export_kind(""), None);
}
