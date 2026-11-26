//! `SspiMarshalAuthIdentity`: writes the `auth_identity_marshalled` layout.

use crate::identity::{
    char_size, AuthIdentity, Credentials, AUTH_IDENTITY_VERSION, EX2_CB_OFFSET,
    SEC_E_INTERNAL_ERROR,
};
use crate::read_u32;

/// `sizeof(struct auth_identity_marshalled)`.
pub(crate) const HEADER_LEN: usize = 44;
/// The trailing `DWORD` that Wine adds to every marshalled buffer.
const TRAILER_LEN: u32 = 4;

/// `struct auth_identity_marshalled`. Offsets are from the start of the buffer, and 0 means NULL.
pub(crate) struct Header {
    pub version: u32,
    pub length: u32,
    pub user_off: u32,
    pub user_len: u32,
    pub domain_off: u32,
    pub domain_len: u32,
    pub password_off: u32,
    pub password_len: u32,
    pub flags: u32,
    pub package_list_off: u32,
    pub package_list_len: u32,
}

impl Header {
    /// Reads the header from the first `HEADER_LEN` bytes. The caller checks the length.
    pub(crate) fn read(bytes: &[u8]) -> Header {
        let f = |i: usize| read_u32(bytes, 4 * i);
        Header {
            version: f(0),
            length: f(1),
            user_off: f(2),
            user_len: f(3),
            domain_off: f(4),
            domain_len: f(5),
            password_off: f(6),
            password_len: f(7),
            flags: f(8),
            package_list_off: f(9),
            package_list_len: f(10),
        }
    }

    /// Writes the header to the front of `out`. Returns `None` if `out` is shorter than the header.
    fn write(&self, out: &mut [u8]) -> Option<()> {
        let fields = [
            self.version,
            self.length,
            self.user_off,
            self.user_len,
            self.domain_off,
            self.domain_len,
            self.password_off,
            self.password_len,
            self.flags,
            self.package_list_off,
            self.package_list_len,
        ];
        let bytes: Vec<u8> = fields.iter().flat_map(|v| v.to_le_bytes()).collect();
        out.get_mut(..HEADER_LEN)?.copy_from_slice(&bytes);
        Some(())
    }
}

/// Copies one field to `*pos`, as `SspiMarshalAuthIdentity` does, and returns its offset.
/// A NULL field gets offset 0 and writes nothing. A non-NULL field with zero length still gets
/// a nonzero offset. The byte count is `len * cs` modulo 2^32, as in C.
fn put_field(
    out: &mut [u8],
    pos: &mut usize,
    src: Option<&[u8]>,
    len: u32,
    cs: u32,
) -> Result<u32, u32> {
    let Some(src) = src else {
        return Ok(0);
    };
    let n = len.wrapping_mul(cs) as usize;
    let dst = out.get_mut(*pos..*pos + n).ok_or(SEC_E_INTERNAL_ERROR)?;
    dst.copy_from_slice(src.get(..n).ok_or(SEC_E_INTERNAL_ERROR)?);
    let off = *pos as u32;
    *pos += n;
    Ok(off)
}

/// Builds the marshalled buffer. The size is `44 + 4 + units * cs`, where `units` is the sum of all
/// declared lengths (NULL fields included), all modulo 2^32, as in C. A copy that does not fit in
/// that buffer, or a source shorter than its declared length, is an error.
fn marshal_fields(
    creds: &Credentials<&[u8]>,
    package_list: Option<&[u8]>,
    package_list_length: u32,
) -> Result<Vec<u8>, u32> {
    let cs = char_size(creds.flags);
    let units = creds
        .user_length
        .wrapping_add(creds.domain_length)
        .wrapping_add(creds.password_length)
        .wrapping_add(package_list_length);
    let size = (HEADER_LEN as u32 + TRAILER_LEN).wrapping_add(units.wrapping_mul(cs));
    let mut out = vec![0u8; size as usize];
    let mut pos = HEADER_LEN;
    let user_off = put_field(&mut out, &mut pos, creds.user, creds.user_length, cs)?;
    let domain_off = put_field(&mut out, &mut pos, creds.domain, creds.domain_length, cs)?;
    let password_off = put_field(
        &mut out,
        &mut pos,
        creds.password,
        creds.password_length,
        cs,
    )?;
    let package_list_off = put_field(&mut out, &mut pos, package_list, package_list_length, cs)?;
    Header {
        version: AUTH_IDENTITY_VERSION,
        length: HEADER_LEN as u32,
        user_off,
        user_len: creds.user_length,
        domain_off,
        domain_len: creds.domain_length,
        password_off,
        password_len: creds.password_length,
        flags: creds.flags,
        package_list_off,
        package_list_len: package_list_length,
    }
    .write(&mut out)
    .ok_or(SEC_E_INTERNAL_ERROR)?;
    Ok(out)
}

/// EX2 is already a flat buffer, so the marshal copies its first `cbStructureLength` bytes.
fn copy_ex2(bytes: &[u8]) -> Result<Vec<u8>, u32> {
    if bytes.len() < EX2_CB_OFFSET + 4 {
        return Err(SEC_E_INTERNAL_ERROR);
    }
    let cb = read_u32(bytes, EX2_CB_OFFSET) as usize;
    bytes
        .get(..cb)
        .map(<[u8]>::to_vec)
        .ok_or(SEC_E_INTERNAL_ERROR)
}

/// `SspiMarshalAuthIdentity`. Inputs of an unknown version are rejected with `SEC_E_INTERNAL_ERROR`.
pub fn marshal_auth_identity(identity: &AuthIdentity<&[u8]>) -> Result<Vec<u8>, u32> {
    match identity {
        AuthIdentity::Wide(creds) => marshal_fields(creds, None, 0),
        AuthIdentity::Exw {
            creds,
            package_list,
            package_list_length,
        } => marshal_fields(creds, *package_list, *package_list_length),
        AuthIdentity::Ex2(bytes) => copy_ex2(bytes),
        AuthIdentity::Other => Err(SEC_E_INTERNAL_ERROR),
    }
}
