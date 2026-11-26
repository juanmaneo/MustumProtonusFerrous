//! Oracle-derived test vectors for the ported icmui logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `SetupColorMatchingW`
//! (`dlls/icmui/icmui.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/icmui/`, and running the
//! harness. All 16 printed rows ran clean under ASan and UBSan, so none are excluded.

use super::*;

/// `(row, cms as the harness passed it, the last error before the call, ret, last_error)`.
/// The port takes no `cms`, so the input columns document the harness inputs only.
const SETUP_COLOR_MATCHING_W_ROWS: [(u32, &str, u32, bool, u32); 12] = [
    (1, "NULL", 0, false, 87),
    (2, "NULL", 5, false, 87),
    (3, "0x1", 0, false, 87),
    (4, "0x3", 0, false, 87),
    (5, "0x1000", 0xffff_ffff, false, 87),
    (6, "zeroed", 0, false, 87),
    (7, "filled_ff", 0, false, 87),
    (8, "zeroed", 87, false, 87),
    (9, "filled_ff", 120, false, 87),
    (10, "NULL", 87, false, 87),
    (11, "0xffffffffffffffff", 0, false, 87),
    (12, "zeroed+1", 0, false, 87),
];

/// `(row, the last error before the call)`. Every row raised `EXCEPTION_WINE_STUB` once with
/// the record fields the port checks below, and left the last error unchanged.
const SETUP_COLOR_MATCHING_A_ROWS: [(u32, u32); 4] =
    [(13, 0), (14, 87), (15, 120), (16, 0xffff_ffff)];

#[test]
fn oracle_setup_color_matching_w_rows() {
    for (row, cms, prev_error, ret, last_error) in SETUP_COLOR_MATCHING_W_ROWS {
        assert_eq!(
            setup_color_matching_w(),
            SetupColorMatchingWResult { ret, last_error },
            "row {row:02} cms={cms} prev_err={prev_error}"
        );
    }
}

#[test]
fn oracle_setup_color_matching_a_stub_rows() {
    // Each row printed raised=1 code=0x80000100 flags=0x1 nparams=2 record_nested=1
    // addr_set=1 module=icmui.dll function=SetupColorMatchingA.
    for (row, prev_error) in SETUP_COLOR_MATCHING_A_ROWS {
        assert_eq!(
            setup_color_matching_a(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "icmui.dll",
                function: "SetupColorMatchingA",
            },
            "row {row} prev_err={prev_error}"
        );
    }
}
