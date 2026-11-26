//! Oracle-derived test vectors for the ported fntcache logic.
//!
//! The `service_handler` and `ServiceMain` rows were captured by compiling the verbatim bodies from
//! `dlls/fntcache/main.c` (lines 28-29, 31-58 and 63-90) with minimal Windows shims under
//! `target/canon/fntcache/`, using `gcc -fsanitize=address,undefined -g`, and running the harness.
//! ASan and UBSan reported nothing, so no vectors were excluded: 22 rows (12 handler, 10 ServiceMain).
//! The `SERVICE_STATUS` fields are listed as `[type, state, accepted, win32_exit, service_exit,
//! checkpoint, wait_hint]`. The export rows come from the reference source (`fntcache.spec` and `main.c`),
//! not from executing C.

use super::*;

/// Flattens a status into its seven `SERVICE_STATUS` fields, in declaration order.
fn fields(s: &ServiceStatus) -> [u32; 7] {
    [
        s.service_type,
        s.current_state,
        s.controls_accepted,
        s.win32_exit_code,
        s.service_specific_exit_code,
        s.check_point,
        s.wait_hint,
    ]
}

#[test]
fn oracle_service_handler_rows() {
    // (ctrl, ret, status, stop_event_set)
    let rows: [(u32, u32, [u32; 7], bool); 12] = [
        (0, 0, [48, 4, 1, 0, 0, 0, 0], false),
        (1, 0, [48, 3, 0, 0, 0, 0, 0], true),
        (2, 0, [48, 4, 1, 0, 0, 0, 0], false),
        (3, 0, [48, 4, 1, 0, 0, 0, 0], false),
        (4, 0, [48, 4, 1, 0, 0, 0, 0], false),
        (5, 0, [48, 3, 0, 0, 0, 0, 0], true),
        (6, 0, [48, 4, 1, 0, 0, 0, 0], false),
        (7, 0, [48, 4, 1, 0, 0, 0, 0], false),
        (8, 0, [48, 4, 1, 0, 0, 0, 0], false),
        (14, 0, [48, 4, 1, 0, 0, 0, 0], false),
        (15, 0, [48, 4, 1, 0, 0, 0, 0], false),
        (0xFFFF_FFFF, 0, [48, 4, 1, 0, 0, 0, 0], false),
    ];
    for (ctrl, ret, status, set) in rows {
        let r = service_handler(ctrl);
        assert_eq!(r.ret, ret, "ctrl {ctrl}");
        assert_eq!(fields(&r.status), status, "ctrl {ctrl}");
        assert_eq!(r.stop_event_set, set, "ctrl {ctrl}");
    }
}

#[test]
fn oracle_service_main_rows() {
    type MainRow = (bool, &'static [u32], bool, &'static [[u32; 7]]);
    // RUNNING as published by ServiceMain (accepts STOP|SHUTDOWN, wait hint 10000).
    const RUN: [u32; 7] = [48, 4, 5, 0, 0, 0, 10000];
    // RUNNING as published by the handler's default branch (accepts STOP only).
    const DEF: [u32; 7] = [48, 4, 1, 0, 0, 0, 0];
    // STOP_PENDING from the handler (accepts nothing).
    const PEND: [u32; 7] = [48, 3, 0, 0, 0, 0, 0];
    // Final STOPPED write: `status` is reused, so the wait hint of RUN is still there.
    const STOPPED: [u32; 7] = [48, 1, 0, 0, 0, 0, 10000];
    // (registered, controls, returned, statuses)
    let rows: [MainRow; 10] = [
        (false, &[], true, &[]),
        (false, &[1], true, &[]),
        (true, &[], false, &[RUN]),
        (true, &[1], true, &[RUN, PEND, STOPPED]),
        (true, &[5], true, &[RUN, PEND, STOPPED]),
        (true, &[2], false, &[RUN, DEF]),
        (true, &[4, 1], true, &[RUN, DEF, PEND, STOPPED]),
        (true, &[1, 2], true, &[RUN, PEND, STOPPED]),
        (true, &[6, 0, 5], true, &[RUN, DEF, DEF, PEND, STOPPED]),
        (true, &[3, 4], false, &[RUN, DEF, DEF]),
    ];
    for (registered, controls, returned, statuses) in rows {
        let run = service_main(registered, controls);
        assert_eq!(
            run.returned, returned,
            "registered {registered} controls {controls:?}"
        );
        let got: Vec<[u32; 7]> = run.statuses.iter().map(fields).collect();
        assert_eq!(
            got, statuses,
            "registered {registered} controls {controls:?}"
        );
    }
}

#[test]
fn oracle_spec_export_rows() {
    // Reference source: fntcache.spec lists three exports. main.c defines only ServiceMain
    // (line 63). A grep of dlls/fntcache finds no DllRegisterServer or DllUnregisterServer body.
    assert_eq!(
        SPEC_EXPORTS,
        ["DllRegisterServer", "DllUnregisterServer", "ServiceMain"]
    );
    assert!(!defined_in_main_c("DllRegisterServer"));
    assert!(!defined_in_main_c("DllUnregisterServer"));
    assert!(defined_in_main_c("ServiceMain"));
}
