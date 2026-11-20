//! Headless port of the deterministic helper in Wine's `dlls/wow64win/gdi.c`.
//!
//! Ported:
//! - `gdi_handle_type`: [gdi_handle_type]. The C version takes an `HGDIOBJ`, truncates it to
//!   the low 32 bits with `HandleToUlong`, and masks it with `NTGDI_HANDLE_TYPE_MASK`. The port
//!   takes the handle as a `u64` so the truncation is explicit.
//!
//! Skipped: the `wow64_NtGdi*` syscall thunks, which read 32-bit argument arrays and call into
//! `win32u`, and the `wow64_NtGdiDdDDI*` thunks, which need the D3DKMT runtime.

/// `NTGDI_HANDLE_TYPE_MASK` from Wine's `include/ntgdi.h`.
pub const NTGDI_HANDLE_TYPE_MASK: u32 = 0x007f_0000;

/// The GDI handle-type bits of a handle. The handle is truncated to its low 32 bits first, as
/// `HandleToUlong` does, so bits 32 to 63 never reach the result.
pub fn gdi_handle_type(obj: u64) -> u32 {
    (obj as u32) & NTGDI_HANDLE_TYPE_MASK
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
