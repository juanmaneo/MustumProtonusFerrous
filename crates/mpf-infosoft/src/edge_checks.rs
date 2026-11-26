//! Edge-case tests for the ported `wb_BreakText` splitter.
//!
//! These cover cursor boundaries, the buffer-refill loop, the injected separator test, and the
//! C-locale classifier. Expectations that are not oracle-derived say so in a comment.

use super::*;

fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn owned(words: &[(usize, &str)]) -> Vec<(usize, String)> {
    words
        .iter()
        .map(|&(pos, text)| (pos, text.to_owned()))
        .collect()
}

/// Runs `break_text` over `ts` and returns the words as `(src_pos, text)`.
fn split_with(
    ts: &mut TextSource,
    is_separator: impl Fn(u16) -> bool,
    fill_text_buffer: impl FnMut(&mut TextSource) -> bool,
) -> Vec<(usize, String)> {
    let mut words = Vec::new();
    break_text(
        ts,
        is_separator,
        |word, pos| words.push((pos, String::from_utf16(word).unwrap())),
        fill_text_buffer,
    );
    words
}

/// Splits one buffer with the default C-locale separator test.
fn split(text: &str) -> Vec<(usize, String)> {
    let buffer = utf16(text);
    let end = buffer.len();
    split_with(
        &mut TextSource {
            buffer,
            cur: 0,
            end,
        },
        is_c_locale_separator,
        |_| false,
    )
}

/// Fill callback that serves `chunks` in order, as the harness does: each chunk replaces the
/// buffer and resets `cur` to 0.
fn serve(chunks: &[&str]) -> impl FnMut(&mut TextSource) -> bool {
    let mut pending = chunks
        .iter()
        .map(|c| utf16(c))
        .collect::<Vec<_>>()
        .into_iter();
    move |src| match pending.next() {
        Some(buffer) => {
            src.end = buffer.len();
            src.buffer = buffer;
            src.cur = 0;
            true
        }
        None => false,
    }
}

#[test]
fn cursor_at_or_past_end_emits_nothing() {
    // Not oracle-derived: the scan loop never runs when cur >= end, and call_sink(0) is a no-op.
    for (cur, end) in [(2, 2), (3, 1)] {
        let mut ts = TextSource {
            buffer: utf16("ab"),
            cur,
            end,
        };
        assert!(split_with(&mut ts, is_c_locale_separator, |_| false).is_empty());
        assert_eq!(ts.cur, cur);
    }
}

#[test]
fn cursor_ends_at_end_of_buffer() {
    // Not oracle-derived: after the last pass, cur equals end whether the buffer ends in a word
    // or in separators.
    for text in ["hello", " x ", "a b,", ",,,"] {
        let buffer = utf16(text);
        let end = buffer.len();
        let mut ts = TextSource {
            buffer,
            cur: 0,
            end,
        };
        split_with(&mut ts, is_c_locale_separator, |_| false);
        assert_eq!(ts.cur, end, "{text:?}");
    }
}

#[test]
fn fill_returning_false_stops_the_loop() {
    // Not oracle-derived: Wine loops only while the fill returns S_OK, so any other result
    // stops the loop after the current pass.
    let mut ts = TextSource {
        buffer: utf16("ab"),
        cur: 0,
        end: 2,
    };
    let mut calls = 0;
    let words = split_with(&mut ts, is_c_locale_separator, |_| {
        calls += 1;
        false
    });
    assert_eq!(words, owned(&[(0, "ab")]));
    assert_eq!(calls, 1);
}

#[test]
fn word_split_across_many_buffers_emits_one_fragment_per_buffer() {
    // Not oracle-derived: follows from the word state carried over each buffer boundary.
    let mut ts = TextSource {
        buffer: utf16("a"),
        cur: 0,
        end: 1,
    };
    let words = split_with(&mut ts, is_c_locale_separator, serve(&["b", "c"]));
    assert_eq!(words, owned(&[(0, "a"), (0, "b"), (0, "c")]));
}

#[test]
fn separator_predicate_alone_decides_word_boundaries() {
    // Not oracle-derived: with a comma-only predicate, spaces and NUL are word characters.
    let mut ts = TextSource {
        buffer: utf16("a b,c\x00d"),
        cur: 0,
        end: 7,
    };
    let words = split_with(&mut ts, |ch| ch == u16::from(b','), |_| false);
    assert_eq!(words, owned(&[(0, "a b"), (4, "c\x00d")]));
}

#[test]
fn non_ascii_code_units_are_word_characters_by_default() {
    // Not oracle-derived: the C-locale classifier knows only ASCII, so Latin-1 letters, CJK
    // letters and U+00A0 do not split words.
    assert_eq!(
        split("caf\u{e9} \u{3042}\u{3044}"),
        owned(&[(0, "caf\u{e9}"), (5, "\u{3042}\u{3044}")])
    );
    assert_eq!(split("a\u{a0}b"), owned(&[(0, "a\u{a0}b")]));
}

#[test]
fn c_locale_separator_boundaries() {
    // Also in SEPARATORS_0_127 from the oracle run; listed here as named boundaries.
    let separators = [
        0x00, 0x09, 0x0d, 0x20, 0x21, 0x2f, 0x3a, 0x40, 0x5b, 0x60, 0x7b, 0x7e,
    ];
    let word_chars = [
        0x01, 0x08, 0x0e, 0x1c, 0x1f, 0x30, 0x39, 0x41, 0x5a, 0x61, 0x7a, 0x7f,
    ];
    for ch in separators {
        assert!(is_c_locale_separator(ch), "{ch:#x}");
    }
    for ch in word_chars {
        assert!(!is_c_locale_separator(ch), "{ch:#x}");
    }
}

#[test]
fn code_units_above_0x7f_are_not_separators() {
    // Not oracle-derived: the classifier is ASCII only. 0x80..=0xFF fail the ASCII checks, and
    // wider code units take the out-of-range branch.
    for ch in [0x80, 0xa0, 0xe9, 0xff, 0x100, 0x3000, 0xffff] {
        assert!(!is_c_locale_separator(ch), "{ch:#x}");
    }
}
