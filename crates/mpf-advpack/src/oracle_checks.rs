//! Tests against Wine's oracle vectors.
//!
//! The rows are output of Wine's verbatim helpers from `dlls/advpack/advpack.c`, `install.c` and
//! `files.c`, compiled with `gcc -fsanitize=address,undefined` by `target/canon/advpack/oracle.c`
//! through a Win32 shim. No sanitizer report was produced for the oracle or edge rows.

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
const STRIP_VECTORS: &[(&str, &str, usize, usize, &str)] = &[
    ("quoted_word", "'ab'\0", 5, 3, "ab\0\0\0"),
    ("unquoted_word", "ab\0", 3, 3, "ab\0"),
    ("size_one_no_strip", "'\0", 1, 1, "'\0"),
    ("empty_quoted_pair", "''\0", 3, 1, "\0\0\0"),
    ("embedded_quotes", "'a'b'\0", 6, 4, "a'b\0\0\0"),
];

#[rustfmt::skip]
const GET_VECTORS: &[GetRow] = &[
    ("first_of_list", "one,two\0", Some(0), false, Some("one"), "one\0two\0", Some(4)),
    ("quoted_comma", "\"a,b\",c\0", Some(0), true, Some("a,b"), "\"a,b\0\0c\0", Some(6)),
    ("quoted_unterminated", "\"abc,d\0", Some(0), true, Some("\"abc"), "\"abc\0d\0", Some(5)),
    ("empty_first_field", ",x\0", Some(0), false, None, "\0x\0", Some(1)),
    ("offset_start", "x,y,z\0", Some(2), false, Some("y"), "x,y\0z\0", Some(4)),
];

#[rustfmt::skip]
const CONVERT_VECTORS: &[ConvertRow] = &[
    ("two_files", "a:b", Some(("a\0b\0\0", 2))),
    ("padded_three_files", " a:b:c ", Some(("a\0b\0c\0\0", 3))),
    ("double_colon", "a::b", Some(("a\0\0b\0\0", 3))),
    ("only_separators", ":: ", None),
    ("single_char", "a", None),
    ("empty", "", None),
];

#[rustfmt::skip]
const LIST_VECTORS: &[(&str, &str, &str, bool)] = &[
    ("found_case_insensitive", "SETUP.inf", "setup.inf\0other.txt\0\0", true),
    ("found_last", "b.txt", "a.txt\0b.txt\0\0", true),
    ("prefix_not_match", "a.tx", "a.txt\0\0", false),
    ("empty_list", "a", "\0", false),
];

#[rustfmt::skip]
const PATH_VECTORS: &[(&str, Option<&str>, bool)] = &[
    ("drive_path", Some("C:\\dir"), true),
    ("unc_path", Some("\\\\server\\share"), true),
    ("drive_no_backslash", Some("C:dir"), false),
    ("null_pointer", None, false),
];

#[test]
fn strip_quotes_matches_wine() {
    for (name, before, size_in, size_out, after) in STRIP_VECTORS {
        let mut buffer = wide(before);
        let mut size = *size_in;
        strip_quotes(&mut buffer, &mut size);
        assert_eq!(size, *size_out, "case={name}: size");
        assert_eq!(buffer, wide(after), "case={name}: buffer");
    }
}

#[test]
fn get_parameter_matches_wine() {
    for (name, before, params_in, quoted, token, after, params_out) in GET_VECTORS {
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
fn convert_file_list_matches_wine() {
    for (name, input, expected) in CONVERT_VECTORS {
        let got = convert_file_list(input.as_bytes());
        let want = expected.map(|(list, files)| (list.as_bytes().to_vec(), files));
        assert_eq!(got, want, "case={name}");
    }
}

#[test]
fn file_in_list_matches_wine() {
    for (name, file, list, expected) in LIST_VECTORS {
        assert_eq!(
            file_in_list(file.as_bytes(), list.as_bytes()),
            *expected,
            "case={name}"
        );
    }
}

#[test]
fn is_full_path_matches_wine() {
    for (name, path, expected) in PATH_VECTORS {
        let units = path.map(wide);
        assert_eq!(is_full_path(units.as_deref()), *expected, "case={name}");
    }
}
