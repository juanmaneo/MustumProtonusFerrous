//! Port of `get_machine_wow64_dir` in Wine's `dlls/wow64/wow64_private.h` at commit
//! `e2aa6cd75ae60acd90ff384feee171ce994f0f6d`.
//!
//! The C helper is a pure `switch` over a `USHORT` machine type that returns one of
//! three NT paths or `NULL`. The port takes the machine type as a `u16` and returns
//! the same path as `&str`. Every path is ASCII, so its UTF-16 code units match the
//! C `WCHAR` literals.
//!
//! Ported (1 function):
//! - `get_machine_wow64_dir`: [`get_machine_wow64_dir`].
//!
//! Skipped: the rest of `wow64_private.h`, including the thunk pointers, the TEB and
//! argument decoders, and the 32-to-64 structure converters. They need the thunk
//! state, the process TEB or the memory of the caller. `file.c`'s `starts_with_path`
//! and `get_file_redirect` need the object-attribute structures and the file-ID cache.
//!
//! Divergences from C:
//! - A machine type that C's `switch` does not handle returns `None`, as `NULL` does in C.
//! - The return type is `&'static str`, not `const WCHAR *`. The strings have the same content.

/// `IMAGE_FILE_MACHINE_TARGET_HOST` from Wine's `include/winnt.h`.
pub const IMAGE_FILE_MACHINE_TARGET_HOST: u16 = 0x0001;
/// `IMAGE_FILE_MACHINE_I386` from Wine's `include/winnt.h`.
pub const IMAGE_FILE_MACHINE_I386: u16 = 0x014c;
/// `IMAGE_FILE_MACHINE_ARMNT` from Wine's `include/winnt.h`.
pub const IMAGE_FILE_MACHINE_ARMNT: u16 = 0x01c4;

/// Returns the WoW64 system directory for a machine type, or `None` if the type has none.
///
/// Matches `get_machine_wow64_dir` in Wine. `IMAGE_FILE_MACHINE_TARGET_HOST` maps to
/// `system32`, `IMAGE_FILE_MACHINE_I386` to `syswow64` and `IMAGE_FILE_MACHINE_ARMNT`
/// to `sysarm32`.
pub fn get_machine_wow64_dir(machine: u16) -> Option<&'static str> {
    match machine {
        IMAGE_FILE_MACHINE_TARGET_HOST => Some("\\??\\C:\\windows\\system32"),
        IMAGE_FILE_MACHINE_I386 => Some("\\??\\C:\\windows\\syswow64"),
        IMAGE_FILE_MACHINE_ARMNT => Some("\\??\\C:\\windows\\sysarm32"),
        _ => None,
    }
}

#[cfg(test)]
mod edge_checks;

#[cfg(test)]
mod oracle_checks;
