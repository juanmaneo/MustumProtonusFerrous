use super::{validate_daily_days_interval, E_INVALIDARG, S_OK};

#[test]
fn signed_short_boundary_values() {
    assert_eq!(validate_daily_days_interval(i16::MIN), E_INVALIDARG);
    assert_eq!(validate_daily_days_interval(-1), E_INVALIDARG);
    assert_eq!(validate_daily_days_interval(0), E_INVALIDARG);
    assert_eq!(validate_daily_days_interval(1), S_OK);
    assert_eq!(validate_daily_days_interval(i16::MAX), S_OK);
}
