//! One test per row of the sanitizer-built harness (target/canon/xmllite, inputs.txt). Each
//! comment gives the harness command, and each assertion gives the C output for it.

use crate::{
    advance, char_entity, detect_encoding, escape_chars, escape_string, get_code_page, is_char,
    is_ncnamestartchar, is_pubchar, is_valid_name, is_valid_ncname, is_valid_nmtoken,
    is_valid_pubid, is_valid_qname, next_write_count, parse_encoding_name, parse_encoding_prefix,
    surrogate_entity, utf16, write_comment, Position, XmlEncoding, WC_E_NAMECHARACTER,
    WC_E_PUBLICID, WC_E_XMLCHARACTER, WR_E_INVALIDSURROGATEPAIR,
};

/// The harness's "1 0 ..." line for a character class over `units`.
fn classes(check: fn(u16) -> bool, units: &[u16]) -> String {
    units
        .iter()
        .map(|&unit| u8::from(check(unit)).to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

// class is_char 9 a d 20 d7ff d800 dfff e000 fffd fffe 0 1f -> 1 1 1 1 1 1 1 1 1 0 0 0
#[test]
fn c01_is_char() {
    let units = [
        0x9, 0xa, 0xd, 0x20, 0xd7ff, 0xd800, 0xdfff, 0xe000, 0xfffd, 0xfffe, 0x0, 0x1f,
    ];
    assert_eq!(classes(is_char, &units), "1 1 1 1 1 1 1 1 1 0 0 0");
}

// class is_pubchar 20 27 28 2d 3b 3c 61 d -> 1 0 0 1 1 0 1 1
#[test]
fn c02_is_pubchar() {
    let units = [0x20, 0x27, 0x28, 0x2d, 0x3b, 0x3c, 0x61, 0xd];
    assert_eq!(classes(is_pubchar, &units), "1 0 0 1 1 0 1 1");
}

// class is_ncnamestartchar 41 5f 31 2d c0 d7 b7 3a -> 1 1 0 0 1 0 0 0
#[test]
fn c03_is_ncnamestartchar() {
    let units = [0x41, 0x5f, 0x31, 0x2d, 0xc0, 0xd7, 0xb7, 0x3a];
    assert_eq!(classes(is_ncnamestartchar, &units), "1 1 0 0 1 0 0 0");
}

// parse =utf-16 -1 -> UTF-16
#[test]
fn e01_parse_utf16_name() {
    assert_eq!(parse_encoding_name(&utf16("utf-16")), XmlEncoding::Utf16);
}

// parse =UTF-1 5 -> UTF-16 (a prefix of the name matches)
#[test]
fn e02_parse_prefix_of_utf16() {
    assert_eq!(parse_encoding_prefix(&utf16("UTF-1")), XmlEncoding::Utf16);
}

// parse =US-ASCI -1 -> Unknown (NUL-terminated, so a prefix does not match)
#[test]
fn e03_parse_truncated_name() {
    assert_eq!(parse_encoding_name(&utf16("US-ASCI")), XmlEncoding::Unknown);
}

// codepage utf8 -> 0x00000000 65001
#[test]
fn e04_code_page_utf8() {
    assert_eq!(get_code_page(XmlEncoding::Utf8), Ok(65001));
}

// detect ef bb bf 3c 3f 78 -> 0x00000000 UTF-8 3
#[test]
fn d01_detect_utf8_bom() {
    let data = [0xef, 0xbb, 0xbf, 0x3c, 0x3f, 0x78];
    assert_eq!(detect_encoding(&data), Ok((XmlEncoding::Utf8, 3)));
}

// detect ff fe 3c 00 -> 0x00000000 UTF-16 2
#[test]
fn d02_detect_utf16_bom() {
    let data = [0xff, 0xfe, 0x3c, 0x00];
    assert_eq!(detect_encoding(&data), Ok((XmlEncoding::Utf16, 2)));
}

// detect 3c 00 3f 00 -> 0x00000000 UTF-16 0
#[test]
fn d03_detect_utf16_start_tag() {
    let data = [0x3c, 0x00, 0x3f, 0x00];
    assert_eq!(detect_encoding(&data), Ok((XmlEncoding::Utf16, 0)));
}

// detect 3c c3 a9 3f -> 0x00000000 Unknown 0 (the second byte is read as a signed char)
#[test]
fn d04_detect_signed_second_byte() {
    let data = [0x3c, 0xc3, 0xa9, 0x3f];
    assert_eq!(detect_encoding(&data), Ok((XmlEncoding::Unknown, 0)));
}

// detect 3c 3f -> 0xc00cee01 Unknown 0 (fewer than three bytes)
#[test]
fn d05_detect_too_short() {
    assert_eq!(detect_encoding(&[0x3c, 0x3f]), Err(crate::MX_E_INPUTEND));
}

// ncname =1a -> 0xc00cee2c 0
#[test]
fn n01_ncname_digit_start() {
    assert_eq!(is_valid_ncname(&utf16("1a")), Err(WC_E_NAMECHARACTER));
}

// name =ab:c -> 0x00000000 4
#[test]
fn n02_name_with_colon() {
    assert_eq!(is_valid_name(&utf16("ab:c")), Ok(4));
}

// qname =a: -> 0x00000000 2
#[test]
fn n03_qname_empty_local_part() {
    assert_eq!(is_valid_qname(&utf16("a:")), Ok(2));
}

// nmtoken =1a-:b -> 1 5
#[test]
fn n04_nmtoken_with_colon() {
    assert_eq!(is_valid_nmtoken(&utf16("1a-:b")), Some(5));
}

// pubid #61,7f -> 0xc00cee42 0
#[test]
fn n05_pubid_del_character() {
    assert_eq!(is_valid_pubid(&[0x61, 0x7f]), Err(WC_E_PUBLICID));
}

// nextcount #d83d,de00 2 -> 0x00000000 2
#[test]
fn w01_next_count_surrogate_pair() {
    assert_eq!(next_write_count(&[0xd83d, 0xde00]), Ok(2));
}

// nextcount #d83d 1 -> 0xc00cef0c 0 (the high surrogate has no low surrogate after it)
#[test]
fn w02_next_count_lone_high_surrogate() {
    assert_eq!(next_write_count(&[0xd83d]), Err(WR_E_INVALIDSURROGATEPAIR));
}

// escape =a<b&c>d -1 -> 0x00000000 =a&lt;b&amp;c&gt;d
#[test]
fn w03_escape_markup_characters() {
    let mut out = Vec::new();
    assert_eq!(escape_string(&utf16("a<b&c>d"), &mut out), Ok(()));
    assert_eq!(String::from_utf16(&out).unwrap(), "a&lt;b&amp;c&gt;d");
}

// escape #61,0,62 3 -> 0xc00cee2b =a (the NUL is not a character; the output so far is kept)
#[test]
fn w04_escape_stops_at_nul() {
    let mut out = Vec::new();
    assert_eq!(
        escape_chars(&[0x61, 0, 0x62], &mut out),
        Err(WC_E_XMLCHARACTER)
    );
    assert_eq!(String::from_utf16(&out).unwrap(), "a");
}

// posadv 1 4294967295 61 -> 1 0
#[test]
fn r01_advance_wraps_column() {
    let mut position = Position {
        line_number: 1,
        line_position: u32::MAX,
    };
    advance(&mut position, 0x61);
    assert_eq!(
        position,
        Position {
            line_number: 1,
            line_position: 0,
        }
    );
}

// charentity fffe -> 0x00000000 =&#xfffe;
#[test]
fn w05_char_entity() {
    assert_eq!(
        String::from_utf16(&char_entity(0xfffe)).unwrap(),
        "&#xfffe;"
    );
}

// surrogate d83d de00 -> 0x00000000 =&#x1F600;
#[test]
fn w06_surrogate_entity() {
    let entity = surrogate_entity(0xd83d, 0xde00).unwrap();
    assert_eq!(String::from_utf16(&entity).unwrap(), "&#x1F600;");
}

// comment =a--b- -> 0x00000000 =<!--a- -b- -->
#[test]
fn w07_comment_dash_pairs() {
    let comment = write_comment(&utf16("a--b-"));
    assert_eq!(String::from_utf16(&comment).unwrap(), "<!--a- -b- -->");
}
