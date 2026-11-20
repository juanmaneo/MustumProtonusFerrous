//! Headless port of the deterministic parts of Wine's `dlls/vbscript`, the VBScript engine.
//!
//! The C reference is a scripting engine. Its parser, interpreter, COM dispatch, VARIANT
//! conversions and the Win32 and oleaut32 calls it makes need a live process. The helpers below
//! run here as pure functions over values. The integrator converts each VARIANT to the Rust type
//! that the function takes, and passes the seed that `GetTickCount` would supply.
//!
//! Ported (names in `dlls/vbscript`):
//! - `get_next_rnd`, `Global_Randomize` and `Global_Rnd` in `global.c`: [rnd].
//! - `Global_Hex` and `Global_Oct` in `global.c`: [radix].
//! - `Global_Escape` and `Global_Unescape` in `global.c`: [escape].
//! - `Global_Left`, `Global_Right`, `Global_Mid` and `Global_ChrW` in `global.c`: [strings].
//! - `Global_RGB` in `global.c`: [color].
//! - the `&` branch of `lex_amp`, with `parse_hex_literal` and `parse_oct_literal` in `lex.c`:
//!   [literal].
//! - the per-match replacement of `RegExp2_Replace` in `vbregexp.c`: [replace].
//! - `get_safearray_size` in `utils.c`: [array].
//!
//! Skipped:
//! - the parser and the interpreter, the rest of `lex.c` (identifiers, strings, dates and
//!   operators), and the `IDispatch` and `BuiltinDisp` plumbing;
//! - the `to_int`, `to_float`, `to_double` and `to_string` conversions, and the VARIANT null and
//!   empty checks;
//! - `Randomize` with no argument, which reads `GetTickCount`. The integrator supplies the seed;
//! - `Global_InStr`, which calls the NLS `FindStringOrdinal`, and `Global_Round`, which calls
//!   oleaut32 `VarR8Round`;
//! - the regular-expression matcher, and the loop of `RegExp2_Replace` around the per-match
//!   expansion;
//! - the other `Global_*` builtins;
//! - all `TRACE`, `WARN`, `FIXME` and `ERR` logging.
//!
//! Deviations from Wine, where Wine has undefined behaviour or a crash:
//! - [rnd::rnd] wraps the 32-bit multiply in `get_next_rnd`. C multiplies signed ints, which
//!   overflows. The low 24 bits, which are the state, match the `-fwrapv` build of the oracle.
//! - [replace::expand] copies `$00` as text. C reads `parens[-1]` there and crashes.
//! - Reads past the end of a slice return NUL, as the C lexers read their terminator. The
//!   escape functions stop at the first NUL, as the C loops do.
//!
//! The oracle that checks these ports is `target/canon/vbscript/oracle.txt`, which is produced
//! from the verbatim C by the drivers in `target/canon/vbscript`.

pub mod array;
pub mod color;
pub mod escape;
pub mod literal;
pub mod radix;
pub mod replace;
pub mod rnd;
pub mod strings;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The VBScript errors that the ported functions return, identified by their `VBSE` code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VbsError {
    /// `VBSE_ILLEGAL_FUNC_CALL`: an argument is out of range.
    IllegalFuncCall,
    /// `VBSE_SYNTAX_ERROR`: a literal is malformed.
    SyntaxError,
}

impl VbsError {
    /// The `VBSE` code, which is the low 16 bits of the HRESULT that C returns.
    pub fn code(self) -> u16 {
        match self {
            VbsError::IllegalFuncCall => 5,
            VbsError::SyntaxError => 1002,
        }
    }
}

/// The unit at `i`, or NUL past the end. The C code reads its terminator this way.
pub(crate) fn unit_at(src: &[u16], i: usize) -> u16 {
    src.get(i).copied().unwrap_or(0)
}

/// The value of a hex digit in either case, or `None`. This is `hex_to_int` in `lex.c` and
/// `global.c`.
pub(crate) fn hex_value(c: u16) -> Option<u32> {
    match c {
        0x30..=0x39 => Some(u32::from(c - 0x30)),
        0x41..=0x46 => Some(u32::from(c - 0x41 + 10)),
        0x61..=0x66 => Some(u32::from(c - 0x61 + 10)),
        _ => None,
    }
}
