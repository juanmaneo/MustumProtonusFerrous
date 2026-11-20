//! The adapter address text from Wine's `wprint_mac` in `netapi32.c`.

/// `wprint_mac`: formats the adapter address as uppercase hex pairs, for at most `capacity - 1`
/// digits, and ends the result with a NUL. A zero `capacity` gives an empty result. A missing
/// address gives an empty string.
///
/// The C code writes whole bytes and can then pass the end of the buffer. Here the digits stop at
/// `capacity - 1`, so an odd budget ends on the high nibble of a byte. The whole address is
/// formatted, although the C code reads its bytes from the 8-byte array in `MIB_IFROW`, so a
/// longer address is read past that array there.
pub fn format_mac(address: Option<&[u8]>, capacity: usize) -> Vec<u16> {
    if capacity == 0 {
        return Vec::new();
    }
    let Some(address) = address else {
        return vec![0];
    };
    let digits = (2 * address.len()).min(capacity - 1);
    let nibbles = address.iter().flat_map(|&byte| [byte >> 4, byte & 0x0f]);
    let mut out: Vec<u16> = nibbles
        .take(digits)
        .map(|nibble| u16::from(hex_digit(nibble)))
        .collect();
    out.push(0);
    out
}

fn hex_digit(nibble: u8) -> u8 {
    if nibble > 9 {
        b'A' + nibble - 10
    } else {
        b'0' + nibble
    }
}
