//! Oracle-derived test vectors for the ported wevtsvc logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `eventlog_update_status`,
//! `eventlog_handler` and `ServiceMain` (`dlls/wevtsvc/wevtsvc.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/wevtsvc/` (`harness.c`), with the
//! Win32 calls replaced by recorders. The harness printed 19 rows and ASan and UBSan flagged
//! none, so no vector is excluded.

use super::*;

/// The `SERVICE_STATUS` fields that every row of the harness reports, besides `current_state`.
fn record(state: u32) -> ServiceStatus {
    ServiceStatus {
        service_type: 48,
        current_state: state,
        controls_accepted: 5,
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }
}

/// `(control, reported state, done_event signalled)` for rows 03 to 19.
const HANDLER_ROWS: [(u32, u32, u32); 17] = [
    (0, 4, 0),
    (1, 3, 1),
    (2, 4, 0),
    (3, 4, 0),
    (4, 4, 0),
    (5, 3, 1),
    (6, 4, 0),
    (7, 4, 0),
    (15, 4, 0),
    (16, 4, 0),
    (127, 4, 0),
    (128, 4, 0),
    (256, 4, 0),
    (65, 4, 0),
    (2_147_483_646, 4, 0),
    (4_294_967_294, 4, 0),
    (4_294_967_295, 4, 0),
];

#[test]
fn oracle_service_main_rows() {
    // row 01 ServiceMain register_ok=0 -> seq=[] events=0
    assert_eq!(service_main_states(false), Vec::<u32>::new());
    // row 02 ServiceMain register_ok=1 -> seq=[4,1] events=0
    assert_eq!(service_main_states(true), [4, 1]);
}

#[test]
fn oracle_handler_rows() {
    for (control, state, events) in HANDLER_ROWS {
        let action = handler_action(control);
        assert_eq!(action.reported_state, state, "control={control}");
        assert_eq!(u32::from(action.signal_done), events, "control={control}");
        assert_eq!(service_status(action.reported_state), record(state));
    }
}
