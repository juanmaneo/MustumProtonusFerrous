//! Edge cases and direct coverage for Wine logic not represented by the selected oracle rows.

use crate::{
    expiry_to_timestamp, flags_gss_to_asc_ret, flags_gss_to_isc_ret, flags_isc_req_to_gss,
    get_buffer_index, krb5_error_to_status, principal_to_name_and_realm, status_gss_to_sspi,
    SEC_E_CONTEXT_EXPIRED, SEC_E_INTERNAL_ERROR, SEC_E_INVALID_HANDLE, SEC_E_INVALID_TOKEN,
    SEC_E_MESSAGE_ALTERED, SEC_E_NO_CREDENTIALS, SEC_E_OK, SEC_E_OUT_OF_SEQUENCE,
    SEC_E_QOP_NOT_SUPPORTED, SEC_E_SECPKG_NOT_FOUND, SEC_I_CONTINUE_NEEDED, STATUS_SUCCESS,
    STATUS_UNSUCCESSFUL,
};

#[test]
fn error_codes_only_distinguish_success_from_failure() {
    for error in [i32::MIN, -1, 1, i32::MAX] {
        assert_eq!(krb5_error_to_status(error), STATUS_UNSUCCESSFUL);
    }
    assert_eq!(krb5_error_to_status(0), STATUS_SUCCESS);
}

#[test]
fn principal_splits_only_at_the_first_separator() {
    assert_eq!(
        principal_to_name_and_realm(b""),
        (b"".as_slice(), b"".as_slice())
    );
    assert_eq!(
        principal_to_name_and_realm(b"@"),
        (b"".as_slice(), b"".as_slice())
    );
    assert_eq!(
        principal_to_name_and_realm(b"a@@b"),
        (b"a".as_slice(), b"@b".as_slice())
    );
    assert_eq!(
        principal_to_name_and_realm(b"user@realm"),
        (b"user".as_slice(), b"realm".as_slice())
    );
}

#[test]
fn every_gss_status_mapping_and_fallback_matches_wine() {
    let cases = [
        (0, SEC_E_OK),
        (1 << 16, SEC_E_SECPKG_NOT_FOUND),
        (6 << 16, SEC_E_MESSAGE_ALTERED),
        (7 << 16, SEC_E_NO_CREDENTIALS),
        (8 << 16, SEC_E_INVALID_HANDLE),
        (9 << 16, SEC_E_INVALID_TOKEN),
        (10 << 16, SEC_E_NO_CREDENTIALS),
        (11 << 16, SEC_E_CONTEXT_EXPIRED),
        (12 << 16, SEC_E_CONTEXT_EXPIRED),
        (14 << 16, SEC_E_QOP_NOT_SUPPORTED),
        (1, SEC_I_CONTINUE_NEEDED),
        (1 << 1, SEC_E_INVALID_TOKEN),
        (1 << 2, SEC_E_INVALID_TOKEN),
        (1 << 3, SEC_E_OUT_OF_SEQUENCE),
        (1 << 4, SEC_E_OUT_OF_SEQUENCE),
        (13 << 16, SEC_E_INTERNAL_ERROR),
        (0x0001_0001, SEC_E_INTERNAL_ERROR),
    ];
    for (input, expected) in cases {
        assert_eq!(status_gss_to_sspi(input), expected, "status {input:#x}");
    }
}

#[test]
fn flag_maps_drop_unrecognized_bits_and_handle_zero() {
    assert_eq!(flags_gss_to_asc_ret(0), 0);
    assert_eq!(flags_gss_to_isc_ret(0), 0);
    assert_eq!(flags_isc_req_to_gss(0), 0);
    assert_eq!(flags_gss_to_asc_ret(u32::MAX), 0x001a_821f);
    assert_eq!(flags_gss_to_isc_ret(u32::MAX), 0x0007_421f);
    assert_eq!(flags_isc_req_to_gss(u32::MAX), 0x0000_707f);
    assert_eq!(flags_gss_to_asc_ret(0x8000_0000), 0);
    assert_eq!(flags_gss_to_isc_ret(0x8000_0000), 0);
    assert_eq!(flags_isc_req_to_gss(0x8000_0000), 0);
}

#[test]
fn buffer_lookup_handles_missing_descriptors_and_empty_lists() {
    assert_eq!(get_buffer_index(None, 0), None);
    assert_eq!(get_buffer_index(Some(&[]), 0), None);
    assert_eq!(get_buffer_index(Some(&[3, 7, 11]), 11), Some(2));
    assert_eq!(get_buffer_index(Some(&[3, 7, 11]), 4), None);
}

#[test]
fn timestamp_halves_preserve_signed_and_wrapping_arithmetic() {
    assert_eq!(expiry_to_timestamp(-1, 0), (u32::MAX, -1));
    assert_eq!(expiry_to_timestamp(-10_000_000, 1), (0, 0));
    assert_eq!(expiry_to_timestamp(i64::MAX, 1), (9_999_999, i32::MIN));
    assert_eq!(
        expiry_to_timestamp(0, u32::MAX),
        (u32::MAX - 9_999_999, 9_999_999)
    );
}
