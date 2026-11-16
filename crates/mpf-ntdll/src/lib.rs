//! ntdll core: NTSTATUS, RTL unicode strings, heap, critical sections,
//! object handles, and the SEH/panic exception boundary.
//!
//! Mirrors `dlls/ntdll` in the reference Wine tree (see
//! `repos/wine/dlls/ntdll/unicode.c`, `rtlstr.c`, `heap.c`, `sync.c`,
//! `om.c`, `exception.c`).

pub mod critical_section;
pub mod exception;
pub mod heap;
pub mod ntstatus;
pub mod object;
pub mod unicode;
