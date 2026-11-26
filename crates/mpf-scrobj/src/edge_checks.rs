//! Boundary and branch tests. Expectations here are not oracle rows: each one is read from the
//! C code in `dlls/scrobj/scrobj.c` and is marked "Not an oracle row" where it is not in
//! `target/canon/scrobj/rows.txt`.

use super::*;
use std::cell::RefCell;

fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn edge_constants() {
    // Not an oracle row: values from include/winerror.h and scrobj.c.
    assert_eq!(S_OK, 0);
    assert_eq!(E_FAIL as u32, 0x8000_4005);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(PROP_GETTER, 1);
    assert_eq!(PROP_SETTER, 2);
}

#[test]
fn edge_find_script_host_returns_first_match() {
    // Not an oracle row: wcscmp equality, first match wins.
    let jscript = w("JScript");
    let duplicated: [&[u16]; 2] = [&jscript, &jscript];
    assert_eq!(find_script_host(&duplicated, &jscript), Some(0));
}

#[test]
fn edge_is_case_xml_name_folds_low_byte_only() {
    // Not an oracle row. _memicmp folds each byte, so U+0141 (bytes 41 01) matches U+0161
    // (bytes 61 01) ignoring case. is_xml_name compares code units and rejects it.
    let capital_l_stroke = [0x0141_u16];
    assert!(is_case_xml_name(Some(&capital_l_stroke), "\u{161}"));
    assert!(!is_xml_name(Some(&capital_l_stroke), "\u{161}"));
}

#[test]
fn edge_ascii_case_fold_is_letters_only() {
    // Not an oracle row. '@' (0x40) and '`' (0x60) are adjacent to 'A' and 'a' but are not
    // letters, so they must not fold to each other. 'Z' and 'z' must fold.
    assert!(!is_case_xml_name(Some(&[0x0040]), "`"));
    assert!(is_case_xml_name(Some(&[0x005A]), "z"));
    assert!(!is_case_xml_name(Some(&[0x005B]), "{"));
}

#[test]
fn edge_name_checks_on_empty_and_longer_names() {
    // Not an oracle row. A zero-length qualified name equals the empty name, as memcmp of zero
    // bytes does in C. A longer qualified name never matches a shorter one.
    assert!(is_xml_name(Some(&[]), ""));
    assert!(is_case_xml_name(Some(&[]), ""));
    assert!(!is_xml_name(None, ""));
    assert!(!is_xml_name(Some(&w("gets")), "get"));
    assert!(!is_case_xml_name(Some(&w("gets")), "get"));
}

#[test]
fn edge_property_accessor_names() {
    // Not an oracle row: the names built for `get_`/`put_`, including the empty member name.
    assert_eq!(property_accessor_names(&[]), (w("get_"), w("put_")));
    assert_eq!(
        property_accessor_names(&w("Baz")),
        (w("get_Baz"), w("put_Baz"))
    );
}

#[test]
fn edge_lookup_script_properties_first_host_wins() {
    // Not an oracle row: both hosts know "Foo", so the lower index is returned.
    let get =
        |host: usize, name: &[u16]| (name == w("Foo").as_slice()).then_some(10 + host as DispId);
    assert_eq!(
        lookup_script_properties(3, get, &w("Foo")),
        Ok(ScriptReference { host: 0, id: 10 })
    );
}

#[test]
fn edge_property_lookup_skips_setter_when_getter_missing() {
    // Not an oracle row: a missing getter means get_<name> is the only name tried, so put_<name>
    // is never looked up. The getter is tried on both hosts.
    let seen = RefCell::new(Vec::new());
    let get = |_host: usize, name: &[u16]| {
        seen.borrow_mut().push(name.to_vec());
        None
    };
    let members = [ScriptletMember {
        kind: MemberType::Property,
        name: w("Bar"),
    }];
    assert_eq!(lookup_object_members(2, get, &members), Err(E_FAIL));
    assert_eq!(*seen.borrow(), vec![w("get_Bar"), w("get_Bar")]);
}

#[test]
fn edge_object_lookup_stops_at_first_failure() {
    // Not an oracle row: the first failing member aborts the loop, so later members are not
    // looked up.
    let seen = RefCell::new(Vec::new());
    let get = |_host: usize, name: &[u16]| {
        seen.borrow_mut().push(name.to_vec());
        None
    };
    let members = [
        ScriptletMember {
            kind: MemberType::Method,
            name: w("Missing"),
        },
        ScriptletMember {
            kind: MemberType::Method,
            name: w("Foo"),
        },
    ];
    assert_eq!(lookup_object_members(1, get, &members), Err(E_FAIL));
    assert_eq!(*seen.borrow(), vec![w("Missing")]);
}

#[test]
fn edge_node_reader_skips_leading_whitespace_and_pi() {
    // Not an oracle row: skipped nodes are dropped, and the next real node is returned.
    let nodes = [
        XmlNode::Whitespace,
        XmlNode::ProcessingInstruction,
        XmlNode::Other,
    ];
    let mut reader = NodeReader::new(&nodes);
    assert_eq!(reader.next_node(), Ok(XmlNode::Other));
}

#[test]
fn edge_node_reader_past_end_keeps_failing() {
    // Not an oracle row: reading past the end fails every time and does not panic.
    let nodes = [XmlNode::Whitespace];
    let mut reader = NodeReader::new(&nodes);
    assert_eq!(reader.next_node(), Err(E_FAIL));
    assert_eq!(reader.next_node(), Err(E_FAIL));
    assert_eq!(reader.expect_end_element(), Err(E_FAIL));
}

#[test]
fn edge_expect_end_element_accepts_end_after_whitespace() {
    // Not an oracle row: whitespace before the end element is skipped.
    let nodes = [XmlNode::Whitespace, XmlNode::EndElement];
    let mut reader = NodeReader::new(&nodes);
    assert_eq!(reader.expect_end_element(), Ok(()));
}

#[test]
fn edge_parse_property_stops_at_end_element() {
    // Not an oracle row: the loop consumes the closing element and leaves the next node unread.
    let get = w("get");
    let nodes = [
        XmlNode::Element {
            name: &get,
            empty: true,
        },
        XmlNode::EndElement,
        XmlNode::Other,
    ];
    let mut reader = NodeReader::new(&nodes);
    assert_eq!(parse_property_flags(false, &mut reader), Ok(PROP_GETTER));
    assert_eq!(reader.next_node(), Ok(XmlNode::Other));
}

#[test]
fn edge_parse_property_empty_ignores_children() {
    // Not an oracle row: an empty property element returns 0 without reading any node.
    let mut reader = NodeReader::new(&[XmlNode::Other]);
    assert_eq!(parse_property_flags(true, &mut reader), Ok(0));
    assert_eq!(reader.next_node(), Ok(XmlNode::Other));
}

#[test]
fn edge_expect_no_attributes_passes_through_read_error() {
    // Not an oracle row: a failed GetAttributeCount is returned unchanged.
    assert_eq!(expect_no_attributes(Err(E_NOTIMPL)), Err(E_NOTIMPL));
}
