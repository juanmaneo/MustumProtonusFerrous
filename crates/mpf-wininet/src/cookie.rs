//! Cookie domain legality from Wine's `dlls/wininet/cookie.c` (`is_domain_legal_for_cookie`).

use crate::wide::{find, nicmp_eq};

/// Why a cookie domain is not legal. The C code reports the first two as `ERROR_INVALID_NAME` and
/// `ERROR_INVALID_PARAMETER`, and returns FALSE with no error for the third.
#[derive(Debug, PartialEq)]
pub enum CookieDomainError {
    InvalidName,
    InvalidParameter,
    Rejected,
}

/// Whether the cookie `domain` may be set from the host `full`. Both must have a dot, and the
/// domain must be a case-insensitive suffix of the host that starts at a label boundary.
pub fn is_domain_legal(domain: &[u16], full: &[u16]) -> Result<(), CookieDomainError> {
    if domain.is_empty()
        || domain[0] == u16::from(b'.')
        || full.is_empty()
        || full[0] == u16::from(b'.')
    {
        return Err(CookieDomainError::InvalidName);
    }
    if domain.len() > full.len() || find(domain, b'.').is_none() || find(full, b'.').is_none() {
        return Err(CookieDomainError::Rejected);
    }
    let start = full.len() - domain.len();
    if !nicmp_eq(domain, &full[start..])
        || (full.len() > domain.len() && full[start - 1] != u16::from(b'.'))
    {
        return Err(CookieDomainError::InvalidParameter);
    }
    Ok(())
}
