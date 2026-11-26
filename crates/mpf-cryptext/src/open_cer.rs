//! `CryptExtOpenCERW` and `CryptExtOpenCER` from `dlls/cryptext/cryptext_main.c`. The OS calls
//! become inputs: `query_object` stands in for `CryptQueryObject`, and the returned [`CerDialog`]
//! says which dialog the C code would show.

use crate::strings::{strdup_ansi_to_wide, until_nul};
use crate::stubs::{HRESULT, S_OK};

/// Text of the `MessageBoxW` shown for a file that is not a valid certificate.
pub const INVALID_CERT_TEXT: &str = "This is not a valid certificate";

const MB_OK: u32 = 0x0000_0000;
const MB_ICONERROR: u32 = 0x0000_0010;

/// The dialog that `CryptExtOpenCERW` shows.
#[derive(Debug, PartialEq, Eq)]
pub enum CerDialog {
    /// `MessageBoxW(NULL, text, filename, flags)`. `caption` is the file name, `None` when NULL.
    InvalidCertificate {
        text: &'static str,
        caption: Option<Vec<u16>>,
        flags: u32,
    },
    /// `CryptUIDlgViewCertificateW` on the loaded context, which is then freed.
    ViewCertificate,
}

/// Result of `CryptExtOpenCERW` or `CryptExtOpenCER`.
#[derive(Debug, PartialEq, Eq)]
pub struct OpenCer {
    pub hr: HRESULT,
    pub dialog: CerDialog,
}

/// `CryptExtOpenCERW`. `query_object` gets the file name as `CryptQueryObject` sees it (`None`
/// for NULL, otherwise the string up to its first NUL) and reports whether a certificate loaded.
pub fn crypt_ext_open_cer_w(
    filename: Option<&[u16]>,
    query_object: impl FnOnce(Option<&[u16]>) -> bool,
) -> OpenCer {
    let filename = filename.map(until_nul);
    let dialog = if query_object(filename) {
        CerDialog::ViewCertificate
    } else {
        CerDialog::InvalidCertificate {
            text: INVALID_CERT_TEXT,
            caption: filename.map(<[u16]>::to_vec),
            flags: MB_OK | MB_ICONERROR,
        }
    };
    OpenCer { hr: S_OK, dialog }
}

/// `CryptExtOpenCER`. Converts the ANSI file name with `strdupAW`, then runs
/// [`crypt_ext_open_cer_w`] on the result.
pub fn crypt_ext_open_cer(
    filename: Option<&[u8]>,
    ansi_unit: impl Fn(u8) -> u16,
    query_object: impl FnOnce(Option<&[u16]>) -> bool,
) -> OpenCer {
    let filename_w = strdup_ansi_to_wide(filename, ansi_unit);
    crypt_ext_open_cer_w(filename_w.as_deref(), query_object)
}
