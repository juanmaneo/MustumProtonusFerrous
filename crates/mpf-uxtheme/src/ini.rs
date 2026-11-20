//! The theme INI reader of `uxini.c`: `UXINI_GetNextSection` and
//! `UXINI_GetNextValue`.
//!
//! Lines are trimmed of leading and trailing whitespace. A `;` starts a comment
//! that runs to the end of its line, and blank and comment-only lines are
//! skipped.

use crate::{is_space, unit_at};

const NEWLINE: u16 = b'\n' as u16;
const SEMICOLON: u16 = b';' as u16;
const OPEN_BRACKET: u16 = b'[' as u16;
const EQUALS: u16 = b'=' as u16;

/// A `name = value` line. Both parts are slices of the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry<'a> {
    pub name: &'a [u16],
    pub value: &'a [u16],
}

/// Reads the sections and values of a theme INI text in order.
#[derive(Debug, Clone)]
pub struct IniReader<'a> {
    text: &'a [u16],
    cur: usize,
}

impl<'a> IniReader<'a> {
    pub fn new(text: &'a [u16]) -> Self {
        Self { text, cur: 0 }
    }

    /// Skips lines up to the next `[` line and returns the section name between
    /// its brackets. The closing bracket is assumed, not checked.
    pub fn next_section(&mut self) -> Option<&'a [u16]> {
        let text = self.text;
        while let Some((start, end)) = self.next_line() {
            if text[start] == OPEN_BRACKET {
                let len = (end - start).saturating_sub(2);
                return Some(&text[start + 1..start + 1 + len]);
            }
        }
        None
    }

    /// Returns the next `name = value` line of the current section.
    ///
    /// A `[` line is left unread, so that `next_section` returns it. A line with
    /// no `=` ends the run and is consumed. Either way, `None` is returned.
    pub fn next_value(&mut self) -> Option<Entry<'a>> {
        let text = self.text;
        let (start, end) = self.next_line()?;
        if text[start] == OPEN_BRACKET {
            self.cur = start;
            return None;
        }
        let line = &text[start..end];
        let equals = line.iter().position(|&unit| unit == EQUALS)?;
        let mut name_len = equals;
        while name_len > 0 && is_space(line[name_len - 1]) {
            name_len -= 1;
        }
        let mut value_start = equals + 1;
        while value_start < line.len() && is_space(line[value_start]) {
            value_start += 1;
        }
        Some(Entry {
            name: &line[..name_len],
            value: &line[value_start..],
        })
    }

    /// Returns the bounds of the next line that is neither blank nor a comment,
    /// with its trailing whitespace trimmed. The line's comment is consumed.
    fn next_line(&mut self) -> Option<(usize, usize)> {
        let text = self.text;
        loop {
            if self.cur >= text.len() {
                return None;
            }
            while self.cur < text.len() && is_space(text[self.cur]) {
                self.cur += 1;
            }
            let start = self.cur;
            while self.cur < text.len() && text[self.cur] != NEWLINE && text[self.cur] != SEMICOLON
            {
                self.cur += 1;
            }
            let end = self.cur;
            if unit_at(text, self.cur) == SEMICOLON {
                while self.cur < text.len() && text[self.cur] != NEWLINE {
                    self.cur += 1;
                }
            }
            if unit_at(text, start) == SEMICOLON {
                continue;
            }
            if end == start {
                return None;
            }
            let mut trimmed = end;
            while is_space(text[trimmed - 1]) {
                trimmed -= 1;
            }
            return Some((start, trimmed));
        }
    }
}
