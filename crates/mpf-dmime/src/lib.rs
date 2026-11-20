//! Deterministic byte parsing from Wine's `dlls/dmime/midi.c`.
//!
//! This crate ports `read_variable_length_number`, the MIDI variable-length quantity decoder. The
//! parser accepts a byte slice and reports both its decoded value and consumed length, without
//! depending on a COM stream or any other operating-system service.
//!
//! The C decoder keeps shifting a 32-bit `DWORD` and does not impose MIDI's usual four-byte
//! limit. This port preserves that unsigned wrapping behavior for overlong encodings. A short
//! input is reported as incomplete along with the partial value and number of bytes consumed.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub mod variable_length;
