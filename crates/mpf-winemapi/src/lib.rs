//! Headless subset of Wine's `dlls/winemapi` (the MAPI provider).
//!
//! The ported functions compute what the C bodies decide from their arguments:
//! the session handle, the name-resolution result, the `xdg-email` argument
//! list and the `mailto:` string for the `winebrowser` fallback. Byte slices
//! stand for C strings and must not contain an interior NUL. The process heap,
//! process spawning, the shell and the Win32 escaping calls are not modelled;
//! the caller performs those steps.
//!
//! Ported:
//! - `MAPILogon`, `MAPILogoff`: `logon`, `logoff`.
//! - `MAPIResolveName`: `resolve_name`.
//! - `MAPISendMail` argument checks: `send_mail_argv`.
//! - `send_mail_xdg` argument list: `xdg_argv`, built on `add_target`, `is_non_empty` and `strip_smtp_prefix`.
//! - `send_mail_winebrowser` `mailto:` string: `mailto_url`. The caller passes the escaped subject and body.
//!
//! Skipped:
//! - `MAPIAddress`, `MAPIDeleteMail`, `MAPIDetails`, `MAPIFindNext`, `MAPIReadMail`, `MAPISaveMail`, `MAPISendDocuments`: `FIXME` stubs that return `MAPI_E_NOT_SUPPORTED`.
//! - Allocation failure paths (`MAPI_E_INSUFFICIENT_MEMORY` and the heap failures in `send_mail_winebrowser`): the process heap is not modelled.
//! - `add_file`, `strdupAtoW`: code page conversion and `wine_get_unix_file_name`. The caller passes unix paths.
//! - `__wine_unix_spawnvp`, `ShellExecuteA`: running `xdg-email` and `winebrowser.exe`.
//! - `escape_string` and the space escape of the `mailto:` string: `shlwapi` escaping.
//! - `TRACE` and `FIXME` output.

/// `SUCCESS_SUCCESS`: the MAPI call succeeded.
pub const SUCCESS_SUCCESS: u32 = 0;
/// `MAPI_E_FAILURE`: the MAPI call failed.
pub const MAPI_E_FAILURE: u32 = 2;
/// `MAPI_TO`: a primary recipient.
pub const MAPI_TO: u32 = 1;
/// `MAPI_CC`: a carbon-copy recipient.
pub const MAPI_CC: u32 = 2;
/// `MAPI_BCC`: a blind carbon-copy recipient.
pub const MAPI_BCC: u32 = 3;

/// One `MapiRecipDesc`. `address` is `None` where the C `lpszAddress` is NULL.
pub struct Recipient<'a> {
    /// `ulRecipClass`.
    pub class: u32,
    /// `lpszAddress`.
    pub address: Option<&'a [u8]>,
}

/// The parts of a `MapiMessage` that the send path reads.
pub struct Message<'a> {
    /// `nRecipCount`.
    pub recip_count: u32,
    /// `lpRecips`; `None` is the C NULL pointer.
    pub recips: Option<&'a [Recipient<'a>]>,
    /// The `lpszPathName` of each file, already resolved to a unix path by the caller. `None` entries are skipped.
    pub files: &'a [Option<&'a [u8]>],
    /// `lpszSubject`.
    pub subject: Option<&'a [u8]>,
    /// `lpszNoteText`.
    pub note_text: Option<&'a [u8]>,
}

impl<'a> Message<'a> {
    /// The first `recip_count` recipients, as the C loop reads them. A count larger than the list is clamped to the list.
    pub fn recipients(&self) -> &'a [Recipient<'a>] {
        let all = self.recips.unwrap_or(&[]);
        &all[..(self.recip_count as usize).min(all.len())]
    }
}

mod sendmail;
mod session;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use sendmail::{
    add_target, is_c_space, is_non_empty, mailto_url, send_mail_argv, strip_smtp_prefix, xdg_argv,
};
pub use session::{logoff, logon, resolve_name, ResolvedRecipient};
