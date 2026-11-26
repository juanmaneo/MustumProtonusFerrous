//! Port of the portable logic in Wine's `dlls/diasymreader/writer.c`.
//!
//! Ported:
//! - `SymWriter_GetDebugInfo`: sizing and encoding of the CodeView `RSDS`
//!   debug-directory record for the PDB path, with the UTF-16 to UTF-8
//!   conversion done as `WideCharToMultiByte(CP_UTF8, ..., -1, ...)` does it:
//!   [`get_debug_info`], [`PdbInfo`], [`DebugInfo`], [`ImageDebugDirectory`]
//!
//! Skipped:
//! - `SymWriter_Initialize`: the `wcsncpy_s(..., _TRUNCATE)` copy into `pdb_filename`
//!   and the `_time32` timestamp. The port takes the stored buffer as input.
//! - `SymWriter_GetSignatureAge`: accessor for the stored timestamp and age.
//! - `SymWriter_CreateInstance`, `UuidCreate`, the critical section, COM vtables and
//!   reference counting.
//! - `SymDocumentWriter_*` and the `SymWriter_*` exports that only log `FIXME` and
//!   return `S_OK`, plus `GetDebugInfoWithPadding`, `SetPath`, `OpenMod` and `GetPath`,
//!   which return `E_NOTIMPL`.
//! - `main.c`: class factory and `DllGetClassObject`.

/// `MAX_PATH`: the length of `SymWriter::pdb_filename`.
pub const MAX_PATH: usize = 260;

/// `IMAGE_DEBUG_TYPE_CODEVIEW`.
pub const IMAGE_DEBUG_TYPE_CODEVIEW: u32 = 2;

/// `FIELD_OFFSET(OMFSignatureRSDS, name)`: `Signature[4]`, `GUID guid`, `unsigned int age`.
const RSDS_NAME_OFFSET: usize = 24;

/// The `SymWriter` fields that `SymWriter_GetDebugInfo` reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdbInfo {
    /// `pdb_guid` in its in-memory layout: `Data1` to `Data3` little-endian, then `Data4`.
    pub guid: [u8; 16],
    /// `pdb_age`.
    pub age: u32,
    /// `pdb_filename`. The name ends at its first NUL.
    pub filename: [u16; MAX_PATH],
}

/// The `IMAGE_DEBUG_DIRECTORY` fields that `SymWriter_GetDebugInfo` writes. The C code
/// leaves `TimeDateStamp`, `AddressOfRawData` and `PointerToRawData` untouched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageDebugDirectory {
    pub characteristics: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub kind: u32,
    pub size_of_data: u32,
}

/// `ERROR_INSUFFICIENT_BUFFER`: the caller's buffer is smaller than the record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InsufficientBuffer;

/// Outputs of `SymWriter_GetDebugInfo`. `size` and `directory` are set even when the
/// buffer is too small.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DebugInfo {
    /// `*pcData`: the size of the RSDS record.
    pub size: u32,
    /// `*pIDD`.
    pub directory: ImageDebugDirectory,
    /// `Ok` for `S_OK`. `Err` when `data` was supplied and is smaller than `size`.
    pub status: Result<(), InsufficientBuffer>,
}

/// `SymWriter_GetDebugInfo`. A `data` of `None` plays the role of a NULL buffer, so only
/// the size and directory are reported. A buffer smaller than the record is left untouched.
pub fn get_debug_info(pdb: &PdbInfo, data: Option<&mut [u8]>) -> DebugInfo {
    let name = utf8_nul_terminated(&pdb.filename);
    let size = (RSDS_NAME_OFFSET + name.len()) as u32;
    let directory = ImageDebugDirectory {
        characteristics: 0,
        major_version: 0,
        minor_version: 0,
        kind: IMAGE_DEBUG_TYPE_CODEVIEW,
        size_of_data: size,
    };
    let status = match data {
        None => Ok(()),
        Some(buf) if buf.len() < size as usize => Err(InsufficientBuffer),
        Some(buf) => {
            buf[..4].copy_from_slice(b"RSDS");
            buf[4..20].copy_from_slice(&pdb.guid);
            buf[20..RSDS_NAME_OFFSET].copy_from_slice(&pdb.age.to_le_bytes());
            buf[RSDS_NAME_OFFSET..size as usize].copy_from_slice(&name);
            Ok(())
        }
    };
    DebugInfo {
        size,
        directory,
        status,
    }
}

/// `WideCharToMultiByte(CP_UTF8, 0, s, -1, ...)`: the UTF-8 of `s` up to its first NUL,
/// plus the NUL. Each unpaired surrogate becomes U+FFFD, as in Wine's `RtlUnicodeToUTF8N`.
fn utf8_nul_terminated(s: &[u16]) -> Vec<u8> {
    let len = s.iter().position(|&u| u == 0).unwrap_or(s.len());
    let mut out = Vec::new();
    for ch in char::decode_utf16(s[..len].iter().copied())
        .map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER))
    {
        let mut utf8 = [0u8; 4];
        out.extend_from_slice(ch.encode_utf8(&mut utf8).as_bytes());
    }
    out.push(0);
    out
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
