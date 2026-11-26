//! Boundary and branch tests for `copy_multisz`.
//!
//! Expectations marked "not oracle" are not taken from the harness. They follow by hand from
//! the C source, or from the choices this module documents.

use super::*;
use crate::oracle_checks::{
    run, utf16_to_ansi_model, utf8_to_ansi_model, utf8_to_utf16_model, Mode,
};
use std::cell::Cell;
use std::fmt::Debug;

/// Calls `ok` on the first conversion, then fails every later call. This reaches the copy
/// pass's failure after the size pass has succeeded.
fn succeed_once<'a, T>(
    calls: &'a Cell<usize>,
    ok: &'a dyn Fn(&[u8]) -> Option<Vec<T>>,
) -> impl Fn(&[u8]) -> Option<Vec<T>> + 'a {
    move |src: &[u8]| {
        let n = calls.get();
        calls.set(n + 1);
        if n == 0 {
            ok(src)
        } else {
            None
        }
    }
}

/// The failure paths, for a converter `ok` that succeeds on well-formed input.
fn check_failures<T>(ok: &dyn Fn(&[u8]) -> Option<Vec<T>>)
where
    T: Copy + Default + PartialEq + Debug,
{
    // not oracle: a failing conversion in the size pass.
    let never = |_: &[u8]| None::<Vec<T>>;
    assert_eq!(
        copy_multisz(b"ab\0\0", &never, Some(Dest::Length)),
        Err(Error::InvalidParameter)
    );
    // not oracle: an empty result would never advance the position, so it counts as a failure.
    let empty = |_: &[u8]| Some(Vec::<T>::new());
    assert_eq!(
        copy_multisz(b"a\0\0", &empty, Some(Dest::Length)),
        Err(Error::InvalidParameter)
    );
    // not oracle: the copy pass fails after the size pass has converted the one string.
    let calls = Cell::new(0);
    let once = succeed_once(&calls, ok);
    assert_eq!(
        copy_multisz(b"ab\0\0", &once, Some(Dest::Allocate)),
        Err(Error::NoMemory)
    );
    let calls = Cell::new(0);
    let once = succeed_once(&calls, ok);
    let mut buf = [T::default(); 8];
    assert_eq!(
        copy_multisz(b"ab\0\0", &once, Some(Dest::Buffer(&mut buf[..]))),
        Err(Error::NoMemory)
    );
    // not oracle: the required length is 4 units, one more than a 3-unit buffer holds.
    let mut short = [T::default(); 3];
    assert_eq!(
        copy_multisz(b"ab\0\0", ok, Some(Dest::Buffer(&mut short[..]))),
        Err(Error::InsufficientBuffer { required: 4 })
    );
    // not oracle: `run` reports len 0 for failures other than SCARD_E_INSUFFICIENT_BUFFER.
    assert_eq!(
        run(b"ab", ok, &Mode::Query),
        (Error::InvalidParameter.status(), 0, Vec::new())
    );
}

#[test]
fn failures_for_u8() {
    check_failures::<u8>(&utf8_to_ansi_model);
}

#[test]
fn failures_for_u16() {
    check_failures::<u16>(&utf8_to_utf16_model);
}

#[test]
fn no_destination_returns_nothing_without_scanning() {
    // not oracle: the C early return `if (!dst && !dst_len)` runs before the scan.
    let never = |_: &[u8]| None::<Vec<u8>>;
    assert_eq!(copy_multisz(b"ab", &never, None), Ok(Written::Nothing));
}

#[test]
fn length_counts_each_string_and_the_final_nul() {
    // not oracle: "a\0" (2 units) + "b\0" (2 units) + the final NUL.
    assert_eq!(
        copy_multisz(b"a\0b\0\0", &utf8_to_ansi_model, Some(Dest::Length)),
        Ok(Written::Length(5))
    );
}

#[test]
fn list_without_final_empty_string_ends_at_buffer_end() {
    // not oracle: C reads past the buffer here. This port ends the list at the end of `src`.
    assert_eq!(
        copy_multisz(b"ab\0", &utf8_to_ansi_model, Some(Dest::Allocate)),
        Ok(Written::Allocated(vec![0x61, 0x62, 0x00, 0x00]))
    );
}

#[test]
fn string_without_nul_is_invalid_parameter() {
    // not oracle: a string that never reaches its NUL fails to convert in the size pass.
    let mut buf = [0u8; 8];
    assert_eq!(
        copy_multisz(b"ab", &utf8_to_ansi_model, Some(Dest::Length)),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        copy_multisz(b"ab", &utf8_to_ansi_model, Some(Dest::Allocate)),
        Err(Error::InvalidParameter)
    );
    assert_eq!(
        copy_multisz(b"ab", &utf8_to_ansi_model, Some(Dest::Buffer(&mut buf[..]))),
        Err(Error::InvalidParameter)
    );
}

#[test]
fn buffer_exact_fit_and_one_unit_short() {
    // not oracle: the boundary of the C check `*dst_len < total_len`.
    let mut exact = [0u8; 4];
    assert_eq!(
        copy_multisz(
            b"ab\0\0",
            &utf8_to_ansi_model,
            Some(Dest::Buffer(&mut exact[..]))
        ),
        Ok(Written::Units(4))
    );
    assert_eq!(exact, [0x61, 0x62, 0x00, 0x00]);

    let mut short = [0u8; 3];
    assert_eq!(
        copy_multisz(
            b"ab\0\0",
            &utf8_to_ansi_model,
            Some(Dest::Buffer(&mut short[..]))
        ),
        Err(Error::InsufficientBuffer { required: 4 })
    );
    assert_eq!(short, [0, 0, 0]);
}

#[test]
fn buffer_larger_than_list_keeps_its_tail() {
    // not oracle: only the leading `Units` entries are written.
    let mut buf = [0xAA_u8; 8];
    assert_eq!(
        copy_multisz(
            b"ab\0\0",
            &utf8_to_ansi_model,
            Some(Dest::Buffer(&mut buf[..]))
        ),
        Ok(Written::Units(4))
    );
    assert_eq!(buf, [0x61, 0x62, 0x00, 0x00, 0xAA, 0xAA, 0xAA, 0xAA]);
}

#[test]
fn position_advances_by_converted_units_not_utf8_bytes() {
    // not oracle: Wine's behaviour, kept on purpose. "é" is two UTF-8 bytes but converts to
    // one unit and a NUL, so the position lands on the NUL after it and "ab" is dropped.
    let src = b"\xc3\xa9\x00ab\x00\x00";
    assert_eq!(
        copy_multisz(src, &utf8_to_ansi_model, Some(Dest::Allocate)),
        Ok(Written::Allocated(vec![0xe9, 0x00, 0x00]))
    );
    assert_eq!(
        copy_multisz(src, &utf8_to_utf16_model, Some(Dest::Allocate)),
        Ok(Written::Allocated(vec![0x00e9, 0x0000, 0x0000]))
    );
}

#[test]
fn utf8_model_decodes_every_length_and_splits_surrogates() {
    // not oracle: hand-computed from the shim in oracle.c. U+00E9, U+20AC, U+1F600, then NUL.
    assert_eq!(
        utf8_to_utf16_model(b"\xc3\xa9\xe2\x82\xac\xf0\x9f\x98\x80\x00"),
        Some(vec![0x00e9, 0x20ac, 0xd83d, 0xde00, 0x0000])
    );
}

#[test]
fn utf8_model_maps_malformed_input_to_replacement() {
    // not oracle: hand-computed from the lenient shim, one byte consumed per U+FFFD.
    let cases: [(&[u8], &[u16]); 7] = [
        (b"\x80\x00", &[0xfffd, 0]),
        (b"\xc0\x00", &[0xfffd, 0]),
        (b"\xf5\x00", &[0xfffd, 0]),
        (b"\xc3A\x00", &[0xfffd, 0x41, 0]),
        (b"\xe0\x80\x80\x00", &[0xfffd, 0xfffd, 0xfffd, 0]),
        (
            b"\xf0\x80\x80\x80\x00",
            &[0xfffd, 0xfffd, 0xfffd, 0xfffd, 0],
        ),
        (b"\xe2\x82\x00", &[0xfffd, 0xfffd, 0]),
    ];
    for (src, want) in cases {
        assert_eq!(utf8_to_utf16_model(src).as_deref(), Some(want), "{src:?}");
    }
    assert_eq!(utf8_to_utf16_model(b"\xe2\x82"), None);
}

#[test]
fn ansi_model_maps_units_above_ff_to_question_mark() {
    // not oracle: hand-computed from the WideCharToMultiByte shim.
    assert_eq!(utf16_to_ansi_model(&[0x41, 0x20ac, 0]), vec![b'A', b'?', 0]);
}

#[test]
fn error_statuses_match_scard_codes() {
    // not oracle: the `#define` values in oracle.c.
    assert_eq!(Error::InvalidParameter.status(), 0x8010_0004_u32 as i32);
    assert_eq!(Error::NoMemory.status(), 0x8010_0006_u32 as i32);
    assert_eq!(
        Error::InsufficientBuffer { required: 7 }.status(),
        0x8010_0008_u32 as i32
    );
}
