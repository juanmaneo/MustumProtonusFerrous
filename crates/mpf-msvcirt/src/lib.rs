//! Partial `msvcirt` iostream logic based on Wine's `dlls/msvcirt/msvcirt.c`: the
//! integer extraction and insertion paths, the field padding they share, and the
//! `ios` state queries.
//!
//! Ported:
//! - `istream::getint`, the integer scanner: [`scan::scan_integer`].
//! - `istream::internal_read_integer` and `internal_read_unsigned_integer`, which
//!   parse the scanned token with a 32-bit `strtol`/`strtoul` and clamp it to the
//!   target range: [`read::read_integer`] and [`read::read_unsigned_integer`].
//! - `ostream::writepad_len`, the field layout: [`pad::writepad`].
//! - `ostream::internal_print_integer`, the `printf` conversion and prefix:
//!   [`print::format_integer`] and [`print::print_integer`].
//! - `ios::good`, `fail`, `eof` and `bad`: [`good`], [`fail`], [`eof`] and [`bad`].
//!
//! Not ported:
//! - `istream::getdouble` and so `read_float`. It puts back `str[i--]`, the slot just
//!   past the last character it stored, so the byte it rewinds comes from memory the
//!   function never wrote.
//! - `ostream::print_float`. It formats with `%g`, `%e` and `%f` through `_scprintf`.
//! - Stream plumbing: `streambuf` buffers, `ipfx`/`isfx`, `opfx`/`osfx`, tie flushing,
//!   `skipws` whitespace skipping, and locks. The caller owns the stream. The scanner
//!   never sets badbit; Wine sets it when a putback fails, so a caller whose stream
//!   cannot rewind the extracted characters must set it itself.
//! - Stream accessors (`setf`, `width`, `fill`, `precision`) and the `xalloc` and
//!   `bitalloc` counters. The caller resets the field width to 0 after an insertion,
//!   which `osfx` does in Wine.
//!
//! Integers use the Windows ABI widths: `LONG` and `ULONG` are 32-bit, and state and
//! flags are the bit sets of `msvcirt.h`.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub mod pad;
pub mod print;
pub mod read;
pub mod scan;

/// `FLAGS_left`.
pub const FLAGS_LEFT: u32 = 0x2;
/// `FLAGS_internal`.
pub const FLAGS_INTERNAL: u32 = 0x8;
/// `FLAGS_dec`.
pub const FLAGS_DEC: u32 = 0x10;
/// `FLAGS_oct`.
pub const FLAGS_OCT: u32 = 0x20;
/// `FLAGS_hex`.
pub const FLAGS_HEX: u32 = 0x40;
/// `FLAGS_showbase`.
pub const FLAGS_SHOWBASE: u32 = 0x80;
/// `FLAGS_uppercase`.
pub const FLAGS_UPPERCASE: u32 = 0x200;
/// `FLAGS_showpos`.
pub const FLAGS_SHOWPOS: u32 = 0x400;
/// `ios_basefield`: the radix flags.
pub const IOS_BASEFIELD: u32 = FLAGS_DEC | FLAGS_OCT | FLAGS_HEX;

/// `IOSTATE_eofbit`.
pub const IOSTATE_EOFBIT: u32 = 0x1;
/// `IOSTATE_failbit`.
pub const IOSTATE_FAILBIT: u32 = 0x2;
/// `IOSTATE_badbit`.
pub const IOSTATE_BADBIT: u32 = 0x4;

/// `ios::good`: no state bit is set.
pub fn good(state: u32) -> bool {
    state == 0
}

/// `ios::fail`: the failbit or the badbit is set.
pub fn fail(state: u32) -> bool {
    state & (IOSTATE_FAILBIT | IOSTATE_BADBIT) != 0
}

/// `ios::eof`: the eofbit is set.
pub fn eof(state: u32) -> bool {
    state & IOSTATE_EOFBIT != 0
}

/// `ios::bad`: the badbit is set.
pub fn bad(state: u32) -> bool {
    state & IOSTATE_BADBIT != 0
}
