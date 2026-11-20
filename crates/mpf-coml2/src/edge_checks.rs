//! Safe edges beyond the fixed oracle rows: bounded input, NUL handling, and byte layout.

use crate::{
    fmt_id_to_prop_stg_name, prop_stg_name_to_fmt_id, Error, Fmtid, FMTID_DOC_SUMMARY_INFORMATION,
    FMTID_USER_DEFINED_PROPERTIES,
};

fn nul_terminated(text: &str) -> Vec<u16> {
    text.encode_utf16().chain([0]).collect()
}

#[test]
fn byte_layout_round_trips() {
    let bytes = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88,
    ];
    assert_eq!(Fmtid::from_bytes(bytes).to_bytes(), bytes);
}

#[test]
fn name_parsing_stops_at_nul_and_user_properties_round_trip_as_document_summary() {
    assert_eq!(
        prop_stg_name_to_fmt_id(Some(&[
            5,
            b'D' as u16,
            b'o' as u16,
            b'c' as u16,
            0,
            b'X' as u16
        ])),
        Err(Error::InvalidName)
    );
    let name = fmt_id_to_prop_stg_name(Some(&FMTID_USER_DEFINED_PROPERTIES)).unwrap();
    assert_eq!(name, nul_terminated("\u{5}DocumentSummaryInformation"));
    assert_eq!(
        prop_stg_name_to_fmt_id(Some(&name)),
        Ok(FMTID_DOC_SUMMARY_INFORMATION)
    );
}

#[test]
fn generic_names_round_trip_and_require_a_complete_encoded_value() {
    let fmtid = Fmtid::from_bytes([
        0x67, 0x45, 0x23, 0x01, 0xab, 0x89, 0xef, 0xcd, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd,
        0xef,
    ]);
    let name = fmt_id_to_prop_stg_name(Some(&fmtid)).unwrap();
    assert_eq!(prop_stg_name_to_fmt_id(Some(&name)), Ok(fmtid));
    assert_eq!(
        prop_stg_name_to_fmt_id(Some(&[5, b'~' as u16, 0])),
        Err(Error::InvalidName)
    );
    assert_eq!(
        prop_stg_name_to_fmt_id(Some(&[5, b'A' as u16, 0])),
        Err(Error::InvalidName)
    );
}
