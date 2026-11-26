//! Oracle-derived test vectors for the ported msimg32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `vSetDdrawflag`
//! (`dlls/msimg32/msimg32_main.c`) and `DllMain` (`libs/winecrt0/dll_main.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/msimg32/`, and running the harness.
//! The 20 printed rows are all kept: 8 `vSetDdrawflag` rows and 12 `DllMain` rows. ASan and UBSan
//! flagged none of them, so none are excluded.

use super::*;

#[test]
fn oracle_v_set_ddrawflag_rows() {
    // (last error before the call, last error after the call), from `ddrawflag prior=.. out=..`
    let rows: [(u32, u32); 8] = [
        (0, 120),
        (1, 120),
        (5, 120),
        (87, 120),
        (120, 120),
        (2147483647, 120),
        (2147483648, 120),
        (4294967295, 120),
    ];
    for (prior, out) in rows {
        let mut last_error = prior;
        v_set_ddrawflag(&mut last_error);
        assert_eq!(last_error, out, "prior last error {prior}");
    }
}

#[test]
fn oracle_dll_main_rows() {
    // (inst, reason, inst passed to DisableThreadLibraryCalls), from `dllmain inst=.. reason=..`.
    // `None` means the C body did not call it. Every row returns TRUE.
    let rows: [(usize, u32, Option<usize>); 12] = [
        (0, 0, None),
        (0, 1, Some(0)),
        (0, 2, None),
        (0, 3, None),
        (0, 4, None),
        (0, 4294967295, None),
        (4096, 0, None),
        (4096, 1, Some(4096)),
        (4096, 2, None),
        (4096, 3, None),
        (4096, 4, None),
        (4096, 4294967295, None),
    ];
    for (inst, reason, disabled) in rows {
        assert_eq!(
            dll_main(inst, reason),
            DllMainOutcome {
                ret: true,
                disable_thread_library_calls: disabled,
            },
            "inst {inst} reason {reason}"
        );
    }
}
