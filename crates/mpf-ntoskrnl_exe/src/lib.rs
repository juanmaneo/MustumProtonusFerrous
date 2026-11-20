//! Pure byte-order helpers from Wine's `dlls/ntoskrnl.exe/ntoskrnl.c`.
//!
//! This subset ports `RtlUshortByteSwap`, `RtlUlongByteSwap`, and
//! `RtlUlonglongByteSwap`. The Wine exports are implemented in x86 assembly;
//! these Rust functions preserve their value-level behavior without modeling
//! the platform-specific calling convention.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// Reverses the byte order of a 16-bit value (`RtlUshortByteSwap`).
#[must_use]
pub const fn rtl_ushort_byte_swap(value: u16) -> u16 {
    value.swap_bytes()
}

/// Reverses the byte order of a 32-bit value (`RtlUlongByteSwap`).
#[must_use]
pub const fn rtl_ulong_byte_swap(value: u32) -> u32 {
    value.swap_bytes()
}

/// Reverses the byte order of a 64-bit value (`RtlUlonglongByteSwap`).
#[must_use]
pub const fn rtl_ulonglong_byte_swap(value: u64) -> u64 {
    value.swap_bytes()
}
