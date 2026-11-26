//! Oracle-derived test vectors for the ported srvsvc logic.
//!
//! Vectors were captured by compiling the verbatim Wine `dlls/srvsvc/srvsvc.c` under
//! `target/canon/srvsvc/` with `gcc -fsanitize=address,undefined -g`, with shims for the Windows
//! types and API. `harness.c` includes that file and records the `SetServiceStatus`,
//! `SetEvent`, `RegisterServiceCtrlHandlerW` and `WaitForSingleObject` calls. The 18 printed rows
//! are all kept: ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

/// The record for a state, with the fixed fields the C `svc_update_status` sets. The oracle
/// prints `type=0x30 accepted=0x5` and zero for every other field.
fn record(state: u32) -> ServiceStatus {
    ServiceStatus {
        service_type: 0x30,
        current_state: state,
        controls_accepted: 0x5,
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }
}

fn records(states: &[u32]) -> Vec<ServiceStatus> {
    states.iter().map(|&state| record(state)).collect()
}

/// One row of the `ServiceMain` oracle: register result, controls, returned flag, reported states.
type ServiceRow = (u32, bool, &'static [u32], bool, &'static [u32]);

#[test]
fn oracle_svc_handler_rows() {
    // (row, control, reported state, sets the done event)
    let rows: [(u32, u32, u32, bool); 9] = [
        (1, 0, 4, false),
        (2, 1, 3, true),
        (3, 2, 4, false),
        (4, 3, 4, false),
        (5, 4, 4, false),
        (6, 5, 3, true),
        (7, 6, 4, false),
        (8, 0x7f, 4, false),
        (9, 0xffff_ffff, 4, false),
    ];
    for (row, control, state, set_event) in rows {
        let outcome = svc_handler(control);
        assert_eq!(
            outcome,
            ControlOutcome {
                status: record(state),
                set_done_event: set_event,
            },
            "row {row:02}"
        );
    }
}

#[test]
fn oracle_service_main_rows() {
    // (row, register ok, controls, returned, reported states)
    let rows: [ServiceRow; 9] = [
        (10, false, &[], true, &[]),
        (11, true, &[1], true, &[2, 4, 3, 1]),
        (12, true, &[5], true, &[2, 4, 3, 1]),
        (13, true, &[], false, &[2, 4]),
        (14, true, &[2, 4], false, &[2, 4, 4, 4]),
        (15, true, &[2, 1, 5], true, &[2, 4, 4, 3, 1]),
        (16, true, &[0, 5, 1], true, &[2, 4, 4, 3, 1]),
        (17, false, &[1], true, &[]),
        (18, true, &[1, 1], true, &[2, 4, 3, 1]),
    ];
    for (row, register_ok, controls, returned, states) in rows {
        let run = service_main(register_ok, controls);
        assert_eq!(run.registered_name, "LanmanServer", "row {row}");
        assert_eq!(run.returned, returned, "row {row}");
        assert_eq!(run.statuses, records(states), "row {row}");
    }
}
