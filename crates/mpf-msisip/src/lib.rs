//! Port of the portable logic in Wine's `dlls/msisip/main.c`.
//!
//! Ported:
//! - `MsiSIPGetSignedDataMsg`: the DER header read, the size query, the buffer
//!   check and the copy, over a signature stream the caller has already read:
//!   [`get_signed_data_msg`]
//! - `MsiSIPIsMyTypeOfFile`: the MSI class-ID test and the subject it reports:
//!   [`msi_subject_for_clsid`], [`MY_SUBJECT`]
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer`: SIP provider registration
//!   through `CryptSIPAddProvider` and `CryptSIPRemoveProvider`.
//! - Storage and stream I/O in both exports (`StgOpenStorage`,
//!   `IStorage_OpenStream`, `IStream_Read`, `IStorage_Stat`). The caller supplies
//!   the stream bytes and the storage class.
//! - The `stub` exports `MsiSIPPutSignedDataMsg`, `MsiSIPRemoveSignedDataMsg`,
//!   `MsiSIPCreateIndirectData` and `MsiSIPVerifyIndirectData`.
//!
//! Deviations, only where Wine writes past the end of the caller's buffer
//! (undefined behaviour that safe Rust cannot reproduce):
//! - A wrapped message whose size fits the buffer but whose header does not
//!   returns `InsufficientBuffer`. Wine copies the header anyway.
//! - A wrapped message whose body exceeds the room left in the buffer copies
//!   only that room and returns `Invalid`. Wine writes the whole body.

mod der;
mod msi;

pub use msi::{
    msi_subject_for_clsid, Guid, CLSID_MSI_DATABASE, CLSID_MSI_PATCH, CLSID_MSI_TRANSFORM,
    MY_SUBJECT,
};

/// `X509_ASN_ENCODING | PKCS_7_ASN_ENCODING`, written to `*pdwEncodingType` on success.
pub const ENCODING_TYPE: u32 = 0x0000_0001 | 0x0001_0000;

/// Outcome of `MsiSIPGetSignedDataMsg` for one stream and one buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignedDataMsg {
    /// The stream does not start with a definite-length DER `SEQUENCE`, or it is
    /// truncated before the end of the header. The call returns `FALSE`.
    Invalid,
    /// `pbSignedDataMsg` is NULL. `*pcbSignedDataMsg` receives the message size.
    SizeQuery(u32),
    /// The buffer is too small. `ERROR_INSUFFICIENT_BUFFER` is set and
    /// `*pcbSignedDataMsg` receives the message size.
    InsufficientBuffer(u32),
    /// The message was copied. `*pdwEncodingType` and `*pcbSignedDataMsg` are set.
    Copied { size: u32 },
}

/// `MsiSIPGetSignedDataMsg` over a signature stream already read into `stream`.
///
/// `buffer` is `pbSignedDataMsg`. Its length is the input `*pcbSignedDataMsg`.
pub fn get_signed_data_msg(stream: &[u8], buffer: Option<&mut [u8]>) -> SignedDataMsg {
    let Some(header) = der::parse_der_header(stream) else {
        return SignedDataMsg::Invalid;
    };
    let size = header.message_len();
    let Some(buffer) = buffer else {
        return SignedDataMsg::SizeQuery(size);
    };
    let header_len = usize::from(header.header_len);
    if buffer.len() < size as usize || buffer.len() < header_len {
        return SignedDataMsg::InsufficientBuffer(size);
    }
    buffer[..header_len].copy_from_slice(&stream[..header_len]);
    let body = &stream[header_len..];
    let content_len = header.content_len as usize;
    let room = buffer.len() - header_len;
    let copied = body.len().min(content_len).min(room);
    buffer[header_len..header_len + copied].copy_from_slice(&body[..copied]);
    if copied == content_len {
        SignedDataMsg::Copied { size }
    } else {
        SignedDataMsg::Invalid
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
