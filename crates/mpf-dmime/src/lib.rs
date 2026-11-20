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

/// Decodes one MIDI variable-length number through the crate's primary API.
pub fn decode_variable_length(bytes: &[u8]) -> variable_length::DecodeResult {
    variable_length::read_variable_length_number(bytes)
}

#[cfg(test)]
mod root_api_tests {
    #[test]
    fn decode_variable_length_matches_the_decoder_module() {
        let bytes = [0x81, 0x01];
        assert_eq!(
            super::decode_variable_length(&bytes),
            super::variable_length::read_variable_length_number(&bytes)
        );
    }
}
