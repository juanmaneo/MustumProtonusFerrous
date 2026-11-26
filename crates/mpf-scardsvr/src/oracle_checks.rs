//! Oracle-derived test vectors for the ported scardsvr logic.
//!
//! Vectors were captured by compiling the verbatim `dlls/scardsvr/scardsvr.c` with recording
//! mocks for its Win32 calls, and the verbatim `__wine_spec_unimplemented_stub` from
//! `dlls/ntdll/exception.c` with `RtlRaiseException` replaced by a recorder, using
//! `gcc -fsanitize=address,undefined -g` under `target/canon/scardsvr/`. The harness printed
//! 23 rows. ASan and UBSan flagged none of them, so no vector is excluded.

use super::*;

/// (ctrl, current_state, controls_accepted, sets_stop_event, ret) from oracle rows 01-17.
/// Every row reports `dwServiceType` 0x10 and zero exit codes, checkpoint and wait hint, and
/// calls `SetServiceStatus` once.
const HANDLER_ROWS: [(u32, u32, u32, bool, u32); 17] = [
    (0, 4, 1, false, 0),
    (1, 3, 0, true, 0),
    (2, 4, 1, false, 0),
    (3, 4, 1, false, 0),
    (4, 4, 1, false, 0),
    (5, 3, 0, true, 0),
    (6, 4, 1, false, 0),
    (7, 4, 1, false, 0),
    (8, 4, 1, false, 0),
    (9, 4, 1, false, 0),
    (10, 4, 1, false, 0),
    (11, 4, 1, false, 0),
    (12, 4, 1, false, 0),
    (13, 4, 1, false, 0),
    (14, 4, 1, false, 0),
    (15, 4, 1, false, 0),
    (16, 4, 1, false, 0),
];

#[test]
fn oracle_service_handler_rows() {
    for (row, &(ctrl, state, accepts, sets_event, ret)) in HANDLER_ROWS.iter().enumerate() {
        let expected = HandlerOutcome {
            status: ServiceStatus {
                service_type: 0x10,
                current_state: state,
                controls_accepted: accepts,
                win32_exit_code: 0,
                service_specific_exit_code: 0,
                check_point: 0,
                wait_hint: 0,
            },
            sets_stop_event: sets_event,
            ret,
        };
        assert_eq!(service_handler(ctrl), expected, "row {:02}", row + 1);
    }
}

#[test]
fn oracle_start_status_row() {
    // row 18 main reg=ok call=1 status type=0x10 state=4 accepts=0x5 win32=0 svc=0 cp=0 wait=10000
    assert_eq!(
        start_status(),
        ServiceStatus {
            service_type: 0x10,
            current_state: 4,
            controls_accepted: 5,
            win32_exit_code: 0,
            service_specific_exit_code: 0,
            check_point: 0,
            wait_hint: 10000,
        }
    );
}

#[test]
fn oracle_calais_main_wait_row() {
    // row 19 main reg=ok call=2 wait_timeout=4294967295 wait_calls=1
    assert_eq!(
        calais_main(true)[3],
        MainEffect::WaitForStopEvent {
            timeout: 4_294_967_295
        }
    );
}

#[test]
fn oracle_stopped_status_row() {
    // row 20 main reg=ok call=3 status type=0x10 state=1 accepts=0 win32=0 svc=0 cp=0 wait=10000 total_set_status=2
    assert_eq!(
        stopped_status(),
        ServiceStatus {
            service_type: 0x10,
            current_state: 1,
            controls_accepted: 0,
            win32_exit_code: 0,
            service_specific_exit_code: 0,
            check_point: 0,
            wait_hint: 10000,
        }
    );
}

#[test]
fn oracle_calais_main_effect_rows() {
    // row 21 main reg=ok name=scardsvr handler_set=1 create_event=1
    // row 22 main reg=fail set_status=0 wait_calls=0 create_event=1 name=scardsvr
    assert_eq!(
        calais_main(true),
        [
            MainEffect::CreateStopEvent,
            MainEffect::RegisterHandler { name: "scardsvr" },
            MainEffect::SetStatus(start_status()),
            MainEffect::WaitForStopEvent { timeout: INFINITE },
            MainEffect::SetStatus(stopped_status()),
        ]
    );
    assert_eq!(
        calais_main(false),
        [
            MainEffect::CreateStopEvent,
            MainEffect::RegisterHandler { name: "scardsvr" },
        ]
    );
}

#[test]
fn oracle_init_smart_card_service_row() {
    // row 23 stub InitSmartCardService -> raised=1 code=0x80000100 flags=0x1 nargs=2 p0=scardsvr.dll p1=InitSmartCardService
    assert_eq!(
        init_smart_card_service(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "scardsvr.dll",
            function: "InitSmartCardService",
        }
    );
}
