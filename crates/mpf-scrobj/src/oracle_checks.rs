//! Vectors from the Wine oracle in `target/canon/scrobj` (`rows.txt`). Each assertion comment
//! gives the oracle row number. The oracle printed 79 rows and excluded none.

use super::*;

const E_OUTOFMEMORY: HRESULT = 0x8007_000e_u32 as HRESULT;

fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn dispid(host: usize, name: &[u16]) -> Option<DispId> {
    const TABLE: [(usize, &str, DispId); 5] = [
        (0, "Foo", 7),
        (1, "Bar", 9),
        (0, "get_Baz", 3),
        (1, "put_Baz", 4),
        (1, "get_Qux", 5),
    ];
    TABLE
        .iter()
        .find(|(h, n, _)| *h == host && w(n).as_slice() == name)
        .map(|(_, _, id)| *id)
}

fn script(host: usize, id: DispId) -> ScriptReference {
    ScriptReference { host, id }
}

fn method(name: &str, host: usize, id: DispId) -> ObjectMember {
    ObjectMember::Method {
        name: w(name),
        script: script(host, id),
    }
}

fn property(name: &str, get: ScriptReference, put: ScriptReference) -> ObjectMember {
    ObjectMember::Property {
        name: w(name),
        get,
        put,
    }
}

fn member(kind: MemberType, name: &str) -> ScriptletMember {
    ScriptletMember {
        kind,
        name: w(name),
    }
}

fn el(name: &[u16], empty: bool) -> XmlNode<'_> {
    XmlNode::Element { name, empty }
}

fn parse(empty: bool, nodes: &[XmlNode<'_>]) -> Result<u32, HRESULT> {
    parse_property_flags(empty, &mut NodeReader::new(nodes))
}

#[test]
fn oracle_find_script_host() {
    let jscript = w("JScript");
    let vbscript = w("VBScript");
    let perl = w("PerlScript");
    let two: [&[u16]; 2] = [&jscript, &vbscript];
    let perl_only: [&[u16]; 1] = [&perl];
    let lower = w("vbscript");
    let perl_name = w("Perl");

    assert_eq!(find_script_host(&two, &vbscript), Some(1)); // row 01
    assert_eq!(find_script_host(&two, &jscript), Some(0)); // row 02
    assert_eq!(find_script_host(&two, &lower), None); // row 03
    assert_eq!(find_script_host(&[], &jscript), None); // row 04
    assert_eq!(find_script_host(&perl_only, &perl_name), None); // row 05
}

#[test]
fn oracle_lookup_script_properties() {
    assert_eq!(
        lookup_script_properties(2, dispid, &w("Bar")),
        Ok(script(1, 9))
    ); // row 06
    assert_eq!(
        lookup_script_properties(2, dispid, &w("Foo")),
        Ok(script(0, 7))
    ); // row 07
    assert_eq!(
        lookup_script_properties(2, dispid, &w("Missing")),
        Err(E_FAIL)
    ); // row 08
    assert_eq!(
        lookup_script_properties(2, dispid, &w("put_Baz")),
        Ok(script(1, 4))
    ); // row 09
    assert_eq!(lookup_script_properties(0, dispid, &w("Foo")), Err(E_FAIL)); // row 10
}

#[test]
fn oracle_lookup_object_members() {
    assert_eq!(lookup_object_members(2, dispid, &[]), Ok(vec![])); // row 11
    assert_eq!(
        lookup_object_members(2, dispid, &[member(MemberType::Method, "Foo")]),
        Ok(vec![method("Foo", 0, 7)])
    ); // row 12
    assert_eq!(
        lookup_object_members(2, dispid, &[member(MemberType::Property, "Baz")]),
        Ok(vec![property("Baz", script(0, 3), script(1, 4))])
    ); // row 13
    assert_eq!(
        lookup_object_members(2, dispid, &[member(MemberType::Property, "Bar")]),
        Err(E_FAIL)
    ); // row 14
    assert_eq!(
        lookup_object_members(2, dispid, &[member(MemberType::Property, "Qux")]),
        Err(E_FAIL)
    ); // row 15
    assert_eq!(
        lookup_object_members(2, dispid, &[member(MemberType::Method, "Missing")]),
        Err(E_FAIL)
    ); // row 16
    assert_eq!(
        lookup_object_members(
            2,
            dispid,
            &[
                member(MemberType::Method, "Bar"),
                member(MemberType::Property, "Baz"),
            ]
        ),
        Ok(vec![
            method("Bar", 1, 9),
            property("Baz", script(0, 3), script(1, 4)),
        ])
    ); // row 17
    assert_eq!(
        lookup_object_members(
            2,
            dispid,
            &[
                member(MemberType::Property, "Baz"),
                member(MemberType::Method, "Missing"),
            ]
        ),
        Err(E_FAIL)
    ); // row 18
}

#[test]
fn oracle_parse_property_flags() {
    let get = w("get");
    let put = w("put");
    let upper_put = w("PUT");
    let upper_get = w("GeT");
    let value = w("value");
    let getter = w("getter");
    let end = XmlNode::EndElement;
    let text = XmlNode::Other;

    assert_eq!(parse(true, &[]), Ok(0)); // row 19
    assert_eq!(parse(false, &[end]), Err(E_NOTIMPL)); // row 20
    assert_eq!(parse(false, &[el(&get, true), end]), Ok(1)); // row 21
    assert_eq!(parse(false, &[el(&put, true), end]), Ok(2)); // row 22
    assert_eq!(parse(false, &[el(&get, true), el(&put, true), end]), Ok(3)); // row 23
    assert_eq!(
        parse(false, &[el(&upper_put, true), el(&upper_get, true), end]),
        Ok(3)
    ); // row 24
    assert_eq!(parse(false, &[el(&get, false), end, end]), Ok(1)); // row 25
    assert_eq!(
        parse(false, &[el(&put, false), end, el(&get, true), end]),
        Ok(3)
    ); // row 26
    assert_eq!(parse(false, &[el(&get, false), text]), Err(E_FAIL)); // row 27
    assert_eq!(
        parse(false, &[el(&get, true), el(&get, true), end]),
        Err(E_FAIL)
    ); // row 28
    assert_eq!(
        parse(false, &[el(&put, true), el(&put, true), end]),
        Err(E_FAIL)
    ); // row 29
    assert_eq!(parse(false, &[el(&value, true), end]), Err(E_FAIL)); // row 30
    assert_eq!(parse(false, &[el(&getter, true), end]), Err(E_FAIL)); // row 31
    assert_eq!(parse(false, &[text]), Err(E_FAIL)); // row 32
    assert_eq!(parse(false, &[]), Err(E_FAIL)); // row 33
    assert_eq!(parse(false, &[el(&get, false)]), Err(E_FAIL)); // row 34
    assert_eq!(
        parse(false, &[el(&get, false), el(&put, true), end]),
        Err(E_FAIL)
    ); // row 35
    assert_eq!(
        parse(
            false,
            &[
                XmlNode::Whitespace,
                el(&get, true),
                XmlNode::ProcessingInstruction,
                XmlNode::Whitespace,
                end,
            ]
        ),
        Ok(1)
    ); // row 36
}

#[test]
fn oracle_xml_name_checks() {
    let get = w("get");
    let upper_get = w("GET");
    let getter = w("getter");
    let ge = w("ge");
    let text_qname: Vec<u16> = Vec::new();

    assert!(!is_xml_name(None, "get")); // row 37
    assert!(!is_xml_name(Some(upper_get.as_slice()), "get")); // row 38
    assert!(is_case_xml_name(Some(upper_get.as_slice()), "get")); // row 39
    assert!(is_xml_name(Some(get.as_slice()), "get")); // row 40
    assert!(!is_case_xml_name(Some(getter.as_slice()), "get")); // row 41
    assert!(!is_case_xml_name(Some(ge.as_slice()), "get")); // row 42
    assert!(!is_case_xml_name(Some(text_qname.as_slice()), "get")); // row 43: oracle text node has an empty qname
}

#[test]
fn oracle_expect_no_attributes() {
    assert_eq!(expect_no_attributes(Ok(0)), Ok(())); // row 44
    assert_eq!(expect_no_attributes(Ok(2)), Err(E_FAIL)); // row 45
    assert_eq!(expect_no_attributes(Err(E_OUTOFMEMORY)), Err(E_OUTOFMEMORY)); // row 46
}

#[test]
fn oracle_global_methods_return_e_notimpl() {
    assert_eq!(global_get_type_info_count(), E_NOTIMPL); // row 47
    assert_eq!(global_get_type_info(), E_NOTIMPL); // row 48
    assert_eq!(global_get_disp_id(), E_NOTIMPL); // row 49
    assert_eq!(global_invoke_ex(), E_NOTIMPL); // row 50
    assert_eq!(global_delete_member_by_name(), E_NOTIMPL); // row 51
    assert_eq!(global_delete_member_by_disp_id(), E_NOTIMPL); // row 52
    assert_eq!(global_get_member_properties(), E_NOTIMPL); // row 53
    assert_eq!(global_get_member_name(), E_NOTIMPL); // row 54
    assert_eq!(global_get_next_disp_id(), E_NOTIMPL); // row 55
    assert_eq!(global_get_name_space_parent(), E_NOTIMPL); // row 56
}

#[test]
fn oracle_scriptlet_methods_return_e_notimpl() {
    assert_eq!(scriptlet_get_type_info_count(), E_NOTIMPL); // row 57
    assert_eq!(scriptlet_get_type_info(), E_NOTIMPL); // row 58
    assert_eq!(scriptlet_delete_member_by_name(), E_NOTIMPL); // row 59
    assert_eq!(scriptlet_delete_member_by_disp_id(), E_NOTIMPL); // row 60
    assert_eq!(scriptlet_get_member_properties(), E_NOTIMPL); // row 61
    assert_eq!(scriptlet_get_member_name(), E_NOTIMPL); // row 62
    assert_eq!(scriptlet_get_next_disp_id(), E_NOTIMPL); // row 63
    assert_eq!(scriptlet_get_name_space_parent(), E_NOTIMPL); // row 64
}

#[test]
fn oracle_typelib_methods_return_e_notimpl() {
    assert_eq!(scriptlet_typelib_add_url(), E_NOTIMPL); // row 65
    assert_eq!(scriptlet_typelib_put_path(), E_NOTIMPL); // row 66
    assert_eq!(scriptlet_typelib_get_path(), E_NOTIMPL); // row 67
    assert_eq!(scriptlet_typelib_put_doc(), E_NOTIMPL); // row 68
    assert_eq!(scriptlet_typelib_get_doc(), E_NOTIMPL); // row 69
    assert_eq!(scriptlet_typelib_put_name(), E_NOTIMPL); // row 70
    assert_eq!(scriptlet_typelib_get_name(), E_NOTIMPL); // row 71
    assert_eq!(scriptlet_typelib_put_major_version(), E_NOTIMPL); // row 72
    assert_eq!(scriptlet_typelib_get_major_version(), E_NOTIMPL); // row 73
    assert_eq!(scriptlet_typelib_put_minor_version(), E_NOTIMPL); // row 74
    assert_eq!(scriptlet_typelib_get_minor_version(), E_NOTIMPL); // row 75
    assert_eq!(scriptlet_typelib_write(), E_NOTIMPL); // row 76
    assert_eq!(scriptlet_typelib_reset(), E_NOTIMPL); // row 77
    assert_eq!(scriptlet_typelib_put_guid(), E_NOTIMPL); // row 78
}

#[test]
fn oracle_lock_server_returns_s_ok() {
    assert_eq!(scriptlet_factory_lock_server(true), S_OK); // row 79
}
