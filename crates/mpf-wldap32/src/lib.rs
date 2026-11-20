//! wldap32 subset for MPF: LDAP result mapping, filter escaping and host URL helpers.
//!
//! Ported from `dlls/wldap32`. Of the 156 exports in `wldap32.spec`, these deterministic parts
//! are ported:
//! - `LdapMapErrorToWin32` as [`ldap_map_error_to_win32`], which reads the `errormap` table of
//!   `error.c`. The OpenLDAP-to-WLDAP32 switch of `error.c` is [`map_error`].
//! - `ldap_escape_filter_elementA` as [`escape_filter_element`], and its size query as
//!   [`escape_filter_element_size`]. The spec's `ldap_escape_filter_element` is an alias of it.
//! - `split_hostnames`, `has_ldap_scheme`, `join_hostnames` and `urlify_hostnames` from `init.c`
//!   as [`split_hostnames`], [`has_ldap_scheme`], [`join_hostnames`] and [`urlify_hostnames`].
//!   These are static helpers that `ldap_init` and `ldap_open` use to build the server URL list.
//!
//! Skipped:
//! - Stub exports, which Wine marks FIXME.
//! - `ldap_escape_filter_elementW`: its write path always returns `WLDAP32_LDAP_PARAM_ERROR`, so
//!   the size query is the only part with behaviour, and it is [`escape_filter_element_size`].
//! - `ldap_err2string` and its variants: they load text from the DLL's resources.
//! - `ldap_dn2ufn` and `ldap_explode_dn`: they forward to the Unix-side libldap.
//! - `ldap_ufn2dn`: Wine copies its input unchanged (`dn.c`, marked FIXME).
//! - BER, paging, control, option, parse and pointer-array wrappers, and all connection,
//!   bind and search calls: they need LDAP handles or a server.
//!
//! Deviations from Wine, where Wine has undefined behaviour:
//! - [`split_hostnames`] ends a trailing-whitespace list's last name before the whitespace.
//! - [`join_hostnames`] returns an empty list for zero hosts, not an unterminated buffer.
//! - [`join_hostnames`] prints ports of 100000 and above in full. Wine overflows its 7-byte buffer.

mod error;
mod filter;
mod hosts;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

use crate::error::PARAM_ERROR;

pub use error::{ldap_map_error_to_win32, map_error};
pub use filter::{escape_filter_element, escape_filter_element_size};
pub use hosts::{has_ldap_scheme, join_hostnames, split_hostnames, urlify_hostnames};

/// Failure modes of [`escape_filter_element`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeError {
    /// The source is NULL, or the destination is shorter than the escaped size.
    ParamError,
}

impl EscapeError {
    /// The WLDAP32 result code the C entry point returns for this failure.
    pub fn code(self) -> u32 {
        match self {
            Self::ParamError => PARAM_ERROR,
        }
    }
}
