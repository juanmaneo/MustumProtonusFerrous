//! Deterministic rows emitted by the ASan/UBSan Wine C oracle in `target/canon/kerberos`.

use crate::{
    expiry_to_timestamp, flags_gss_to_asc_ret, flags_gss_to_isc_ret, flags_isc_req_to_gss,
    get_buffer_index, krb5_error_to_status, principal_to_name_and_realm, status_gss_to_sspi,
};

#[test]
fn oracle_rows_match_wine_c() {
    let errors = [(0, 0), (1, 3_221_225_473)];
    for (input, expected) in errors {
        assert_eq!(krb5_error_to_status(input), expected);
    }

    let principals: &[(&[u8], &[u8], &[u8])] = &[
        (b"alice@EXAMPLE.COM", b"alice", b"EXAMPLE.COM"),
        (b"svc/host@REALM@TAIL", b"svc/host", b"REALM@TAIL"),
        (b"no-realm", b"no-realm", b""),
    ];
    for &(input, expected_name, expected_realm) in principals {
        assert_eq!(
            principal_to_name_and_realm(input),
            (expected_name, expected_realm)
        );
    }

    let statuses = [
        (0, 0),
        (65_536, 2_148_074_245),
        (393_216, 2_148_074_255),
        (458_752, 2_148_074_254),
        (589_824, 2_148_074_248),
        (1, 590_610),
        (4, 2_148_074_248),
        (8, 2_148_074_256),
        (851_968, 2_148_074_244),
        (4_279_383_126, 2_148_074_244),
        (655_360, 2_148_074_254),
    ];
    for (input, expected) in statuses {
        assert_eq!(status_gss_to_sspi(input), expected);
    }

    assert_eq!(flags_gss_to_asc_ret(0xffff_707f), 1_737_247);
    assert_eq!(flags_isc_req_to_gss(0xfff7_421f), 28_799);
    assert_eq!(flags_gss_to_isc_ret(0xffff_707f), 475_679);

    let first = [1, 2, 3];
    let duplicate = [1, 2, 2];
    let missing = [1];
    assert_eq!(get_buffer_index(Some(&first), 2), Some(1));
    assert_eq!(get_buffer_index(Some(&duplicate), 2), Some(1));
    assert_eq!(get_buffer_index(Some(&missing), 9), None);

    assert_eq!(
        expiry_to_timestamp(0x1122_3344_5566_7788, 0),
        (1_432_778_632, 287_454_020)
    );
    assert_eq!(
        expiry_to_timestamp(0x1122_3344_5566_7788, 1),
        (1_442_778_632, 287_454_020)
    );
}
