//! Oracle-derived test vectors for the ported mtxdm logic.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `GetDispenserManager`
//! (`dlls/mtxdm/mtxdm_main.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/mtxdm/`. The harness replaces `CoCreateInstance` with a recorder that returns
//! the row's HRESULT. The 18 printed rows raised no ASan or UBSan reports, so none are excluded.

use super::*;

/// One entry per printed row, in print order. Each entry is whether the `dispenser` argument was
/// NULL, and the HRESULT the recorder returned. In every printed row the recorder saw one call,
/// `ret_out` equalled `ret_in`, `outer` was NULL, `clsctx` was `0x17`, the class was
/// `ecabb0c0-7f19-11d2-978e-0000f8757e2a`, the interface was
/// `5cb31e10-2b5f-11cf-be10-00aa00a2fa25`, and `ppv` was the pointer the C code was given.
const ORACLE_ROWS: [(bool, u32); 18] = [
    (true, 0x0000_0000),
    (true, 0x8000_4002),
    (true, 0x8004_0154),
    (true, 0x8000_4003),
    (true, 0x8007_0057),
    (true, 0x8007_000E),
    (true, 0x8008_0005),
    (true, 0x8000_4005),
    (true, 0x8000_4001),
    (false, 0x0000_0000),
    (false, 0x8000_4002),
    (false, 0x8004_0154),
    (false, 0x8000_4003),
    (false, 0x8007_0057),
    (false, 0x8007_000E),
    (false, 0x8008_0005),
    (false, 0x8000_4005),
    (false, 0x8000_4001),
];

#[test]
fn oracle_get_dispenser_manager_rows() {
    for (row, &(dispenser_null, ret)) in ORACLE_ROWS.iter().enumerate() {
        let mut slot: usize = 0;
        let dispenser = if dispenser_null {
            None
        } else {
            Some(&mut slot)
        };
        let expected_ppv = dispenser.as_deref().map(|p| p as *const usize as usize);

        let mut seen = None;
        let got = get_dispenser_manager(dispenser, |clsid, clsctx, iid, ppv| {
            seen = Some((
                *clsid,
                clsctx,
                *iid,
                ppv.as_deref().map(|p| p as *const usize as usize),
            ));
            ret as HRESULT
        });

        let (clsid, clsctx, iid, ppv) = seen.expect("CoCreateInstance was called");
        assert_eq!(got, ret as HRESULT, "row {}", row + 1);
        assert_eq!(clsid, CLSID_DISPENSER_MANAGER, "row {}", row + 1);
        assert_eq!(clsctx, 0x17, "row {}", row + 1);
        assert_eq!(iid, IID_IDISPENSER_MANAGER, "row {}", row + 1);
        assert_eq!(ppv, expected_ppv, "row {}", row + 1);
    }
}
