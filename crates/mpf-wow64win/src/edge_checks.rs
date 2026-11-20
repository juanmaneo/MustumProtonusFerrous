//! Edge checks for the cases the oracle rows do not reach: the bits just outside the type mask,
//! and the high 32 bits of a 64-bit handle.

use super::*;

#[test]
fn bits_outside_the_type_field_are_ignored() {
    assert_eq!(gdi_handle_type(0x0000_ffff), 0);
    assert_eq!(gdi_handle_type(0x0080_0000), 0);
    assert_eq!(gdi_handle_type(0x0080_ffff), 0);
    assert_eq!(gdi_handle_type(0x0000_0000_ff80_0000), 0);
}

#[test]
fn type_field_is_the_full_seven_bit_range() {
    for shift in 0..7 {
        let bit = 1_u64 << (16 + shift);
        assert_eq!(gdi_handle_type(bit), bit as u32);
    }
    assert_eq!(gdi_handle_type(0x007f_ffff), 0x007f_0000);
}

#[test]
fn high_32_bits_never_reach_the_result() {
    assert_eq!(gdi_handle_type(0xffff_ffff_0000_0000), 0);
    assert_eq!(gdi_handle_type(0x0001_0000_0001_0000), 0x0001_0000);
}
