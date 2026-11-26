//! `SspiUnmarshalAuthIdentity`: reads the marshalled layout back into an identity.

use crate::identity::{
    char_size, AuthIdentity, Credentials, AUTH_IDENTITY_MARSHALLED, AUTH_IDENTITY_VERSION,
    AUTH_IDENTITY_VERSION_2, EX2_CB_OFFSET, EX2_LEN, SEC_E_INVALID_TOKEN,
};
use crate::marshal::{Header, HEADER_LEN};
use crate::read_u32;

/// `SspiUnmarshalAuthIdentity`. Input shorter than a header, an unknown version, and any field
/// that reaches outside the input are rejected with `SEC_E_INVALID_TOKEN`.
pub fn unmarshal_auth_identity(bytes: &[u8]) -> Result<AuthIdentity<Vec<u8>>, u32> {
    if bytes.len() < HEADER_LEN {
        return Err(SEC_E_INVALID_TOKEN);
    }
    let header = Header::read(bytes);
    match header.version {
        AUTH_IDENTITY_VERSION => unmarshal_exw(bytes, &header),
        AUTH_IDENTITY_VERSION_2 => unmarshal_ex2(bytes),
        _ => Err(SEC_E_INVALID_TOKEN),
    }
}

fn unmarshal_exw(bytes: &[u8], h: &Header) -> Result<AuthIdentity<Vec<u8>>, u32> {
    if h.length != HEADER_LEN as u32 {
        return Err(SEC_E_INVALID_TOKEN);
    }
    let cs = char_size(h.flags);
    let creds = Credentials {
        user: field(bytes, h.user_off, h.user_len, cs)?,
        user_length: h.user_len,
        domain: field(bytes, h.domain_off, h.domain_len, cs)?,
        domain_length: h.domain_len,
        password: field(bytes, h.password_off, h.password_len, cs)?,
        password_length: h.password_len,
        flags: h.flags | AUTH_IDENTITY_MARSHALLED,
    };
    Ok(AuthIdentity::Exw {
        creds,
        package_list: field(bytes, h.package_list_off, h.package_list_len, cs)?,
        package_list_length: h.package_list_len,
    })
}

/// One marshalled field. Offset 0 is NULL. Any other offset must start inside `bytes`.
fn field(bytes: &[u8], off: u32, len: u32, cs: u32) -> Result<Option<Vec<u8>>, u32> {
    if off == 0 {
        return Ok(None);
    }
    let n = len.wrapping_mul(cs) as usize;
    let data = take(bytes, off as usize, n).ok_or(SEC_E_INVALID_TOKEN)?;
    Ok(Some(data.to_vec()))
}

/// `bytes[off..off + n]`, or `None` if that range leaves `bytes`. An empty range is always valid,
/// as a zero-length `memcpy` is in C.
fn take(bytes: &[u8], off: usize, n: usize) -> Option<&[u8]> {
    if n == 0 {
        return Some(&[]);
    }
    bytes.get(off..)?.get(..n)
}

/// EX2 copies its first `cbStructureLength` bytes. Wine copies into a `sizeof(EX2)` allocation, so a
/// longer structure overflows there (row `u8` in the oracle). This port keeps all `cbStructureLength`
/// bytes. The output is at least `EX2_LEN` bytes and is zero-filled past the copied bytes.
fn unmarshal_ex2(bytes: &[u8]) -> Result<AuthIdentity<Vec<u8>>, u32> {
    let cb = read_u32(bytes, EX2_CB_OFFSET) as usize;
    if cb > bytes.len() {
        return Err(SEC_E_INVALID_TOKEN);
    }
    let mut out = vec![0u8; EX2_LEN.max(cb)];
    out[..cb].copy_from_slice(&bytes[..cb]);
    Ok(AuthIdentity::Ex2(out))
}
