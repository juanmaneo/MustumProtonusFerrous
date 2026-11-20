//! Edge cases beyond the oracle rows: null and zero-length handles, bounds and
//! overflow limits, the local wire record's alignment and flag handling, and an
//! exhaustive VARTYPE check against the oracle set.

use super::*;
use crate::hstring_wire::local_layout;
use crate::oracle_checks::hstr;
use std::rc::Rc;

/// A null handle is the empty string, and a zero length never allocates.
#[test]
fn null_and_zero_length() {
    assert_eq!(create_string(None, 0), Ok(None));
    assert_eq!(create_string(Some(&[]), 0), Ok(None));
    assert_eq!(create_string(Some(&[0x61]), 0), Ok(None));
    assert_eq!(get_len(&None), 0);
    assert!(is_empty(&None));
    assert!(!has_embedded_null(&None));
    assert!(raw_buffer(&None).is_empty());
}

/// A slice shorter than the requested length is `E_INVALIDARG`, where Wine reads past its end.
#[test]
fn create_short_slice() {
    assert_eq!(create_string(Some(&[0x61]), 2), Err(E_INVALIDARG));
}

/// A reference needs a terminator at `ptr[len]`, so a slice without one is `E_INVALIDARG`.
#[test]
fn reference_needs_terminator_slot() {
    assert_eq!(create_string_reference(Some(&[0x61]), 1), Err(E_INVALIDARG));
    assert_eq!(create_string_reference(Some(&[0]), 0), Ok(None));
    assert_eq!(create_string_reference(None, 0), Ok(None));
    assert_eq!(create_string_reference(None, 1), Err(E_POINTER));
}

/// Duplicates share the allocation, as Wine's reference count does, and a null stays null.
#[test]
fn duplicate_shares_allocation() {
    let s = hstr("abc");
    let d = duplicate_string(&s);
    assert!(Rc::ptr_eq(s.as_ref().unwrap(), d.as_ref().unwrap()));
    assert_eq!(duplicate_string(&None), None);
}

/// A `start` equal to the length is null, a `start` past it is `E_BOUNDS`, and a span
/// past the end is `E_BOUNDS` even when `start + len` wraps in 32 bits.
#[test]
fn substring_bounds() {
    let s = hstr("hello");
    assert_eq!(substring(&s, 0), Ok(s.clone()));
    assert_eq!(substring(&s, 5), Ok(None));
    assert_eq!(substring(&s, 6), Err(E_BOUNDS));
    assert_eq!(substring(&None, 0), Ok(None));
    assert_eq!(substring_with_length(&s, 1, u32::MAX), Err(E_BOUNDS));
    assert_eq!(substring_with_length(&s, 2, 4), Err(E_BOUNDS));
    assert_eq!(substring_with_length(&s, 1, 3), Ok(hstr("ell")));
    assert_eq!(substring_with_length(&s, 5, 0), Ok(None));
    assert_eq!(substring_with_length(&s, 6, 0), Err(E_BOUNDS));
    assert_eq!(substring_with_length(&None, 0, 0), Ok(None));
    assert_eq!(substring_with_length(&None, 0, 1), Err(E_BOUNDS));
}

/// A null operand makes the result the other operand, shared as a duplicate.
#[test]
fn concat_null_shares_operand() {
    let s = hstr("ab");
    let left = concat(&None, &s).unwrap();
    let right = concat(&s, &None).unwrap();
    assert!(Rc::ptr_eq(s.as_ref().unwrap(), left.as_ref().unwrap()));
    assert!(Rc::ptr_eq(s.as_ref().unwrap(), right.as_ref().unwrap()));
    assert_eq!(concat(&None, &None), Ok(None));
}

/// A result over the cap is `E_OUTOFMEMORY`, where Wine wraps the length. The cap is a
/// parameter, so this runs without a 4 GiB string. A null operand skips the cap.
#[test]
fn concat_limit() {
    assert_eq!(
        concat_bounded(&hstr("ab"), &hstr("cd"), 3),
        Err(E_OUTOFMEMORY)
    );
    assert_eq!(
        concat_bounded(&hstr("ab"), &hstr("cd"), 4),
        Ok(hstr("abcd"))
    );
    assert_eq!(concat_bounded(&None, &hstr("ab"), 0), Ok(hstr("ab")));
}

/// Units compare unsigned, an embedded NUL is an ordinary unit, and null sorts first.
#[test]
fn compare_units_and_null() {
    assert_eq!(compare_ordinal(&hstr("a\0"), &hstr("a")), 1);
    assert_eq!(compare_ordinal(&hstr("a"), &hstr("a\0")), -1);
    assert_eq!(
        compare_ordinal(&from_units(&[0x8000]), &from_units(&[0x7FFF])),
        1
    );
    assert_eq!(compare_ordinal(&None, &None), 0);
    assert_eq!(compare_ordinal(&None, &hstr("a")), -1);
    assert_eq!(compare_ordinal(&hstr("a"), &None), 1);
    assert_eq!(compare_ordinal(&hstr("same"), &hstr("same")), 0);
}

/// A null trim set is `E_INVALIDARG`, even when the string is null.
#[test]
fn trim_null_set_is_invalid() {
    assert_eq!(trim_start(&hstr("ab"), &None), Err(E_INVALIDARG));
    assert_eq!(trim_start(&None, &None), Err(E_INVALIDARG));
    assert_eq!(trim_end(&hstr("ab"), &None), Err(E_INVALIDARG));
    assert_eq!(trim_end(&None, &None), Err(E_INVALIDARG));
}

/// A null string trims to null, whatever the set.
#[test]
fn trim_null_string_is_null() {
    assert_eq!(trim_start(&None, &hstr(" ")), Ok(None));
    assert_eq!(trim_end(&None, &hstr(" ")), Ok(None));
}

/// Trimming that removes nothing returns the operand itself.
#[test]
fn trim_no_match_shares_operand() {
    let s = hstr("ab");
    let start = trim_start(&s, &hstr("q")).unwrap();
    let end = trim_end(&s, &hstr("q")).unwrap();
    assert!(Rc::ptr_eq(s.as_ref().unwrap(), start.as_ref().unwrap()));
    assert!(Rc::ptr_eq(s.as_ref().unwrap(), end.as_ref().unwrap()));
}

/// Only the ends are trimmed, and a string made only of set units trims to null.
#[test]
fn trim_ends_only() {
    assert_eq!(trim_start(&hstr("abcab"), &hstr("ab")), Ok(hstr("cab")));
    assert_eq!(trim_end(&hstr("abcab"), &hstr("ab")), Ok(hstr("abc")));
    assert_eq!(trim_start(&hstr("  "), &hstr(" ")), Ok(None));
    assert_eq!(trim_end(&hstr("  "), &hstr(" ")), Ok(None));
}

/// A buffer holds `len` units and one terminator slot.
#[test]
fn string_buffer_slot_count() {
    let mut buf = StringBuffer::new(2);
    assert_eq!(buf.as_mut_slice().len(), 3);
}

/// A zero-length buffer promotes to null.
#[test]
fn string_buffer_zero_length() {
    assert_eq!(StringBuffer::new(0).promote(), Ok(None));
}

/// A buffer promotes when its terminator slot holds zero.
#[test]
fn string_buffer_promotes_zero_terminated() {
    let mut buf = StringBuffer::new(2);
    buf.as_mut_slice().copy_from_slice(&[0x61, 0x62, 0]);
    assert_eq!(buf.promote(), Ok(hstr("ab")));
}

/// Records start on 8-byte boundaries, so start 9 aligns to 16 before the 12-byte record.
#[test]
fn size_aligns_start() {
    assert_eq!(user_size(0, 9, &hstr("ab")), Ok(28));
    assert_eq!(user_size(0, 0, &None), Ok(8));
    assert_eq!(user_size(0, 1, &None), Ok(16));
}

/// Only the low 16 bits select the in-process size. Every other context sizes the
/// local record, including `MSHCTX_DIFFERENTMACHINE` (2), `MSHCTX_NOSHAREDMEM` (1)
/// and `MSHCTX_CROSSCONTEXT` (4).
#[test]
fn size_context_flags() {
    assert_eq!(user_size(1, 0, &hstr("ab")), Ok(12));
    assert_eq!(user_size(2, 0, &hstr("ab")), Ok(12));
    assert_eq!(user_size(4, 0, &hstr("ab")), Ok(12));
    assert_eq!(user_size(0x0003_0003, 0, &hstr("ab")), Ok(16));
    assert_eq!(user_size(0x0003_0000, 0, &hstr("ab")), Ok(12));
}

/// A size past `u32::MAX` is `OutOfBounds`, where Wine wraps the `ULONG`.
#[test]
fn size_overflow() {
    assert_eq!(user_size(0, 0xFFFF_FFF0, &None), Ok(0xFFFF_FFF8));
    assert_eq!(
        user_size(0, 0xFFFF_FFF8, &None),
        Err(WireError::OutOfBounds)
    );
    assert_eq!(
        user_size(MSHCTX_INPROC, 0xFFFF_FFF8, &None),
        Err(WireError::OutOfBounds)
    );
    assert_eq!(
        user_size(0, u32::MAX, &hstr("a")),
        Err(WireError::OutOfBounds)
    );
}

/// Marshal refuses `MSHCTX_INPROC` and `MSHCTX_DIFFERENTMACHINE` on the low 16 bits
/// only. The high flag bits do not change the choice.
#[test]
fn marshal_refuses_inproc_and_differentmachine() {
    let mut buf = [0u8; 16];
    let s = hstr("ab");
    assert_eq!(
        user_marshal(MSHCTX_INPROC, &mut buf, 0, &s),
        Err(WireError::InProcess)
    );
    assert_eq!(
        user_marshal(0x0003_0003, &mut buf, 0, &s),
        Err(WireError::InProcess)
    );
    assert_eq!(
        user_marshal(MSHCTX_DIFFERENTMACHINE, &mut buf, 0, &s),
        Err(WireError::DifferentMachine)
    );
    assert_eq!(
        user_marshal(0x0002_0002, &mut buf, 0, &s),
        Err(WireError::DifferentMachine)
    );
    assert_eq!(user_marshal(0x0003_0000, &mut buf, 0, &s), Ok(12));
}

/// A record must fit the buffer, so a buffer one byte short is `OutOfBounds`.
#[test]
fn marshal_buffer_too_small() {
    let s = hstr("ab");
    assert_eq!(
        user_marshal(0, &mut [0u8; 11], 0, &s),
        Err(WireError::OutOfBounds)
    );
    assert_eq!(user_marshal(0, &mut [0u8; 12], 0, &s), Ok(12));
}

/// Offsets near `usize::MAX` saturate in alignment and fail the bounds check
/// instead of wrapping.
#[test]
fn marshal_huge_offset() {
    let mut buf = [0u8; 16];
    let s = hstr("ab");
    assert_eq!(
        user_marshal(0, &mut buf, usize::MAX, &s),
        Err(WireError::OutOfBounds)
    );
    assert_eq!(
        user_marshal(0, &mut buf, usize::MAX - 8, &s),
        Err(WireError::OutOfBounds)
    );
}

/// A null string marshals as the 8-byte header with a zero byte count.
#[test]
fn marshal_null_is_header_only() {
    let mut buf = [0xAA; 16];
    assert_eq!(user_marshal(0, &mut buf, 0, &None), Ok(8));
    assert_eq!(buf[..8], [0x57, 0x64, 0x74, 0x50, 0, 0, 0, 0]);
    assert_eq!(buf[8], 0xAA);
}

/// The byte count is a `u32`, so 2^31 units (2^32 bytes) do not fit and `u32::MAX` bytes do.
#[test]
fn local_layout_u32_limit() {
    assert_eq!(
        local_layout(0, 1 << 31, usize::MAX),
        Err(WireError::OutOfBounds)
    );
    assert_eq!(
        local_layout(0, (1 << 31) - 1, usize::MAX),
        Ok((0, 8 + (u32::MAX as usize - 1), u32::MAX - 1))
    );
}

/// `MSHCTX_INPROC` is refused on the low 16 bits, and other contexts read the local record.
#[test]
fn unmarshal_refuses_inproc() {
    let buf = [0u8; 16];
    assert_eq!(
        user_unmarshal(MSHCTX_INPROC, &buf, 0),
        Err(WireError::InProcess)
    );
    assert_eq!(
        user_unmarshal(0x0003_0003, &buf, 0),
        Err(WireError::InProcess)
    );
    assert_eq!(user_unmarshal(0x0003_0000, &buf, 0), Ok((8, None)));
}

/// The header needs 8 bytes at the aligned start, so a short or unreachable header is `OutOfBounds`.
#[test]
fn unmarshal_short_header() {
    assert_eq!(user_unmarshal(0, &[0; 7], 0), Err(WireError::OutOfBounds));
    assert_eq!(user_unmarshal(0, &[0; 7], 1), Err(WireError::OutOfBounds));
    assert_eq!(
        user_unmarshal(0, &[0; 16], usize::MAX),
        Err(WireError::OutOfBounds)
    );
}

/// The data must fit the buffer, so a byte count that runs past the end is `OutOfBounds`.
#[test]
fn unmarshal_short_data() {
    let buf = [0x57, 0x64, 0x74, 0x50, 4, 0, 0, 0, 0x61, 0];
    assert_eq!(user_unmarshal(0, &buf, 0), Err(WireError::OutOfBounds));
}

/// Byte counts of 0 and 1 both give a null string, since neither holds a whole unit.
#[test]
fn unmarshal_byte_counts_below_two() {
    let mut buf = [0x57, 0x64, 0x74, 0x50, 0, 0, 0, 0];
    assert_eq!(user_unmarshal(0, &buf, 0), Ok((8, None)));
    buf[4] = 1;
    assert_eq!(user_unmarshal(0, &buf, 0), Ok((8, None)));
}

/// The context word is not checked, and `MSHCTX_DIFFERENTMACHINE` reads the local record.
#[test]
fn unmarshal_ignores_context_word() {
    let buf = [0, 0, 0, 0, 2, 0, 0, 0, 0x61, 0];
    assert_eq!(user_unmarshal(0, &buf, 0), Ok((10, hstr("a"))));
    assert_eq!(
        user_unmarshal(MSHCTX_DIFFERENTMACHINE, &buf, 0),
        Ok((10, hstr("a")))
    );
}

/// Marshal then unmarshal gives back the same string and end offset at every start
/// from 0 to 9, and the marshalled end equals `user_size`.
#[test]
fn round_trip_every_start() {
    for sample in ["", "ab", "ab\0cd", "\u{1D11E}"] {
        let s = hstr(sample);
        for start in 0..10u32 {
            let size = user_size(0, start, &s).unwrap() as usize;
            let mut buf = vec![0xAA; size];
            let end = user_marshal(0, &mut buf, start as usize, &s).unwrap();
            assert_eq!(end, size);
            assert_eq!(
                user_unmarshal(0, &buf, start as usize),
                Ok((end, s.clone()))
            );
        }
    }
}

fn clone_of<T: Clone>(value: &T) -> T {
    value.clone()
}

/// Every derived `WireError` impl runs: clone, equality and the `Debug` format.
#[test]
fn wire_error_derives() {
    for err in [
        WireError::InProcess,
        WireError::DifferentMachine,
        WireError::OutOfBounds,
    ] {
        let copy = clone_of(&err);
        assert_eq!(copy, err);
        assert!(!format!("{err:?}").is_empty());
    }
}

/// The 73 VARTYPEs `propvar_validatetype` accepts, from `target/canon/combase/propvar_oracle.out`.
const ORACLE_VALID_VT: [u16; 73] = [
    0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008, 0x0009, 0x000A, 0x000B,
    0x000D, 0x000E, 0x0010, 0x0011, 0x0012, 0x0013, 0x0014, 0x0015, 0x0016, 0x0017, 0x001E, 0x001F,
    0x0040, 0x0041, 0x0042, 0x0043, 0x0044, 0x0045, 0x0046, 0x0047, 0x0048, 0x1002, 0x1003, 0x1004,
    0x1005, 0x1006, 0x1007, 0x1008, 0x100A, 0x100B, 0x100C, 0x1010, 0x1011, 0x1012, 0x1013, 0x1014,
    0x1015, 0x101E, 0x101F, 0x1040, 0x1047, 0x1048, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007,
    0x2008, 0x2009, 0x200A, 0x200B, 0x200C, 0x200D, 0x200E, 0x2010, 0x2011, 0x2012, 0x2013, 0x2016,
    0x2017,
];

/// Every VARTYPE from 0 to 0xFFFF agrees with the oracle set.
#[test]
fn validate_vartype_matches_oracle_set() {
    for vt in 0..=u16::MAX {
        let expected = if ORACLE_VALID_VT.contains(&vt) {
            Ok(())
        } else {
            Err(STG_E_INVALIDPARAMETER)
        };
        assert_eq!(validate_vartype(vt), expected, "vt {vt:#06X}");
    }
}
