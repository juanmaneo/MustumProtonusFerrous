//! Edge tests for the port: the paths the oracle rows do not reach. Expected values come from
//! reading the C code, and the differences are listed in the crate docs.

use super::*;
use crate::entity::ENTITIES;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn node_name_stops_at_each_separator_byte() {
    const STOPS: &[u8] = b"\t\n\x0b\x0c\r >";
    for c in 0..=255u8 {
        let tag = [b'<', b'a', c, b'b', b'>'];
        let expected: &[u8] = if STOPS.contains(&c) { b"a" } else { &tag[1..4] };
        assert_eq!(node_name(&tag), expected, "byte {c:#04x}");
    }
}

#[test]
fn node_name_without_terminator_or_input() {
    assert_eq!(node_name(b"<abc"), b"abc");
    assert_eq!(node_name(b"<"), b"");
    assert_eq!(node_name(b""), b"");
    assert_eq!(node_name(b"<>"), b"");
    assert_eq!(node_name(b"</a href=x>"), b"/a");
}

#[test]
fn attr_value_keeps_value_case_and_takes_first_match() {
    assert_eq!(
        attr_value(b"<a HREF=\"Dir/Page.htm\">", b"href"),
        Some(&b"Dir/Page.htm"[..])
    );
    assert_eq!(
        attr_value(b"<p name=\"one\" name=\"two\">", b"NAME"),
        Some(&b"one"[..])
    );
}

#[test]
fn attr_value_handles_empty_name_and_value() {
    assert_eq!(attr_value(b"<object type=\"\">", b"type"), Some(&b""[..]));
    assert_eq!(attr_value(b"<a =\"x\">", b""), Some(&b"x"[..]));
}

#[test]
fn attr_value_needs_the_quoted_form() {
    assert_eq!(attr_value(b"<p name = \"x\">", b"name"), None);
    assert_eq!(attr_value(b"<p name='x'>", b"name"), None);
    assert_eq!(attr_value(b"<p name=\"abc>", b"name"), None);
    assert_eq!(attr_value(b"", b"name"), None);
}

#[test]
fn attr_value_matches_the_end_of_a_longer_name() {
    assert_eq!(
        attr_value(b"<param xname=\"a\" name=\"b\">", b"name"),
        Some(&b"a"[..])
    );
}

#[test]
fn attr_value_folds_ascii_case_only() {
    assert_eq!(attr_value(b"<p \xC9=\"x\">", b"\xC9"), Some(&b"x"[..]));
    assert_eq!(attr_value(b"<p \xC9=\"x\">", b"\xE9"), None);
}

#[test]
fn strip_schema_ignores_case_for_every_prefix() {
    assert_eq!(strip_schema(&wide("ITS:a.htm")), &wide("a.htm")[..]);
    assert_eq!(strip_schema(&wide("Ms-Its:a.htm")), &wide("a.htm")[..]);
    assert_eq!(
        strip_schema(&wide("MK:@msitstore:a.htm")),
        &wide("a.htm")[..]
    );
}

#[test]
fn strip_schema_leaves_other_urls_alone() {
    for url in [
        "http://x",
        "its",
        "ms-its",
        "mk:@MSITStore",
        "x its:a",
        "\u{0129}ts:a",
        "",
    ] {
        assert_eq!(strip_schema(&wide(url)), &wide(url)[..], "{url}");
    }
}

#[test]
fn is_chm_url_is_case_sensitive_and_prefix_only() {
    assert!(is_chm_url(&wide("mk:@MSITStore:")));
    assert!(is_chm_url(&wide("mk:@MSITStore:x.chm")));
    assert!(!is_chm_url(&wide("mk:@MSITStoreX.chm")));
    assert!(!is_chm_url(&wide("mk:@MSITStore")));
    assert!(!is_chm_url(&wide("mk:@msitstore:x")));
    assert!(!is_chm_url(&wide("its:x")));
    assert!(!is_chm_url(&wide("\u{0101}k:@MSITStore:")));
}

#[test]
fn decode_entities_copies_text_without_references() {
    assert_eq!(decode_entities(b""), Vec::<u8>::new());
    assert_eq!(
        decode_entities(b"plain \xE9 text"),
        b"plain \xE9 text".to_vec()
    );
}

#[test]
fn decode_entities_keeps_unfinished_references() {
    assert_eq!(decode_entities(b"a &"), b"a &".to_vec());
    assert_eq!(decode_entities(b"&amp"), b"&amp".to_vec());
    assert_eq!(decode_entities(b"&gt"), b"&gt".to_vec());
    assert_eq!(decode_entities(b"&;"), b"&;".to_vec());
    assert_eq!(decode_entities(b"&#;"), b"&#;".to_vec());
    assert_eq!(decode_entities(b"&#x41;"), b"&#x41;".to_vec());
    assert_eq!(decode_entities(b"&&amp;"), b"&&".to_vec());
}

#[test]
fn decode_entities_keeps_names_missing_from_the_table() {
    for name in ["AMP", "ampx", "am", "zzz", "AAA", "AEli", "yuml2", ""] {
        let fragment = format!("&{name};");
        assert_eq!(
            decode_entities(fragment.as_bytes()),
            fragment.as_bytes(),
            "{name}"
        );
    }
}

#[test]
fn decode_entities_decodes_every_table_entry() {
    for &(name, symbol) in ENTITIES {
        let fragment = format!("<&{name};>");
        let expected = [&b"<"[..], &[symbol][..], &b">"[..]].concat();
        assert_eq!(decode_entities(fragment.as_bytes()), expected, "{name}");
    }
}

#[test]
fn decode_entities_replaces_each_reference() {
    assert_eq!(
        decode_entities(b"&lt;&gt;&amp;&quot;&nbsp;"),
        b"<>&\" ".to_vec()
    );
    assert_eq!(decode_entities(b"\xE9 &eacute;"), b"\xE9 \xE9".to_vec());
    assert_eq!(decode_entities(b"&lt;"), b"<".to_vec());
}
