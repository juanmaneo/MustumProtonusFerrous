//! Oracle-derived vectors for the ported `wb_BreakText` word splitter.
//!
//! Captured by compiling the verbatim `wb_BreakText` and `call_sink` bodies from
//! `dlls/infosoft/wordbreaker.c` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/infosoft/` and running the harness. It printed 24 rows and no ASan or UBSan
//! reports, so no row is excluded. The `SEPARATORS_0_127` line from the same run is checked
//! by `oracle_c_locale_separators_0_to_127`.

use super::*;

/// Runs `break_text` the way the harness does: the first chunk is preloaded, and each later
/// chunk arrives through the fill callback, which replaces the buffer and resets `cur` to 0.
fn run(chunks: &[&str]) -> Vec<(usize, String)> {
    let wide: Vec<Vec<u16>> = chunks.iter().map(|c| c.encode_utf16().collect()).collect();
    let mut ts = TextSource {
        buffer: wide[0].clone(),
        cur: 0,
        end: wide[0].len(),
    };
    let mut next = 1;
    let mut words = Vec::new();
    break_text(
        &mut ts,
        is_c_locale_separator,
        |word, pos| words.push((pos, String::from_utf16(word).unwrap())),
        |src| {
            if next == wide.len() {
                return false;
            }
            let chunk = &wide[next];
            next += 1;
            src.buffer = chunk.clone();
            src.cur = 0;
            src.end = chunk.len();
            true
        },
    );
    words
}

/// One oracle row: the input chunks, and the expected words as `(src_pos, text)`.
type Row = (&'static [&'static str], &'static [(usize, &'static str)]);

/// Rows copied from the harness output.
const ROWS: &[Row] = &[
    // 01
    (&[""], &[]),
    // 02
    (&["   \x09\x0a"], &[]),
    // 03
    (&["hello"], &[(0, "hello")]),
    // 04
    (&["hello world"], &[(0, "hello"), (6, "world")]),
    // 05
    (&["  foo,bar!baz  "], &[(2, "foo"), (6, "bar"), (10, "baz")]),
    // 06
    (
        &["a\x09b\x0ac\x0bd\x0ce\x0df"],
        &[(0, "a"), (2, "b"), (4, "c"), (6, "d"), (8, "e"), (10, "f")],
    ),
    // 07
    (&["x\x00y"], &[(0, "x"), (2, "y")]),
    // 08
    (&["\x00\x00ab\x00"], &[(2, "ab")]),
    // 09
    (&["one two three"], &[(0, "one"), (4, "two"), (8, "three")]),
    // 10
    (&["!!!~`@[]{}"], &[]),
    // 11
    (&["ab\x7fcd ef"], &[(0, "ab\x7fcd"), (6, "ef")]),
    // 12
    (&["tab\x1cx\x1fy"], &[(0, "tab\x1cx\x1fy")]),
    // 13
    (
        &["a1-2_3 b4.5"],
        &[(0, "a1"), (3, "2"), (5, "3"), (7, "b4"), (10, "5")],
    ),
    // 14
    (&[",,,x"], &[(3, "x")]),
    // 15
    (&["x "], &[(0, "x")]),
    // 16
    (
        &["hel", "lo wor", "ld"],
        &[(0, "hel"), (0, "lo"), (3, "wor"), (0, "ld")],
    ),
    // 17
    (&["abc", "def"], &[(0, "abc"), (0, "def")]),
    // 18
    (&["ab ", "", "cd"], &[(0, "ab"), (0, "cd")]),
    // 19
    (&["a", " ", "b"], &[(0, "a"), (0, "b")]),
    // 20
    (
        &["a b", "c d e"],
        &[(0, "a"), (2, "b"), (0, "c"), (2, "d"), (4, "e")],
    ),
    // 21
    (&["xy", ""], &[(0, "xy")]),
    // 22
    (
        &["abcdefghijabcdefghijabcdefghijabcdefghij"],
        &[(0, "abcdefghijabcdefghijabcdefghijabcdefghij")],
    ),
    // 23
    (&["a      b"], &[(0, "a"), (7, "b")]),
    // 24
    (&["\x00", "\x00\x00"], &[]),
];

#[test]
fn oracle_break_text_rows() {
    for (chunks, words) in ROWS {
        let want: Vec<(usize, String)> = words
            .iter()
            .map(|&(pos, text)| (pos, text.to_owned()))
            .collect();
        assert_eq!(run(chunks), want, "chunks {chunks:?}");
    }
}

#[test]
fn oracle_c_locale_separators_0_to_127() {
    // SEPARATORS_0_127: NUL, iswpunct and iswspace in the C locale.
    const SEPARATORS: [u16; 39] = [
        0, 9, 10, 11, 12, 13, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58,
        59, 60, 61, 62, 63, 64, 91, 92, 93, 94, 95, 96, 123, 124, 125, 126,
    ];
    let got: Vec<u16> = (0..128).filter(|&c| is_c_locale_separator(c)).collect();
    assert_eq!(got, SEPARATORS);
}
