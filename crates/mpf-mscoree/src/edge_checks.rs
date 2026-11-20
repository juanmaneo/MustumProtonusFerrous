//! Inputs the oracle rows do not reach: missing NUL terminators, characters
//! after the terminator, non-ASCII digits, and trailing text after each
//! component position.

use crate::{parse_runtime_version, RuntimeVersion};

fn units(input: &str) -> Vec<u16> {
    input.encode_utf16().collect()
}

#[test]
fn rejects_slices_without_a_terminator() {
    assert_eq!(parse_runtime_version(&[]), None);
    assert_eq!(parse_runtime_version(&units("v1")), None);
    assert_eq!(parse_runtime_version(&units("v1.2.3")), None);
}

#[test]
fn stops_at_the_first_nul() {
    let mut input = units("v1");
    input.extend([0, u16::from(b'.'), u16::from(b'2')]);
    assert_eq!(
        parse_runtime_version(&input),
        Some(RuntimeVersion {
            major: 1,
            minor: 0,
            build: 0,
        })
    );
    assert_eq!(
        parse_runtime_version(&[0, u16::from(b'v'), u16::from(b'1'), 0]),
        None
    );
}

#[test]
fn rejects_non_ascii_digits_and_bad_separators() {
    let cases = [
        "v\u{0661}",
        "v1\u{0661}",
        "v1x",
        "v1.2x",
        "v1.2.3x",
        "v1.2.",
        "v1.2..3",
        "v1.2.3.",
        "v1 ",
        "v1.2 ",
    ];
    for input in cases {
        assert_eq!(parse_runtime_version(&wide(input)), None, "input {input:?}");
    }
}

#[test]
fn accepts_uppercase_prefix_with_full_build() {
    assert_eq!(
        parse_runtime_version(&wide("V4.0.30319")),
        Some(RuntimeVersion {
            major: 4,
            minor: 0,
            build: 30319,
        })
    );
}

fn wide(input: &str) -> Vec<u16> {
    input.encode_utf16().chain(std::iter::once(0)).collect()
}
