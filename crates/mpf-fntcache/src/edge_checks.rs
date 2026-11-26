//! Edge-case tests for the ported fntcache logic.
//!
//! Each block says whether its expectations are oracle-derived. Expectations marked "not oracle-derived"
//! come from reading `main.c` or the Wine headers, not from the harness run.

use super::*;

#[test]
fn constants_match_headers() {
    // Not oracle-derived: values from target/repos/wine/include/winsvc.h, winnt.h and winerror.h.
    assert_eq!(SERVICE_CONTROL_STOP, 1);
    assert_eq!(SERVICE_CONTROL_SHUTDOWN, 5);
    assert_eq!(SERVICE_STOPPED, 1);
    assert_eq!(SERVICE_STOP_PENDING, 3);
    assert_eq!(SERVICE_RUNNING, 4);
    assert_eq!(SERVICE_ACCEPT_STOP, 1);
    assert_eq!(SERVICE_ACCEPT_SHUTDOWN, 4);
    assert_eq!(SERVICE_WIN32, 0x10 | 0x20);
    assert_eq!(NO_ERROR, 0);
}

#[test]
fn only_stop_and_shutdown_signal_the_stop_event() {
    // Not oracle-derived: sweeps 0..=20 around the two stop codes. The expectation is the switch in main.c.
    for ctrl in 0..=20 {
        let expect = ctrl == SERVICE_CONTROL_STOP || ctrl == SERVICE_CONTROL_SHUTDOWN;
        assert_eq!(service_handler(ctrl).stop_event_set, expect, "ctrl {ctrl}");
        assert_eq!(service_handler(ctrl).ret, NO_ERROR, "ctrl {ctrl}");
    }
}

#[test]
fn stop_and_shutdown_publish_stop_pending_with_nothing_accepted() {
    // Oracle-derived (rows ctrl 1 and 5). The wait hint and the other fields are zero.
    for ctrl in [SERVICE_CONTROL_STOP, SERVICE_CONTROL_SHUTDOWN] {
        let r = service_handler(ctrl);
        assert_eq!(r.status.current_state, SERVICE_STOP_PENDING);
        assert_eq!(r.status.controls_accepted, 0);
        assert_eq!(r.status.service_type, SERVICE_WIN32);
        assert_eq!(r.status.win32_exit_code, 0);
        assert_eq!(r.status.service_specific_exit_code, 0);
        assert_eq!(r.status.check_point, 0);
        assert_eq!(r.status.wait_hint, 0);
    }
}

#[test]
fn default_branch_keeps_stop_only_accepted() {
    // Oracle-derived (default rows). Bug-compatible: the default branch never widens
    // dwControlsAccepted to STOP|SHUTDOWN, even though ServiceMain advertises both.
    for ctrl in [0u32, 2, 3, 4, 6, 14, 0xFFFF_FFFF] {
        let r = service_handler(ctrl);
        assert_eq!(r.status.current_state, SERVICE_RUNNING, "ctrl {ctrl}");
        assert_eq!(
            r.status.controls_accepted, SERVICE_ACCEPT_STOP,
            "ctrl {ctrl}"
        );
    }
}

#[test]
fn unregistered_service_main_never_reads_controls() {
    // Not oracle-derived: a registration failure with controls queued. The oracle row only covers one control.
    let run = service_main(false, &[1, 5, 2]);
    assert!(run.returned);
    assert!(run.statuses.is_empty());
}

#[test]
fn stopped_status_keeps_running_wait_hint() {
    // Oracle-derived (row ctrl=[1]). The final STOPPED write reuses ServiceMain's status,
    // so it keeps wait_hint 10000 and accepts nothing.
    let run = service_main(true, &[1]);
    let last = run.statuses.last().copied().unwrap();
    assert_eq!(last.current_state, SERVICE_STOPPED);
    assert_eq!(last.controls_accepted, 0);
    assert_eq!(last.wait_hint, 10000);
}

#[test]
fn first_stop_ends_the_wait_and_later_controls_are_not_read() {
    // Oracle-derived for [1, 2] and [4, 1]. Not oracle-derived for [1, 5]: a second stop is
    // ignored the same way a default control is.
    let run = service_main(true, &[1, 5]);
    assert!(run.returned);
    assert_eq!(run.statuses.len(), 3);
    assert_eq!(run.statuses[1].current_state, SERVICE_STOP_PENDING);
}

#[test]
fn blocked_wait_reports_every_status_written_before_blocking() {
    // Oracle-derived for [] and [3, 4]; [3, 4, 0] is not. Each handled control adds one status, and the run never returns.
    assert_eq!(service_main(true, &[]).statuses.len(), 1, "RUNNING only");
    let run = service_main(true, &[3, 4, 0]);
    assert!(!run.returned);
    assert_eq!(run.statuses.len(), 4);
    assert!(run.statuses[1..]
        .iter()
        .all(|s| s.current_state == SERVICE_RUNNING));
}

#[test]
fn running_status_accepts_stop_and_shutdown() {
    // Oracle-derived (RUN row). ServiceMain publishes RUNNING with STOP|SHUTDOWN and wait hint 10000.
    let run = service_main(true, &[]);
    assert_eq!(run.statuses[0].current_state, SERVICE_RUNNING);
    assert_eq!(
        run.statuses[0].controls_accepted,
        SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN
    );
    assert_eq!(run.statuses[0].wait_hint, 10000);
}
