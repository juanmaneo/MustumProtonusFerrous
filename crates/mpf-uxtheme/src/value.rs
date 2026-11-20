//! Property value parsers from `msstyles.c`: the `MSSTYLES_GetProperty*`
//! functions and `MSSTYLES_GetFont`.
//!
//! Each parser takes the value text of a property, which is the `lpValue` and
//! `dwValueLen` pair of Wine's `THEME_PROPERTY` as a slice.

use crate::number::next_number;
use crate::string::{copy_units, next_token};
use crate::{unit_at, Hresult, E_PROP_ID_UNSUPPORTED};

/// `FW_REGULAR`.
pub const FW_REGULAR: i32 = 400;
/// `FW_BOLD`.
pub const FW_BOLD: i32 = 700;
/// `DEFAULT_CHARSET`.
pub const DEFAULT_CHARSET: u8 = 1;

/// `LF_FACESIZE`, the buffer size of a face name.
const LF_FACESIZE: usize = 32;
/// The buffer size of an attribute name in `MSSTYLES_GetFont`.
const ATTR_BUF_SIZE: usize = 32;
/// `MAX_INTLIST_COUNT`.
const MAX_INTLIST_COUNT: usize = 402;

/// `RECT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// `MARGINS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Margins {
    pub left_width: i32,
    pub right_width: i32,
    pub top_height: i32,
    pub bottom_height: i32,
}

/// The fields of `LOGFONTW` that `MSSTYLES_GetFont` sets. `point_size` is the
/// unscaled `lfHeight`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Font {
    pub face_name: Vec<u16>,
    pub point_size: i32,
    pub weight: i32,
    pub italic: bool,
    pub underline: bool,
    pub strike_out: bool,
    pub char_set: u8,
}

/// `MSSTYLES_GetPropertyBool`: true when the value starts with `t` or `T`.
pub fn parse_bool(value: &[u16]) -> bool {
    matches!(unit_at(value, 0), 0x74 | 0x54)
}

/// `MSSTYLES_GetPropertyColor`: reads red, green and blue, and packs them as
/// `RGB`, keeping the low byte of each.
pub fn parse_color(value: &[u16]) -> Result<u32, Hresult> {
    let (red, cur) = int_at(value, 0, value.len())?;
    let (green, cur) = int_at(value, cur, value.len())?;
    let (blue, _) = int_at(value, cur, value.len())?;
    Ok((red as u8 as u32) | ((green as u8 as u32) << 8) | ((blue as u8 as u32) << 16))
}

/// `MSSTYLES_GetPropertyInt`.
pub fn parse_int(value: &[u16]) -> Result<i32, Hresult> {
    int_at(value, 0, value.len()).map(|(number, _)| number)
}

/// `MSSTYLES_GetPropertyIntList`: reads integers until one fails, up to 402.
pub fn parse_int_list(value: &[u16]) -> Vec<i32> {
    let mut values = Vec::new();
    let mut cur = 0;
    while values.len() < MAX_INTLIST_COUNT {
        let Some((number, next)) = next_number(value, cur, value.len()) else {
            break;
        };
        values.push(number);
        cur = next;
    }
    values
}

/// `MSSTYLES_GetPropertyPosition`: returns the `(x, y)` pair.
pub fn parse_position(value: &[u16]) -> Result<(i32, i32), Hresult> {
    let (x, cur) = int_at(value, 0, value.len())?;
    let (y, _) = int_at(value, cur, value.len())?;
    Ok((x, y))
}

/// `MSSTYLES_GetPropertyString`: copies the value into a buffer of `max_chars`
/// units, including the terminator. A negative `max_chars` is read as unsigned,
/// so it does not limit the copy, as Wine's `min` does.
pub fn parse_string(value: &[u16], max_chars: i32) -> Vec<u16> {
    let limit = (value.len() + 1).min(max_chars as u32 as usize);
    copy_units(value, 0, limit)
}

/// `MSSTYLES_GetPropertyRect`: reads left, top, right and bottom into `rect`.
///
/// The first three values are optional, and a field is written only when its
/// value parses. A missing bottom is an error, and then `rect.bottom` is left
/// unchanged.
pub fn parse_rect(value: &[u16], rect: &mut Rect) -> Result<(), Hresult> {
    let mut cur = 0;
    for field in [&mut rect.left, &mut rect.top, &mut rect.right] {
        if let Some((number, next)) = next_number(value, cur, value.len()) {
            *field = number;
            cur = next;
        }
    }
    rect.bottom = int_at(value, cur, value.len())?.0;
    Ok(())
}

/// `MSSTYLES_GetPropertyMargins`: reads left, right, top and bottom into
/// `margins`, with the same rules as [`parse_rect`].
pub fn parse_margins(value: &[u16], margins: &mut Margins) -> Result<(), Hresult> {
    let mut cur = 0;
    for field in [
        &mut margins.left_width,
        &mut margins.right_width,
        &mut margins.top_height,
    ] {
        if let Some((number, next)) = next_number(value, cur, value.len()) {
            *field = number;
            cur = next;
        }
    }
    margins.bottom_height = int_at(value, cur, value.len())?.0;
    Ok(())
}

/// `MSSTYLES_GetFont`: reads a face name, a point size and optional attributes
/// from `value`.
///
/// `after` is the unit that follows the value in the theme file. A token that
/// ends at the end of `value` is trimmed against it, as Wine does, so it is
/// needed to reproduce Wine's output exactly.
pub fn parse_font(value: &[u16], after: u16) -> Result<Font, Hresult> {
    let end = value.len();
    let mut text = value.to_vec();
    text.push(after);
    let (face_name, cur) = next_token(&text, 0, end, LF_FACESIZE).ok_or(E_PROP_ID_UNSUPPORTED)?;
    let (point_size, mut cur) = int_at(&text, cur, end)?;
    let mut font = Font {
        face_name,
        point_size,
        weight: FW_REGULAR,
        italic: false,
        underline: false,
        strike_out: false,
        char_set: DEFAULT_CHARSET,
    };
    while let Some((attr, next)) = next_token(&text, cur, end, ATTR_BUF_SIZE) {
        cur = next;
        if eq_ascii_ci(&attr, "bold") {
            font.weight = FW_BOLD;
        } else if eq_ascii_ci(&attr, "italic") {
            font.italic = true;
        } else if eq_ascii_ci(&attr, "underline") {
            font.underline = true;
        } else if eq_ascii_ci(&attr, "strikeout") {
            font.strike_out = true;
        }
    }
    Ok(font)
}

fn int_at(text: &[u16], start: usize, end: usize) -> Result<(i32, usize), Hresult> {
    next_number(text, start, end).ok_or(E_PROP_ID_UNSUPPORTED)
}

/// Whether `units` equals the ASCII `word`, ignoring ASCII case.
fn eq_ascii_ci(units: &[u16], word: &str) -> bool {
    units.len() == word.len()
        && units
            .iter()
            .zip(word.bytes())
            .all(|(&unit, byte)| unit < 0x80 && (unit as u8).eq_ignore_ascii_case(&byte))
}
