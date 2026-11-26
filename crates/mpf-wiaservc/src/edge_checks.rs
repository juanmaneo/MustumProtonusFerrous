//! Boundary and branch checks. Unless a test says otherwise, its expectation is not oracle-derived:
//! it comes from the C source order or from `wiaservc.spec`, and the comment marks it as such.

use crate::com::{failed, IID_IUNKNOWN, IID_IWIADEVMGR};
use crate::oracle_checks::FakeHost;
use crate::{
    export_kind, wine_spec_unimplemented_stub, ClassFactory, EnumWiaDevInfo, ExportKind, Service,
    StubException, CLASS_E_NOAGGREGATION, EXPORTS, E_NOTIMPL, E_OUTOFMEMORY, STUB_EXPORTS,
};

#[test]
fn failed_boundaries() {
    // Not oracle-derived: `FAILED(hr)` is `hr < 0` from `include/winerror.h`.
    assert!(failed(-1));
    assert!(failed(i32::MIN));
    assert!(!failed(0));
    assert!(!failed(1));
    assert!(!failed(i32::MAX));
}

#[test]
fn export_table_lists_the_spec_exports() {
    // Not oracle-derived: EXPORTS is the table of `wiaservc.spec`, and STUB_EXPORTS must name the
    // same `@ stub` entries in the same order.
    assert_eq!(EXPORTS.len(), 57);
    let private: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Private)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(
        private,
        ["ServiceMain", "DllRegisterServer", "DllUnregisterServer"]
    );
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let helpers: Vec<&str> = STUB_EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(stubs, helpers);
}

#[test]
fn export_kind_lookup() {
    // Not oracle-derived: lookups against the `wiaservc.spec` table.
    assert_eq!(export_kind("ServiceMain"), Some(ExportKind::Private));
    assert_eq!(export_kind("DllRegisterServer"), Some(ExportKind::Private));
    assert_eq!(
        export_kind("DllUnregisterServer"),
        Some(ExportKind::Private)
    );
    assert_eq!(export_kind("wiasDebugTrace"), Some(ExportKind::Stub));
    assert_eq!(export_kind("wiasdebugtrace"), None);
    assert_eq!(export_kind("DllGetClassObject"), None);
}

#[test]
fn stub_record_takes_any_module_and_function() {
    // Not oracle-derived: the record shape is the one `__wine_spec_unimplemented_stub` raises
    // for any export, here with an ordinal-style function name.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "#42"),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "other.dll",
            function: "#42",
        }
    );
}

#[test]
fn create_instance_checks_outer_before_allocating() {
    // Not oracle-derived: `pUnkOuter` is tested before `wiadevmgr_Constructor` runs, so an
    // aggregation request reports CLASS_E_NOAGGREGATION even when the allocation would fail.
    let factory = ClassFactory;
    assert_eq!(
        factory
            .create_instance(true, &IID_IWIADEVMGR, false)
            .unwrap_err(),
        CLASS_E_NOAGGREGATION
    );
    assert_eq!(
        factory
            .create_instance(false, &IID_IUNKNOWN, false)
            .unwrap_err(),
        E_OUTOFMEMORY
    );
}

#[test]
fn enumerator_release_to_zero_reports_zero() {
    // Not oracle-derived: at zero the C body frees the object. The port has no heap, so it
    // reports the zero count.
    let e = EnumWiaDevInfo::new(true).unwrap();
    assert_eq!(e.release(), 0);
}

#[test]
fn enumerator_next_and_get_count_write_zero() {
    // Not oracle-derived: the zero counts are the `*fetched` and `*count` writes from the C bodies.
    let e = EnumWiaDevInfo::new(true).unwrap();
    assert_eq!(e.next(), (E_NOTIMPL, 0));
    assert_eq!(e.get_count(), (E_NOTIMPL, 0));
}

#[test]
fn service_main_continues_past_a_failed_status_write() {
    // Not oracle-derived: no oracle row covers a failing SetServiceStatus inside ServiceMain.
    // UpdateStatus only signals the stop event on failure (service.c), so ServiceMain carries on.
    let host = FakeHost {
        fail_set_status: true,
        ..FakeHost::default()
    };
    let mut service = Service::new(host);
    service.service_main();
    assert_eq!(
        service.host().trace.join(";"),
        "create;register;set(16,2,0,0,0,0,3000);setevent;coinit;cosec;coreg;set(16,4,7,0,0,0,0);setevent;wait;revoke;set(16,1,7,0,0,0,0);setevent;close;uninit"
    );
}
