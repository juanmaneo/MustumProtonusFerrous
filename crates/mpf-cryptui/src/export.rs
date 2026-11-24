//! File-name extension rules from Wine's `export_append_extension` in
//! `dlls/cryptui/main.c`.
//!
//! The extensions are ASCII, so the ASCII-only case-insensitive comparison here
//! matches Wine's `wcsicmp` for them.

pub const CRYPTUI_WIZ_EXPORT_CTL_CONTEXT: u32 = 2;
pub const CRYPTUI_WIZ_EXPORT_CRL_CONTEXT: u32 = 3;
pub const CRYPTUI_WIZ_EXPORT_CERT_STORE: u32 = 4;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_PFX: u32 = 2;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_PKCS7: u32 = 3;

/// Returns the extension Wine uses for an export format and subject choice.
/// PKCS#7 and PFX are chosen by format; every other format uses the subject.
pub fn export_extension(format: u32, subject_choice: u32) -> &'static str {
    match format {
        CRYPTUI_WIZ_EXPORT_FORMAT_PKCS7 => ".p7b",
        CRYPTUI_WIZ_EXPORT_FORMAT_PFX => ".pfx",
        _ => match subject_choice {
            CRYPTUI_WIZ_EXPORT_CRL_CONTEXT => ".crl",
            CRYPTUI_WIZ_EXPORT_CTL_CONTEXT => ".ctl",
            CRYPTUI_WIZ_EXPORT_CERT_STORE => ".sst",
            _ => ".cer",
        },
    }
}

/// Appends the extension for the export format and subject choice unless the
/// text from the last dot of `file_name` already matches it, ignoring ASCII
/// case. A name without a dot always gets the extension.
pub fn export_file_name(file_name: &str, format: u32, subject_choice: u32) -> String {
    let extension = export_extension(format, subject_choice);
    let has_extension = match file_name.rfind('.') {
        Some(dot) => file_name[dot..].eq_ignore_ascii_case(extension),
        None => false,
    };
    let mut name = file_name.to_owned();
    if !has_extension {
        name.push_str(extension);
    }
    name
}
