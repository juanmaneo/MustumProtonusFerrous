//! Name and blob composition of `SspiPrepareForCredWrite`.

use crate::{nul_terminated, wide_bytes};

/// `CRED_TYPE_DOMAIN_PASSWORD`.
pub const CRED_TYPE_DOMAIN_PASSWORD: u32 = 2;

/// Outputs of `SspiPrepareForCredWrite`. The strings have no NUL terminator here.
pub struct CredWrite {
    pub cred_type: u32,
    pub username: Vec<u16>,
    pub target_name: Vec<u16>,
    pub blob: Vec<u8>,
}

/// `SspiPrepareForCredWrite` after `SspiEncodeAuthIdentityAsStrings`. Wine dereferences `user` and
/// `password` unconditionally, so they are not optional here. A zero-length `domain` is absent, as
/// in `dup_auth_str`. Every string is cut at its first NUL.
pub fn prepare_for_cred_write(
    user: &[u16],
    domain: Option<&[u16]>,
    target: Option<&[u16]>,
    password: &[u16],
) -> CredWrite {
    let user = nul_terminated(user);
    let username = match domain.filter(|d| !d.is_empty()) {
        Some(domain) => {
            let mut name = nul_terminated(domain).to_vec();
            name.push(u16::from(b'\\'));
            name.extend_from_slice(user);
            name
        }
        None => user.to_vec(),
    };
    let target_name = match target {
        Some(target) => nul_terminated(target).to_vec(),
        None => username.clone(),
    };
    CredWrite {
        cred_type: CRED_TYPE_DOMAIN_PASSWORD,
        username,
        target_name,
        blob: wide_bytes(nul_terminated(password)),
    }
}
