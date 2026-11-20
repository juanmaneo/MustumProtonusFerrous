use super::*;

#[test]
fn empty_prefixes_follow_c_string_comparisons() {
    assert_eq!(parse_verb(b""), 0);
    assert_eq!(parse_verb(b"OPTIONS\0tail"), 0);
    assert_eq!(parse_header_name(b""), 0);
    assert_eq!(parse_header_name(b"Host\0tail"), 28);
    assert_eq!(
        parse_header_name(b"not-a-header"),
        HTTP_HEADER_REQUEST_MAXIMUM
    );
}

#[test]
fn token_parser_stops_at_every_non_token_category() {
    assert_eq!(parse_token(b""), 0);
    assert_eq!(parse_token(b"a b"), 1);
    assert_eq!(parse_token(b"a/b"), 1);
    assert_eq!(parse_token(b"a\x7fmore"), 1);
    assert_eq!(parse_token(b"a\xffmore"), 1);
    assert_eq!(parse_token(b"!#$%&'*+-.^_`|~"), 15);
}

#[test]
fn header_parser_handles_empty_and_truncated_lines() {
    assert_eq!(
        parse_header(b""),
        ParsedHeader {
            name: b"",
            value: b""
        }
    );
    assert_eq!(
        parse_header(b"X"),
        ParsedHeader {
            name: b"X",
            value: b""
        }
    );
    assert_eq!(
        parse_header(b"X: value\r\n"),
        ParsedHeader {
            name: b"X",
            value: b"value"
        }
    );
    assert_eq!(
        parse_header(b"X \t: value"),
        ParsedHeader {
            name: b"X",
            value: b"value"
        }
    );
}

#[test]
fn exact_comparison_handles_empty_inputs() {
    assert_eq!(compare_exact(b"", b""), 1);
    assert_eq!(compare_exact(b"", b"x"), 0);
    assert_eq!(compare_exact(b"x", b""), 1);
}

#[test]
fn number_parser_handles_empty_and_stops_before_non_digits() {
    assert_eq!(parse_number(b""), (0, 0));
    assert_eq!(parse_number(b"00042 rest"), (42, 5));
    assert_eq!(parse_number(b"+12"), (0, 0));
}

#[test]
fn path_matching_handles_empty_root_and_boundary_cases() {
    assert_eq!(compare_paths(b"", b"/path"), 0);
    assert_eq!(compare_paths(b"/path", b""), 0);
    assert_eq!(compare_paths(b"/path", b"?query"), 0);
    assert_eq!(compare_paths(b"/", b"/"), 1);
    assert_eq!(compare_paths(b"/a", b"/a/"), 2);
    assert_eq!(compare_paths(b"/long/path", b"/long"), 0);
    assert_eq!(compare_paths(b"/ab", b"/ac"), 0);
}
