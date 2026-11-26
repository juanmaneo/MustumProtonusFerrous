//! Port of the portable logic in Wine's `dlls/cryptxml/cryptxml.c`.
//!
//! Ported:
//! - Document and signature objects and their initial status (`alloc_doc`,
//!   `alloc_sig`), including the signature copy that `CryptXmlOpenToDecode` makes:
//!   [`Cryptxml::open_to_decode`]
//! - `CryptXmlOpenToDecode`: [`Cryptxml::open_to_decode`]
//! - `CryptXmlClose`: [`Cryptxml::close`]
//! - `CryptXmlGetDocContext`, `CryptXmlGetSignature`, `CryptXmlGetStatus`:
//!   [`Cryptxml::get_doc_context`], [`Cryptxml::get_signature`], [`Cryptxml::get_status`]
//! - `CryptXmlVerifySignature`: [`Cryptxml::verify_signature`]
//! - The 13 `@ stub` exports of `cryptxml.spec`, modelled as the
//!   `EXCEPTION_WINE_STUB` fault they raise: [`stub_fault`]
//!
//! Skipped:
//! - `E_OUTOFMEMORY` from `calloc`: allocation failure, not logic.
//! - XML parsing of the blob: Wine keeps the bytes unparsed and accepts any blob.
//!   The `WS_E_INVALID_FORMAT` result in Wine's `todo_wine` test is Windows behaviour
//!   that this source does not produce.
//! - Digest and key checks: Wine reports every signature as valid, so the
//!   `CRYPT_XML_STATUS_ERROR_DIGEST_INVALID` expected by the `todo_wine` test never
//!   appears. The key and flags of `CryptXmlVerifySignature`, and the `config`,
//!   `flags`, `property` and `property_count` of `CryptXmlOpenToDecode`, have no effect.
//! - TRACE and FIXME output.
//! - Handles that were never issued or were already closed. Wine reads freed or wild
//!   memory there, which has no defined result. The port answers `CRYPT_XML_E_HANDLE`,
//!   or `E_INVALIDARG` for a NULL handle.
//! - The unknown-magic branch of `CryptXmlClose` (`E_NOTIMPL`): the port issues only
//!   document and signature objects, so that branch cannot run.

mod handles;
mod stubs;

pub use handles::{
    Cryptxml, DocContext, HResult, Handle, KeyInfo, Signature, Status, CRYPT_XML_E_HANDLE,
    CRYPT_XML_STATUS_ERROR_NOT_RESOLVED, CRYPT_XML_STATUS_NO_ERROR,
    CRYPT_XML_STATUS_SIGNATURE_VALID, DOC_CONTEXT_SIZE, E_INVALIDARG, SIGNATURE_SIZE, STATUS_SIZE,
    S_FALSE, S_OK,
};
pub use stubs::{
    stub_fault, StubFault, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB, MODULE_NAME, STUB_EXPORTS,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
