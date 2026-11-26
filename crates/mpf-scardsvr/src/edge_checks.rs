//! Boundary and branch tests for the ported scardsvr logic.
//!
//! Expectations that are not oracle-derived say so in a comment. Those cover control codes
//! outside the oracle's 0-16 range, the spec file and the stub names.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from Wine's include/ headers.
    assert_eq!(SERVICE_WIN32, 0x10);
    assert_eq!(SERVICE_STOPPED, 1);
    assert_eq!(SERVICE_STOP_PENDING, 3);
    assert_eq!(SERVICE_RUNNING, 4);
    assert_eq!(SERVICE_ACCEPT_STOP, 1);
    assert_eq!(SERVICE_ACCEPT_SHUTDOWN, 4);
    assert_eq!(SERVICE_CONTROL_STOP, 1);
    assert_eq!(SERVICE_CONTROL_SHUTDOWN, 5);
    assert_eq!(ERROR_SUCCESS, 0);
    assert_eq!(INFINITE, u32::MAX);
    assert_eq!(SERVICE_NAME, "scardsvr");
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn only_stop_and_shutdown_signal_the_stop_event() {
    // Not oracle-derived for codes above 16: the same switch arms, checked over a wider range.
    for ctrl in 0..=0x100u32 {
        let outcome = service_handler(ctrl);
        let is_stop = ctrl == SERVICE_CONTROL_STOP || ctrl == SERVICE_CONTROL_SHUTDOWN;
        assert_eq!(outcome.sets_stop_event, is_stop, "ctrl {ctrl}");
        assert_eq!(outcome.ret, ERROR_SUCCESS, "ctrl {ctrl}");
        let state = if is_stop {
            SERVICE_STOP_PENDING
        } else {
            SERVICE_RUNNING
        };
        assert_eq!(outcome.status.current_state, state, "ctrl {ctrl}");
    }
}

#[test]
fn unknown_control_keeps_only_accept_stop() {
    // Not oracle-derived: the default arm for a code outside the oracle's range.
    let outcome = service_handler(u32::MAX);
    assert_eq!(outcome.status.current_state, SERVICE_RUNNING);
    assert_eq!(outcome.status.controls_accepted, SERVICE_ACCEPT_STOP);
    assert!(!outcome.sets_stop_event);
}

#[test]
fn stop_and_shutdown_report_the_same_status() {
    // Not oracle-derived: both codes share one switch arm in the C source.
    assert_eq!(
        service_handler(SERVICE_CONTROL_STOP),
        service_handler(SERVICE_CONTROL_SHUTDOWN)
    );
}

#[test]
fn stop_pending_status_drops_every_accepted_control() {
    // Not oracle-derived: the wait hint and checkpoint are zero in the handler's status, and no
    // control stays accepted. Oracle rows 02 and 06 show the same result for codes 1 and 5.
    let status = service_handler(SERVICE_CONTROL_STOP).status;
    assert_eq!(status.controls_accepted, 0);
    assert_eq!(status.wait_hint, 0);
    assert_eq!(status.check_point, 0);
}

#[test]
fn calais_main_stops_after_registration_when_handle_is_null() {
    // Not oracle-derived: the early return is the same branch as oracle row 22, which only
    // checks the observable calls.
    let effects = calais_main(false);
    assert_eq!(effects.len(), 2);
    assert!(!effects.iter().any(|e| matches!(
        e,
        MainEffect::SetStatus(_) | MainEffect::WaitForStopEvent { .. }
    )));
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the export table comes from dlls/scardsvr/scardsvr.spec.
    assert_eq!(
        EXPORTS,
        [
            ("CalaisMain", ExportKind::Private),
            ("InitSmartCardService", ExportKind::Stub),
        ]
    );
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived. `SvchostPushServiceGlobals` is commented out in the spec file.
    assert_eq!(export_kind("CalaisMain"), Some(ExportKind::Private));
    assert_eq!(export_kind("InitSmartCardService"), Some(ExportKind::Stub));
    assert_eq!(export_kind("calaismain"), None);
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("SvchostPushServiceGlobals"), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the names are arbitrary inputs to the stub helper.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}
