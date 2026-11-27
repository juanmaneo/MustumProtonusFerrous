//! INF tokenizer: the state machine of `parse_buffer` in `dlls/setupapi/parser.c`, without file
//! I/O. Positions are indices into the buffer, and a `None` return where the C state function
//! returns `NULL`.

/// `ERROR_EXPECTED_SECTION_NAME` from `include/setupapi.h`. Returned when a non-blank line comes
/// before the first section header and the file has no `[Strings]` section.
pub const ERROR_EXPECTED_SECTION_NAME: u32 = 0xE000_0000;
/// `ERROR_BAD_SECTION_NAME_LINE` from `include/setupapi.h`. Returned when a section header has no
/// closing `]` on its line.
pub const ERROR_BAD_SECTION_NAME_LINE: u32 = 0xE000_0001;
/// `ERROR_SECTION_NAME_TOO_LONG` from `include/setupapi.h`. Returned for a section name longer
/// than 255 units.
pub const ERROR_SECTION_NAME_TOO_LONG: u32 = 0xE000_0002;

/// `CONTROL_Z`. The input ends at this unit.
const CONTROL_Z: u16 = 0x1a;
/// `MAX_SECTION_NAME_LEN`. Longer section names are an error.
const MAX_SECTION_NAME_LEN: usize = 255;
/// `MAX_FIELD_LEN`. Longer tokens are silently truncated.
const MAX_FIELD_LEN: usize = 511;

const LF: u16 = b'\n' as u16;
const SPACE: u16 = b' ' as u16;
const QUOTE: u16 = b'"' as u16;
const COMMA: u16 = b',' as u16;
const SEMICOLON: u16 = b';' as u16;
const EQUALS: u16 = b'=' as u16;
const LBRACKET: u16 = b'[' as u16;
const RBRACKET: u16 = b']' as u16;
const BACKSLASH: u16 = b'\\' as u16;

/// A parsed INF file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfFile {
    /// Sections in order of first appearance. Names that differ only in ASCII case are one section.
    pub sections: Vec<InfSection>,
    /// The text of every field in the order the parser added them. The value fields of a line are
    /// consecutive, starting at [`InfLine::first_field`].
    pub fields: Vec<Vec<u16>>,
    /// Index of the `[Strings]` section, or `None` when the file has none.
    pub strings_section: Option<usize>,
}

/// A section of an INF file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfSection {
    /// The name, spelled as in the first header with that name.
    pub name: Vec<u16>,
    /// The lines of the section, in file order.
    pub lines: Vec<InfLine>,
}

/// A line of a section: an optional key field followed by value fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InfLine {
    /// Index in [`InfFile::fields`] of the first value field.
    pub first_field: usize,
    /// The number of value fields.
    pub nb_fields: usize,
    /// Index in [`InfFile::fields`] of the key field, or `None` when the line has no key.
    pub key_field: Option<usize>,
}

/// A parse failure: the `ERROR_*` code, and the 1-based line the parser stopped on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InfError {
    /// The error code.
    pub code: u32,
    /// The line the error was found on.
    pub line: u32,
}

/// Parses the text of an INF file, as `parse_buffer` does in `dlls/setupapi/parser.c`.
///
/// `buf` holds UTF-16 code units. A `\x1a` unit ends the input, as it does in the C parser.
pub fn parse_inf_buffer(buf: &[u16]) -> Result<InfFile, InfError> {
    let mut parser = Parser {
        buf,
        start: 0,
        state: State::LineStart,
        stack: Vec::new(),
        file: InfFile {
            sections: Vec::new(),
            fields: Vec::new(),
            strings_section: None,
        },
        cur_section: None,
        line: None,
        line_pos: 1,
        broken_line: None,
        error: None,
        token: Vec::new(),
    };
    let mut pos = Some(0);
    while let Some(p) = pos {
        pos = parser.run_state(p);
    }
    if let Some(code) = parser.error {
        return Err(InfError {
            code,
            line: parser.line_pos,
        });
    }

    let mut file = parser.file;
    let strings: Vec<u16> = "Strings".encode_utf16().collect();
    file.strings_section = find_section(&file.sections, &strings);
    match (file.strings_section, parser.broken_line) {
        (None, Some(line)) => Err(InfError {
            code: ERROR_EXPECTED_SECTION_NAME,
            line,
        }),
        _ => Ok(file),
    }
}

/// The parser states of the C `enum parser_state`.
enum State {
    LineStart,
    SectionName,
    KeyName,
    ValueName,
    EolBackslash,
    Quotes,
    LeadingSpaces,
    TrailingSpaces,
    Comment,
}

struct Parser<'a> {
    buf: &'a [u16],
    /// Start of the text the current token will take, `parser->start` in C.
    start: usize,
    state: State,
    stack: Vec<State>,
    file: InfFile,
    cur_section: Option<usize>,
    /// The line being built, as its section index and its index in that section.
    line: Option<(usize, usize)>,
    line_pos: u32,
    broken_line: Option<u32>,
    error: Option<u32>,
    token: Vec<u16>,
}

impl Parser<'_> {
    fn run_state(&mut self, pos: usize) -> Option<usize> {
        match self.state {
            State::LineStart => self.line_start(pos),
            State::SectionName => self.section_name(pos),
            State::KeyName => self.key_name(pos),
            State::ValueName => self.value_name(pos),
            State::EolBackslash => self.eol_backslash(pos),
            State::Quotes => self.quotes(pos),
            State::LeadingSpaces => self.leading_spaces(pos),
            State::TrailingSpaces => self.trailing_spaces(pos),
            State::Comment => self.comment(pos),
        }
    }

    fn is_eof(&self, p: usize) -> bool {
        p >= self.buf.len() || self.buf[p] == CONTROL_Z
    }

    fn is_eol(&self, p: usize) -> bool {
        p >= self.buf.len() || self.buf[p] == CONTROL_Z || self.buf[p] == LF
    }

    fn push_state(&mut self, state: State) {
        self.stack.push(state);
    }

    fn pop_state(&mut self) {
        self.state = self
            .stack
            .pop()
            .expect("the C parser only pops after a push");
    }

    /// Appends the text from `start` up to `pos` to the token. NUL units become spaces, and the
    /// token is cut at `MAX_FIELD_LEN` units.
    fn push_token(&mut self, pos: usize) {
        let len = (pos - self.start).min(MAX_FIELD_LEN - self.token.len());
        let buf = self.buf;
        self.token
            .extend(
                buf[self.start..self.start + len]
                    .iter()
                    .map(|&c| if c == 0 { SPACE } else { c }),
            );
        self.start = pos;
    }

    /// Makes the token a section header. Returns `None` for a name longer than 255 units.
    fn add_section_from_token(&mut self) -> Option<()> {
        if self.token.len() > MAX_SECTION_NAME_LEN {
            self.error = Some(ERROR_SECTION_NAME_TOO_LONG);
            return None;
        }
        let section = match find_section(&self.file.sections, &self.token) {
            Some(index) => index,
            None => {
                self.file.sections.push(InfSection {
                    name: self.token.clone(),
                    lines: Vec::new(),
                });
                self.file.sections.len() - 1
            }
        };
        self.token.clear();
        self.cur_section = Some(section);
        Some(())
    }

    /// Adds the token as a field of the current line, opening a new line when none is open.
    /// `is_key` makes the field the line's key. A key is only parsed at the start of a line, so
    /// the C `assert(!is_key)` for an open line is not repeated here.
    fn add_field_from_token(&mut self, is_key: bool) {
        let (section, index) = match self.line {
            Some(line) => line,
            None => {
                let section = self
                    .cur_section
                    .expect("names are only parsed inside a section");
                let first_field = self.file.fields.len();
                let lines = &mut self.file.sections[section].lines;
                lines.push(InfLine {
                    first_field,
                    nb_fields: 0,
                    key_field: None,
                });
                let line = (section, lines.len() - 1);
                self.line = Some(line);
                line
            }
        };
        self.file.fields.push(std::mem::take(&mut self.token));
        let line = &mut self.file.sections[section].lines[index];
        if is_key {
            line.key_field = Some(line.first_field);
            line.first_field += 1;
        } else {
            line.nb_fields += 1;
        }
    }

    /// A line with one field and no key uses that field as its key.
    fn close_current_line(&mut self) {
        if let Some((section, index)) = self.line.take() {
            let line = &mut self.file.sections[section].lines[index];
            if line.nb_fields == 1 && line.key_field.is_none() {
                line.key_field = Some(line.first_field);
            }
        }
    }

    fn line_start(&mut self, pos: usize) -> Option<usize> {
        let mut p = pos;
        while !self.is_eof(p) {
            let c = self.buf[p];
            match c {
                LF => {
                    self.line_pos += 1;
                    self.close_current_line();
                }
                SEMICOLON => {
                    self.push_state(State::LineStart);
                    self.state = State::Comment;
                    return Some(p + 1);
                }
                LBRACKET => {
                    self.start = p + 1;
                    self.state = State::SectionName;
                    return Some(p + 1);
                }
                _ if is_space(c) => {}
                _ if self.cur_section.is_some() => {
                    self.start = p;
                    self.state = State::KeyName;
                    return Some(p);
                }
                _ => {
                    self.broken_line.get_or_insert(self.line_pos);
                }
            }
            p += 1;
        }
        self.close_current_line();
        None
    }

    fn section_name(&mut self, pos: usize) -> Option<usize> {
        let mut p = pos;
        while !self.is_eol(p) {
            if self.buf[p] == RBRACKET {
                self.push_token(p);
                self.add_section_from_token()?;
                self.push_state(State::LineStart);
                self.state = State::Comment;
                return Some(p + 1);
            }
            p += 1;
        }
        self.error = Some(ERROR_BAD_SECTION_NAME_LINE);
        None
    }

    fn key_name(&mut self, pos: usize) -> Option<usize> {
        let mut token_end = self.start;
        let mut p = pos;
        while !self.is_eol(p) {
            let c = self.buf[p];
            match c {
                COMMA => break,
                EQUALS => {
                    self.push_token(token_end);
                    self.add_field_from_token(true);
                    self.start = p + 1;
                    self.push_state(State::ValueName);
                    self.state = State::LeadingSpaces;
                    return Some(p + 1);
                }
                SEMICOLON => {
                    self.push_token(token_end);
                    self.add_field_from_token(false);
                    self.push_state(State::LineStart);
                    self.state = State::Comment;
                    return Some(p + 1);
                }
                QUOTE => {
                    self.push_token(p);
                    self.start = p + 1;
                    self.push_state(State::KeyName);
                    self.state = State::Quotes;
                    return Some(p + 1);
                }
                BACKSLASH => {
                    self.push_token(token_end);
                    self.start = p;
                    self.push_state(State::KeyName);
                    self.state = State::EolBackslash;
                    return Some(p);
                }
                _ if !is_space(c) => token_end = p + 1,
                _ => {
                    self.push_token(p);
                    self.push_state(State::KeyName);
                    self.state = State::TrailingSpaces;
                    return Some(p);
                }
            }
            p += 1;
        }
        self.push_token(token_end);
        self.state = State::ValueName;
        Some(p)
    }

    fn value_name(&mut self, pos: usize) -> Option<usize> {
        let mut token_end = self.start;
        let mut p = pos;
        while !self.is_eol(p) {
            let c = self.buf[p];
            match c {
                SEMICOLON => {
                    self.push_token(token_end);
                    self.add_field_from_token(false);
                    self.push_state(State::LineStart);
                    self.state = State::Comment;
                    return Some(p + 1);
                }
                COMMA => {
                    self.push_token(token_end);
                    self.add_field_from_token(false);
                    self.start = p + 1;
                    self.push_state(State::ValueName);
                    self.state = State::LeadingSpaces;
                    return Some(p + 1);
                }
                QUOTE => {
                    self.push_token(p);
                    self.start = p + 1;
                    self.push_state(State::ValueName);
                    self.state = State::Quotes;
                    return Some(p + 1);
                }
                BACKSLASH => {
                    self.push_token(token_end);
                    self.start = p;
                    self.push_state(State::ValueName);
                    self.state = State::EolBackslash;
                    return Some(p);
                }
                _ if c != 0 && !is_space(c) => token_end = p + 1,
                _ => {
                    self.push_token(p);
                    self.push_state(State::ValueName);
                    self.state = State::TrailingSpaces;
                    return Some(p);
                }
            }
            p += 1;
        }
        self.push_token(token_end);
        self.add_field_from_token(false);
        self.state = State::LineStart;
        Some(p)
    }

    fn eol_backslash(&mut self, pos: usize) -> Option<usize> {
        let mut p = pos;
        while !self.is_eof(p) {
            let c = self.buf[p];
            match c {
                LF => {
                    self.line_pos += 1;
                    self.start = p + 1;
                    self.state = State::LeadingSpaces;
                    return Some(p + 1);
                }
                BACKSLASH => {}
                SEMICOLON => {
                    self.push_state(State::EolBackslash);
                    self.state = State::Comment;
                    return Some(p + 1);
                }
                _ if is_space(c) => {}
                _ => {
                    self.push_token(p);
                    self.pop_state();
                    return Some(p);
                }
            }
            p += 1;
        }
        self.start = p;
        self.pop_state();
        Some(p)
    }

    fn quotes(&mut self, pos: usize) -> Option<usize> {
        let mut p = pos;
        while !self.is_eol(p) {
            if self.buf[p] == QUOTE {
                if p + 1 < self.buf.len() && self.buf[p + 1] == QUOTE {
                    // A doubled quote is one quote unit in the token.
                    self.push_token(p + 1);
                    self.start = p + 2;
                    p += 1;
                } else {
                    self.push_token(p);
                    self.start = p + 1;
                    self.pop_state();
                    return Some(p + 1);
                }
            }
            p += 1;
        }
        self.push_token(p);
        self.pop_state();
        Some(p)
    }

    fn leading_spaces(&mut self, pos: usize) -> Option<usize> {
        let mut p = pos;
        while !self.is_eol(p) {
            let c = self.buf[p];
            if c == BACKSLASH {
                self.start = p;
                self.state = State::EolBackslash;
                return Some(p);
            }
            if !is_space(c) {
                break;
            }
            p += 1;
        }
        self.start = p;
        self.pop_state();
        Some(p)
    }

    fn trailing_spaces(&mut self, pos: usize) -> Option<usize> {
        let mut p = pos;
        while !self.is_eol(p) {
            let c = self.buf[p];
            if c == BACKSLASH {
                self.state = State::EolBackslash;
                return Some(p);
            }
            if c != 0 && !is_space(c) {
                break;
            }
            p += 1;
        }
        self.pop_state();
        Some(p)
    }

    fn comment(&mut self, pos: usize) -> Option<usize> {
        let mut p = pos;
        while !self.is_eol(p) {
            p += 1;
        }
        self.pop_state();
        Some(p)
    }
}

/// `iswspace` for the ASCII range: space, and `\t` through `\r`.
fn is_space(c: u16) -> bool {
    c == SPACE || (0x09..=0x0d).contains(&c)
}

/// `wcsicmp` for ASCII letters. Wine's version folds case with its locale tables.
fn wcsicmp(a: &[u16], b: &[u16]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(&x, &y)| ascii_lower(x) == ascii_lower(y))
}

fn ascii_lower(c: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
        c + 0x20
    } else {
        c
    }
}

/// `find_section`: the index of the first section whose name matches `name`.
fn find_section(sections: &[InfSection], name: &[u16]) -> Option<usize> {
    sections
        .iter()
        .position(|section| wcsicmp(name, &section.name))
}
