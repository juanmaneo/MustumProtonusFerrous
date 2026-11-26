//! Oracle-derived test vectors for the ported `D3D10CoreCreateDevice` control flow and the
//! `D3D10CoreRegisterLayers` stub.
//!
//! Vectors were captured by compiling the verbatim Wine bodies from
//! `dlls/d3d10core/d3d10core_main.c` (lines 29-67) with Windows shims, under
//! `target/canon/d3d10core/`, using `gcc -fsanitize=address,undefined -g`. The harness printed
//! 22 rows: 1 for `register_layers` and 21 for create-device. ASan and UBSan reported nothing,
//! so no vector is excluded.

use super::*;

const S_FALSE: Hresult = 1;
const E_POINTER: Hresult = 0x8000_4003_u32 as Hresult;
const E_NOINTERFACE: Hresult = 0x8000_4002_u32 as Hresult;
const E_OUTOFMEMORY: Hresult = 0x8007_000e_u32 as Hresult;
const E_UNEXPECTED: Hresult = 0x8000_ffff_u32 as Hresult;
const DXGI_ERROR_NOT_FOUND: Hresult = 0x887a_0002_u32 as Hresult;
const DXGI_ERROR_UNSUPPORTED: Hresult = 0x887a_0004_u32 as Hresult;

/// Outputs the oracle observed for one `D3D10CoreCreateDevice` call.
struct Observed {
    hr: Hresult,
    create_calls: u32,
    seen_flags: u32,
    seen_levels: Vec<u32>,
    qi_calls: u32,
}

fn run(create_hr: Hresult, qi_hr: Hresult, flags: u32, feature_level: u32) -> Observed {
    let mut create_calls = 0;
    let mut seen_flags = 0;
    let mut seen_levels = Vec::new();
    let mut qi_calls = 0;
    let hr = d3d10_core_create_device(
        flags,
        feature_level,
        |seen, levels| {
            create_calls += 1;
            seen_flags = seen;
            seen_levels = levels.to_vec();
            create_hr
        },
        || {
            qi_calls += 1;
            qi_hr
        },
    );
    Observed {
        hr,
        create_calls,
        seen_flags,
        seen_levels,
        qi_calls,
    }
}

/// `(label, create_hr, qi_hr, flags, feature_level, expected_hr, qi_calls)`, from the oracle's
/// `create=`, `qi=`, `flags=`, `fl=`, `hr=` and `qi_calls=` columns. Every row also had
/// `create_calls=1`, `level_count=1`, `level0 == fl` and `flags_seen == flags`.
const ROWS: &[(&str, Hresult, Hresult, u32, u32, Hresult, u32)] = &[
    ("ok_ok_f0_a000", S_OK, S_OK, 0x0, 0xa000, S_OK, 1),
    ("ok_ok_bgra_b000", S_OK, S_OK, 0x20, 0xb000, S_OK, 1),
    (
        "ok_noiface_b000",
        S_OK,
        E_NOINTERFACE,
        0x0,
        0xb000,
        E_FAIL,
        1,
    ),
    ("ok_fail_st_9100", S_OK, E_FAIL, 0x1, 0x9100, E_FAIL, 1),
    ("ok_sfalse_dbg_b100", S_OK, S_FALSE, 0x2, 0xb100, S_OK, 1),
    ("sfalse_ok_a100", S_FALSE, S_OK, 0x0, 0xa100, S_OK, 1),
    (
        "sfalse_noiface_a000",
        S_FALSE,
        E_NOINTERFACE,
        0x0,
        0xa000,
        E_FAIL,
        1,
    ),
    ("fail_ok_b000", E_FAIL, S_OK, 0x0, 0xb000, E_FAIL, 0),
    (
        "fail_noiface_bgra_b000",
        E_FAIL,
        E_NOINTERFACE,
        0x20,
        0xb000,
        E_FAIL,
        0,
    ),
    (
        "notimpl_ok_0x8_a000",
        E_NOTIMPL,
        S_OK,
        0x8,
        0xa000,
        E_NOTIMPL,
        0,
    ),
    (
        "oom_ok_9100",
        E_OUTOFMEMORY,
        S_OK,
        0x0,
        0x9100,
        E_OUTOFMEMORY,
        0,
    ),
    (
        "unsupported_fail_b100",
        DXGI_ERROR_UNSUPPORTED,
        E_FAIL,
        0x0,
        0xb100,
        DXGI_ERROR_UNSUPPORTED,
        0,
    ),
    (
        "notfound_ok_b000",
        DXGI_ERROR_NOT_FOUND,
        S_OK,
        0x0,
        0xb000,
        DXGI_ERROR_NOT_FOUND,
        0,
    ),
    (
        "unexpected_ok_a000",
        E_UNEXPECTED,
        S_OK,
        0x0,
        0xa000,
        E_UNEXPECTED,
        0,
    ),
    ("ok_ok_fl0", S_OK, S_OK, 0x0, 0x0, S_OK, 1),
    (
        "ok_epointer_flffff",
        S_OK,
        E_POINTER,
        0x0,
        0xffff,
        E_FAIL,
        1,
    ),
    (
        "fail_fail_allones",
        E_FAIL,
        E_FAIL,
        0xffff_ffff,
        0xffff_ffff,
        E_FAIL,
        0,
    ),
    (
        "ok_notimpl_st_b000",
        S_OK,
        E_NOTIMPL,
        0x1,
        0xb000,
        E_FAIL,
        1,
    ),
    ("sfalse_fail_a000", S_FALSE, E_FAIL, 0x0, 0xa000, E_FAIL, 1),
    ("maxpos_ok_b000", i32::MAX, S_OK, 0x0, 0xb000, S_OK, 1),
    ("minneg_ok_a000", i32::MIN, S_OK, 0x0, 0xa000, i32::MIN, 0),
];

#[test]
fn oracle_register_layers_row() {
    // register_layers | hr=0x80004001
    assert_eq!(d3d10_core_register_layers(), 0x8000_4001_u32 as Hresult);
}

#[test]
fn oracle_create_device_rows() {
    for &(label, create_hr, qi_hr, flags, feature_level, expected_hr, qi_calls) in ROWS {
        let obs = run(create_hr, qi_hr, flags, feature_level);
        assert_eq!(obs.hr, expected_hr, "{label}: hr");
        assert_eq!(obs.create_calls, 1, "{label}: create_calls");
        assert_eq!(obs.qi_calls, qi_calls, "{label}: qi_calls");
        assert_eq!(obs.seen_flags, flags, "{label}: flags_seen");
        assert_eq!(obs.seen_levels, [feature_level], "{label}: level0");
    }
}
