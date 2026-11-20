//! The index mappings used to store BITS credentials and send them through WinHTTP.

/// WinHTTP auth targets. These are not the one-based `BG_AUTH_TARGET` values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthTarget {
    Server = 0,
    Proxy = 1,
}

/// WinHTTP auth scheme bit values.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthScheme {
    Basic = 0x01,
    Ntlm = 0x02,
    Passport = 0x04,
    Digest = 0x08,
    Negotiate = 0x10,
}

/// Convert the credential target-array index to a WinHTTP target.
pub fn target_from_index(index: u32) -> Option<AuthTarget> {
    match index {
        0 => Some(AuthTarget::Server),
        1 => Some(AuthTarget::Proxy),
        _ => None,
    }
}

/// Convert the credential scheme-array index to a WinHTTP scheme.
///
/// This order is the order used in `file.c`; it differs from the BITS enum order used by
/// [`index_from_scheme`].
pub fn scheme_from_index(index: u32) -> Option<AuthScheme> {
    match index {
        0 => Some(AuthScheme::Basic),
        1 => Some(AuthScheme::Ntlm),
        2 => Some(AuthScheme::Passport),
        3 => Some(AuthScheme::Digest),
        4 => Some(AuthScheme::Negotiate),
        _ => None,
    }
}

/// Convert a one-based `BG_AUTH_TARGET` value to the credential-array index.
pub fn index_from_target(target: u32) -> Option<u32> {
    match target {
        1 | 2 => Some(target - 1),
        _ => None,
    }
}

/// Convert a one-based `BG_AUTH_SCHEME` value to the credential-array index.
///
/// BITS enum order is BASIC, DIGEST, NTLM, NEGOTIATE, PASSPORT.
pub fn index_from_scheme(scheme: u32) -> Option<u32> {
    match scheme {
        1..=5 => Some(scheme - 1),
        _ => None,
    }
}
