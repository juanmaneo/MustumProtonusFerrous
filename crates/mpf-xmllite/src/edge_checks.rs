//! Edge cases beyond the oracle rows. Unless a comment says otherwise, each expectation is the
//! output of the sanitizer-built C harness for the same input (target/canon/xmllite, edge_out.txt).

use crate::{
    advance, c_str, char_entity, detect_encoding, eq_ascii, escape_chars, escape_string,
    get_code_page, get_encoding_from_codepage, get_encoding_name, is_char, is_namechar,
    is_namestartchar, is_ncnamechar, is_ncnamestartchar, is_pubchar, is_valid_name,
    is_valid_ncname, is_valid_nmtoken, is_valid_pubid, is_valid_qname, is_valid_whitespace,
    is_valid_xml_space_value, is_wchar_space, next_write_count, parse_encoding_name,
    parse_encoding_prefix, predefined_entity, surrogate_entity, utf16, write_comment, Position,
    XmlEncoding, E_NOTIMPL, MX_E_INPUTEND, WC_E_NAMECHARACTER, WC_E_PUBLICID, WC_E_XMLCHARACTER,
    WR_E_INVALIDSURROGATEPAIR,
};

/// The units at the ends of the class ranges, with the C output for the seven class bits in
/// the order is_char, is_wchar_space, is_pubchar, is_ncnamestartchar, is_namestartchar,
/// is_ncnamechar, is_namechar.
const CLASS_EDGES: &[(u16, &str)] = &[
    (0x0000, "0000000"),
    (0x0008, "0000000"),
    (0x0009, "1100000"),
    (0x000a, "1110000"),
    (0x000b, "0000000"),
    (0x000d, "1110000"),
    (0x000e, "0000000"),
    (0x001f, "0000000"),
    (0x0020, "1110000"),
    (0x0021, "1010000"),
    (0x0022, "1000000"),
    (0x0023, "1010000"),
    (0x0025, "1010000"),
    (0x0026, "1000000"),
    (0x0027, "1000000"),
    (0x0028, "1000000"),
    (0x002c, "1000000"),
    (0x002d, "1010011"),
    (0x002e, "1010011"),
    (0x002f, "1010000"),
    (0x0030, "1010011"),
    (0x0039, "1010011"),
    (0x003a, "1010101"),
    (0x003b, "1010000"),
    (0x003c, "1000000"),
    (0x003d, "1010000"),
    (0x003e, "1000000"),
    (0x003f, "1010000"),
    (0x0040, "1010000"),
    (0x0041, "1011111"),
    (0x005a, "1011111"),
    (0x005b, "1000000"),
    (0x005e, "1000000"),
    (0x005f, "1011111"),
    (0x0060, "1000000"),
    (0x0061, "1011111"),
    (0x007a, "1011111"),
    (0x007b, "1000000"),
    (0x007f, "1000000"),
    (0x00b7, "1000011"),
    (0x00bf, "1000000"),
    (0x00c0, "1001111"),
    (0x00d6, "1001111"),
    (0x00d7, "1000000"),
    (0x00d8, "1001111"),
    (0x00f6, "1001111"),
    (0x00f8, "1001111"),
    (0x0300, "1000011"),
    (0x036f, "1000011"),
    (0x037d, "1001111"),
    (0x037f, "1001111"),
    (0x1fff, "1001111"),
    (0x200c, "1001111"),
    (0x200d, "1001111"),
    (0x203f, "1000011"),
    (0x2040, "1000011"),
    (0x2070, "1001111"),
    (0x218f, "1001111"),
    (0x2c00, "1001111"),
    (0x2fef, "1001111"),
    (0x3001, "1001111"),
    (0xd7ff, "1001111"),
    (0xd800, "1001111"),
    (0xdbff, "1001111"),
    (0xdc00, "1001111"),
    (0xdfff, "1001111"),
    (0xe000, "1000000"),
    (0xf900, "1001111"),
    (0xfdcf, "1001111"),
    (0xfdf0, "1001111"),
    (0xfffd, "1001111"),
    (0xfffe, "0000000"),
    (0xffff, "0000000"),
];

#[test]
fn class_edges_match_c() {
    for &(unit, bits) in CLASS_EDGES {
        let got: String = [
            is_char(unit),
            is_wchar_space(unit),
            is_pubchar(unit),
            is_ncnamestartchar(unit),
            is_namestartchar(unit),
            is_ncnamechar(unit),
            is_namechar(unit),
        ]
        .into_iter()
        .map(|bit| if bit { '1' } else { '0' })
        .collect();
        assert_eq!(got, bits, "unit {unit:04x}");
    }
}

#[test]
fn derived_traits() {
    let encodings = [
        XmlEncoding::UsAscii,
        XmlEncoding::Utf16,
        XmlEncoding::Utf8,
        XmlEncoding::Unknown,
    ];
    let names: Vec<String> = encodings
        .iter()
        .map(|encoding| format!("{encoding:?}"))
        .collect();
    assert_eq!(names.join(" "), "UsAscii Utf16 Utf8 Unknown");
    for encoding in encodings {
        assert_eq!(Clone::clone(&encoding), encoding);
    }

    let position = Position {
        line_number: 1,
        line_position: 2,
    };
    assert_eq!(
        format!("{position:?}"),
        "Position { line_number: 1, line_position: 2 }"
    );
    assert_eq!(Clone::clone(&position), position);
}

#[test]
fn names_and_code_pages() {
    assert_eq!(get_encoding_name(XmlEncoding::UsAscii), Some("US-ASCII"));
    assert_eq!(get_encoding_name(XmlEncoding::Utf16), Some("UTF-16"));
    assert_eq!(get_encoding_name(XmlEncoding::Utf8), Some("UTF-8"));
    // Port-defined: the C indexes past its table for Unknown.
    assert_eq!(get_encoding_name(XmlEncoding::Unknown), None);

    assert_eq!(get_code_page(XmlEncoding::UsAscii), Ok(20127));
    assert_eq!(get_code_page(XmlEncoding::Utf16), Ok(1200));
    assert_eq!(get_code_page(XmlEncoding::Unknown), Err(E_NOTIMPL));

    assert_eq!(get_encoding_from_codepage(20127), XmlEncoding::UsAscii);
    assert_eq!(get_encoding_from_codepage(1200), XmlEncoding::Utf16);
    assert_eq!(get_encoding_from_codepage(65001), XmlEncoding::Utf8);
    for code_page in [0, 1201, 1] {
        assert_eq!(get_encoding_from_codepage(code_page), XmlEncoding::Unknown);
    }
}

#[test]
fn parse_nul_terminated_name() {
    for (name, expected) in [
        ("US-ASCII", XmlEncoding::UsAscii),
        ("us-ascii", XmlEncoding::UsAscii),
        ("Utf-16", XmlEncoding::Utf16),
        ("UTF-8", XmlEncoding::Utf8),
        ("utf-8", XmlEncoding::Utf8),
    ] {
        assert_eq!(parse_encoding_name(&utf16(name)), expected, "{name}");
    }

    let mut with_tail = utf16("UTF-16");
    with_tail.extend_from_slice(&[0, 0x78]);
    assert_eq!(parse_encoding_name(&with_tail), XmlEncoding::Utf16);

    for name in [
        "UTF-8x", "UTF-7", "UTF-9", "ZZZ", "T", "UT", "U", "US", "A", "US-ASCI", "",
    ] {
        assert_eq!(
            parse_encoding_name(&utf16(name)),
            XmlEncoding::Unknown,
            "{name}"
        );
    }
}

#[test]
fn parse_counted_name() {
    for name in ["UTF-1", "UT", "U"] {
        assert_eq!(
            parse_encoding_prefix(&utf16(name)),
            XmlEncoding::Utf16,
            "{name}"
        );
    }
    for name in ["US", "US-ASCI", "US-ASCII"] {
        assert_eq!(
            parse_encoding_prefix(&utf16(name)),
            XmlEncoding::UsAscii,
            "{name}"
        );
    }
    assert_eq!(parse_encoding_prefix(&utf16("UTF-8")), XmlEncoding::Utf8);
    assert_eq!(parse_encoding_prefix(&utf16("utf-8")), XmlEncoding::Utf8);

    let mut utf8_with_nul = utf16("UTF-8");
    utf8_with_nul.push(0);
    assert_eq!(parse_encoding_prefix(&utf8_with_nul), XmlEncoding::Utf8);

    for name in ["US-ASCIIX", "UTF-8x", "T", "A", "V", "Z"] {
        assert_eq!(
            parse_encoding_prefix(&utf16(name)),
            XmlEncoding::Unknown,
            "{name}"
        );
    }

    // A zero-length name matches the first table entry, as the C's zero-count compare does.
    // NULL, which is Unknown in C, has no slice form.
    assert_eq!(parse_encoding_prefix(&[]), XmlEncoding::Utf16);
}

#[test]
fn detect_encoding_edges() {
    let short: [&[u8]; 4] = [&[], &[0x41], &[0xff, 0xfe], &[0xef, 0xbb]];
    for data in short {
        assert_eq!(detect_encoding(data), Err(MX_E_INPUTEND), "{data:x?}");
    }

    let cases: [(&[u8], XmlEncoding, usize); 15] = [
        (&[0x3c, 0x3f, 0x78], XmlEncoding::Utf8, 0),
        (&[0x3c, 0x21, 0x78], XmlEncoding::Utf8, 0),
        (&[0x3c, 0x61, 0x62], XmlEncoding::Utf8, 0),
        (&[0x3c, 0x7f, 0x00, 0x00], XmlEncoding::Utf8, 0),
        // A lead byte of 0xc0 would pass the two-byte shift test if the char were unsigned.
        (&[0x3c, 0x80, 0x00, 0x00], XmlEncoding::Unknown, 0),
        (&[0x3c, 0xc0, 0x80, 0x00], XmlEncoding::Unknown, 0),
        (&[0x3c, 0x00, 0x21, 0x00], XmlEncoding::Utf16, 0),
        (&[0x3c, 0x00, 0x3a, 0x00], XmlEncoding::Utf16, 0),
        (&[0x3c, 0x00, 0x41, 0x00], XmlEncoding::Utf16, 0),
        (&[0x3c, 0x00, 0x61, 0x00], XmlEncoding::Utf16, 0),
        (&[0x3c, 0x00, 0x20, 0x00], XmlEncoding::Unknown, 0),
        (&[0x3c, 0x00, 0x00, 0x00], XmlEncoding::Unknown, 0),
        (&[0xef, 0xbb, 0xbf], XmlEncoding::Utf8, 3),
        (&[0xff, 0xfe, 0x41, 0x00], XmlEncoding::Utf16, 2),
        (&[0x00, 0x00, 0x00], XmlEncoding::Unknown, 0),
    ];
    for (data, encoding, bom) in cases {
        assert_eq!(detect_encoding(data), Ok((encoding, bom)), "{data:x?}");
    }

    // The C reads byte 3 past the end of this input. The port reads it as 0, which still
    // gives UTF-16 here.
    assert_eq!(
        detect_encoding(&[0x3c, 0x00, 0x3f]),
        Ok((XmlEncoding::Utf16, 0))
    );
}

#[test]
fn ncname_edges() {
    assert_eq!(is_valid_ncname(&utf16("a.b-c_d")), Ok(7));
    assert_eq!(is_valid_ncname(&[]), Ok(0));
    assert_eq!(is_valid_ncname(&utf16("a:")), Err(WC_E_NAMECHARACTER));
    assert_eq!(is_valid_ncname(&utf16(":")), Err(WC_E_NAMECHARACTER));
    assert_eq!(is_valid_ncname(&[0x41, 0xd800, 0xdc00]), Ok(3));
    assert_eq!(is_valid_ncname(&[0x41, 0, 0x2a]), Ok(1));
}

#[test]
fn name_edges() {
    assert_eq!(is_valid_name(&utf16(":a")), Ok(2));
    assert_eq!(is_valid_name(&utf16("a:b")), Ok(3));
    assert_eq!(is_valid_name(&utf16("1")), Err(WC_E_NAMECHARACTER));
    assert_eq!(is_valid_name(&[0xffff]), Err(WC_E_NAMECHARACTER));
    assert_eq!(is_valid_name(&[]), Ok(0));
    assert_eq!(is_valid_name(&[0x61, 0, 0x2a]), Ok(1));
}

#[test]
fn qname_edges() {
    assert_eq!(is_valid_qname(&utf16("a")), Ok(1));
    assert_eq!(is_valid_qname(&utf16("a:b")), Ok(3));
    assert_eq!(is_valid_qname(&utf16("a:1")), Ok(3));
    assert_eq!(is_valid_qname(&utf16("a-b:c")), Ok(5));
    assert_eq!(is_valid_qname(&utf16("a:-b")), Ok(4));
    assert_eq!(is_valid_qname(&utf16("a:")), Ok(2));
    assert_eq!(is_valid_qname(&[0x61, 0, 0x3a]), Ok(1));

    for name in [":b", "a:b:c", "", "1"] {
        assert_eq!(
            is_valid_qname(&utf16(name)),
            Err(WC_E_NAMECHARACTER),
            "{name}"
        );
    }
    assert_eq!(is_valid_qname(&[0x3a]), Err(WC_E_NAMECHARACTER));
}

#[test]
fn pubid_edges() {
    assert_eq!(is_valid_pubid(&[]), Ok(0));
    assert_eq!(is_valid_pubid(&[0x61, 0x20, 0x62]), Ok(3));
    assert_eq!(is_valid_pubid(&[0x0a, 0x0d, 0x20]), Ok(3));
    assert_eq!(is_valid_pubid(&[0x0a]), Ok(1));
    assert_eq!(is_valid_pubid(&[0x09]), Err(WC_E_PUBLICID));
    assert_eq!(is_valid_pubid(&[0x3c]), Err(WC_E_PUBLICID));
    assert_eq!(is_valid_pubid(&[0x61, 0, 0x3c]), Ok(1));
}

#[test]
fn nmtoken_and_whitespace_edges() {
    assert_eq!(is_valid_nmtoken(&utf16("a:b-c")), Some(5));
    assert_eq!(is_valid_nmtoken(&[0x3a]), Some(1));
    assert_eq!(is_valid_nmtoken(&[]), Some(0));
    assert_eq!(is_valid_nmtoken(&[0x20]), None);
    assert_eq!(is_valid_nmtoken(&[0x61, 0, 0x20]), Some(1));

    assert_eq!(is_valid_whitespace(&[]), Some(0));
    assert_eq!(is_valid_whitespace(&[0x09, 0x20, 0x0d, 0x0a]), Some(4));
    assert_eq!(is_valid_whitespace(&[0x09, 0x0b]), None);
    assert_eq!(is_valid_whitespace(&[0x61]), None);
    assert_eq!(is_valid_whitespace(&[0x20, 0, 0x61]), Some(1));
}

#[test]
fn xml_space_value_edges() {
    assert!(is_valid_xml_space_value(&utf16("preserve")));
    assert!(is_valid_xml_space_value(&utf16("default")));
    for value in ["Preserve", "preserv", "defaults", ""] {
        assert!(!is_valid_xml_space_value(&utf16(value)), "{value}");
    }

    let mut with_tail = utf16("preserve");
    with_tail.extend_from_slice(&[0, 0x78]);
    assert!(is_valid_xml_space_value(&with_tail));
}

#[test]
fn next_write_count_edges() {
    assert_eq!(next_write_count(&utf16("a")), Ok(1));
    assert_eq!(next_write_count(&[0x09]), Ok(1));
    assert_eq!(next_write_count(&[0]), Err(WC_E_XMLCHARACTER));
    assert_eq!(next_write_count(&[0xfffe]), Err(WC_E_XMLCHARACTER));
    // An empty text reads as NUL. Both callers check for it first, so C never reaches this.
    assert_eq!(next_write_count(&[]), Err(WC_E_XMLCHARACTER));
    assert_eq!(next_write_count(&[0xd800]), Err(WR_E_INVALIDSURROGATEPAIR));
    assert_eq!(
        next_write_count(&[0xd800, 0x41]),
        Err(WR_E_INVALIDSURROGATEPAIR)
    );
    assert_eq!(next_write_count(&[0xdc00]), Err(WR_E_INVALIDSURROGATEPAIR));
    assert_eq!(next_write_count(&[0xd83d, 0xde00]), Ok(2));
}

fn escape_with(
    escape: fn(&[u16], &mut Vec<u16>) -> Result<(), i32>,
    text: &[u16],
) -> (Result<(), i32>, Vec<u16>) {
    let mut out = Vec::new();
    let result = escape(text, &mut out);
    (result, out)
}

#[test]
fn escape_counted_edges() {
    assert_eq!(escape_with(escape_chars, &[]), (Ok(()), Vec::new()));
    assert_eq!(escape_with(escape_chars, &[0x3c]), (Ok(()), utf16("&lt;")));
    assert_eq!(escape_with(escape_chars, &[0x26]), (Ok(()), utf16("&amp;")));
    assert_eq!(escape_with(escape_chars, &[0x3e]), (Ok(()), utf16("&gt;")));
    assert_eq!(
        escape_with(escape_chars, &utf16("\"'")),
        (Ok(()), utf16("\"'"))
    );
    assert_eq!(
        escape_with(escape_chars, &[0xd800, 0xdc00]),
        (Ok(()), Vec::from([0xd800, 0xdc00]))
    );
    assert_eq!(
        escape_with(escape_chars, &[0xd800]),
        (Err(WR_E_INVALIDSURROGATEPAIR), Vec::new())
    );
    assert_eq!(
        escape_with(escape_chars, &[0x41, 0xd800, 0x41]),
        (Err(WR_E_INVALIDSURROGATEPAIR), utf16("A"))
    );
    assert_eq!(
        escape_with(escape_chars, &[0x41, 0, 0x41]),
        (Err(WC_E_XMLCHARACTER), utf16("A"))
    );
    assert_eq!(
        escape_with(escape_chars, &utf16("ab")[..1]),
        (Ok(()), utf16("a"))
    );
}

#[test]
fn escape_nul_terminated_edges() {
    assert_eq!(
        escape_with(escape_string, &[0x41, 0, 0x41]),
        (Ok(()), utf16("A"))
    );
    assert_eq!(
        escape_with(escape_string, &[0xd800]),
        (Err(WR_E_INVALIDSURROGATEPAIR), Vec::new())
    );
    assert_eq!(escape_with(escape_string, &[]), (Ok(()), Vec::new()));
}

#[test]
fn char_entity_edges() {
    assert_eq!(char_entity(0), utf16("&#x0;"));
    assert_eq!(char_entity(0x41), utf16("&#x41;"));
    assert_eq!(char_entity(0x10), utf16("&#x10;"));
    assert_eq!(char_entity(0xffff), utf16("&#xffff;"));
}

#[test]
fn surrogate_entity_edges() {
    assert_eq!(surrogate_entity(0xd800, 0xdc00), Ok(utf16("&#x10000;")));
    assert_eq!(surrogate_entity(0xdbff, 0xdfff), Ok(utf16("&#x10FFFF;")));
    for (high, low) in [
        (0x41, 0xde00),
        (0xd83d, 0x41),
        (0xdc00, 0xdc00),
        (0xd800, 0xe000),
    ] {
        assert_eq!(surrogate_entity(high, low), Err(WC_E_XMLCHARACTER));
    }
}

#[test]
fn write_comment_edges() {
    for (text, expected) in [
        ("", "<!---->"),
        ("-", "<!--- -->"),
        ("--", "<!--- - -->"),
        ("---", "<!--- - - -->"),
        ("a-b", "<!--a-b-->"),
        ("a-", "<!--a- -->"),
        ("--a", "<!--- -a-->"),
        ("a--", "<!--a- - -->"),
        ("ab", "<!--ab-->"),
        ("a", "<!--a-->"),
        ("-a", "<!---a-->"),
    ] {
        assert_eq!(write_comment(&utf16(text)), utf16(expected), "{text}");
    }
    assert_eq!(write_comment(&[0x61, 0, 0x62]), utf16("<!--a-->"));
    assert_eq!(write_comment(&[]), utf16("<!---->"));
}

fn after(line: u32, column: u32, units: &[u16]) -> (u32, u32) {
    let mut position = Position {
        line_number: line,
        line_position: column,
    };
    for &unit in units {
        advance(&mut position, unit);
    }
    (position.line_number, position.line_position)
}

#[test]
fn advance_edges() {
    assert_eq!(after(1, 5, &[0x0a]), (2, 1));
    assert_eq!(after(1, 5, &[0x0d]), (1, 1));
    assert_eq!(after(1, 5, &[0x0d, 0x0a]), (2, 1));
    assert_eq!(after(1, 5, &[0x0d, 0x0a, 0x0a]), (3, 1));
    assert_eq!(after(1, 5, &[0x61]), (1, 6));
    assert_eq!(after(u32::MAX, 5, &[0x0a]), (0, 1));
    assert_eq!(after(1, u32::MAX, &[0x61]), (1, 0));
    assert_eq!(after(u32::MAX, u32::MAX, &[0x09]), (u32::MAX, 0));
    assert_eq!(after(1, 5, &[]), (1, 5));
}

#[test]
fn predefined_entity_edges() {
    assert_eq!(predefined_entity(&utf16("lt")), Some(0x3c));
    assert_eq!(predefined_entity(&utf16("gt")), Some(0x3e));
    assert_eq!(predefined_entity(&utf16("amp")), Some(0x26));
    assert_eq!(predefined_entity(&utf16("apos")), Some(0x27));
    assert_eq!(predefined_entity(&utf16("quot")), Some(0x22));
    for name in [
        "LT", "ltx", "l", "gtx", "apo", "x", "ampx", "abc", "quo", "qu", "a", "q", "g", "ap", "",
    ] {
        assert_eq!(predefined_entity(&utf16(name)), None, "{name}");
    }
}

#[test]
fn c_str_and_eq_ascii() {
    assert_eq!(c_str(&[0x41, 0, 0x42]), &[0x41][..]);
    assert_eq!(c_str(&[0x41]), &[0x41][..]);
    assert!(c_str(&[]).is_empty());
    assert!(eq_ascii(&utf16("ab"), "ab"));
    assert!(!eq_ascii(&utf16("a"), "ab"));
    assert!(!eq_ascii(&utf16("ax"), "ab"));
}
