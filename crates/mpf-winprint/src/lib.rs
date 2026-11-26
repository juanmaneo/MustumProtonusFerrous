//! Port of the portable logic in Wine's `dlls/winprint/printproc.c`.
//!
//! Ported:
//! - `EnumPrintProcessorDatatypesW` parameter checks, size query and the
//!   `DATATYPES_INFO_1W` layout: [`enum_print_processor_datatypes_w`], [`Arch`]
//! - `OpenPrintProcessor` argument and datatype checks: [`check_open_print_processor`],
//!   [`OpenData`]
//! - The `get_handle_data` magic check: [`check_handle_data`]
//!
//! Skipped:
//! - The rest of `OpenPrintProcessor`: `OpenPrinterW`, `LocalAlloc` and `wcsdup`.
//! - `PrintDocumentOnPrintProcessor`: the spooler `ReadPrinter`/`WritePrinter` copy loop.
//! - The rest of `ClosePrintProcessor`: `ClosePrinter`, `free` and `LocalFree`.
//! - `ControlPrintProcessor`: a `FIXME` stub that returns `ERROR_CALL_NOT_IMPLEMENTED`.
//! - `DllRegisterServer`: `AddPrintProcessorW` registration.
//! - `struct pp_data` as state: `hport`, `doc_name` and `out_file` are OS handles and heap strings.
//! - The datatypes Wine lists as missing (`RAW [FF appended]`, `NT EMF 1.00x`, `XPS2GDI`).

mod datatypes;

pub use datatypes::{enum_print_processor_datatypes_w, Arch};

/// `PP_MAGIC`: the `magic` field that tags a valid print processor handle.
pub const PP_MAGIC: u32 = 0x9521_73fd;

const RAW: &[u16] = &[b'R' as u16, b'A' as u16, b'W' as u16];
const TEXT: &[u16] = &[b'T' as u16, b'E' as u16, b'X' as u16, b'T' as u16];

/// A failure that `printproc.c` reports through `SetLastError`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpError {
    InvalidHandle,
    InvalidParameter,
    InsufficientBuffer,
    InvalidDatatype,
}

impl PpError {
    /// The Win32 error code that the C code passes to `SetLastError`.
    pub fn win32_code(self) -> u32 {
        match self {
            Self::InvalidHandle => 6,
            Self::InvalidParameter => 87,
            Self::InsufficientBuffer => 122,
            Self::InvalidDatatype => 1804,
        }
    }
}

/// `get_handle_data`: `magic` is the handle's `magic` field, or `None` for a null handle.
pub fn check_handle_data(magic: Option<u32>) -> Result<(), PpError> {
    match magic {
        Some(PP_MAGIC) => Ok(()),
        _ => Err(PpError::InvalidHandle),
    }
}

/// The fields of `PRINTPROCESSOROPENDATA` that `OpenPrintProcessor` validates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenData<'a> {
    pub datatype: Option<&'a [u16]>,
}

/// The argument checks at the top of `OpenPrintProcessor`. `port` is only checked for
/// presence, as in the C code. `datatype` is compared up to its first NUL, as `wcscmp` does.
pub fn check_open_print_processor(
    port: Option<&[u16]>,
    open_data: Option<&OpenData<'_>>,
) -> Result<(), PpError> {
    let datatype = match (port, open_data) {
        (
            Some(_),
            Some(OpenData {
                datatype: Some(datatype),
            }),
        ) => datatype,
        _ => return Err(PpError::InvalidParameter),
    };
    if wide_eq(datatype, RAW) || wide_eq(datatype, TEXT) {
        Ok(())
    } else {
        Err(PpError::InvalidDatatype)
    }
}

/// `wcscmp(s, t) == 0` for a NUL-terminated `s`. A slice without a NUL is compared whole.
fn wide_eq(s: &[u16], t: &[u16]) -> bool {
    let end = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..end] == t
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
