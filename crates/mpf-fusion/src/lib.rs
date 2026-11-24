//! MPF port of the deterministic parts of Wine's `dlls/fusion`: display-name parsing for
//! `CreateAssemblyNameObject` (`asmname.c`), the blob length and RVA helpers of `assembly.c`, and
//! `token_to_str` from `fusionpriv.h`. COM objects, files, mappings and the registry are not
//! ported.

mod display_name;
mod metadata;
mod token;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use display_name::{create_assembly_name, AssemblyName, PeKind, CANOF_PARSE_DISPLAY_NAME};
pub use metadata::{decode_blob_length, rva_to_offset, Section};
pub use token::{token_to_str, BYTES_PER_TOKEN, TOKEN_LENGTH};

/// `HRESULT`: a signed 32-bit status code, negative for errors.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const E_INVALIDARG: Hresult = 0x8007_0057_u32 as Hresult;
pub const FUSION_E_PRIVATE_ASM_DISALLOWED: Hresult = 0x8013_1044_u32 as Hresult;
pub const FUSION_E_INVALID_NAME: Hresult = 0x8013_1047_u32 as Hresult;

/// `FAILED`: an `HRESULT` is an error when its sign bit is set.
pub fn failed(hr: Hresult) -> bool {
    hr < 0
}
