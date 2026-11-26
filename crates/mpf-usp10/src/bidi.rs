//! Bidi reordering and strength classification from Wine's Uniscribe,
//! `dlls/gdi32/uniscribe/bidi.c`.

use crate::direction::bidi_class;

/// Bidi class values from `enum directions` in `bidi.c`. The class table stores
/// these numbers, so they must keep their values. `NSM` and the isolate
/// controls are not named here because the port never matches on them.
pub(crate) const ON: u16 = 0;
pub(crate) const NI: u16 = ON;
pub(crate) const L: u16 = 1;
pub(crate) const R: u16 = 2;
pub(crate) const AN: u16 = 3;
pub(crate) const EN: u16 = 4;
pub(crate) const AL: u16 = 5;
pub(crate) const CS: u16 = 7;
pub(crate) const ES: u16 = 8;
pub(crate) const ET: u16 = 9;
pub(crate) const BN: u16 = 10;
pub(crate) const S: u16 = 11;
pub(crate) const WS: u16 = 12;
pub(crate) const B: u16 = 13;
pub(crate) const RLO: u16 = 14;
pub(crate) const RLE: u16 = 15;
pub(crate) const LRO: u16 = 16;
pub(crate) const LRE: u16 = 17;
pub(crate) const PDF: u16 = 18;

pub const BIDI_NEUTRAL: u16 = 0;
pub const BIDI_STRONG: u16 = 1;
pub const BIDI_WEAK: u16 = 2;

/// `BIDI_ReorderV2lLevel`: rule L2 as a recursion over the embedding levels.
/// Reverses `indices[..cch]` from the highest level down to the lowest odd
/// level, and returns how many entries this level consumed.
pub fn bidi_reorder_v2l_level(
    level: i32,
    indices: &mut [i32],
    plevel: &[u8],
    cch: usize,
    f_reverse: bool,
) -> usize {
    let f_reverse = f_reverse || (level & 1) != 0;
    let mut ich = 0;
    while ich < cch {
        let lv = i32::from(plevel[ich]);
        if lv < level {
            break;
        }
        if lv > level {
            ich += bidi_reorder_v2l_level(
                level + 1,
                &mut indices[ich..],
                &plevel[ich..],
                cch - ich,
                f_reverse,
            ) - 1;
        }
        ich += 1;
    }
    if f_reverse {
        indices[..ich].reverse();
    }
    ich
}

/// `BIDI_ReorderL2vLevel`: the inverse of [`bidi_reorder_v2l_level`], applied to
/// an already reordered string. Returns how many entries this level consumed.
pub fn bidi_reorder_l2v_level(
    level: i32,
    indices: &mut [i32],
    plevel: &[u8],
    cch: usize,
    f_reverse: bool,
) -> usize {
    let f_reverse = f_reverse || (level & 1) != 0;
    let mut ich = 0;
    let mut has_higher = false;
    while ich < cch {
        let lv = i32::from(plevel[ich]);
        if lv < level {
            break;
        }
        has_higher |= lv > level;
        ich += 1;
    }
    if f_reverse {
        indices[..ich].reverse();
    }
    if has_higher {
        ich = 0;
        while ich < cch {
            let lv = i32::from(plevel[ich]);
            if lv < level {
                break;
            }
            if lv > level {
                ich += bidi_reorder_l2v_level(
                    level + 1,
                    &mut indices[ich..],
                    &plevel[ich..],
                    cch - ich,
                    f_reverse,
                ) - 1;
            }
            ich += 1;
        }
    }
    ich
}

/// `BIDI_GetStrengths`: maps the first `count` code units to a strong, weak or
/// neutral strength. Always returns `true`, as the C does.
pub fn bidi_get_strengths(
    string: &[u16],
    count: usize,
    legacy_bidi_class: bool,
    strength: &mut [u16],
) -> bool {
    let strength = &mut strength[..count];
    classify(&string[..count], strength, legacy_bidi_class);
    for s in strength.iter_mut() {
        *s = match *s {
            L | LRE | LRO | R | AL | RLE | RLO => BIDI_STRONG,
            PDF | EN | ES | ET | AN | CS | BN => BIDI_WEAK,
            B | S | WS | ON => BIDI_NEUTRAL,
            // NSM and any other class take the C `default:` arm.
            _ => BIDI_NEUTRAL,
        };
    }
    true
}

/// `classify`: each code unit's bidi class. Under the legacy class flag, `+`
/// and `-` that would be European separators become neutral.
fn classify(string: &[u16], chartype: &mut [u16], legacy_bidi_class: bool) {
    for (&ch, class) in string.iter().zip(chartype.iter_mut()) {
        *class = bidi_class(ch);
        if legacy_bidi_class && *class == ES && (ch == u16::from(b'+') || ch == u16::from(b'-')) {
            *class = NI;
        }
    }
}
