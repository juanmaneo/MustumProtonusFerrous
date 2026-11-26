//! Edge-case tests for the ported opcservices logic.
//!
//! These cover boundaries and every branch of the three ports. Expectations
//! marked "not oracle-derived" were worked out by reading the C source. They
//! are not rows the oracle printed.

use super::*;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn rels_short_directory_is_inserted_in_place() {
    // Not oracle-derived: last slash at index 5, below the six-unit window.
    assert_eq!(
        rels_part_path(&wide("/abcd/x")),
        Some(wide("/abcd/_rels/x.rels"))
    );
}

#[test]
fn rels_six_unit_window_at_index_six() {
    // Not oracle-derived: same branch as the oracle row "/words/x".
    assert_eq!(
        rels_part_path(&wide("/abcde/x")),
        Some(wide("/_rels/x.rels"))
    );
}

#[test]
fn rels_detected_when_rels_dir_precedes_last_slash() {
    // Not oracle-derived: "/_rels" sits in the window before the last slash.
    assert_eq!(rels_part_path(&wide("/x/_rels/y.rels")), None);
}

#[test]
fn rels_ext_length_boundary() {
    // Not oracle-derived: length 6 is not greater than sizeof(".rels") (6 with NUL).
    assert_eq!(
        rels_part_path(&wide("/.rels")),
        Some(wide("/_rels/.rels.rels"))
    );
}

#[test]
fn rels_ext_matches_but_no_rels_dir() {
    // Not oracle-derived: ".rels" suffix matches, "/_rels" prefix does not.
    assert_eq!(
        rels_part_path(&wide("/a.rels")),
        Some(wide("/_rels/a.rels.rels"))
    );
}

#[test]
fn rels_rels_dir_without_rels_ext() {
    // Not oracle-derived: "/_rels" matches at the window, but there is no ".rels" suffix.
    assert_eq!(
        rels_part_path(&wide("/_rels/x")),
        Some(wide("/_rels/x.rels"))
    );
}

#[test]
fn rels_no_slash_is_none() {
    // Not oracle-derived: the C is undefined here (see the module note in rels_uri.rs).
    assert_eq!(rels_part_path(&wide("abc")), None);
    assert_eq!(rels_part_path(&wide("")), None);
}

#[test]
fn rels_non_ascii_units_pass_through() {
    // Not oracle-derived: copies UTF-16 units unchanged, including units above 0xff.
    assert_eq!(
        rels_part_path(&wide("/\u{20ac}/a.xml")),
        Some(wide("/\u{20ac}/_rels/a.xml.rels"))
    );
}

fn views(list: &[(Vec<u16>, Vec<u16>)]) -> Vec<DefaultType<'_>> {
    list.iter()
        .map(|(ext, content_type)| DefaultType {
            ext: ext.as_slice(),
            content_type: content_type.as_slice(),
        })
        .collect()
}

#[test]
fn content_type_no_extension_is_override_even_with_defaults() {
    // Not oracle-derived: the oracle only saw no-extension parts with no defaults present.
    let list = vec![(wide("png"), wide("image/png"))];
    assert_eq!(
        classify_content_type(&views(&list), None, &wide("image/png")),
        ContentTypeAction::AddOverride
    );
}

#[test]
fn content_type_first_matching_default_decides() {
    // Not oracle-derived: the C loop breaks at the first matching default.
    let list = vec![(wide("png"), wide("a")), (wide("png"), wide("b"))];
    let png = wide("png");
    assert_eq!(
        classify_content_type(&views(&list), Some(png.as_slice()), &wide("b")),
        ContentTypeAction::AddOverride
    );
    assert_eq!(
        classify_content_type(&views(&list), Some(png.as_slice()), &wide("a")),
        ContentTypeAction::Nothing
    );
}

#[test]
fn content_type_extension_length_mismatch_is_no_match() {
    // Not oracle-derived: a prefix of the extension is not a match, in either direction.
    let short = vec![(wide("pn"), wide("image/png"))];
    let png = wide("png");
    assert_eq!(
        classify_content_type(&views(&short), Some(png.as_slice()), &wide("image/png")),
        ContentTypeAction::AddDefault
    );
    let long = vec![(wide("png"), wide("image/png"))];
    let pn = wide("pn");
    assert_eq!(
        classify_content_type(&views(&long), Some(pn.as_slice()), &wide("image/png")),
        ContentTypeAction::AddDefault
    );
}

#[test]
fn content_type_non_ascii_extension_folds_only_identical_units() {
    // Not oracle-derived: wcsicmp folds non-ASCII with the locale. The port folds ASCII only.
    let euro = wide("\u{20ac}");
    let list = vec![(euro.clone(), wide("x"))];
    assert_eq!(
        classify_content_type(&views(&list), Some(euro.as_slice()), &wide("x")),
        ContentTypeAction::Nothing
    );
    let e_acute = wide("\u{e9}");
    let list = vec![(e_acute, wide("x"))];
    let e_upper = wide("\u{c9}");
    assert_eq!(
        classify_content_type(&views(&list), Some(e_upper.as_slice()), &wide("x")),
        ContentTypeAction::AddDefault
    );
}

#[test]
fn content_type_empty_extension_matches_empty() {
    // Not oracle-derived: an empty extension compares equal to an empty one.
    let list = vec![(wide(""), wide("x"))];
    let empty = wide("");
    assert_eq!(
        classify_content_type(&views(&list), Some(empty.as_slice()), &wide("x")),
        ContentTypeAction::Nothing
    );
}

#[test]
fn array_count_within_capacity_keeps_capacity() {
    // Not oracle-derived: the oracle row (4, 4, 8) covers equality only.
    assert_eq!(array_reserve_capacity(10, 3, 8), Some(10));
    assert_eq!(array_reserve_capacity(10, 10, 8), Some(10));
}

#[test]
fn array_growth_starts_from_capacity_above_minimum() {
    // Not oracle-derived: starts from the existing capacity, not from 4.
    assert_eq!(array_reserve_capacity(5, 6, 8), Some(10));
}

#[test]
fn array_growth_doubles_many_times() {
    // Not oracle-derived: four doublings from the minimum of 4.
    assert_eq!(array_reserve_capacity(1, 100, 8), Some(128));
}

#[test]
fn array_growth_clamps_to_maximum() {
    // Not oracle-derived: the doubling stops at max / 2, so the clamp to max applies.
    // The C would then fail realloc (5 * size is near SIZE_MAX), which is not modelled.
    let size = usize::MAX / 5;
    assert_eq!(array_reserve_capacity(0, 5, size), Some(5));
}

#[test]
fn array_count_at_maximum_is_accepted() {
    // Not oracle-derived: max is exactly 8 for this size, so 8 fits and 9 does not.
    let size = usize::MAX / 8;
    assert_eq!(array_reserve_capacity(0, 8, size), Some(8));
    assert_eq!(array_reserve_capacity(0, 9, size), None);
}

#[test]
fn array_zero_size_is_none_only_when_growing() {
    // Not oracle-derived: the C returns TRUE before dividing when count fits, and
    // divides by zero otherwise. The port returns None instead of trapping.
    assert_eq!(array_reserve_capacity(4, 4, 0), Some(4));
    assert_eq!(array_reserve_capacity(0, 1, 0), None);
}

#[test]
fn array_minimum_of_four_is_not_clamped_to_maximum() {
    // Not oracle-derived: max is 3 here, below the minimum of 4. The C's
    // `new_capacity * size` then wraps, and the wrapped allocation is not modelled.
    // The port reports the requested capacity, 4.
    let size = usize::MAX / 3;
    assert_eq!(array_reserve_capacity(0, 2, size), Some(4));
}
