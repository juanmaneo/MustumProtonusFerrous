//! Port of the portable logic in Wine's `dlls/cryptsp/arc4.c`, `crypt.c` and `lmhash.c`.
//!
//! Ported:
//! - `SystemFunction032` (RC4 keystream XOR, `arc4_init` and `arc4_ProcessString`): [`system_function_032`]
//! - `SystemFunction006` (LM hash of a password, `LMhash`): [`system_function_006`]
//! - `SystemFunction008` and alias `SystemFunction009` (LM challenge response): [`system_function_008`]
//! - `SystemFunction012` and `SystemFunction013` (two DES blocks, each with its own 7-byte key),
//!   with aliases 014 to 023: [`system_function_012`], [`system_function_013`]
//! - `SystemFunction024` and `SystemFunction025` (two DES blocks under one 4-byte key),
//!   with aliases 026 and 027: [`system_function_024`], [`system_function_025`]
//! - `SystemFunction030` and alias `SystemFunction031` (equality of two 16-byte blocks): [`system_function_030`]
//!
//! The DES block (`SystemFunction001` and `SystemFunction002`) belongs to `dlls/cryptbase`
//! and is ported as `mpf-cryptbase`. It enters through the [`Des`] trait, so this crate has no
//! dependencies.
//!
//! Skipped:
//! - `SystemFunction007`, `SystemFunction010` and `SystemFunction011`: MD4 wrappers. The MD4
//!   core (`MD4Init`, `MD4Update`, `MD4Final`) is in `dlls/ntdll/crypt.c`, not in this DLL.
//! - `CheckSignatureInFile` and `SystemFunction035`: a FIXME stub that returns TRUE.
//! - `SystemFunction033`: `@ stub` in `cryptsp.spec`.
//! - The `ustring` and `UNICODE_STRING` arguments: slice lengths replace `Length`. The NULL
//!   checks have no equivalent here, because references cannot be null.

mod des;
mod lm;
mod rc4;

pub use des::{
    system_function_012, system_function_013, system_function_024, system_function_025, Des,
};
pub use lm::{system_function_006, system_function_008};
pub use rc4::system_function_032;

pub type NtStatus = i32;

pub const STATUS_SUCCESS: NtStatus = 0;
pub const STATUS_UNSUCCESSFUL: NtStatus = 0xC000_0001u32 as i32;

/// `SystemFunction030` (alias `SystemFunction031`): whether two 16-byte blocks are equal.
pub fn system_function_030(b1: &[u8; 16], b2: &[u8; 16]) -> bool {
    b1 == b2
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
