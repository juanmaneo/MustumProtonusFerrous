//! Twenty rows checked against the sanitizer-built Wine oracle in
//! `target/canon/cryptdlg/oracle.txt`.
use super::*;
use confidence::{
    CERT_CONFIDENCE_SIG, CERT_CONFIDENCE_TIME, CERT_CONFIDENCE_TIMENEST,
    CERT_TRUST_IS_NOT_SIGNATURE_VALID, CERT_TRUST_IS_NOT_TIME_NESTED, CERT_TRUST_IS_NOT_TIME_VALID,
};

#[test]
fn oracle_trust_confidence_rows() {
    let rows = [
        (0, 0x1110_0000),
        (1, 0x1010_0000),
        (2, 0x1100_0000),
        (3, 0x1000_0000),
        (8, 0x0110_0000),
        (9, 0x0010_0000),
        (10, 0x0100_0000),
        (11, 0),
    ];
    for (status, expected) in rows {
        assert_eq!(confidence::trust_status_to_confidence(status), expected);
    }
}

#[test]
fn oracle_cert_trust_init_rows() {
    let mut steps = [0; TRUSTERROR_MAX_STEPS];
    assert_eq!(cert_trust_init(None), S_FALSE);
    assert_eq!(cert_trust_init(Some(&steps)), S_OK);
    steps[TRUSTERROR_STEP_FINAL_WVTINIT] = 1;
    assert_eq!(cert_trust_init(Some(&steps)), S_FALSE);
    steps[TRUSTERROR_STEP_FINAL_WVTINIT] = 0;
    steps[4] = 1;
    assert_eq!(cert_trust_init(Some(&steps)), S_OK);
}

#[test]
fn oracle_hex_rows() {
    let empty = hex::format_hex(&[], None);
    assert_eq!(
        (empty.success, empty.chars_needed, empty.text),
        (true, 1, None)
    );

    let one = hex::format_hex(&[0x0a], Some(3));
    assert_eq!(
        (one.success, one.chars_needed, one.text.as_deref()),
        (true, 3, Some("0a"))
    );

    let short = hex::format_hex(&[0x0a, 0xaf], Some(5));
    assert_eq!(
        (short.success, short.chars_needed, short.text.as_deref()),
        (false, 6, Some(""))
    );

    let exact = hex::format_hex(&[0x0a, 0xaf], Some(6));
    assert_eq!(
        (exact.success, exact.chars_needed, exact.text.as_deref()),
        (true, 6, Some("0a af"))
    );
}

#[test]
fn oracle_cps_rows() {
    let single = cps::format_cps("Policy", false, None);
    assert_eq!(
        (single.success, single.chars_needed, single.text),
        (true, 10, None)
    );

    let multi_query = cps::format_cps("Policy", true, None);
    assert_eq!(
        (
            multi_query.success,
            multi_query.chars_needed,
            multi_query.text
        ),
        (true, 25, None)
    );

    let partial = cps::format_cps("Policy", false, Some(8));
    assert_eq!(
        (
            partial.success,
            partial.chars_needed,
            partial.text.as_deref()
        ),
        (false, 10, Some("Policy"))
    );

    let multi = cps::format_cps("Policy", true, Some(25));
    assert_eq!(
        (multi.success, multi.chars_needed, multi.text.as_deref()),
        (true, 25, Some("               Policy\r\n"))
    );
}

#[test]
fn oracle_rows_use_the_wine_flag_constants() {
    assert_eq!(
        confidence::trust_status_to_confidence(
            CERT_TRUST_IS_NOT_SIGNATURE_VALID
                | CERT_TRUST_IS_NOT_TIME_VALID
                | CERT_TRUST_IS_NOT_TIME_NESTED
        ),
        0
    );
    assert_eq!(
        CERT_CONFIDENCE_SIG | CERT_CONFIDENCE_TIME | CERT_CONFIDENCE_TIMENEST,
        0x1110_0000
    );
}
