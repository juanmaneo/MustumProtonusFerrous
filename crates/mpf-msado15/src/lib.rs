//! Port of the criteria parser in Wine's `dlls/msado15/recordset.c` at commit
//! `e2aa6cd75ae60acd90ff384feee171ce994f0f6d`. `parse_criteria` splits a Find
//! criterion such as `[first name] = 'Bob'` into a column, a comparison operator
//! and a value, and `recordset_Find` requires that the criterion uses the whole
//! string.
//!
//! Ported: bracketed and bare column names; the operators `<`, `<=`, `<>`, `=`,
//! `>`, `>=` and `like`; quoted values with doubled quotes, `#`-delimited values
//! and bare values; and the `*` wildcards of `like`, which give a prefix match, a
//! contains match, or plain equality when there is no trailing `*`.
//!
//! Skipped: the `FIXME` diagnostics, the `BSTR` allocation (the strings are
//! `Vec<u16>`) and the `E_OUTOFMEMORY` path.
//!
//! Deviations: white space and case folding follow the C locale, so only ASCII
//! white space and ASCII letters are recognised. The text is read up to its first
//! NUL, as the C code reads a NUL-terminated string. The whole-string check from
//! `recordset_Find` is folded into [`parse_criteria`], which returns `None` where
//! Wine returns `adErrInvalidArgument`.

const APOS: u16 = b'\'' as u16;
const EQUALS: u16 = b'=' as u16;
const GREATER: u16 = b'>' as u16;
const HASH: u16 = b'#' as u16;
const LBRACKET: u16 = b'[' as u16;
const LESS: u16 = b'<' as u16;
const RBRACKET: u16 = b']' as u16;
const STAR: u16 = b'*' as u16;

/// The comparison operator of a criterion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompareOp {
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
    /// `like` with a trailing `*`.
    BeginsWith,
    /// `like` with a leading and a trailing `*`.
    Contains,
    Ne,
}

impl CompareOp {
    /// The `DBCOMPAREOPS` value that Wine stores for this operator.
    pub const fn code(self) -> u32 {
        match self {
            Self::Lt => 0,
            Self::Le => 1,
            Self::Eq => 2,
            Self::Ge => 3,
            Self::Gt => 4,
            Self::BeginsWith => 5,
            Self::Contains => 6,
            Self::Ne => 7,
        }
    }
}

/// One parsed criterion, `column op value`. The strings are UTF-16, as the BSTRs
/// that Wine builds are.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Criteria {
    pub column: Vec<u16>,
    pub op: CompareOp,
    pub value: Vec<u16>,
}

/// Parses `text` as one criterion. Returns `None` when the text is not a single
/// criterion, as `recordset_Find` requires, and where Wine returns
/// `adErrInvalidArgument`.
pub fn parse_criteria(text: &[u16]) -> Option<Criteria> {
    let end = text.iter().position(|&c| c == 0).unwrap_or(text.len());
    let text = &text[..end];
    let mut cursor = Cursor { text, pos: 0 };
    let criteria = cursor.criterion()?;
    (cursor.pos == text.len()).then_some(criteria)
}

struct Cursor<'a> {
    text: &'a [u16],
    pos: usize,
}

impl Cursor<'_> {
    /// The unit at `index`, or NUL past the end, which stops every scan.
    fn at(&self, index: usize) -> u16 {
        self.text.get(index).copied().unwrap_or(0)
    }

    fn peek(&self) -> u16 {
        self.at(self.pos)
    }

    fn peek_at(&self, n: usize) -> u16 {
        self.at(self.pos + n)
    }

    fn skip_spaces(&mut self) {
        while is_space(self.peek()) {
            self.pos += 1;
        }
    }

    /// Moves past the units up to a NUL or up to a unit that `stop` accepts.
    fn scan_to(&mut self, stop: impl Fn(u16) -> bool) {
        while self.peek() != 0 && !stop(self.peek()) {
            self.pos += 1;
        }
    }

    /// Moves past one unit, unless the text has ended.
    fn step(&mut self) {
        if self.peek() != 0 {
            self.pos += 1;
        }
    }

    fn criterion(&mut self) -> Option<Criteria> {
        self.skip_spaces();
        if self.peek() == 0 {
            return None;
        }
        let (col_b, col_e) = self.column();
        if self.peek() == 0 || col_b == col_e {
            return None;
        }

        self.skip_spaces();
        let mut op = self.operator()?;
        self.skip_spaces();
        if self.peek() == 0 {
            return None;
        }

        let (mut val_b, mut val_e, mut val_len) = self.value();
        if op == CompareOp::BeginsWith {
            if self.at(val_b) == STAR && (val_e == val_b + 1 || self.at(val_e - 1) != STAR) {
                return None;
            }
            if val_b == val_e || self.at(val_e - 1) != STAR {
                op = CompareOp::Eq;
            } else if self.at(val_b) == STAR {
                op = CompareOp::Contains;
                val_b += 1;
                val_e -= 1;
                val_len -= 2;
            } else {
                val_e -= 1;
                val_len -= 1;
            }
        }

        let column = self.text[col_b..col_e].to_vec();
        let raw = &self.text[val_b..val_e];
        let value = if val_len == raw.len() {
            raw.to_vec()
        } else {
            unescape(raw)
        };
        self.skip_spaces();
        Some(Criteria { column, op, value })
    }

    /// Scans the column name: `[name]`, or the units up to white space or `=`.
    /// Returns the span of the name.
    fn column(&mut self) -> (usize, usize) {
        if self.peek() == LBRACKET {
            self.pos += 1;
            let begin = self.pos;
            self.scan_to(|c| c == RBRACKET);
            let end = self.pos;
            self.step();
            (begin, end)
        } else {
            let begin = self.pos;
            self.scan_to(|c| is_space(c) || c == EQUALS);
            (begin, self.pos)
        }
    }

    fn operator(&mut self) -> Option<CompareOp> {
        let (op, len) = match self.peek() {
            GREATER if self.peek_at(1) == EQUALS => (CompareOp::Ge, 2),
            GREATER => (CompareOp::Gt, 1),
            LESS if self.peek_at(1) == EQUALS => (CompareOp::Le, 2),
            LESS if self.peek_at(1) == GREATER => (CompareOp::Ne, 2),
            LESS => (CompareOp::Lt, 1),
            EQUALS => (CompareOp::Eq, 1),
            _ if self.is_like() => (CompareOp::BeginsWith, 4),
            _ => return None,
        };
        self.pos += len;
        Some(op)
    }

    fn is_like(&self) -> bool {
        b"like"
            .iter()
            .enumerate()
            .all(|(i, &k)| ascii_lower(self.peek_at(i)) == u16::from(k))
    }

    /// Scans the value and returns its span and its length. The length counts a
    /// doubled quote once, so it can be shorter than the span. The span ends before
    /// the closing delimiter, which is consumed.
    fn value(&mut self) -> (usize, usize, usize) {
        let (begin, len) = if self.peek() == APOS {
            self.pos += 1;
            let begin = self.pos;
            let mut len = 0;
            while self.peek() != 0 && (self.peek() != APOS || self.peek_at(1) == APOS) {
                if self.peek() == APOS {
                    self.pos += 1;
                }
                self.pos += 1;
                len += 1;
            }
            (begin, len)
        } else if self.peek() == HASH {
            self.pos += 1;
            let begin = self.pos;
            self.scan_to(|c| c == HASH);
            (begin, self.pos - begin)
        } else {
            let begin = self.pos;
            self.scan_to(is_space);
            (begin, self.pos - begin)
        };
        let end = self.pos;
        self.step();
        (begin, end, len)
    }
}

/// The white space that the C locale's `iswspace` accepts: space, and the controls
/// from tab to carriage return.
fn is_space(c: u16) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}

fn ascii_lower(c: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
        c + 32
    } else {
        c
    }
}

/// Collapses each doubled quote in `raw` to one quote.
fn unescape(raw: &[u16]) -> Vec<u16> {
    let mut out = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        out.push(raw[i]);
        if raw[i] == APOS {
            i += 1;
        }
        i += 1;
    }
    out
}

#[cfg(test)]
mod edge_checks;

#[cfg(test)]
mod oracle_checks;
