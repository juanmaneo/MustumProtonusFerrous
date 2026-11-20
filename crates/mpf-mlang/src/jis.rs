//! Japanese JIS (ISO-2022-JP) and Shift-JIS conversion: the ports of `ConvertJIS2SJIS`,
//! `ConvertSJIS2JIS`, `jis2sjis`, `sjis2jis`, `han2zen`, `enter_shift` and `exit_shift`.

use crate::classes::{is_hankata, is_maru, is_nigori, is_sjis_lead, is_sjis_trail};

/// `enter_shift`: `ESC $ B`, which switches to the JIS X 0208 set.
const ENTER_JIS: [u8; 3] = [0x1b, b'$', b'B'];
/// `exit_shift`: `ESC ( B`, which switches back to ASCII.
const EXIT_JIS: [u8; 3] = [0x1b, b'(', b'B'];

/// First byte of the full-width pair for each half-width katakana from 161 to 223 (`char1`).
const HAN2ZEN_LEAD: [u8; 63] = [
    129, 129, 129, 129, 129, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 129, 131, 131, 131,
    131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131,
    131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131, 131,
    131, 131, 131, 131, 129, 129,
];

/// Second byte of the full-width pair for each half-width katakana from 161 to 223 (`char2`).
const HAN2ZEN_TRAIL: [u8; 63] = [
    66, 117, 118, 65, 69, 146, 64, 66, 68, 70, 72, 131, 133, 135, 98, 91, 65, 67, 69, 71, 73, 74,
    76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96, 99, 101, 103, 105, 106, 107, 108, 109, 110, 113,
    116, 119, 122, 125, 126, 128, 129, 130, 132, 134, 136, 137, 138, 139, 140, 141, 143, 147, 74,
    75,
];

/// `ConvertJIS2SJIS`: converts ISO-2022-JP to Shift-JIS. Returns `None` where Wine returns `0`
/// on truncated input: an escape at the end, or a shifted pair with one byte.
pub fn jis_to_sjis(input: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(input.len());
    let mut shifted = false;
    let mut i = 0;
    while i < input.len() {
        let mut p = input[i];
        if p == 0x1b {
            i += 1;
            let p2 = *input.get(i)?;
            if p2 == b'$' || p2 == b'(' {
                i += 1;
            }
            shifted = p2 == b'K' || p2 == b'$';
        } else if shifted {
            i += 1;
            let mut p2 = *input.get(i)?;
            jis2sjis(&mut p, &mut p2);
            out.push(p);
            out.push(p2);
        } else {
            out.push(p);
        }
        i += 1;
    }
    Some(out)
}

/// `ConvertSJIS2JIS`: converts Shift-JIS to ISO-2022-JP. Returns `None` where Wine returns `0`
/// on truncated input: a lead byte at the end, or a half-width katakana with no byte after it.
pub fn sjis_to_jis(input: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(input.len());
    let mut shifted = false;
    let mut i = 0;
    while i < input.len() {
        let mut p = input[i];
        if p == b'\n' || p == b'\r' {
            exit_jis(&mut out, &mut shifted);
            out.push(p);
        } else if is_sjis_lead(p) {
            i += 1;
            let mut p2 = *input.get(i)?;
            if is_sjis_trail(p2) {
                sjis2jis(&mut p, &mut p2);
                enter_jis(&mut out, &mut shifted);
            }
            out.push(p);
            out.push(p2);
        } else if is_hankata(p) {
            let mut p2 = *input.get(i + 1)?;
            i += usize::from(han2zen(&mut p, &mut p2));
            sjis2jis(&mut p, &mut p2);
            enter_jis(&mut out, &mut shifted);
            out.push(p);
            out.push(p2);
        } else {
            exit_jis(&mut out, &mut shifted);
            out.push(p);
        }
        i += 1;
    }
    exit_jis(&mut out, &mut shifted);
    Some(out)
}

fn enter_jis(out: &mut Vec<u8>, shifted: &mut bool) {
    if !*shifted {
        out.extend_from_slice(&ENTER_JIS);
        *shifted = true;
    }
}

fn exit_jis(out: &mut Vec<u8>, shifted: &mut bool) {
    if *shifted {
        out.extend_from_slice(&EXIT_JIS);
        *shifted = false;
    }
}

/// `jis2sjis`: converts a JIS X 0208 pair to Shift-JIS in place. The results wrap to a byte, as
/// the C stores into `unsigned char` do.
pub fn jis2sjis(p1: &mut u8, p2: &mut u8) {
    let c1 = *p1;
    let c2 = *p2;
    let row: i32 = if c1 < 95 { 112 } else { 176 };
    let cell: i32 = if c1 % 2 == 1 {
        31 + i32::from(c2 > 95)
    } else {
        126
    };
    *p1 = (((i32::from(c1) + 1) >> 1) + row) as u8;
    *p2 = (i32::from(c2) + cell) as u8;
}

/// `sjis2jis`: converts a Shift-JIS pair to a JIS X 0208 pair in place. A lead byte below 112,
/// or from 160 to 175, is outside the Shift-JIS lead ranges. The C shift of the negative value
/// is undefined there, so the result is the two's complement wrap that gcc gives. `sjis_to_jis`
/// never passes such a byte.
pub fn sjis2jis(p1: &mut u8, p2: &mut u8) {
    let c1 = i32::from(*p1);
    let c2 = *p2;
    let shift = c2 < 159;
    let row = if c1 < 160 { 112 } else { 176 };
    let cell = if shift { 31 + u8::from(c2 > 127) } else { 126 };
    *p1 = (((c1 - row) << 1) - i32::from(shift)) as u8;
    *p2 = c2.wrapping_sub(cell);
}

/// `han2zen`: maps a half-width katakana pair to its full-width pair in place, and returns
/// `true` when the pair takes a voiced or semi-voiced mark. The mapping is applied even when no
/// mark follows, and `sjis_to_jis` then converts the mapped pair, as `mlang.c` does.
pub fn han2zen(p1: &mut u8, p2: &mut u8) -> bool {
    let nigori = *p2 == 222 && (is_nigori(*p1) || *p1 == 179);
    let maru = *p2 == 223 && is_maru(*p1);
    if is_hankata(*p1) {
        let index = usize::from(*p1 - 161);
        *p1 = HAN2ZEN_LEAD[index];
        *p2 = HAN2ZEN_TRAIL[index];
    }
    // Each base that takes a mark maps into the range that the mark moves, so the marks always
    // apply. The only exception is the pair (131, 69), which is set directly.
    if nigori {
        if *p1 == 131 && *p2 == 69 {
            *p2 = 148;
        } else {
            *p2 += 1;
        }
    } else if maru {
        *p2 += 2;
    }
    nigori || maru
}
