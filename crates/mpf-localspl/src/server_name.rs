//! UNC name parsing from `provider.c`: `copy_servername_from_name`, `get_basename_from_name`
//! and `get_file_part`.

use crate::{fold_ascii, SEP};

/// `MAX_COMPUTERNAME_LENGTH`.
const MAX_COMPUTERNAME_LENGTH: usize = 15;

/// Result of [`copy_servername_from_name`]: the C return value and the server name
/// the C code copies into `target`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServerName<'a> {
    /// Not a UNC name, or an empty server part. C returns 0 and leaves `target` empty.
    Empty,
    /// Server part longer than `MAX_COMPUTERNAME_LENGTH`. C returns `-len` and leaves `target` empty.
    TooLong(usize),
    /// The local computer. C copies the server and returns 0.
    Local(&'a [u16]),
    /// A remote server. C copies the server and returns its length.
    Remote(&'a [u16]),
}

/// `copy_servername_from_name`. `computer_name` is what `GetComputerNameW` returns,
/// or `None` when that call fails.
pub fn copy_servername_from_name<'a>(
    name: Option<&'a [u16]>,
    computer_name: Option<&[u16]>,
) -> ServerName<'a> {
    let [SEP, SEP, rest @ ..] = name.unwrap_or_default() else {
        return ServerName::Empty;
    };
    let server_len = rest.iter().position(|&c| c == SEP).unwrap_or(rest.len());
    let server = &rest[..server_len];
    if server.is_empty() {
        ServerName::Empty
    } else if server_len > MAX_COMPUTERNAME_LENGTH {
        ServerName::TooLong(server_len)
    } else if is_local(server, computer_name) {
        ServerName::Local(server)
    } else {
        ServerName::Remote(server)
    }
}

/// The C code passes a 16-unit buffer to `GetComputerNameW`, so a longer name fails
/// and the server is treated as remote.
fn is_local(server: &[u16], computer_name: Option<&[u16]>) -> bool {
    computer_name
        .filter(|name| name.len() <= MAX_COMPUTERNAME_LENGTH)
        .is_some_and(|name| {
            name.len() == server.len()
                && server
                    .iter()
                    .zip(name)
                    .all(|(&a, &b)| fold_ascii(a) == fold_ascii(b))
        })
}

/// `get_basename_from_name`: the part after the server of a UNC name, or the name itself.
/// `None` for `NULL` and for a UNC name with no basename.
pub fn get_basename_from_name(name: Option<&[u16]>) -> Option<&[u16]> {
    let name = name?;
    let [SEP, SEP, rest @ ..] = name else {
        return Some(name);
    };
    let sep = rest.iter().position(|&c| c == SEP)?;
    let basename = &rest[sep + 1..];
    (!basename.is_empty()).then_some(basename)
}

/// `get_file_part`: the text after the last path separator, or the whole name.
pub fn get_file_part(name: &[u16]) -> &[u16] {
    match name.iter().rposition(|&c| c == SEP) {
        Some(pos) => &name[pos + 1..],
        None => name,
    }
}
