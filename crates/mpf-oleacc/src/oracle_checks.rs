use super::{convert_child_id, get_oleacc_version_info, VT_I4};

#[test]
fn child_id_matches_wine_oracle_rows() {
    let rows = [
        (3, 0, 0),
        (3, 1, 1),
        (3, -1, -1),
        (3, i32::MIN, i32::MIN),
        (3, i32::MAX, i32::MAX),
        (0, 0, -1),
        (1, 123, -1),
        (2, -42, -1),
        (4, 42, -1),
        (5, i32::MIN, -1),
        (6, i32::MAX, -1),
        (8, 7, -1),
        (11, -9, -1),
        (12, 0, -1),
        (16, 37, -1),
        (17, 99, -1),
        (0xffff, 42, -1),
        (0x1003, 5, -1),
        (0x8003, -32768, -1),
        (0x7fff, i32::MAX, -1),
    ];

    for (vt, value, expected) in rows {
        assert_eq!(
            convert_child_id(vt, value),
            expected,
            "vt={vt}, value={value}"
        );
    }
}

#[test]
fn version_info_matches_wine_oracle() {
    assert_eq!(get_oleacc_version_info(), (0x0007_0000, 0));
}

#[test]
fn accepted_variant_tag_is_the_oracle_tag() {
    assert_eq!(VT_I4, 3);
}
