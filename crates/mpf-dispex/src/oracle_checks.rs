//! Oracle-derived test vectors for the ported dispex logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/dispex/usrmarshal.c` (lines 38-171) with the minimal Windows type shims
//! in `target/canon/dispex/harness.c`, using
//! `gcc -fsanitize=address,undefined -g`. The harness printed 23 rows
//! (`target/canon/dispex/rows.txt`): 17 proxy-to-stub round trips and 6 direct stub calls.
//! ASan and UBSan flagged none of them, so no vectors are excluded (0 excluded).
//! The `VARIANT` shim is 16 bytes and `UINT` is 4 bytes, the sizes used for `copy_block_size`.
//! The test-side composition below mirrors the C call order; it is not a port of Wine code.

use super::*;

const VT_I4: u16 = 3;
const DISP_E_MEMBERNOTFOUND: u32 = 0x8002_0003;
const VT_BYREF_I4: u16 = 0x4003;
const VARIANT_SIZE: u32 = 16;
const UINT_SIZE: u32 = 4;

#[derive(Clone, Copy)]
struct Callee {
    hr: u32,
    mutate: bool,
    fill: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct Seen {
    wflags: u16,
    result_null: bool,
    excep_null: bool,
    vts: Vec<u16>,
}

#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    hr: u32,
    seen: Option<Seen>,
    fill: bool,
    alloc_requested: Option<u32>,
    after: Vec<u16>,
}

fn seen(wflags: u16, result_null: bool, excep_null: bool, vts: &[u16]) -> Option<Seen> {
    Some(Seen {
        wflags,
        result_null,
        excep_null,
        vts: vts.to_vec(),
    })
}

/// Callee-side VT change for the mutate rows: flips slot 0 between `VT_I4` and 2.
fn mutate_first(vts: &mut [u16]) {
    vts[0] = if vts[0] == VT_I4 { 2 } else { VT_I4 };
}

/// Mirrors `IDispatchEx_InvokeEx_Proxy` -> `IDispatchEx_InvokeEx_Stub` -> the callee shim.
fn run_proxy(
    w_flags: u16,
    res_given: bool,
    excep_given: bool,
    vts: &[u16],
    alloc_ok: bool,
    callee: Callee,
) -> Outcome {
    let byref = vts.iter().filter(|&&vt| vt & VT_BYREF != 0).count() as u32;
    let alloc_requested =
        (byref > 0).then(|| copy_block_size(vts.len() as u32, byref, VARIANT_SIZE, UINT_SIZE));
    let flags = proxy_flags(w_flags, res_given, excep_given);
    let Some(split) = split_args(vts, alloc_ok) else {
        return Outcome {
            hr: E_OUTOFMEMORY,
            seen: None,
            fill: false,
            alloc_requested,
            after: vts.to_vec(),
        };
    };
    let sf = stub_flags(flags);
    let mut now = seed_ref_args(&split);
    let before = now.clone();
    let seen_now = Seen {
        wflags: sf.invoke_flags,
        result_null: sf.result_null,
        excep_null: sf.excep_null,
        vts: before.clone(),
    };
    if callee.mutate && !now.is_empty() {
        mutate_first(&mut now);
    }
    let hr = callee_result(callee.hr, &before, &now, byref as usize);
    let fill = deferred_fill_in_runs(hr, !sf.excep_null, callee.fill);
    clear_ref_slots(&mut now, &split.ref_idx);
    Outcome {
        hr,
        seen: Some(seen_now),
        fill,
        alloc_requested,
        after: if byref > 0 { vts.to_vec() } else { now },
    }
}

const OK: Callee = Callee {
    hr: S_OK,
    mutate: false,
    fill: false,
};

#[test]
fn oracle_proxy_rows_1_to_6() {
    // P01_plain
    assert_eq!(
        run_proxy(1, true, true, &[3, 2], true, OK),
        Outcome {
            hr: 0,
            seen: seen(1, false, false, &[3, 2]),
            fill: false,
            alloc_requested: None,
            after: vec![3, 2],
        }
    );
    // P02_flag_mask
    assert_eq!(
        run_proxy(0xffff, true, true, &[3], true, OK),
        Outcome {
            hr: 0,
            seen: seen(0xf, false, false, &[3]),
            fill: false,
            alloc_requested: None,
            after: vec![3],
        }
    );
    // P03_null_result
    assert_eq!(
        run_proxy(1, false, true, &[3], true, OK),
        Outcome {
            hr: 0,
            seen: seen(1, true, false, &[3]),
            fill: false,
            alloc_requested: None,
            after: vec![3],
        }
    );
    // P04_null_excep
    assert_eq!(
        run_proxy(2, true, false, &[3], true, OK),
        Outcome {
            hr: 0,
            seen: seen(2, false, true, &[3]),
            fill: false,
            alloc_requested: None,
            after: vec![3],
        }
    );
    // P05_both_null
    assert_eq!(
        run_proxy(0x10, false, false, &[3], true, OK),
        Outcome {
            hr: 0,
            seen: seen(0, true, true, &[3]),
            fill: false,
            alloc_requested: None,
            after: vec![3],
        }
    );
    // P06_byref_one
    assert_eq!(
        run_proxy(1, true, true, &[VT_BYREF_I4, 2], true, OK),
        Outcome {
            hr: 0,
            seen: seen(1, false, false, &[VT_BYREF_I4, 2]),
            fill: false,
            alloc_requested: Some(52),
            after: vec![VT_BYREF_I4, 2],
        }
    );
}

#[test]
fn oracle_proxy_rows_7_to_12() {
    let mutate = Callee {
        hr: S_OK,
        mutate: true,
        fill: false,
    };
    // P07_byref_mutated
    assert_eq!(
        run_proxy(1, true, true, &[VT_BYREF_I4, 2], true, mutate),
        Outcome {
            hr: DISP_E_BADCALLEE,
            seen: seen(1, false, false, &[VT_BYREF_I4, 2]),
            fill: false,
            alloc_requested: Some(52),
            after: vec![VT_BYREF_I4, 2],
        }
    );
    // P08_byref_mut_fail
    assert_eq!(
        run_proxy(
            1,
            true,
            true,
            &[VT_BYREF_I4, 2],
            true,
            Callee {
                hr: DISP_E_MEMBERNOTFOUND,
                ..mutate
            }
        ),
        Outcome {
            hr: DISP_E_MEMBERNOTFOUND,
            seen: seen(1, false, false, &[VT_BYREF_I4, 2]),
            fill: false,
            alloc_requested: Some(52),
            after: vec![VT_BYREF_I4, 2],
        }
    );
    // P09_exception_fill
    assert_eq!(
        run_proxy(
            1,
            true,
            true,
            &[3],
            true,
            Callee {
                hr: DISP_E_EXCEPTION,
                mutate: false,
                fill: true
            }
        ),
        Outcome {
            hr: DISP_E_EXCEPTION,
            seen: seen(1, false, false, &[3]),
            fill: true,
            alloc_requested: None,
            after: vec![3],
        }
    );
    // P10_exc_no_pei
    assert_eq!(
        run_proxy(
            1,
            true,
            false,
            &[3],
            true,
            Callee {
                hr: DISP_E_EXCEPTION,
                mutate: false,
                fill: true
            }
        ),
        Outcome {
            hr: DISP_E_EXCEPTION,
            seen: seen(1, false, true, &[3]),
            fill: false,
            alloc_requested: None,
            after: vec![3],
        }
    );
    // P11_exc_byref_mut
    assert_eq!(
        run_proxy(
            1,
            true,
            true,
            &[VT_BYREF_I4, 2],
            true,
            Callee {
                hr: DISP_E_EXCEPTION,
                mutate: true,
                fill: true
            }
        ),
        Outcome {
            hr: DISP_E_EXCEPTION,
            seen: seen(1, false, false, &[VT_BYREF_I4, 2]),
            fill: true,
            alloc_requested: Some(52),
            after: vec![VT_BYREF_I4, 2],
        }
    );
    // P12_oom_byref
    assert_eq!(
        run_proxy(1, true, true, &[VT_BYREF_I4, 2], false, OK),
        Outcome {
            hr: E_OUTOFMEMORY,
            seen: None,
            fill: false,
            alloc_requested: Some(52),
            after: vec![VT_BYREF_I4, 2],
        }
    );
}

#[test]
fn oracle_proxy_rows_13_to_17() {
    // P13_oom_no_byref
    assert_eq!(
        run_proxy(1, true, true, &[3], false, OK),
        Outcome {
            hr: 0,
            seen: seen(1, false, false, &[3]),
            fill: false,
            alloc_requested: None,
            after: vec![3],
        }
    );
    // P14_size_4args_2ref
    assert_eq!(
        run_proxy(1, true, true, &[VT_BYREF_I4, VT_BYREF_I4, 2, 3], true, OK),
        Outcome {
            hr: 0,
            seen: seen(1, false, false, &[VT_BYREF_I4, VT_BYREF_I4, 2, 3]),
            fill: false,
            alloc_requested: Some(104),
            after: vec![VT_BYREF_I4, VT_BYREF_I4, 2, 3],
        }
    );
    // P15_size_1ref
    assert_eq!(
        run_proxy(1, true, true, &[0x4001], true, OK),
        Outcome {
            hr: 0,
            seen: seen(1, false, false, &[0x4001]),
            fill: false,
            alloc_requested: Some(36),
            after: vec![0x4001],
        }
    );
    // P16_mixed_byref
    assert_eq!(
        run_proxy(1, true, true, &[2, 0x4002, 3, 0x4003, 0x4000], true, OK),
        Outcome {
            hr: 0,
            seen: seen(1, false, false, &[2, 0x4002, 3, 0x4003, 0x4000]),
            fill: false,
            alloc_requested: Some(140),
            after: vec![2, 0x4002, 3, 0x4003, 0x4000],
        }
    );
    // P17_zero_args
    assert_eq!(
        run_proxy(1, true, true, &[], true, OK),
        Outcome {
            hr: 0,
            seen: seen(1, false, false, &[]),
            fill: false,
            alloc_requested: None,
            after: vec![],
        }
    );
}

/// Direct stub call: `dw_flags`, copy VTs, and an optional `(ref_idx, ref_vt)`.
/// Returns `(hr, seen_flags, seen_vts, after_vts)`.
fn run_stub(
    dw_flags: u32,
    copy: &[u16],
    byref: Option<(u32, u16)>,
    callee: Callee,
) -> (u32, StubFlags, Vec<u16>, Vec<u16>) {
    let (ref_idx, ref_vts) = match byref {
        Some((idx, vt)) => (vec![idx], vec![vt]),
        None => (vec![], vec![]),
    };
    let split = ArgSplit {
        copy_vts: copy.to_vec(),
        ref_vts,
        ref_idx,
    };
    let sf = stub_flags(dw_flags);
    let mut now = seed_ref_args(&split);
    let before = now.clone();
    if callee.mutate && !now.is_empty() {
        mutate_first(&mut now);
    }
    let hr = callee_result(callee.hr, &before, &now, split.ref_idx.len());
    clear_ref_slots(&mut now, &split.ref_idx);
    (hr, sf, before, now)
}

#[test]
fn oracle_stub_rows() {
    // S01_high_bits_dropped
    let (hr, sf, seen_vts, after) = run_stub(0x10005, &[3, 2], None, OK);
    assert_eq!(hr, 0);
    assert_eq!(
        sf,
        StubFlags {
            invoke_flags: 5,
            result_null: false,
            excep_null: false
        }
    );
    assert_eq!(seen_vts, vec![3, 2]);
    assert_eq!(after, vec![3, 2]);
    // S02_null_result_bit
    let (hr, sf, seen_vts, _) = run_stub(0x20000, &[3], None, OK);
    assert_eq!(hr, 0);
    assert_eq!(
        sf,
        StubFlags {
            invoke_flags: 0,
            result_null: true,
            excep_null: false
        }
    );
    assert_eq!(seen_vts, vec![3]);
    // S03_null_excep_bit
    let (hr, sf, _, _) = run_stub(0x40000, &[3], None, OK);
    assert_eq!(hr, 0);
    assert_eq!(
        sf,
        StubFlags {
            invoke_flags: 0,
            result_null: false,
            excep_null: true
        }
    );
    // S04_all_bits
    let (hr, sf, _, _) = run_stub(0xffff_ffff, &[3], None, OK);
    assert_eq!(hr, 0);
    assert_eq!(
        sf,
        StubFlags {
            invoke_flags: 0xffff,
            result_null: true,
            excep_null: true
        }
    );
    // S05_ref_seeded_cleared
    let (hr, _, seen_vts, after) = run_stub(1, &[3, 0], Some((1, 0x4002)), OK);
    assert_eq!(hr, 0);
    assert_eq!(seen_vts, vec![3, 0x4002]);
    assert_eq!(after, vec![3, 0]);
    // S06_ref_mutated_badcallee
    let (hr, _, seen_vts, after) = run_stub(
        1,
        &[3, 0],
        Some((1, 0x4002)),
        Callee {
            hr: S_OK,
            mutate: true,
            fill: false,
        },
    );
    assert_eq!(hr, DISP_E_BADCALLEE);
    assert_eq!(seen_vts, vec![3, 0x4002]);
    assert_eq!(after, vec![2, 0]);
}
