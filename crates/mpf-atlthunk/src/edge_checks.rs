//! Edge-case tests for the ported atlthunk logic.
//!
//! Boundary displacements, 32-bit truncation, pool exhaustion and release.
//! Expectations marked "not oracle-derived" come from reading the C source,
//! not from a captured run.

use super::*;

#[test]
fn x86_64_zero_displacement() {
    // Not oracle-derived: arg and proc sit exactly at the end of their fields.
    let thunk = 0x1000;
    let bytes = encode_x86_64(thunk, thunk + 7, thunk + 13);
    assert_eq!(
        bytes,
        [0x48, 0x8b, 0x0d, 0, 0, 0, 0, 0xff, 0x25, 0, 0, 0, 0]
    );
}

#[test]
fn x86_64_negative_displacement_wraps_to_u32() {
    // Not oracle-derived: backwards targets give two's complement rel32 values.
    let bytes = encode_x86_64(0x1000, 0x0f00, 0x0800);
    assert_eq!(&bytes[3..7], &(-0x107i32).to_le_bytes());
    assert_eq!(&bytes[9..13], &(-0x80di32).to_le_bytes());
}

#[test]
fn i386_truncates_addresses_to_32_bits() {
    // Not oracle-derived: C casts pointers to DWORD, keeping the low 32 bits.
    let bytes = encode_i386(0x1_0000_1234, 0xffff_ffff_8000_0000);
    assert_eq!(bytes[1..5], 0x1234u32.to_le_bytes());
    assert_eq!(bytes[11..15], 0x8000_0000u32.to_le_bytes());
}

#[test]
fn aarch64_backwards_literal_sets_high_bits() {
    // Not oracle-derived: ldr x0 literal target is 64 words before the thunk.
    // Shifting the negative word count left by 5 fills bits 11..31, and the OR
    // with 0x58000000 leaves them set. That is what the C expression does too.
    let bytes = encode_aarch64(0x2000, 0x1f00, 0x1e00);
    assert_eq!(bytes[0..4], 0xffff_f800u32.to_le_bytes());
}

#[test]
fn aarch64_zero_literal_offsets() {
    // Not oracle-derived: the first literal load targets its own word, the
    // second targets the word after the first instruction (proc = thunk + 4).
    let bytes = encode_aarch64(0x3000, 0x3000, 0x3004);
    assert_eq!(bytes[0..4], 0x5800_0000u32.to_le_bytes());
    assert_eq!(bytes[4..8], 0x5800_0010u32.to_le_bytes());
    assert_eq!(bytes[8..12], 0xd61f_0200u32.to_le_bytes());
    assert_eq!(bytes[12..16], [0, 0, 0, 0]);
}

#[test]
fn thunk_code_offset_boundaries() {
    // Not oracle-derived: index * size, checked at the last slot of each pool.
    assert_eq!(thunk_code_offset(0, 13), 0);
    assert_eq!(thunk_code_offset(314, 13), 4082);
    assert_eq!(thunk_code_offset(272, 15), 4080);
    assert_eq!(thunk_code_offset(255, 16), 4080);
}

#[test]
fn allocate_fills_pool_then_reports_exhaustion() {
    // Not oracle-derived: slot hand-out order and the NULL-ing of current_pool.
    let mut p = PoolCounters::default();
    assert_eq!(p.allocate(3), (0, false));
    assert_eq!(p.allocate(3), (1, false));
    assert_eq!(p.allocate(3), (2, true));
    assert_eq!(p.first_free, 3);
}

#[test]
fn free_reports_release_only_when_every_slot_is_back() {
    // Not oracle-derived: InterlockedIncrement result compared against capacity.
    let mut p = PoolCounters::default();
    for _ in 0..3 {
        p.allocate(3);
    }
    assert!(!p.free(3));
    assert!(!p.free(3));
    assert!(p.free(3));
}

#[test]
fn releasable_only_when_no_slot_is_outstanding() {
    // Not oracle-derived: DllMain compares free_count with first_free.
    let mut p = PoolCounters::default();
    assert!(p.is_releasable());
    p.allocate(2);
    assert!(!p.is_releasable());
    p.free(2);
    assert!(p.is_releasable());
}

#[test]
fn single_slot_pool_is_exhausted_on_first_allocate() {
    // Not oracle-derived: capacity 1 exercises both transitions at once.
    let mut p = PoolCounters::default();
    assert_eq!(p.allocate(1), (0, true));
    assert!(p.free(1));
}
