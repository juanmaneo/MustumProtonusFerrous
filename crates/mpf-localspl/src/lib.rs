//! Port of the portable logic in Wine's `dlls/localspl/localmon.c` and `provider.c`.
//!
//! Ported:
//! - Name-prefix rules of `get_type_from_name` (`check_filename` FALSE): [`classify_port_name`], [`PortType`]
//! - `multi_sz_lenW`: [`multi_sz_len_bytes`]
//! - `copy_servername_from_name`: [`copy_servername_from_name`], [`ServerName`]
//! - `get_basename_from_name`: [`get_basename_from_name`]
//! - `get_file_part`: [`get_file_part`]
//! - `validate_envW` table lookup: [`validate_env`], [`PrintEnv`]
//!
//! Skipped:
//! - The file probe in `get_type_from_name` (`CreateFileW`) and `get_type_from_local_name` (registry).
//! - `localmon_*` and `fp*` entry points: spooler, port and registry plumbing.
//! - `cups.c`: CUPS and socket I/O.
//! - `get_spool_filename`: depends on the Win32 system directory.
//! - Form and job buffer layout (`form_struct_size`, `fill_builtin_form_info`, `get_job`).
//! - Injected instead of called: `GetComputerNameW`, `GetVersion`, and the build-time `env_arch`.
//! - Case-insensitive comparisons (`wcsnicmp`, `lstrcmpiW`) fold ASCII only; Wine's `lstrcmpiW` uses `CompareStringW` with `NORM_IGNORECASE`.

mod env;
mod multi_sz;
mod server_name;

pub use env::{validate_env, PrintEnv};
pub use multi_sz::multi_sz_len_bytes;
pub use server_name::{
    copy_servername_from_name, get_basename_from_name, get_file_part, ServerName,
};

/// Path separator, `'\\'` in the C source.
const SEP: u16 = '\\' as u16;

/// Port types from `localspl_private.h`. `Unixname` shares the value of `PORT_IS_WINE`.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortType {
    Lpt = 1,
    Com = 2,
    File = 3,
    Unixname = 5,
    Pipe = 6,
    Cups = 7,
    Lpr = 8,
}

/// The name-prefix rules of `get_type_from_name` with `check_filename` FALSE.
/// `None` means no rule matched: the C code then returns `PORT_IS_UNKNOWN`
/// (or probes the file system when `check_filename` is TRUE).
pub fn classify_port_name(name: &[u16]) -> Option<PortType> {
    if starts_with_ascii(name, "LPT") {
        Some(PortType::Lpt)
    } else if starts_with_ascii(name, "COM") {
        Some(PortType::Com)
    } else if eq_ascii(name, "FILE:") {
        Some(PortType::File)
    } else if name.first() == Some(&u16::from(b'/')) {
        Some(PortType::Unixname)
    } else if name.first() == Some(&u16::from(b'|')) {
        Some(PortType::Pipe)
    } else if starts_with_ascii(name, "CUPS:") {
        Some(PortType::Cups)
    } else if starts_with_ascii(name, "LPR:") {
        Some(PortType::Lpr)
    } else {
        None
    }
}

fn starts_with_ascii(name: &[u16], prefix: &str) -> bool {
    name.len() >= prefix.len()
        && name
            .iter()
            .zip(prefix.bytes())
            .all(|(&c, b)| c == u16::from(b))
}

fn eq_ascii(name: &[u16], s: &str) -> bool {
    name.len() == s.len() && starts_with_ascii(name, s)
}

/// `lstrcmpiW` against an ASCII string.
fn eq_ascii_ignore_case(name: &[u16], s: &str) -> bool {
    name.len() == s.len()
        && name
            .iter()
            .zip(s.bytes())
            .all(|(&c, b)| fold_ascii(c) == fold_ascii(u16::from(b)))
}

/// ASCII upper-casing shared by `wcsnicmp` and `lstrcmpiW`.
fn fold_ascii(c: u16) -> u16 {
    if (u16::from(b'a')..=u16::from(b'z')).contains(&c) {
        c - 32
    } else {
        c
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
