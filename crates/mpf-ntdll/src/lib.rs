//! ntdll core: NTSTATUS codes and RTL unicode string handling.
//!
//! Mirrors `dlls/ntdll` in the reference Wine tree (see
//! `repos/wine/dlls/ntdll/unicode.c`, `rtlstr.c`).

pub mod ntstatus;
pub mod unicode;
