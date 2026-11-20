//! Edge-case tests for the `mpf-dxdiagn` port: boundary values and path
//! shapes that the oracle rows do not cover.

use super::*;

fn leaf(name: &str) -> Container {
    Container {
        name: name.to_string(),
        children: Vec::new(),
    }
}

fn root_with(children: Vec<Container>) -> Container {
    Container {
        name: "root".to_string(),
        children,
    }
}

#[test]
fn depth_unknown_formats_default_to_32() {
    assert_eq!(depth_for_pixelformat(0), 32);
    assert_eq!(depth_for_pixelformat(u32::MAX), 32);
    assert_eq!(depth_for_pixelformat(D3DFMT_P8 + 1), 32);
}

#[test]
fn vendor_unknown_ids_return_unknown() {
    assert_eq!(vendor_manufacturer(0), "Unknown");
    assert_eq!(vendor_manufacturer(0x1003), "Unknown");
    assert_eq!(vendor_manufacturer(u32::MAX), "Unknown");
}

#[test]
fn guid_uses_lowercase_zero_padded_fields() {
    assert_eq!(
        format_guid(0xABCD, 0x1, 0x2, [0xFF, 0, 0, 0, 0, 0, 0, 0x0A]),
        "0000abcd-0001-0002-ff00-00000000000a"
    );
    assert_eq!(
        format_guid(u32::MAX, u16::MAX, u16::MAX, [u8::MAX; 8]),
        "ffffffff-ffff-ffff-ffff-ffffffffffff"
    );
}

#[test]
fn path_empty_on_leaf_returns_self() {
    let cont = leaf("solo");
    assert_eq!(resolve_child_path(&cont, ""), Ok(&cont));
}

#[test]
fn path_empty_on_container_with_children_is_not_found() {
    let cont = root_with(vec![leaf("a")]);
    assert_eq!(resolve_child_path(&cont, ""), Err(ContainerError::NotFound));
}

#[test]
fn path_trailing_period_on_leaf_resolves() {
    let cont = root_with(vec![leaf("a")]);
    assert_eq!(resolve_child_path(&cont, "a."), Ok(&cont.children[0]));
}

#[test]
fn path_lone_period_looks_up_empty_name() {
    let cont = root_with(vec![leaf("a")]);
    assert_eq!(
        resolve_child_path(&cont, "."),
        Err(ContainerError::NotFound)
    );
}

#[test]
fn path_empty_child_name_is_matchable() {
    let cont = root_with(vec![leaf("")]);
    assert_eq!(resolve_child_path(&cont, ""), Ok(&cont.children[0]));
    assert_eq!(resolve_child_path(&cont, "."), Ok(&cont.children[0]));
}

#[test]
fn path_double_period_treats_middle_segment_as_empty() {
    let cont = root_with(vec![Container {
        name: "a".to_string(),
        children: vec![leaf("")],
    }]);
    assert_eq!(
        resolve_child_path(&cont, "a.."),
        Ok(&cont.children[0].children[0])
    );
}

#[test]
fn path_missing_intermediate_segment_is_not_found() {
    let cont = root_with(vec![leaf("a")]);
    assert_eq!(
        resolve_child_path(&cont, "nope.a"),
        Err(ContainerError::NotFound)
    );
}

#[test]
fn path_missing_segment_before_trailing_period_is_not_found() {
    let cont = root_with(vec![leaf("a")]);
    assert_eq!(
        resolve_child_path(&cont, "nope."),
        Err(ContainerError::NotFound)
    );
}

#[test]
fn path_matching_is_case_sensitive() {
    let cont = root_with(vec![leaf("Leaf")]);
    assert_eq!(
        resolve_child_path(&cont, "leaf"),
        Err(ContainerError::NotFound)
    );
    assert_eq!(resolve_child_path(&cont, "Leaf"), Ok(&cont.children[0]));
}

#[test]
fn path_nested_resolution_reaches_deep_child() {
    let cont = root_with(vec![Container {
        name: "mid".to_string(),
        children: vec![Container {
            name: "inner".to_string(),
            children: vec![leaf("deep")],
        }],
    }]);
    assert_eq!(
        resolve_child_path(&cont, "mid.inner.deep"),
        Ok(&cont.children[0].children[0].children[0])
    );
}
