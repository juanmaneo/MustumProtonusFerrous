//! Reader position tracking and predefined entity names from reader.c.

use crate::eq_ascii;

const CR: u16 = 0x0d;
const LF: u16 = 0x0a;

const PREDEFINED: [(&str, u16); 5] = [
    ("lt", 0x3c),
    ("gt", 0x3e),
    ("amp", 0x26),
    ("apos", 0x27),
    ("quot", 0x22),
];

/// `struct reader_position`. Lines and columns are 1-based.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub line_number: u32,
    pub line_position: u32,
}

/// `reader_update_position`: moves past one character. CR starts the column over, and LF
/// starts a new line. Both counters wrap, as the C's unsigned ones do.
pub fn advance(position: &mut Position, ch: u16) {
    match ch {
        CR => position.line_position = 1,
        LF => {
            position.line_number = position.line_number.wrapping_add(1);
            position.line_position = 1;
        }
        _ => position.line_position = position.line_position.wrapping_add(1),
    }
}

/// `get_predefined_entity`: the character that one of the five predefined entities names.
pub fn predefined_entity(name: &[u16]) -> Option<u16> {
    PREDEFINED
        .iter()
        .find(|(text, _)| eq_ascii(name, text))
        .map(|&(_, ch)| ch)
}
