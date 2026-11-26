//! Auth identity types and `SspiEncodeStringsAsAuthIdentity`.

use crate::{nul_terminated, wide_bytes};

pub const SEC_E_INTERNAL_ERROR: u32 = 0x8009_0304;
pub const SEC_E_INVALID_TOKEN: u32 = 0x8009_0308;

pub const AUTH_IDENTITY_VERSION: u32 = 0x200;
pub const AUTH_IDENTITY_VERSION_2: u32 = 0x201;
pub const AUTH_IDENTITY_ANSI: u32 = 0x1;
pub const AUTH_IDENTITY_UNICODE: u32 = 0x2;
pub const AUTH_IDENTITY_MARSHALLED: u32 = 0x4;

/// `sizeof(SEC_WINNT_AUTH_IDENTITY_EX2)`, the allocation Wine's unmarshal makes.
pub(crate) const EX2_LEN: usize = 48;
/// Offset of `cbStructureLength` in `SEC_WINNT_AUTH_IDENTITY_EX2`.
pub(crate) const EX2_CB_OFFSET: usize = 8;

/// String fields of an identity. Lengths count characters; `char_size(flags)` gives the bytes per character.
#[derive(PartialEq, Eq)]
pub struct Credentials<B> {
    pub user: Option<B>,
    pub user_length: u32,
    pub domain: Option<B>,
    pub domain_length: u32,
    pub password: Option<B>,
    pub password_length: u32,
    pub flags: u32,
}

/// An auth identity. C tells the structs apart by the version or by a pointer in the first ULONG,
/// so the variant replaces that dispatch.
#[derive(PartialEq, Eq)]
pub enum AuthIdentity<B> {
    /// `SEC_WINNT_AUTH_IDENTITY_W`, which has no package list.
    Wide(Credentials<B>),
    /// `SEC_WINNT_AUTH_IDENTITY_EXW` (version 0x200).
    Exw {
        creds: Credentials<B>,
        package_list: Option<B>,
        package_list_length: u32,
    },
    /// `SEC_WINNT_AUTH_IDENTITY_EX2` (version 0x201), as its raw bytes.
    Ex2(B),
    /// Any other version. Marshalling rejects it.
    Other,
}

/// Bytes per character: 1 for ANSI, 2 otherwise.
pub(crate) fn char_size(flags: u32) -> u32 {
    if flags & AUTH_IDENTITY_ANSI != 0 {
        1
    } else {
        2
    }
}

/// `SspiEncodeStringsAsAuthIdentity`. Each string is cut at its first NUL, as `lstrlenW` does.
/// `Some(&[])` is kept as a present, empty field. The result is always UNICODE and MARSHALLED.
pub fn encode_strings_as_auth_identity(
    username: Option<&[u16]>,
    domainname: Option<&[u16]>,
    creds: Option<&[u16]>,
) -> Result<AuthIdentity<Vec<u8>>, u32> {
    if [username, domainname, creds].iter().all(Option::is_none) {
        return Err(SEC_E_INVALID_TOKEN);
    }
    let (user, user_length) = encode_field(username);
    let (domain, domain_length) = encode_field(domainname);
    let (password, password_length) = encode_field(creds);
    Ok(AuthIdentity::Exw {
        creds: Credentials {
            user,
            user_length,
            domain,
            domain_length,
            password,
            password_length,
            flags: AUTH_IDENTITY_UNICODE | AUTH_IDENTITY_MARSHALLED,
        },
        package_list: None,
        package_list_length: 0,
    })
}

fn encode_field(s: Option<&[u16]>) -> (Option<Vec<u8>>, u32) {
    match s {
        Some(s) => {
            let s = nul_terminated(s);
            (Some(wide_bytes(s)), s.len() as u32)
        }
        None => (None, 0),
    }
}
