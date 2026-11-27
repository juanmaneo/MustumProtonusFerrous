//! `GetToken` from `tools/winebuild/parser.c`, for one line of a spec file.

/// Reads the tokens of one line. Wine's `GetToken` can also read continuation lines, which this port does not.
pub(crate) struct Tokenizer<'a> {
    line: &'a [u8],
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub(crate) fn new(line: &'a [u8]) -> Self {
        Self { line, pos: 0 }
    }

    /// `GetToken`: returns the next token, or `None` at the end of the line or at a comment.
    pub(crate) fn next_token(&mut self) -> Option<Vec<u8>> {
        let line = self.line;
        // As in a C string, a NUL byte ends the line.
        let at = |i: usize| line.get(i).copied().unwrap_or(0);
        let mut p = self.pos;
        while is_space(at(p)) {
            p += 1;
        }
        self.pos = p;
        if at(p) == 0 || is_comment(at(p)) {
            return None;
        }
        if is_separator(at(p)) {
            self.pos = p + 1;
            return Some(vec![at(p)]);
        }
        let mut token = Vec::new();
        while at(p) != 0 && !is_separator(at(p)) && !is_space(at(p)) {
            if at(p) == b'\\' {
                p += 1;
            }
            if at(p) != 0 {
                token.push(at(p));
                p += 1;
            }
        }
        self.pos = p;
        Some(token)
    }
}

/// `isspace` in the C locale.
fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// `separator_chars` for spec files: a separator is always a token of its own.
fn is_separator(byte: u8) -> bool {
    matches!(byte, b'(' | b')')
}

/// `comment_chars` for spec files: a comment starts only where a token would start.
fn is_comment(byte: u8) -> bool {
    matches!(byte, b'#' | b';')
}
