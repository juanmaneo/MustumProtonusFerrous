//! `MAPILogon`, `MAPILogoff` and `MAPIResolveName` from `main.c`.

use crate::{MAPI_E_FAILURE, MAPI_TO, SUCCESS_SUCCESS};

/// `MAPILogon`: stores the session handle `1` when the caller asks for one. The profile and password are not read.
pub fn logon(session: Option<&mut u32>) -> u32 {
    if let Some(session) = session {
        *session = 1;
    }
    SUCCESS_SUCCESS
}

/// `MAPILogoff`: there is no session state to release.
pub fn logoff() -> u32 {
    SUCCESS_SUCCESS
}

/// The recipient that `MAPIResolveName` allocates.
pub struct ResolvedRecipient {
    /// `ulRecipClass`, always `MAPI_TO`.
    pub class: u32,
    /// `lpszName`: the name as given.
    pub name: Vec<u8>,
    /// `lpszAddress`: `SMTP:` followed by the name.
    pub address: Vec<u8>,
}

/// `MAPIResolveName`: a missing or empty name fails; any other name becomes an SMTP address.
pub fn resolve_name(name: Option<&[u8]>) -> Result<ResolvedRecipient, u32> {
    let name = match name {
        Some(name) if !name.is_empty() => name,
        _ => return Err(MAPI_E_FAILURE),
    };
    let mut address = b"SMTP:".to_vec();
    address.extend_from_slice(name);
    Ok(ResolvedRecipient {
        class: MAPI_TO,
        name: name.to_vec(),
        address,
    })
}
