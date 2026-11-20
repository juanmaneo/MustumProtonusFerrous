//! Replays the Wine oracle (`target/canon/dxdiagn/oracle.c`) against this crate.
//! Each row is one line of `inputs.txt` with the verbatim C output in
//! `expected.txt`; the C harness ran clean under `-fsanitize=address,undefined`.

use super::*;

/// Tree 0: root -> { mid1 -> {leaf1, leaf2}, Mid2 -> {leaf3}, leaf0 }.
fn tree0() -> Container {
    let leaf = |name: &str| Container {
        name: name.to_string(),
        children: Vec::new(),
    };
    let mid = |name: &str, children: Vec<Container>| Container {
        name: name.to_string(),
        children,
    };
    Container {
        name: "root".to_string(),
        children: vec![
            mid("mid1", vec![leaf("leaf1"), leaf("leaf2")]),
            mid("Mid2", vec![leaf("leaf3")]),
            leaf("leaf0"),
        ],
    }
}

/// Tree 1: a leaf root named `solo`.
fn tree1() -> Container {
    Container {
        name: "solo".to_string(),
        children: Vec::new(),
    }
}

#[test]
fn oracle_depth_rows() {
    // inputs: depth 41 | 24 | 23 | 22 | 21
    let rows: [(u32, u32); 5] = [(41, 8), (24, 15), (23, 16), (22, 32), (21, 32)];
    for (format, expected) in rows {
        assert_eq!(depth_for_pixelformat(format), expected, "depth {format}");
    }
}

#[test]
fn oracle_vendor_rows() {
    // inputs: vendor 1002 | 10de | 15ad | 1af4 | 8086 | 1234
    let rows: [(u32, &str); 6] = [
        (0x1002, "ATI Technologies Inc."),
        (0x10de, "NVIDIA"),
        (0x15ad, "VMware"),
        (0x1af4, "Red Hat"),
        (0x8086, "Intel Corporation"),
        (0x1234, "Unknown"),
    ];
    for (id, expected) in rows {
        assert_eq!(vendor_manufacturer(id), expected, "vendor {id:#06x}");
    }
}

#[test]
fn oracle_guid_rows() {
    // inputs: guid zeros | 12345678 9abc def0 1 2 3 4 5 a b c
    assert_eq!(
        format_guid(0, 0, 0, [0; 8]),
        "00000000-0000-0000-0000-000000000000"
    );
    assert_eq!(
        format_guid(0x1234_5678, 0x9abc, 0xdef0, [1, 2, 3, 4, 5, 0xa, 0xb, 0xc]),
        "12345678-9abc-def0-0102-0304050a0b0c"
    );
}

#[test]
fn oracle_path_rows() {
    // (tree, path, expected name or None for E_INVALIDARG), copied from the oracle.
    let rows: [(usize, &str, Option<&str>); 12] = [
        (0, "mid1.leaf1", Some("leaf1")),
        (0, "mid1.leaf1.", Some("leaf1")),
        (0, "Mid2.leaf3", Some("leaf3")),
        (0, "mid2.leaf3", None),
        (0, "mid1.nope", None),
        (0, "", None),
        (1, "", Some("solo")),
        (0, "leaf0.", Some("leaf0")),
        (0, "root", None),
        (0, "mid1..leaf1", None),
        (0, ".", None),
        (0, "mid1", Some("mid1")),
    ];
    for (tree, path, expected) in rows {
        let root = if tree == 0 { tree0() } else { tree1() };
        let got = resolve_child_path(&root, path).map(|c| c.name.as_str());
        assert_eq!(got.ok(), expected, "tree {tree} path {path:?}");
    }
}
