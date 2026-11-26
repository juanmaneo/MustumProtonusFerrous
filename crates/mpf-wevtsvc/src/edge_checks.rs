//! Edge-case tests for the ported wevtsvc logic.
//!
//! These cover the constant values, the control dispatch boundaries, and the fixed fields of the
//! status record. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winsvc.h. The harness used the same numbers.
    assert_eq!(SERVICE_WIN32, 0x30);
    assert_eq!(SERVICE_ACCEPT_STOP, 0x1);
    assert_eq!(SERVICE_ACCEPT_SHUTDOWN, 0x4);
    assert_eq!(SERVICE_STOPPED, 1);
    assert_eq!(SERVICE_STOP_PENDING, 3);
    assert_eq!(SERVICE_RUNNING, 4);
    assert_eq!(SERVICE_CONTROL_STOP, 1);
    assert_eq!(SERVICE_CONTROL_SHUTDOWN, 5);
}

#[test]
fn neighbours_of_stop_and_shutdown_do_not_signal() {
    // Not oracle-derived: the codes next to 1 and 5 take the default branch. The oracle rows
    // cover 0, 2, 4, 6 and the large values; these cover the rest of the range around them.
    for control in [SERVICE_CONTROL_STOP - 1, SERVICE_CONTROL_STOP + 1, 3, 4, 6] {
        let action = handler_action(control);
        assert!(!action.signal_done, "control={control}");
        assert_eq!(action.reported_state, SERVICE_RUNNING, "control={control}");
    }
    for control in [SERVICE_CONTROL_SHUTDOWN - 1, SERVICE_CONTROL_SHUTDOWN + 1] {
        let action = handler_action(control);
        assert!(!action.signal_done, "control={control}");
        assert_eq!(action.reported_state, SERVICE_RUNNING, "control={control}");
    }
}

#[test]
fn stop_and_shutdown_are_the_only_signalling_controls() {
    // Not oracle-derived: the two arms of the match, checked at the boundaries of u32.
    for control in [SERVICE_CONTROL_STOP, SERVICE_CONTROL_SHUTDOWN] {
        assert_eq!(
            handler_action(control),
            HandlerAction {
                reported_state: SERVICE_STOP_PENDING,
                signal_done: true,
            }
        );
    }
    assert_eq!(
        handler_action(u32::MAX),
        HandlerAction {
            reported_state: SERVICE_RUNNING,
            signal_done: false,
        }
    );
}

#[test]
fn status_fields_other_than_state_are_fixed() {
    // Not oracle-derived for the full range of states: the harness only reported 1, 3 and 4.
    // The C body sets the other fields without reading the state.
    for state in [
        0,
        SERVICE_STOPPED,
        SERVICE_STOP_PENDING,
        SERVICE_RUNNING,
        u32::MAX,
    ] {
        assert_eq!(
            service_status(state),
            ServiceStatus {
                service_type: SERVICE_WIN32,
                current_state: state,
                controls_accepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
                win32_exit_code: 0,
                service_specific_exit_code: 0,
                check_point: 0,
                wait_hint: 0,
            }
        );
    }
}
