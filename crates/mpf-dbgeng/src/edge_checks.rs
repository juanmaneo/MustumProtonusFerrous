//! Edge-case tests for the ported dbgeng logic.
//!
//! Boundaries of the buffer and size contract plus the module search. Expectations
//! not taken from the oracle are marked "not oracle-derived" in a comment.

use super::*;

#[test]
fn size_only_reports_len_plus_one() {
    // Not oracle-derived: NULL buffer with a size pointer, empty and non-empty input.
    let mut size = 0;
    assert_eq!(return_string(b"", None, 0, Some(&mut size)), S_FALSE);
    assert_eq!(size, 1);
    assert_eq!(return_string(b"ab", None, 3, Some(&mut size)), S_OK);
    assert_eq!(size, 3);
}

#[test]
fn null_buffer_keeps_size_contract() {
    // Not oracle-derived: NULL buffer still drives the return value from buffer_size.
    assert_eq!(return_string(b"abc", None, 4, None), S_OK);
    assert_eq!(return_string(b"abc", None, 3, None), S_FALSE);
}

#[test]
fn zero_buffer_size_leaves_buffer_untouched() {
    // Not oracle-derived: buffer_size 0 with a real buffer writes nothing, even the NUL.
    let mut buf = [b'#'; 4];
    assert_eq!(return_string(b"", Some(&mut buf), 0, None), S_FALSE);
    assert_eq!(buf, [b'#'; 4]);
}

#[test]
fn exact_fit_and_one_short_boundary() {
    // Not oracle-derived: len == buffer_size - 1 fits exactly, len == buffer_size truncates.
    let mut buf = [b'#'; 4];
    assert_eq!(return_string(b"abc", Some(&mut buf), 4, None), S_OK);
    assert_eq!(&buf, b"abc\0");
    assert_eq!(return_string(b"abcd", Some(&mut buf), 4, None), S_FALSE);
    assert_eq!(&buf, b"abc\0");
}

#[test]
fn truncation_never_writes_past_buffer_size() {
    // Not oracle-derived: a larger backing slice keeps bytes after buffer_size intact.
    let mut buf = [b'#'; 8];
    assert_eq!(return_string(b"abcdef", Some(&mut buf), 3, None), S_FALSE);
    assert_eq!(&buf, b"ab\0#####");
}

#[test]
fn module_search_edges() {
    // Not oracle-derived: empty table, duplicate bases resolve to the first index,
    // and the last index is reachable.
    assert_eq!(module_index_by_base(&[], 0), None);
    assert_eq!(module_index_by_base(&[7, 7, 9], 7), Some(0));
    assert_eq!(module_index_by_base(&[7, 9], 9), Some(1));
}
