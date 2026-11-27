//! Boundary and branch tests for the msxml2 port. Each expectation that does not come from an
//! oracle row says so in its comment.

use super::*;

#[test]
fn exports_are_in_spec_order() {
    // Not oracle-derived: the four names are the lines of `msxml2.spec`, in order.
    let names: Vec<&str> = EXPORTS.iter().map(|&(name, _)| name).collect();
    assert_eq!(
        names,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer",
        ]
    );
}

#[test]
fn export_kind_finds_each_spec_export() {
    // Not oracle-derived: the kinds follow from the spec lines. The oracle shows only the flags
    // and errors, not the target text of the forwarder.
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Undefined));
    assert_eq!(
        export_kind("DllGetClassObject"),
        Some(ExportKind::Forward("msxml3.DllGetClassObject"))
    );
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::Undefined)
    );
    assert_eq!(
        export_kind("DllUnregisterServer"),
        Some(ExportKind::Undefined)
    );
}

#[test]
fn export_kind_misses_other_names() {
    // Not oracle-derived: lookups are exact and case-sensitive, so a wrong case, a prefix, a
    // longer name and the empty name are all misses.
    assert_eq!(export_kind("dllcanunloadnow"), None);
    assert_eq!(export_kind("DllGetClass"), None);
    assert_eq!(export_kind("DllGetClassObjectX"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn forward_target_is_set_only_for_forwarders() {
    // Not oracle-derived: the target text is the spec's `msxml3.DllGetClassObject`.
    assert_eq!(
        export_kind("DllGetClassObject").and_then(ExportKind::forward_target),
        Some("msxml3.DllGetClassObject")
    );
    assert_eq!(ExportKind::Undefined.forward_target(), None);
    assert_eq!(
        ExportKind::Forward("dll.name").forward_target(),
        Some("dll.name")
    );
}

#[test]
fn braced_string_at_the_bounds() {
    // Not oracle-derived: the all-zero and all-ones GUIDs bound every field width.
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    assert_eq!(
        zero.to_braced_string(),
        "{00000000-0000-0000-0000-000000000000}"
    );
    let max = Guid {
        data1: u32::MAX,
        data2: u16::MAX,
        data3: u16::MAX,
        data4: [0xff; 8],
    };
    assert_eq!(
        max.to_braced_string(),
        "{ffffffff-ffff-ffff-ffff-ffffffffffff}"
    );
}

#[test]
fn braced_string_keeps_leading_zeros() {
    // Not oracle-derived: small values must keep their leading zeros in every group.
    let guid = Guid {
        data1: 0x0000_000a,
        data2: 0x000b,
        data3: 0x000c,
        data4: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
    };
    assert_eq!(
        guid.to_braced_string(),
        "{0000000a-000b-000c-0102-030405060708}"
    );
}

#[test]
fn typelib_guid_returns_each_table_entry() {
    // Not oracle-derived: every table entry resolves to its own GUID.
    for &(name, guid) in TYPELIB_GUIDS.iter() {
        assert_eq!(typelib_guid(name), Some(guid), "{name}");
    }
}

#[test]
fn typelib_guid_misses() {
    // Not oracle-derived: lookups are exact and case-sensitive, so a wrong case, a longer name
    // and the empty name are misses.
    assert_eq!(typelib_guid("msxml2"), None);
    assert_eq!(typelib_guid("ixmldomnode"), None);
    assert_eq!(typelib_guid("IXMLDOMNode2"), None);
    assert_eq!(typelib_guid(""), None);
}

#[test]
fn typelib_table_names_and_guids_are_unique() {
    // Not oracle-derived: the table holds 37 entries, with no repeated name or GUID.
    assert_eq!(TYPELIB_GUIDS.len(), 37);
    let mut names: Vec<&str> = TYPELIB_GUIDS.iter().map(|&(name, _)| name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), 37);
    let mut braced: Vec<String> = TYPELIB_GUIDS
        .iter()
        .map(|(_, guid)| guid.to_braced_string())
        .collect();
    braced.sort();
    braced.dedup();
    assert_eq!(braced.len(), 37);
}

#[test]
fn member_constants_match_the_table() {
    // Not oracle-derived: the first and last constants and the table length, from the IDL.
    assert_eq!(NODE_INVALID, 0);
    assert_eq!(NODE_NOTATION, 12);
    assert_eq!(DOM_NODE_TYPE.len(), 13);
    assert_eq!(
        member_value(DOM_NODE_TYPE, "NODE_NOTATION"),
        Some(NODE_NOTATION)
    );
}

#[test]
fn member_value_misses() {
    // Not oracle-derived: a name past the last member, a wrong-case name, the empty name and an
    // empty table are all misses.
    assert_eq!(member_value(DOM_NODE_TYPE, "NODE_DOCUMENT_FRAGMENT_"), None);
    assert_eq!(member_value(DOM_NODE_TYPE, "node_element"), None);
    assert_eq!(member_value(DOM_NODE_TYPE, ""), None);
    assert_eq!(member_value(&[], "NODE_INVALID"), None);
}

#[test]
fn member_name_returns_the_first_match() {
    // Not oracle-derived: a synthetic table with a repeated value, so the first entry wins.
    const TABLE: &[Member] = &[
        Member {
            name: "FIRST",
            value: 1,
        },
        Member {
            name: "SECOND",
            value: 1,
        },
        Member {
            name: "ZERO",
            value: 0,
        },
    ];
    assert_eq!(member_name(TABLE, 1), Some("FIRST"));
    assert_eq!(member_name(TABLE, 0), Some("ZERO"));
}

#[test]
fn member_name_misses() {
    // Not oracle-derived: values outside the enum, and an empty table, are misses.
    assert_eq!(member_name(DOM_NODE_TYPE, -1), None);
    assert_eq!(member_name(DOM_NODE_TYPE, 13), None);
    assert_eq!(member_name(&[], 0), None);
}

#[test]
fn member_name_reverses_member_value() {
    // Not oracle-derived: the enum values are distinct, so each value names exactly one member
    // and the reverse lookup gets back to the same name.
    for member in DOM_NODE_TYPE {
        assert_eq!(member_name(DOM_NODE_TYPE, member.value), Some(member.name));
    }
}
