use super::{convert_child_id, get_oleacc_version_info, VT_I4};

#[test]
fn preserves_all_i32_values_for_the_supported_variant() {
    for value in [i32::MIN, -1, 0, 1, i32::MAX] {
        assert_eq!(convert_child_id(VT_I4, value), value);
    }
}

#[test]
fn rejects_neighboring_and_unknown_variant_tags() {
    for vt in [VT_I4 - 1, VT_I4 + 1, u16::MAX] {
        assert_eq!(convert_child_id(vt, 17), -1);
    }
}

#[test]
fn version_info_is_constant() {
    assert_eq!(get_oleacc_version_info(), (458_752, 0));
}
