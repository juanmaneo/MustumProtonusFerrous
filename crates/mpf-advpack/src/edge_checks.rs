//! Edge cases that the oracle rows do not reach: sizes below and past the token, quotes that
//! are not paired, empty fields, and NUL-truncated strings. The table rows are Wine's output
//! for these inputs, from the `//E` rows of `target/canon/advpack/oracle.c`.

use crate::{convert_file_list, file_in_list, get_parameter, is_full_path, strip_quotes};

const SEPARATOR: u16 = b',' as u16;

type GetRow = (
    &'static str,
    &'static str,
    Option<usize>,
    bool,
    Option<&'static str>,
    &'static str,
    Option<usize>,
);
type ConvertRow = (&'static str, &'static str, Option<(&'static str, u32)>);

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[rustfmt::skip]
const STRIP_EDGE: &[(&str, &str, usize, usize, &str)] = &[
    ("size_two_quote", "'\0", 2, 0, "\0\0"),
    ("size_zero_no_strip", "'\0", 0, 0, "'\0"),
    ("leading_quote_only", "'ab\0", 4, 4, "'ab\0"),
    ("trailing_quote_only", "ab'\0", 4, 4, "ab'\0"),
    ("size_two_plain", "ab\0", 2, 2, "ab\0"),
];

#[rustfmt::skip]
const GET_EDGE: &[GetRow] = &[
    ("null_params", "x\0", None, false, None, "x\0", None),
    ("last_field", "x\0", Some(0), false, Some("x"), "x\0", None),
    ("quoted_empty", "\"\",x\0", Some(0), true, None, "\"\0\0x\0", Some(3)),
    ("quoted_last_no_separator", "\"ab\"\0", Some(0), true, Some("ab"), "\"ab\0\0", None),
    ("quoted_flag_plain_first", "a,b\0", Some(0), true, Some("a"), "a\0b\0", Some(2)),
    ("quoted_flag_off", "\"a,b\"\0", Some(0), false, Some("\"a"), "\"a\0b\"\0", Some(3)),
];

#[rustfmt::skip]
const CONVERT_EDGE: &[ConvertRow] = &[
    ("tab_and_colon_trim", "\t:ab\t", Some(("ab\0\0", 1))),
    ("three_inner_colons", "a:::b", Some(("a\0\0\0b\0\0", 4))),
    ("two_chars_no_colon", "ab", Some(("ab\0\0", 1))),
    ("trailing_colon_pair", "a:", None),
];

#[rustfmt::skip]
const LIST_EDGE: &[(&str, &str, &str, bool)] = &[
    ("empty_file_name", "", "a\0\0", false),
    ("no_double_terminator", "b", "a\0b", true),
];

#[rustfmt::skip]
const PATH_EDGE: &[(&str, Option<&str>, bool)] = &[
    ("too_short_drive", Some("C:"), false),
    ("exact_three_drive", Some("C:\\"), true),
    ("lowercase_drive", Some("c:\\x"), true),
    ("single_backslash", Some("\\x"), false),
    ("colon_then_slash", Some("C:/x"), false),
    ("plain_relative", Some("abc"), false),
];

#[test]
fn strip_quotes_edge_rows_match_wine() {
    for (name, before, size_in, size_out, after) in STRIP_EDGE {
        let mut buffer = wide(before);
        let mut size = *size_in;
        strip_quotes(&mut buffer, &mut size);
        assert_eq!(size, *size_out, "case={name}: size");
        assert_eq!(buffer, wide(after), "case={name}: buffer");
    }
}

#[test]
fn strip_quotes_leaves_units_past_size_alone() {
    let mut buffer = wide("'ab'\0XYZ");
    let mut size = 5;
    strip_quotes(&mut buffer, &mut size);
    assert_eq!(size, 3);
    assert_eq!(buffer, wide("ab\0\0\0XYZ"));
}

#[test]
fn get_parameter_edge_rows_match_wine() {
    for (name, before, params_in, quoted, token, after, params_out) in GET_EDGE {
        let mut buffer = wide(before);
        let mut params = *params_in;
        let got = get_parameter(&mut buffer, &mut params, SEPARATOR, *quoted)
            .map(|unit| String::from_utf16(unit).unwrap());
        assert_eq!(got.as_deref(), *token, "case={name}: token");
        assert_eq!(buffer, wide(after), "case={name}: buffer");
        assert_eq!(params, *params_out, "case={name}: params");
    }
}

#[test]
fn list_walk_stops_at_empty_field() {
    // Wine's callers loop `while (ptr = get_parameter(...))`, so an empty field ends the walk.
    let mut buffer = wide("one,,three\0");
    let mut params = Some(0);
    let first = get_parameter(&mut buffer, &mut params, SEPARATOR, false)
        .map(|unit| String::from_utf16(unit).unwrap());
    assert_eq!(first.as_deref(), Some("one"));
    assert_eq!(
        get_parameter(&mut buffer, &mut params, SEPARATOR, false),
        None
    );
    assert_eq!(params, Some(5));
    let third = get_parameter(&mut buffer, &mut params, SEPARATOR, false)
        .map(|unit| String::from_utf16(unit).unwrap());
    assert_eq!(third.as_deref(), Some("three"));
    assert_eq!(params, None);
}

#[test]
fn convert_file_list_edge_rows_match_wine() {
    for (name, input, expected) in CONVERT_EDGE {
        let got = convert_file_list(input.as_bytes());
        let want = expected.map(|(list, files)| (list.as_bytes().to_vec(), files));
        assert_eq!(got, want, "case={name}");
    }
}

#[test]
fn file_in_list_edge_rows_match_wine() {
    for (name, file, list, expected) in LIST_EDGE {
        assert_eq!(
            file_in_list(file.as_bytes(), list.as_bytes()),
            *expected,
            "case={name}"
        );
    }
}

#[test]
fn is_full_path_edge_rows_match_wine() {
    for (name, path, expected) in PATH_EDGE {
        let units = path.map(wide);
        assert_eq!(is_full_path(units.as_deref()), *expected, "case={name}");
    }
}

#[test]
fn is_full_path_stops_at_nul_like_lstrlenw() {
    // The NUL at index 2 ends the string, so it is shorter than three units.
    let units = [u16::from(b'C'), u16::from(b':'), 0, u16::from(b'\\')];
    assert!(!is_full_path(Some(&units)));
}
