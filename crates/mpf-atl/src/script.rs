use crate::error::Error;
use crate::text::is_space;

pub(crate) const CLOSE: u16 = b'}' as u16;
pub(crate) const EQUALS: u16 = b'=' as u16;
pub(crate) const MINUS: u16 = b'-' as u16;
pub(crate) const OPEN: u16 = b'{' as u16;
pub(crate) const PLUS: u16 = b'+' as u16;
const QUOTE: u16 = b'\'' as u16;

/// A read position in a script. Mirrors the `LPCOLESTR *str` argument of `get_word`.
pub(crate) struct Cursor<'a> {
    data: &'a [u16],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(data: &'a [u16]) -> Self {
        Cursor { data, pos: 0 }
    }

    pub(crate) fn at_end(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn peek(&self) -> Option<u16> {
        self.data.get(self.pos).copied()
    }

    /// Consumes `c` when it is the next character.
    pub(crate) fn eat(&mut self, c: u16) -> bool {
        if self.peek() == Some(c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    /// Consumes a `{` followed by whitespace, which opens a nested block.
    pub(crate) fn eat_block_open(&mut self) -> bool {
        if self.peek() == Some(OPEN) && self.data.get(self.pos + 1).copied().is_some_and(is_space) {
            self.pos += 1;
            self.skip_spaces();
            true
        } else {
            false
        }
    }

    fn skip_spaces(&mut self) {
        while self.peek().is_some_and(is_space) {
            self.pos += 1;
        }
    }

    /// `get_word`: the next word, or an empty word at end of input. Trailing whitespace is skipped.
    pub(crate) fn next_word(&mut self) -> Result<Vec<u16>, Error> {
        self.skip_spaces();
        let Some(first) = self.peek() else {
            return Ok(Vec::new());
        };
        let word = if first == CLOSE || first == EQUALS {
            self.pos += 1;
            vec![first]
        } else if first == QUOTE {
            self.quoted()?
        } else {
            self.bare()
        };
        self.skip_spaces();
        Ok(word)
    }

    /// A `'...'` word in which `''` stands for one quote.
    fn quoted(&mut self) -> Result<Vec<u16>, Error> {
        let mut word = Vec::new();
        let mut start = self.pos + 1;
        loop {
            let offset = self.data[start..]
                .iter()
                .position(|&c| c == QUOTE)
                .ok_or(Error::Exception)?;
            let close = start + offset;
            word.extend_from_slice(&self.data[start..close]);
            if self.data.get(close + 1) != Some(&QUOTE) {
                self.pos = close + 1;
                return Ok(word);
            }
            word.push(QUOTE);
            start = close + 2;
        }
    }

    /// An unquoted word, which runs to the next whitespace or the end of input.
    fn bare(&mut self) -> Vec<u16> {
        let start = self.pos;
        while self.peek().is_some_and(|c| !is_space(c)) {
            self.pos += 1;
        }
        self.data[start..self.pos].to_vec()
    }
}
