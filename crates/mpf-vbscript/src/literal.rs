//! The `&` branch of `lex_amp` in `dlls/vbscript/lex.c`, with `parse_hex_literal` and
//! `parse_oct_literal`. The forms are `&Hhex`, `&Ooct` and a bare `&` followed by octal digits.

use crate::{hex_value, unit_at, VbsError};

const AMP: u16 = b'&' as u16;
const ZERO: u16 = b'0' as u16;
const EIGHT: u16 = b'8' as u16;
const NINE: u16 = b'9' as u16;
const H_LOWER: u16 = b'h' as u16;
const H_UPPER: u16 = b'H' as u16;
const O_LOWER: u16 = b'o' as u16;
const O_UPPER: u16 = b'O' as u16;

/// A numeric literal, and the offset just past its last unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal {
    pub value: i32,
    pub end: usize,
}

/// A lexer error, and the offset that C stores in `error_loc`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexError {
    pub error: VbsError,
    pub loc: usize,
}

fn oct_value(c: u16) -> Option<u64> {
    match c {
        0x30..=0x37 => Some(u64::from(c - 0x30)),
        _ => None,
    }
}

/// `lex_amp` for the `&` at `amp`, which the caller guarantees. `Ok(None)` means the `&` is an
/// operator, and the lexer resumes at `amp + 1`.
pub fn amp_literal(src: &[u16], amp: usize) -> Result<Option<Literal>, LexError> {
    let p = amp + 1;
    let marker = unit_at(src, p);
    if (marker == H_LOWER || marker == H_UPPER) && hex_value(unit_at(src, p + 1)).is_some() {
        return hex_literal(src, amp, p).map(Some);
    }
    if (marker == O_LOWER || marker == O_UPPER) && oct_value(unit_at(src, p + 1)).is_some() {
        return oct_literal(src, amp, p).map(Some);
    }
    if oct_value(marker).is_some() {
        return oct_literal(src, amp, amp).map(Some);
    }
    Ok(None)
}

/// `parse_hex_literal`. `ptr` is the `H`. Leading zeros are skipped, and more than eight
/// significant digits overflow.
fn hex_literal(src: &[u16], amp: usize, mut ptr: usize) -> Result<Literal, LexError> {
    while unit_at(src, ptr + 1) == ZERO {
        ptr += 1;
    }
    let begin = ptr;
    let mut l: u32 = 0;
    loop {
        ptr += 1;
        let Some(d) = hex_value(unit_at(src, ptr)) else {
            break;
        };
        l = l.wrapping_mul(16).wrapping_add(d);
    }
    if begin + 9 < ptr {
        return Err(overflow(src, amp, ptr));
    }
    Ok(finish(src, ptr, l))
}

/// `parse_oct_literal`. `ptr` is the unit before the first octal digit. A `8` or `9` after the
/// digits is a syntax error. A value above `UINT_MAX` overflows.
fn oct_literal(src: &[u16], amp: usize, mut ptr: usize) -> Result<Literal, LexError> {
    let mut l: u64 = 0;
    loop {
        ptr += 1;
        let Some(d) = oct_value(unit_at(src, ptr)) else {
            break;
        };
        l = l * 8 + d;
        if l > u64::from(u32::MAX) {
            return Err(overflow(src, amp, ptr));
        }
    }
    if matches!(unit_at(src, ptr), EIGHT | NINE) {
        return Err(LexError {
            error: VbsError::SyntaxError,
            loc: ptr,
        });
    }
    Ok(finish(src, ptr, l as u32))
}

/// `literal_overflow_error`. C walks back from `ptr` to the `&`, which is `amp` unless `ptr`
/// is on that `&`.
fn overflow(src: &[u16], amp: usize, ptr: usize) -> LexError {
    let loc = if unit_at(src, ptr) == AMP { ptr } else { amp };
    LexError {
        error: VbsError::SyntaxError,
        loc,
    }
}

/// The value and end of a literal with a valid value. A trailing `&` makes it a `Long`, and the
/// unit is consumed. Otherwise a value that fits in 16 bits is an `Integer` and sign-extends.
fn finish(src: &[u16], ptr: usize, l: u32) -> Literal {
    if unit_at(src, ptr) == AMP {
        return Literal {
            value: l as i32,
            end: ptr + 1,
        };
    }
    let value = if l <= 0xffff {
        i32::from(l as u16 as i16)
    } else {
        l as i32
    };
    Literal { value, end: ptr }
}
