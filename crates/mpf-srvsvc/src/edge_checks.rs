//! Edge-case tests for the ported srvsvc logic.
//!
//! These cover the control-code boundary around the stop codes, the registration failure path,
//! and the order of the reported states. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winsvc.h and include/winnt.h.
    assert_eq!(SERVICE_STOPPED, 1);
    assert_eq!(SERVICE_START_PENDING, 2);
    assert_eq!(SERVICE_STOP_PENDING, 3);
    assert_eq!(SERVICE_RUNNING, 4);
    assert_eq!(SERVICE_ACCEPT_STOP, 0x1);
    assert_eq!(SERVICE_ACCEPT_SHUTDOWN, 0x4);
    assert_eq!(SERVICE_WIN32, 0x30);
    assert_eq!(SERVICE_CONTROL_STOP, 1);
    assert_eq!(SERVICE_CONTROL_SHUTDOWN, 5);
    assert_eq!(SERVICE_NAME, "LanmanServer");
}

#[test]
fn svc_update_status_fixed_fields() {
    // Not oracle-derived: the fixed fields are the constant assignments on lines 37-42 of
    // srvsvc.c. The oracle rows confirm type and accepted, and that the rest are zero.
    let status = svc_update_status(SERVICE_STOPPED);
    assert_eq!(
        status,
        ServiceStatus {
            service_type: SERVICE_WIN32,
            current_state: SERVICE_STOPPED,
            controls_accepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
            win32_exit_code: 0,
            service_specific_exit_code: 0,
            check_point: 0,
            wait_hint: 0,
        }
    );
    assert_eq!(
        svc_update_status(SERVICE_RUNNING).current_state,
        SERVICE_RUNNING
    );
}

#[test]
fn only_stop_and_shutdown_signal_the_done_event() {
    // Not oracle-derived: the codes on either side of the two stop codes. The oracle covers
    // 0, 1, 2, 3, 4, 5, 6, 0x7f and 0xffffffff; this checks the same boundary with u32::MAX - 1.
    for control in [0, 2, 3, 4, 6, 0x7f, u32::MAX - 1, u32::MAX] {
        let outcome = svc_handler(control);
        assert!(!outcome.set_done_event, "control {control}");
        assert_eq!(
            outcome.status.current_state, SERVICE_RUNNING,
            "control {control}"
        );
    }
    for control in [SERVICE_CONTROL_STOP, SERVICE_CONTROL_SHUTDOWN] {
        let outcome = svc_handler(control);
        assert!(outcome.set_done_event, "control {control}");
        assert_eq!(
            outcome.status.current_state, SERVICE_STOP_PENDING,
            "control {control}"
        );
    }
}

#[test]
fn registration_failure_sends_nothing_and_ignores_controls() {
    // Not oracle-derived: the oracle row 17 covers one stop code. This checks that a failed
    // registration never reports a state, whatever the controls are.
    let run = service_main(false, &[SERVICE_CONTROL_STOP, 4, 4]);
    assert!(run.returned);
    assert!(run.statuses.is_empty());
    assert_eq!(run.registered_name, SERVICE_NAME);
}

#[test]
fn many_running_controls_then_stop_reports_in_order() {
    // Not oracle-derived: the oracle covers two running controls before a stop (row 14 and 15).
    // This runs longer sequences and checks the full state order.
    let run = service_main(true, &[4, 4, 4, 0, SERVICE_CONTROL_SHUTDOWN, 4]);
    assert!(run.returned);
    let states: Vec<u32> = run
        .statuses
        .iter()
        .map(|status| status.current_state)
        .collect();
    assert_eq!(states, [2, 4, 4, 4, 4, 4, 3, 1]);
}
