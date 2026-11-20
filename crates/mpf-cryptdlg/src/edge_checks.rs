//! Capacity boundaries and status bits outside the specific oracle rows.
use super::*;

#[test]
fn confidence_ignores_unrelated_error_bits() {
    assert_eq!(confidence::trust_status_to_confidence(u32::MAX), 0);
    assert_eq!(
        confidence::trust_status_to_confidence(0x8000_0000),
        0x1110_0000
    );
}

#[test]
fn trust_init_checks_only_the_final_wvtinit_step() {
    let mut steps = [u32::MAX; TRUSTERROR_MAX_STEPS];
    steps[TRUSTERROR_STEP_FINAL_WVTINIT] = 0;
    assert_eq!(cert_trust_init(Some(&steps)), S_OK);
}

#[test]
fn hex_capacity_boundaries_and_empty_output() {
    let too_short = hex::format_hex(&[0xff], Some(2));
    assert_eq!((too_short.success, too_short.chars_needed), (false, 3));
    assert_eq!(too_short.text.as_deref(), Some(""));

    let empty = hex::format_hex(&[], Some(1));
    assert_eq!((empty.success, empty.chars_needed), (true, 1));
    assert_eq!(empty.text.as_deref(), Some(""));
}

#[test]
fn cps_capacity_boundaries_and_empty_value() {
    let no_indent = cps::format_cps("", true, Some(15));
    assert_eq!((no_indent.success, no_indent.chars_needed), (false, 19));
    assert_eq!(no_indent.text.as_deref(), Some(""));

    let indent_only = cps::format_cps("A", true, Some(16));
    assert_eq!((indent_only.success, indent_only.chars_needed), (false, 20));
    assert_eq!(indent_only.text.as_deref(), Some("               "));

    let value_only = cps::format_cps("", false, Some(2));
    assert_eq!((value_only.success, value_only.chars_needed), (false, 4));
    assert_eq!(value_only.text.as_deref(), Some(""));

    let exact = cps::format_cps("", false, Some(4));
    assert_eq!((exact.success, exact.chars_needed), (true, 4));
    assert_eq!(exact.text.as_deref(), Some(", "));
}
