//! Headless port of the deterministic parts of Wine's `dlls/shcore/main.c`.
//!
//! The C reference is the shell core library. Its pure parts run here on Rust values: a command
//! line becomes argument vectors, strings are copied into bounded buffers, a memory stream is a
//! byte vector with a 32-bit position, and a registry integer is parsed from its UTF-16 text.
//! Each function names the C function it follows.
//!
//! Ported (names in `dlls/shcore/main.c`):
//! - `CommandLineToArgvW`: [command_line_to_argv].
//! - `SHAnsiToAnsi` and `SHUnicodeToUnicode`: [sh_ansi_to_ansi], [sh_unicode_to_unicode].
//! - `SHCreateMemStream` and the `memstream_*` methods: [MemStream].
//! - The mode checks of `SHCreateStreamOnFileEx` and `SHCreateStreamOnFileW`: [stream_on_file_ex_flags],
//!   [stream_on_file_w_flags].
//! - `IsOS`: [is_os].
//! - The integer parse of `SHRegGetIntW`: [reg_int_value].
//!
//! Skipped: `DllMain`; the `FIXME` stubs; the file and registry stream methods; the registry
//! functions; the COM, thread and feature-notification exports; the `SHUnicodeToAnsi` and
//! `SHAnsiToUnicode` pair, which convert through the code page; and every call that needs the OS.
//! The caller makes those calls and passes the results in.
//!
//! Deviations from Wine, where Wine has undefined behaviour or depends on the C runtime:
//! - `SHCreateMemStream` with no source makes an empty stream. Wine calls `memcpy` from a NULL
//!   pointer with length 0, which the sanitizers reject, so the oracle leaves it out.
//! - `SetSize(0)` returns `STG_E_INSUFFICIENTMEMORY` and keeps the stream unchanged. Wine's msvcrt
//!   `realloc` frees the buffer for a zero size and returns NULL, and the stream then keeps a
//!   freed pointer.
//! - Growing a stream fills the new bytes with zeros. Wine's `_recalloc` fills from the old block
//!   size. The two agree when that size equals the stream length, as the oracle assumes.
//! - A `Seek` whose target overflows `i64` returns `STG_E_INVALIDFUNCTION`. Wine's signed overflow is
//!   undefined. Wrapping arithmetic on the usual targets gives the same error.
//! - A source longer than `u32::MAX` bytes is cut to `u32::MAX`. The C `UINT` length cannot
//!   describe more.
//! - `SHAnsiToAnsi` and `SHUnicodeToUnicode` return 0 for a source with no NUL in its slice. Wine
//!   would read past the end.
//! - `SHRegGetIntW` takes the value after `REG_EXPAND_SZ` expansion. The caller expands it, because
//!   Wine expands through `ExpandEnvironmentStringsW`.
//! - `CommandLineToArgvW` takes the module path from the caller. Wine's two-pass allocation becomes
//!   one pass with the same result.

mod command_line;
mod mem_stream;
mod os_version;
mod reg_value;
mod stream_open;
mod string_copy;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use command_line::command_line_to_argv;
pub use mem_stream::{MemStream, Stat, STREAM_SEEK_CUR, STREAM_SEEK_END, STREAM_SEEK_SET};
pub use os_version::{
    is_os, OsVersion, VER_PLATFORM_WIN32S, VER_PLATFORM_WIN32_NT, VER_PLATFORM_WIN32_WINDOWS,
};
pub use reg_value::reg_int_value;
pub use stream_open::{stream_on_file_ex_flags, stream_on_file_w_flags, OpenFlags};
pub use string_copy::{sh_ansi_to_ansi, sh_unicode_to_unicode};

/// An `HRESULT`.
pub type Hresult = i32;

/// `S_OK`.
pub const S_OK: Hresult = 0;
/// `S_FALSE`, returned by a read at the end of a stream.
pub const S_FALSE: Hresult = 1;
/// `E_INVALIDARG`.
pub const E_INVALIDARG: Hresult = 0x8007_0057u32 as i32;
/// `STG_E_INVALIDFUNCTION`, returned by a seek to a negative position.
pub const STG_E_INVALIDFUNCTION: Hresult = 0x8003_0001u32 as i32;
/// `STG_E_INSUFFICIENTMEMORY`, returned when a stream cannot grow.
pub const STG_E_INSUFFICIENTMEMORY: Hresult = 0x8003_0008u32 as i32;
/// `STG_E_INVALIDPARAMETER`, returned for an unknown seek origin.
pub const STG_E_INVALIDPARAMETER: Hresult = 0x8003_0057u32 as i32;

/// The length of a NUL-terminated string in `s`: the index of its first zero, or `s.len()`.
pub(crate) fn str_len<T>(s: &[T]) -> usize
where
    T: Copy + Default + PartialEq,
{
    let zero = T::default();
    s.iter().position(|c| *c == zero).unwrap_or(s.len())
}
