//! Port of the portable logic in Wine's `dlls/cryptext/cryptext_main.c`.
//!
//! Ported:
//! - `strdupAW`: [`strdup_ansi_to_wide`] (NULL stays NULL, the C string ends at its first NUL)
//! - `CryptExtOpenCERW` / `CryptExtOpenCER`: the choice between the certificate viewer and the
//!   "not a valid certificate" message box, and the `S_OK` result: [`crypt_ext_open_cer_w`],
//!   [`crypt_ext_open_cer`], [`CerDialog`], [`OpenCer`]
//! - `CryptExtAddPFX` / `CryptExtAddPFXW`: the `E_NOTIMPL` stub returns: [`crypt_ext_add_pfx`],
//!   [`crypt_ext_add_pfx_w`]
//!
//! Skipped:
//! - `CryptQueryObject`, `CryptUIDlgViewCertificateW`, `CertFreeCertificateContext` and
//!   `MessageBoxW`: OS calls. The port takes the query as a closure and returns the dialog to show.
//! - `MultiByteToWideChar(CP_ACP)`: the single-byte mapping is a caller-supplied closure.
//!   Multi-byte code pages are not modelled.
//! - `malloc` failure in `strdupAW`, the `HWND`, `HINSTANCE` and `showcmd` arguments (they only
//!   reach the viewer and the message box parent), and the `FIXME` / `TRACE` output.

mod open_cer;
mod strings;
mod stubs;

pub use open_cer::{
    crypt_ext_open_cer, crypt_ext_open_cer_w, CerDialog, OpenCer, INVALID_CERT_TEXT,
};
pub use strings::strdup_ansi_to_wide;
pub use stubs::{crypt_ext_add_pfx, crypt_ext_add_pfx_w, E_NOTIMPL, HRESULT, S_OK};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
