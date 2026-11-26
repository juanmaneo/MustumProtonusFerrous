/// `OPT_MAX` in `dlls/printui/printui_private.h`: the number of option slots.
pub const OPT_MAX: usize = 12;

/// `FLAG_MAX` in `dlls/printui/printui_private.h`: the number of flag slots.
pub const FLAG_MAX: usize = 5;

/// `optionsW`. The position of a letter is its slot in [`RunDll::options`].
const OPTIONS_W: &[u8; OPT_MAX] = b"abcfhjlmntrv";

/// `flagsW`. The position of a letter is its slot in [`RunDll::flags`].
const FLAGS_W: &[u8; FLAG_MAX] = b"qwyzZ";

/// The commands that take one more character as their subcommand.
const SUBCOMMAND_LETTERS: &[u8] = b"dgiSX";

/// The commands that take the rest of the command line as an extra parameter.
const EXTRA_PARAMETER_LETTERS: &[u8] = b"SX";

/// The state `parse_rundll` fills in: the C `context_t`, without the window handle, the show
/// command and the argument vector.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RunDll {
    /// The BOOL that `parse_rundll` returned.
    pub ok: bool,
    /// The command letter, or `0` when none was given.
    pub command: u16,
    /// The subcommand letter, or `0` when none was given.
    pub subcommand: u16,
    /// The value of each option. `None` when the option is absent or had no value.
    pub options: [Option<Vec<u16>>; OPT_MAX],
    /// Whether each flag was given.
    pub flags: [bool; FLAG_MAX],
    /// The number of arguments taken.
    pub next_arg: usize,
    /// The text that `pNextCharW` still points at, or `None` when it is NULL.
    pub pending: Option<Vec<u16>>,
}

/// `pNextCharW`: a position in one argument. The argument ends at its first NUL.
#[derive(Clone, Copy)]
struct Cursor<'a> {
    text: &'a [u16],
    pos: usize,
}

impl Cursor<'_> {
    /// `*pNextCharW`: the character under the cursor, or `0` at the end of the argument.
    fn current(self) -> u16 {
        self.text.get(self.pos).copied().unwrap_or(0)
    }

    /// The text from the cursor to the end of the argument.
    fn rest(self) -> Vec<u16> {
        self.text[self.pos..].to_vec()
    }
}

struct Parser<'a> {
    argv: &'a [Vec<u16>],
    next_char: Option<Cursor<'a>>,
    cx: RunDll,
}

/// `parse_rundll` from `dlls/printui/printui.c`, over the split command line `argv`.
pub fn parse_rundll(argv: &[Vec<u16>]) -> RunDll {
    let mut parser = Parser {
        argv,
        next_char: None,
        cx: RunDll::default(),
    };
    let ok = parser.parse();
    let pending = parser.next_char.map(|cursor| cursor.rest());
    RunDll {
        ok,
        pending,
        ..parser.cx
    }
}

/// The text of a C string: the characters before its terminating NUL.
pub(crate) fn c_str(text: &[u16]) -> &[u16] {
    let end = text.iter().position(|&c| c == 0).unwrap_or(text.len());
    &text[..end]
}

/// The index of `c` in `letters`, like `wcschr` in the C.
fn index_of(letters: &[u8], c: u16) -> Option<usize> {
    letters.iter().position(|&letter| u16::from(letter) == c)
}

impl Parser<'_> {
    /// `get_next_wstr`: the rest of the current argument, or else the next argument. `None` when
    /// neither is left, and then the cursor is left as it was.
    fn next_wstr(&mut self) -> Option<Vec<u16>> {
        if let Some(cursor) = self.next_char.filter(|cursor| cursor.current() != 0) {
            self.next_char = None;
            return Some(cursor.rest());
        }
        let arg = self.argv.get(self.cx.next_arg)?;
        self.cx.next_arg += 1;
        self.next_char = None;
        Some(c_str(arg).to_vec())
    }

    /// `get_next_wchar`: the next character, moving on to the next argument when
    /// `use_next_argument` is set. Returns `0` at the end.
    fn next_wchar(&mut self, use_next_argument: bool) -> u16 {
        if self.next_char.is_some_and(|cursor| cursor.current() == 0) {
            self.next_char = None;
        }
        if self.next_char.is_none() && use_next_argument {
            if let Some(arg) = self.argv.get(self.cx.next_arg) {
                self.cx.next_arg += 1;
                self.next_char = Some(Cursor {
                    text: c_str(arg),
                    pos: 0,
                });
            }
        }
        let Some(cursor) = self.next_char else {
            return 0;
        };
        let c = cursor.current();
        self.next_char = (c != 0).then_some(Cursor {
            pos: cursor.pos + 1,
            ..cursor
        });
        c
    }

    /// The body of `parse_rundll`. Returns the BOOL it returns.
    fn parse(&mut self) -> bool {
        let mut c = self.next_wchar(true);
        while c != 0 {
            while c == u16::from(b' ') || c == u16::from(b'\t') {
                c = self.next_wchar(true);
            }
            if c == u16::from(b'@') {
                // The file named by `@` is not read, but its argument is still consumed.
                self.next_wstr();
                return false;
            } else if c == u16::from(b'/') {
                c = self.next_wchar(false);
                while c != 0 {
                    if let Some(index) = index_of(OPTIONS_W, c) {
                        let value = self.next_wstr();
                        self.cx.options[index] = value;
                        c = 0;
                    } else {
                        if let Some(index) = index_of(FLAGS_W, c) {
                            self.cx.flags[index] = true;
                        } else {
                            self.cx.command = c;
                            self.cx.subcommand = 0;
                        }
                        // Help has priority over all commands.
                        if c == u16::from(b'?') {
                            return true;
                        }
                        c = self.next_wchar(false);
                        // The subcommand is the next character, even when that is a flag or
                        // another command letter.
                        if index_of(SUBCOMMAND_LETTERS, self.cx.command).is_some() {
                            self.cx.subcommand = c;
                            c = self.next_wchar(false);
                        }
                    }
                }
                c = self.next_wchar(true);
            } else {
                if index_of(EXTRA_PARAMETER_LETTERS, self.cx.command).is_some() {
                    // Step back onto the first character of the extra parameter. The cursor is
                    // `None` only when whitespace ran to the end of the command line. The C
                    // decrements a NULL pointer there, which is undefined behaviour.
                    self.next_char = self.next_char.map(|cursor| Cursor {
                        pos: cursor.pos - 1,
                        ..cursor
                    });
                    return true;
                }
                return false;
            }
        }
        true
    }
}
