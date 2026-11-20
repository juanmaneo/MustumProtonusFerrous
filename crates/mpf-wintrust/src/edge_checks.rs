//! Hand-written checks that the oracle cannot replay: the sanitizer-flagged vectors, checked
//! against the port's intent, the `Error` type, round-trips, and the trust-status tables. Oracle
//! replays are in `oracle_checks`.

use crate::{
    decode, encode, length, trust, Error, CRYPT_E_ASN1_BADTAG, CRYPT_E_ASN1_CORRUPT,
    CRYPT_E_ASN1_EOD, CRYPT_E_ASN1_LARGE, ERROR_MORE_DATA,
};

fn cloned<T: Clone>(value: T) -> T {
    value.clone()
}

#[test]
fn error_codes_match_wine() {
    assert_eq!(Error::Corrupt.code(), CRYPT_E_ASN1_CORRUPT);
    assert_eq!(Error::BadTag.code(), CRYPT_E_ASN1_BADTAG);
    assert_eq!(Error::EndOfData.code(), CRYPT_E_ASN1_EOD);
    assert_eq!(Error::TooLarge.code(), CRYPT_E_ASN1_LARGE);
    assert_eq!(Error::MoreData { needed: 9 }.code(), ERROR_MORE_DATA);
}

#[test]
fn error_derives_behave_as_values() {
    let more = Error::MoreData { needed: 3 };
    assert_eq!(cloned(more), more);
    assert_ne!(more, Error::MoreData { needed: 4 });
    assert_ne!(Error::Corrupt, Error::BadTag);
    assert_eq!(format!("{more:?}"), "MoreData { needed: 3 }");
    assert_eq!(format!("{:?}", Error::EndOfData), "EndOfData");
}

#[test]
fn length_encode_uses_the_fewest_octets() {
    assert_eq!(length::encode(0), [0x00]);
    assert_eq!(length::encode(0x7f), [0x7f]);
    assert_eq!(length::encode(0x80), [0x81, 0x80]);
    assert_eq!(length::encode(0xff), [0x81, 0xff]);
    assert_eq!(length::encode(0x100), [0x82, 0x01, 0x00]);
    assert_eq!(length::encode(0xffff_ffff), [0x84, 0xff, 0xff, 0xff, 0xff]);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn length_beyond_u32_is_too_large_to_decode() {
    assert_eq!(length::encode(0x1_0000_0000), [0x85, 0x01, 0, 0, 0, 0]);
    assert_eq!(
        length::decode(&[0x30, 0x85, 0x01, 0, 0, 0, 0]),
        Err(Error::TooLarge)
    );
}

#[test]
fn length_round_trips_with_content() {
    for len in [0usize, 1, 0x7f, 0x80, 0xff, 0x100, 0xffff, 0x1_0000] {
        let mut element = vec![0x04];
        element.extend(length::encode(len));
        element.extend(vec![0xa5; len]);
        let decoded = length::decode(&element).unwrap();
        assert_eq!(decoded as usize, len);
        assert_eq!(length::content(&element, decoded).unwrap().len(), len);
    }
}

#[test]
fn boolean_round_trips() {
    assert_eq!(decode::boolean(&encode::boolean(true)), Ok(true));
    assert_eq!(decode::boolean(&encode::boolean(false)), Ok(false));
}

#[test]
fn int_round_trips_through_encode_and_decode() {
    for value in [
        0,
        1,
        -1,
        127,
        128,
        -128,
        -129,
        255,
        256,
        32767,
        -32768,
        i32::MAX,
        i32::MIN,
    ] {
        assert_eq!(decode::int(&encode::int(value)), Ok(value), "value {value}");
    }
}

#[test]
fn octets_round_trip() {
    let long = [0x5a; 200];
    let cases: [&[u8]; 4] = [&[], &[0x00], &[1, 2, 3], &long[..]];
    for data in cases {
        assert_eq!(decode::octets(&encode::octets(data)), Ok(data));
    }
}

#[test]
fn flagged_all_ones_integer_encodes_as_minus_one() {
    assert_eq!(encode::integer(&[0xff]), [0x02, 0x01, 0xff]);
    assert_eq!(encode::int(-1), [0x02, 0x01, 0xff]);
}

#[test]
fn flagged_negative_ints_decode_with_the_port_intent() {
    assert_eq!(decode::int(&[0x02, 0x01, 0xff]), Ok(-1));
    assert_eq!(decode::int(&[0x02, 0x02, 0xff, 0x7f]), Ok(-129));
    assert_eq!(decode::int(&[0x02, 0x02, 0xff, 0x00]), Ok(-256));
    assert_eq!(
        decode::int(&[0x02, 0x04, 0x80, 0x00, 0x00, 0x00]),
        Ok(i32::MIN)
    );
}

#[test]
fn flagged_content_past_the_input_is_end_of_data() {
    assert_eq!(decode::int(&[0x01, 0x01]), Err(Error::EndOfData));
    assert_eq!(decode::int(&[0x02, 0x01]), Err(Error::EndOfData));
    assert_eq!(decode::octets(&[0x04, 0x01]), Err(Error::EndOfData));
    assert_eq!(
        decode::octets(&[0x04, 0x84, 0xff, 0xff, 0xff, 0xff, 0x00]),
        Err(Error::EndOfData)
    );
}

#[test]
fn write_to_follows_the_buffer_protocol() {
    let encoded = [0x02, 0x01, 0x00];
    assert_eq!(encode::write_to(&encoded, None), Ok(3));

    let mut exact = [0u8; 3];
    assert_eq!(encode::write_to(&encoded, Some(&mut exact[..])), Ok(3));
    assert_eq!(exact, encoded);

    let mut roomy = [0xeeu8; 5];
    assert_eq!(encode::write_to(&encoded, Some(&mut roomy[..])), Ok(3));
    assert_eq!(roomy, [0x02, 0x01, 0x00, 0xee, 0xee]);

    let mut small = [0u8; 2];
    assert_eq!(
        encode::write_to(&encoded, Some(&mut small[..])),
        Err(Error::MoreData { needed: 3 })
    );
    assert_eq!(small, [0, 0]);
}

#[test]
fn status_to_error_resolves_overlaps_by_table_order() {
    let revoked_and_expired = trust::CERT_TRUST_IS_REVOKED | trust::CERT_TRUST_IS_NOT_TIME_VALID;
    assert_eq!(
        trust::status_to_error(revoked_and_expired),
        trust::CERT_E_EXPIRED
    );
    let signature_and_root =
        trust::CERT_TRUST_IS_NOT_SIGNATURE_VALID | trust::CERT_TRUST_IS_UNTRUSTED_ROOT;
    assert_eq!(
        trust::status_to_error(signature_and_root),
        trust::TRUST_E_CERT_SIGNATURE
    );
}

#[test]
fn status_to_error_maps_unknown_bits_to_system_error() {
    assert_eq!(
        trust::status_to_error(0x0001_0000),
        trust::TRUST_E_SYSTEM_ERROR
    );
    assert_eq!(trust::status_to_error(0), trust::S_OK);
}
