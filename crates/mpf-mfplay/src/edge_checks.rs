//! Edge cases around the oracle rows: GUID equality granularity, variant tags the C rejects, the
//! argument-check precedence and the start/stop boundaries the 25 rows do not pin down.

use crate::{
    get_position, set_position, set_start_stop_position, Guid, PropVariant, E_INVALIDARG,
    MFP_POSITIONTYPE_100NS, MF_E_OUT_OF_RANGE, VT_EMPTY, VT_I8,
};

/// `VT_NULL`, another variant tag the position functions reject.
const VT_NULL: u16 = 1;

fn pv(vt: u16, h_val: i64) -> PropVariant {
    PropVariant { vt, h_val }
}

#[test]
fn guid_equal_except_data4_is_rejected() {
    // IsEqualGUID compares the whole struct, so a GUID that matches only in the first three
    // fields is still a wrong position type.
    let mut near = MFP_POSITIONTYPE_100NS;
    near.data4[7] = 1;
    assert_eq!(
        set_position(Some(&near), Some(&pv(VT_I8, 1))),
        Err(E_INVALIDARG)
    );
}

#[test]
fn vt_null_is_rejected() {
    assert_eq!(
        set_position(Some(&MFP_POSITIONTYPE_100NS), Some(&pv(VT_NULL, 1))),
        Err(E_INVALIDARG)
    );
}

#[test]
fn format_check_precedes_vt_and_null_checks() {
    // The C checks the format GUID before the XOR null check and the variant tag, so a bad
    // format wins over every other rejection.
    let bad = Guid {
        data1: 1,
        data2: 2,
        data3: 3,
        data4: [4, 5, 6, 7, 8, 9, 10, 11],
    };
    assert_eq!(
        set_position(Some(&bad), Some(&pv(VT_NULL, 0))),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        set_position(Some(&bad), Some(&pv(VT_EMPTY, 0))),
        Err(E_INVALIDARG)
    );
}

#[test]
fn vt_empty_ignores_the_payload() {
    // A VT_EMPTY position clears the position regardless of the stored value, even i64::MIN.
    assert_eq!(
        set_position(Some(&MFP_POSITIONTYPE_100NS), Some(&pv(VT_EMPTY, i64::MIN))),
        Ok(0)
    );
}

#[test]
fn get_position_zero_leaves_the_variant_untouched() {
    // The C writes only the format GUID for a zero value; the caller pre-set the variant to
    // VT_EMPTY. The port models that untouched variant as None.
    let (format, position) = get_position(0);
    assert_eq!(format, MFP_POSITIONTYPE_100NS);
    assert_eq!(position, None);
}

#[test]
fn start_stop_both_cleared() {
    assert_eq!(set_start_stop_position(None, None, None, None), Ok(()));
}

#[test]
fn start_stop_extremes_in_order() {
    assert_eq!(
        set_start_stop_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, i64::MIN)),
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, i64::MAX))
        ),
        Ok(())
    );
}

#[test]
fn start_stop_stop_zero_is_out_of_range() {
    // The stop position is validated before the ordering check, so a zero stop fails even
    // though it would not violate the ordering.
    assert_eq!(
        set_start_stop_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1000)),
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 0))
        ),
        Err(MF_E_OUT_OF_RANGE)
    );
}

#[test]
fn start_stop_empty_stop_counts_as_zero() {
    // A VT_EMPTY stop position means zero, so a positive start exceeds it.
    assert_eq!(
        set_start_stop_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 1)),
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_EMPTY, 0))
        ),
        Err(MF_E_OUT_OF_RANGE)
    );
}

#[test]
fn start_stop_start_bad_vt_propagates() {
    assert_eq!(
        set_start_stop_position(
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_NULL, 1000)),
            Some(&MFP_POSITIONTYPE_100NS),
            Some(&pv(VT_I8, 2000))
        ),
        Err(E_INVALIDARG)
    );
}
