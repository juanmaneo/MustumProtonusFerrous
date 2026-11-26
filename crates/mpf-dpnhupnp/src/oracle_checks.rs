//! Oracle-derived test vectors for the `dpnhupnp` stubs.
//!
//! Vectors were captured by compiling the verbatim `dlls/dpnhupnp/main.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/dpnhupnp/` (minimal
//! Windows type shims only) and running the harness. The 24 printed rows came back
//! with no ASan or UBSan reports, so none were excluded.

use super::*;

const SENTINEL: usize = 0xdead;

fn sample_guid() -> Guid {
    Guid {
        data1: 0x1,
        data2: 0x2,
        data3: 0x3,
        data4: [0, 1, 2, 3, 4, 5, 6, 7],
    }
}

fn opt(present: bool, g: &Guid) -> Option<&Guid> {
    if present {
        Some(g)
    } else {
        None
    }
}

#[test]
fn oracle_direct_play_nat_help_create_rows() {
    // (iid present, ppv present, slot_init) -> (hr, slot after)
    let rows: [(bool, bool, usize, u32, usize); 8] = [
        (false, false, 0, 0x8000_4001, 0),
        (false, false, SENTINEL, 0x8000_4001, SENTINEL),
        (false, true, 0, 0x8000_4001, 0),
        (false, true, SENTINEL, 0x8000_4001, SENTINEL),
        (true, false, 0, 0x8000_4001, 0),
        (true, false, SENTINEL, 0x8000_4001, SENTINEL),
        (true, true, 0, 0x8000_4001, 0),
        (true, true, SENTINEL, 0x8000_4001, SENTINEL),
    ];
    let g = sample_guid();
    for (iid, ppv, init, hr, slot_after) in rows {
        let mut slot = init;
        let got = direct_play_nat_help_create(opt(iid, &g), ppv.then_some(&mut slot));
        assert_eq!(got as u32, hr, "iid={iid} ppv={ppv} init={init:#x}");
        assert_eq!(slot, slot_after, "iid={iid} ppv={ppv} init={init:#x}");
    }
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // (rclsid present, riid present, ppv present, slot_init) -> (hr, slot after)
    let rows: [(bool, bool, bool, usize, u32, usize); 16] = [
        (false, false, false, 0, 0x8000_4005, 0),
        (false, false, false, SENTINEL, 0x8000_4005, SENTINEL),
        (false, false, true, 0, 0x8000_4005, 0),
        (false, false, true, SENTINEL, 0x8000_4005, SENTINEL),
        (false, true, false, 0, 0x8000_4005, 0),
        (false, true, false, SENTINEL, 0x8000_4005, SENTINEL),
        (false, true, true, 0, 0x8000_4005, 0),
        (false, true, true, SENTINEL, 0x8000_4005, SENTINEL),
        (true, false, false, 0, 0x8000_4005, 0),
        (true, false, false, SENTINEL, 0x8000_4005, SENTINEL),
        (true, false, true, 0, 0x8000_4005, 0),
        (true, false, true, SENTINEL, 0x8000_4005, SENTINEL),
        (true, true, false, 0, 0x8000_4005, 0),
        (true, true, false, SENTINEL, 0x8000_4005, SENTINEL),
        (true, true, true, 0, 0x8000_4005, 0),
        (true, true, true, SENTINEL, 0x8000_4005, SENTINEL),
    ];
    let g = sample_guid();
    for (rclsid, riid, ppv, init, hr, slot_after) in rows {
        let mut slot = init;
        let got = dll_get_class_object(opt(rclsid, &g), opt(riid, &g), ppv.then_some(&mut slot));
        assert_eq!(
            got as u32, hr,
            "rclsid={rclsid} riid={riid} ppv={ppv} init={init:#x}"
        );
        assert_eq!(
            slot, slot_after,
            "rclsid={rclsid} riid={riid} ppv={ppv} init={init:#x}"
        );
    }
}
