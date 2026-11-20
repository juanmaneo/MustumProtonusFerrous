//! Replays of the 25 oracle rows in `target/canon/mfplay`. The oracle compiles the verbatim
//! `media_item_set_position` and `media_item_get_position` ranges of `player.c`, plus the ordering
//! check of `media_item_SetStartStopPosition`, with `gcc -fsanitize=address,undefined -g`, with the
//! Win32 shims in `wine_mock.h`, and prints one test per row. Each `oracle_oNN` test reproduces row
//! NN, and its expected value is what the C printed for that row. No row trips the sanitizer.

use crate::{
    get_position, set_position, set_start_stop_position, Guid, PropVariant, E_INVALIDARG,
    E_POINTER, MFP_POSITIONTYPE_100NS, MF_E_OUT_OF_RANGE, VT_EMPTY, VT_I8,
};

/// `VT_I4`, a variant type the position functions reject.
const VT_I4: u16 = 3;

fn pv(vt: u16, h_val: i64) -> PropVariant {
    PropVariant { vt, h_val }
}

fn bad_guid() -> Guid {
    Guid {
        data1: 0x1234_5678,
        data2: 0x9abc,
        data3: 0xdef0,
        data4: [1, 2, 3, 4, 5, 6, 7, 8],
    }
}

#[test]
fn oracle_o01() {
    assert_eq!(
        set_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1234567890123456789))
        ),
        Ok(1234567890123456789i64)
    );
}

#[test]
fn oracle_o02() {
    assert_eq!(
        set_position(Some(&MFP_POSITIONTYPE_100NS), Some(&pv(VT_I8, 1))),
        Ok(1i64)
    );
}

#[test]
fn oracle_o03() {
    assert_eq!(
        set_position(Some(&MFP_POSITIONTYPE_100NS), Some(&pv(VT_I8, -1))),
        Ok(-1i64)
    );
}

#[test]
fn oracle_o04() {
    assert_eq!(
        set_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 9223372036854775807))
        ),
        Ok(9223372036854775807i64)
    );
}

#[test]
fn oracle_o05() {
    assert_eq!(
        set_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, -9223372036854775808))
        ),
        Ok(-9223372036854775808i64)
    );
}

#[test]
fn oracle_o06() {
    assert_eq!(
        set_position(Some(&bad_guid()), Some(&pv(VT_I8, 5))),
        Err(E_INVALIDARG)
    );
}

#[test]
fn oracle_o07() {
    assert_eq!(set_position(Some(&bad_guid()), None), Err(E_INVALIDARG));
}

#[test]
fn oracle_o08() {
    assert_eq!(set_position(None, Some(&pv(VT_I8, 5))), Err(E_POINTER));
}

#[test]
fn oracle_o09() {
    assert_eq!(
        set_position(Some(&MFP_POSITIONTYPE_100NS), None),
        Err(E_POINTER)
    );
}

#[test]
fn oracle_o10() {
    assert_eq!(
        set_position(Some(&MFP_POSITIONTYPE_100NS), Some(&pv(VT_I4, 5))),
        Err(E_INVALIDARG)
    );
}

#[test]
fn oracle_o11() {
    assert_eq!(set_position(None, None), Ok(0i64));
}

#[test]
fn oracle_o12() {
    assert_eq!(
        set_position(Some(&MFP_POSITIONTYPE_100NS), Some(&pv(VT_EMPTY, 0))),
        Ok(0i64)
    );
}

#[test]
fn oracle_o13() {
    assert_eq!(
        set_position(Some(&MFP_POSITIONTYPE_100NS), Some(&pv(VT_I8, 0))),
        Err(MF_E_OUT_OF_RANGE)
    );
}

#[test]
fn oracle_o14() {
    assert_eq!(
        set_position(Some(&MFP_POSITIONTYPE_100NS), Some(&pv(VT_EMPTY, 42))),
        Ok(0i64)
    );
}

#[test]
fn oracle_o15() {
    assert_eq!(get_position(0i64), (MFP_POSITIONTYPE_100NS, None));
}

#[test]
fn oracle_o16() {
    assert_eq!(
        get_position(1234567890123456789i64),
        (
            MFP_POSITIONTYPE_100NS,
            Some(PropVariant {
                vt: VT_I8,
                h_val: 1234567890123456789
            })
        )
    );
}

#[test]
fn oracle_o17() {
    assert_eq!(
        get_position(-1i64),
        (
            MFP_POSITIONTYPE_100NS,
            Some(PropVariant {
                vt: VT_I8,
                h_val: -1
            })
        )
    );
}

#[test]
fn oracle_o18() {
    assert_eq!(
        get_position(-9223372036854775808i64),
        (
            MFP_POSITIONTYPE_100NS,
            Some(PropVariant {
                vt: VT_I8,
                h_val: -9223372036854775808
            })
        )
    );
}

#[test]
fn oracle_o19() {
    assert_eq!(
        set_start_stop_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1000)),
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 2000))
        ),
        Ok(())
    );
}

#[test]
fn oracle_o20() {
    assert_eq!(
        set_start_stop_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 2000)),
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1000))
        ),
        Err(MF_E_OUT_OF_RANGE)
    );
}

#[test]
fn oracle_o21() {
    assert_eq!(
        set_start_stop_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1000)),
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1000))
        ),
        Ok(())
    );
}

#[test]
fn oracle_o22() {
    assert_eq!(
        set_start_stop_position(
            Some(&bad_guid()),
            Some(&pv(VT_I8, 1000)),
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 2000))
        ),
        Err(E_INVALIDARG)
    );
}

#[test]
fn oracle_o23() {
    assert_eq!(
        set_start_stop_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1000)),
            Some(&bad_guid()),
            Some(&pv(VT_I8, 2000))
        ),
        Err(E_INVALIDARG)
    );
}

#[test]
fn oracle_o24() {
    assert_eq!(
        set_start_stop_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 0)),
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1000))
        ),
        Err(MF_E_OUT_OF_RANGE)
    );
}

#[test]
fn oracle_o25() {
    assert_eq!(
        set_start_stop_position(
            None,
            None,
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1000))
        ),
        Ok(())
    );
}
