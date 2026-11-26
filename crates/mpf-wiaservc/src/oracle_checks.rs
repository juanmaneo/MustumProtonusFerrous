//! Checks against the oracle rows in `target/canon/wiaservc/rows.txt`. The harness compiles the
//! verbatim Wine `factory.c`, `wiadevmgr.c` and `service.c` with Win32 and COM recorders, and
//! `__wine_spec_unimplemented_stub` with a recorder in place of `RtlRaiseException`. Each test
//! names the row ids it checks.

use crate::com::{
    Guid, Hresult, CLASS_E_NOAGGREGATION, CLSID_WIADEVMGR, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY,
    IID_ICLASSFACTORY, IID_IENUMWIA_DEV_INFO, IID_IUNKNOWN, IID_IWIADEVMGR,
};
use crate::service::{
    NO_ERROR, SERVICE_ACCEPT_PAUSE_CONTINUE, SERVICE_ACCEPT_SHUTDOWN, SERVICE_ACCEPT_STOP,
    SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_STOP, SERVICE_RUNNING, SERVICE_START_PENDING,
    SERVICE_STOPPED, SERVICE_STOP_PENDING, SERVICE_WIN32_OWN_PROCESS,
};
use crate::{
    export_kind, wine_spec_unimplemented_stub, ClassFactory, EnumWiaDevInfo, Service, ServiceHost,
    ServiceStatus, StubException, WiaDevMgr, STUB_EXPORTS,
};

/// `S_FALSE` from `include/winerror.h`.
const S_FALSE: Hresult = 1;
/// `E_FAIL` from `include/winerror.h`.
const E_FAIL: Hresult = 0x8000_4005u32 as Hresult;

/// The `ROW_STUB` rows of `rows.txt`, in spec order: (module, function, code, flags, number of
/// parameters). The rows are the `__wine_spec_unimplemented_stub` records for the 54 `@ stub`
/// exports.
const ORACLE_STUB_ROWS: [(&str, &str, u32, u32, u32); 54] = [
    ("wiaservc.dll", "wiasCreateChildAppItem", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasCreateDrvItem", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasCreateLogInstance", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasCreatePropContext", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasDebugError", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasDebugTrace", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasDownSampleBuffer", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasFormatArgs", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasFreePropContext", 0x80000100, 0x1, 2),
    (
        "wiaservc.dll",
        "wiasGetChangedValueFloat",
        0x80000100,
        0x1,
        2,
    ),
    (
        "wiaservc.dll",
        "wiasGetChangedValueGuid",
        0x80000100,
        0x1,
        2,
    ),
    (
        "wiaservc.dll",
        "wiasGetChangedValueLong",
        0x80000100,
        0x1,
        2,
    ),
    ("wiaservc.dll", "wiasGetChangedValueStr", 0x80000100, 0x1, 2),
    (
        "wiaservc.dll",
        "wiasGetChildrenContexts",
        0x80000100,
        0x1,
        2,
    ),
    ("wiaservc.dll", "wiasGetContextFromName", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasGetDrvItem", 0x80000100, 0x1, 2),
    (
        "wiaservc.dll",
        "wiasGetImageInformation",
        0x80000100,
        0x1,
        2,
    ),
    ("wiaservc.dll", "wiasGetItemType", 0x80000100, 0x1, 2),
    (
        "wiaservc.dll",
        "wiasGetPropertyAttributes",
        0x80000100,
        0x1,
        2,
    ),
    ("wiaservc.dll", "wiasGetRootItem", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasIsPropChanged", 0x80000100, 0x1, 2),
    (
        "wiaservc.dll",
        "wiasParseEndorserString",
        0x80000100,
        0x1,
        2,
    ),
    ("wiaservc.dll", "wiasPrintDebugHResult", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasQueueEvent", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasReadMultiple", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasReadPropBin", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasReadPropFloat", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasReadPropGuid", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasReadPropLong", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasReadPropStr", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSendEndOfPage", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSetItemPropAttribs", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSetItemPropNames", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSetPropChanged", 0x80000100, 0x1, 2),
    (
        "wiaservc.dll",
        "wiasSetPropertyAttributes",
        0x80000100,
        0x1,
        2,
    ),
    ("wiaservc.dll", "wiasSetValidFlag", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSetValidListFloat", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSetValidListGuid", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSetValidListLong", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSetValidListStr", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSetValidRangeFloat", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasSetValidRangeLong", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasUpdateScanRect", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasUpdateValidFormat", 0x80000100, 0x1, 2),
    (
        "wiaservc.dll",
        "wiasValidateItemProperties",
        0x80000100,
        0x1,
        2,
    ),
    ("wiaservc.dll", "wiasWriteBufToFile", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasWriteMultiple", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasWritePageBufToFile", 0x80000100, 0x1, 2),
    (
        "wiaservc.dll",
        "wiasWritePageBufToStream",
        0x80000100,
        0x1,
        2,
    ),
    ("wiaservc.dll", "wiasWritePropBin", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasWritePropFloat", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasWritePropGuid", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasWritePropLong", 0x80000100, 0x1, 2),
    ("wiaservc.dll", "wiasWritePropStr", 0x80000100, 0x1, 2),
];

/// Stands in for the Win32 and COM calls. Each call appends the token that the oracle harness
/// records, so a test can compare the call sequence with the `trace=` column of a row.
#[derive(Debug, Default)]
pub(crate) struct FakeHost {
    pub(crate) trace: Vec<String>,
    pub(crate) fail_create_event: bool,
    pub(crate) fail_register: bool,
    pub(crate) fail_set_status: bool,
    pub(crate) co_init_ex_hr: Hresult,
    pub(crate) co_init_security_hr: Hresult,
    pub(crate) co_register_hr: Hresult,
    pub(crate) registered_clsid: Option<Guid>,
}

impl ServiceHost for FakeHost {
    fn create_stop_event(&mut self) -> bool {
        self.trace.push("create".into());
        !self.fail_create_event
    }

    fn register_service_ctrl_handler(&mut self) -> bool {
        self.trace.push("register".into());
        !self.fail_register
    }

    fn set_service_status(&mut self, status: &ServiceStatus) -> bool {
        self.trace.push(format!(
            "set({},{},{},{},{},{},{})",
            status.service_type,
            status.current_state,
            status.controls_accepted,
            status.win32_exit_code,
            status.service_specific_exit_code,
            status.check_point,
            status.wait_hint
        ));
        !self.fail_set_status
    }

    fn set_stop_event(&mut self) {
        self.trace.push("setevent".into());
    }

    fn co_initialize_ex(&mut self) -> Hresult {
        self.trace.push("coinit".into());
        self.co_init_ex_hr
    }

    fn co_initialize_security(&mut self) -> Hresult {
        self.trace.push("cosec".into());
        self.co_init_security_hr
    }

    fn co_register_class_object(&mut self, clsid: &Guid) -> Hresult {
        self.trace.push("coreg".into());
        self.registered_clsid = Some(*clsid);
        self.co_register_hr
    }

    fn wait_for_stop_event(&mut self) {
        self.trace.push("wait".into());
    }

    fn co_revoke_class_object(&mut self) {
        self.trace.push("revoke".into());
    }

    fn close_stop_event(&mut self) {
        self.trace.push("close".into());
    }

    fn co_uninitialize(&mut self) {
        self.trace.push("uninit".into());
    }
}

fn trace_of(service: &Service<FakeHost>) -> String {
    service.host().trace.join(";")
}

/// The `SERVICE_STATUS` the oracle rows print for `type=16`, `exit=0`, `svc_exit=0`,
/// `checkpoint=0`.
fn row_status(current_state: u32, controls_accepted: u32, wait_hint: u32) -> ServiceStatus {
    ServiceStatus {
        service_type: SERVICE_WIN32_OWN_PROCESS,
        current_state,
        controls_accepted,
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint,
    }
}

#[test]
fn stub_rows_match_oracle() {
    assert_eq!(STUB_EXPORTS.len(), ORACLE_STUB_ROWS.len());
    for ((name, helper), (module, function, code, flags, number_parameters)) in
        STUB_EXPORTS.iter().zip(ORACLE_STUB_ROWS)
    {
        assert_eq!(*name, function);
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
        assert_eq!(
            wine_spec_unimplemented_stub(module, function),
            helper(),
            "{function}"
        );
        assert_eq!(export_kind(function), Some(crate::ExportKind::Stub));
    }
}

#[test]
fn factory_rows_f01_to_f12() {
    let factory = ClassFactory;

    // F01, F02
    assert_eq!(factory.add_ref(), 2);
    assert_eq!(factory.release(), 1);

    // F03, F04
    assert_eq!(factory.lock_server(true), E_NOTIMPL);
    assert_eq!(factory.lock_server(false), E_NOTIMPL);

    // F05, F06: out=obj
    assert!(factory.query_interface(&IID_IUNKNOWN).is_ok());
    assert!(factory.query_interface(&IID_ICLASSFACTORY).is_ok());

    // F07: out=NULL
    assert_eq!(
        factory.query_interface(&IID_IWIADEVMGR).unwrap_err(),
        E_NOINTERFACE
    );

    // F08: out untouched, the outer check comes before the constructor
    assert_eq!(
        factory
            .create_instance(true, &IID_IWIADEVMGR, true)
            .unwrap_err(),
        CLASS_E_NOAGGREGATION
    );

    // F09, F10: out=obj ref=1
    let devmgr = factory
        .create_instance(false, &IID_IWIADEVMGR, true)
        .unwrap();
    assert_eq!(devmgr.ref_count(), 1);
    let devmgr = factory.create_instance(false, &IID_IUNKNOWN, true).unwrap();
    assert_eq!(devmgr.ref_count(), 1);

    // F11: out=NULL
    assert_eq!(
        factory
            .create_instance(false, &IID_ICLASSFACTORY, true)
            .unwrap_err(),
        E_NOINTERFACE
    );

    // F12: out untouched
    assert_eq!(
        factory
            .create_instance(false, &IID_IWIADEVMGR, false)
            .unwrap_err(),
        E_OUTOFMEMORY
    );
}

#[test]
fn devmgr_rows_d01_to_d17() {
    // D01, D02
    assert_eq!(WiaDevMgr::new(true).unwrap().ref_count(), 1);
    assert_eq!(WiaDevMgr::new(false).unwrap_err(), E_OUTOFMEMORY);

    // D03, D04: out=obj ref=2
    let devmgr = WiaDevMgr::new(true).unwrap();
    assert_eq!(
        devmgr.query_interface(&IID_IUNKNOWN).unwrap().ref_count(),
        2
    );
    let devmgr = WiaDevMgr::new(true).unwrap();
    assert_eq!(
        devmgr.query_interface(&IID_IWIADEVMGR).unwrap().ref_count(),
        2
    );

    // D05: out=NULL ref=1
    let devmgr = WiaDevMgr::new(true).unwrap();
    assert_eq!(
        devmgr.query_interface(&IID_IENUMWIA_DEV_INFO).unwrap_err(),
        E_NOINTERFACE
    );
    assert_eq!(devmgr.ref_count(), 1);

    // D06, D07
    let devmgr = WiaDevMgr::new(true).unwrap();
    assert_eq!(devmgr.add_ref(), 2);
    assert_eq!(devmgr.release(), 1);

    // D08: out=obj ref=1
    let devmgr = WiaDevMgr::new(true).unwrap();
    assert_eq!(devmgr.enum_device_info(true).unwrap().ref_count(), 1);

    // D09: out=NULL
    assert_eq!(devmgr.enum_device_info(false).unwrap_err(), E_OUTOFMEMORY);

    // D10 to D17
    assert_eq!(devmgr.create_device(), E_NOTIMPL);
    assert_eq!(devmgr.select_device_dlg(), E_NOTIMPL);
    assert_eq!(devmgr.select_device_dlg_id(), E_NOTIMPL);
    assert_eq!(devmgr.get_image_dlg(), E_NOTIMPL);
    assert_eq!(devmgr.register_event_callback_program(), E_NOTIMPL);
    assert_eq!(devmgr.register_event_callback_interface(), E_NOTIMPL);
    assert_eq!(devmgr.register_event_callback_clsid(), E_NOTIMPL);
    assert_eq!(devmgr.add_device_dlg(), E_NOTIMPL);
}

#[test]
fn enumerator_rows_e01_to_e10() {
    let enumerator = || EnumWiaDevInfo::new(true).unwrap();

    // E01, E02: out=obj ref=2
    assert_eq!(
        enumerator()
            .query_interface(&IID_IUNKNOWN)
            .unwrap()
            .ref_count(),
        2
    );
    assert_eq!(
        enumerator()
            .query_interface(&IID_IENUMWIA_DEV_INFO)
            .unwrap()
            .ref_count(),
        2
    );

    // E03: out=NULL ref=1
    let e = enumerator();
    assert_eq!(
        e.query_interface(&IID_IWIADEVMGR).unwrap_err(),
        E_NOINTERFACE
    );
    assert_eq!(e.ref_count(), 1);

    // E04, E05
    let e = enumerator();
    assert_eq!(e.add_ref(), 2);
    assert_eq!(e.release(), 1);

    // E06 to E10
    let e = enumerator();
    assert_eq!(e.next(), (E_NOTIMPL, 0));
    assert_eq!(e.skip(), E_NOTIMPL);
    assert_eq!(e.reset(), E_NOTIMPL);
    assert_eq!(e.clone_enumerator(), E_NOTIMPL);
    assert_eq!(e.get_count(), (E_NOTIMPL, 0));
}

#[test]
fn update_status_row_s01() {
    let mut service = Service::new(FakeHost::default());
    service.update_status(SERVICE_START_PENDING, NO_ERROR, 3000);
    assert_eq!(
        service.status(),
        &row_status(SERVICE_START_PENDING, 0, 3000)
    );
    assert_eq!(trace_of(&service), "set(16,2,0,0,0,0,3000)");
}

#[test]
fn update_status_row_s02() {
    let mut service = Service::new(FakeHost::default());
    // The exit code 1234 is passed and ignored: the status keeps 0.
    service.update_status(SERVICE_RUNNING, 1234, 0);
    assert_eq!(
        service.status(),
        &row_status(
            SERVICE_RUNNING,
            SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_PAUSE_CONTINUE | SERVICE_ACCEPT_SHUTDOWN,
            0
        )
    );
    assert_eq!(trace_of(&service), "set(16,4,7,0,0,0,0)");
}

#[test]
fn update_status_row_s03() {
    let host = FakeHost {
        fail_set_status: true,
        ..FakeHost::default()
    };
    let mut service = Service::new(host);
    service.update_status(SERVICE_RUNNING, NO_ERROR, 0);
    assert_eq!(service.status(), &row_status(SERVICE_RUNNING, 7, 0));
    assert_eq!(trace_of(&service), "set(16,4,7,0,0,0,0);setevent");
}

#[test]
fn service_handler_row_s04() {
    let mut service = Service::new(FakeHost::default());
    assert_eq!(service.service_handler(SERVICE_CONTROL_STOP), NO_ERROR);
    assert_eq!(service.status(), &row_status(SERVICE_STOP_PENDING, 7, 0));
    assert_eq!(trace_of(&service), "set(16,3,7,0,0,0,0);setevent");
}

#[test]
fn service_handler_row_s05() {
    let mut service = Service::new(FakeHost::default());
    service.update_status(SERVICE_RUNNING, NO_ERROR, 0);
    service.host_mut().trace.clear();
    assert_eq!(service.service_handler(SERVICE_CONTROL_SHUTDOWN), NO_ERROR);
    assert_eq!(service.status(), &row_status(SERVICE_STOP_PENDING, 7, 0));
    assert_eq!(trace_of(&service), "set(16,3,7,0,0,0,0);setevent");
}

#[test]
fn service_handler_row_s06() {
    let mut service = Service::new(FakeHost::default());
    service.update_status(SERVICE_RUNNING, NO_ERROR, 0);
    service.host_mut().trace.clear();
    // SERVICE_CONTROL_PAUSE (2) is not handled: the current state is reported again.
    assert_eq!(service.service_handler(2), NO_ERROR);
    assert_eq!(service.status(), &row_status(SERVICE_RUNNING, 7, 0));
    assert_eq!(trace_of(&service), "set(16,4,7,0,0,0,0)");
}

#[test]
fn service_handler_row_s07() {
    let mut service = Service::new(FakeHost::default());
    service.update_status(SERVICE_START_PENDING, NO_ERROR, 3000);
    service.host_mut().trace.clear();
    assert_eq!(service.service_handler(2), NO_ERROR);
    assert_eq!(service.status(), &row_status(SERVICE_START_PENDING, 0, 0));
    assert_eq!(trace_of(&service), "set(16,2,0,0,0,0,0)");
}

#[test]
fn service_main_row_m01() {
    let mut service = Service::new(FakeHost::default());
    service.service_main();
    assert_eq!(service.status(), &row_status(SERVICE_STOPPED, 7, 0));
    assert_eq!(
        trace_of(&service),
        "create;register;set(16,2,0,0,0,0,3000);coinit;cosec;coreg;set(16,4,7,0,0,0,0);wait;revoke;set(16,1,7,0,0,0,0);close;uninit"
    );
    assert_eq!(service.host().registered_clsid, Some(CLSID_WIADEVMGR));
}

#[test]
fn service_main_row_m02() {
    let host = FakeHost {
        fail_create_event: true,
        ..FakeHost::default()
    };
    let mut service = Service::new(host);
    service.service_main();
    assert_eq!(service.status(), &ServiceStatus::default());
    assert_eq!(trace_of(&service), "create");
}

#[test]
fn service_main_row_m03() {
    let host = FakeHost {
        fail_register: true,
        ..FakeHost::default()
    };
    let mut service = Service::new(host);
    service.service_main();
    assert_eq!(service.status(), &ServiceStatus::default());
    assert_eq!(trace_of(&service), "create;register");
}

#[test]
fn service_main_row_m04() {
    let host = FakeHost {
        co_init_ex_hr: E_FAIL,
        ..FakeHost::default()
    };
    let mut service = Service::new(host);
    service.service_main();
    assert_eq!(service.status(), &row_status(SERVICE_STOPPED, 7, 0));
    assert_eq!(
        trace_of(&service),
        "create;register;set(16,2,0,0,0,0,3000);coinit;set(16,1,7,0,0,0,0)"
    );
}

#[test]
fn service_main_row_m05() {
    let host = FakeHost {
        co_init_security_hr: E_FAIL,
        ..FakeHost::default()
    };
    let mut service = Service::new(host);
    service.service_main();
    assert_eq!(service.status(), &row_status(SERVICE_STOPPED, 7, 0));
    assert_eq!(
        trace_of(&service),
        "create;register;set(16,2,0,0,0,0,3000);coinit;cosec;set(16,1,7,0,0,0,0)"
    );
}

#[test]
fn service_main_row_m06() {
    let host = FakeHost {
        co_register_hr: E_FAIL,
        ..FakeHost::default()
    };
    let mut service = Service::new(host);
    service.service_main();
    assert_eq!(service.status(), &row_status(SERVICE_STOPPED, 7, 0));
    assert_eq!(
        trace_of(&service),
        "create;register;set(16,2,0,0,0,0,3000);coinit;cosec;coreg;set(16,1,7,0,0,0,0)"
    );
}

#[test]
fn service_main_row_m07() {
    // S_FALSE is a success HRESULT, so StartCount continues.
    let host = FakeHost {
        co_init_ex_hr: S_FALSE,
        ..FakeHost::default()
    };
    let mut service = Service::new(host);
    service.service_main();
    assert_eq!(service.status(), &row_status(SERVICE_STOPPED, 7, 0));
    assert_eq!(
        trace_of(&service),
        "create;register;set(16,2,0,0,0,0,3000);coinit;cosec;coreg;set(16,4,7,0,0,0,0);wait;revoke;set(16,1,7,0,0,0,0);close;uninit"
    );
}
