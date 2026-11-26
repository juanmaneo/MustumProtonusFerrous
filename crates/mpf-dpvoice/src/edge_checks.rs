//! Boundary and branch tests for `get_compression_types` and the `Abi` layout.
//!
//! The layout, the `DVERR_*` values, the name length and the zeroed padding are
//! printed by the oracle harness, so those checks are oracle-derived. Checks marked
//! "not oracle-derived" follow from the C pointer arithmetic by hand, because the
//! oracle normalises `lpszName` to an offset.

use super::*;

fn required(abi: Abi) -> usize {
    abi.struct_size() + PCM_NAME_BYTES
}

/// Calls with a buffer of `len` bytes filled with 0xaa and `*pdwNumElements` starting at 7.
fn call_with_buffer(len: usize, flags: u32, addr: u64, abi: Abi) -> (HRESULT, u32, u32, Vec<u8>) {
    let mut buf = vec![0xaau8; len];
    let mut size = len as u32;
    let mut num = 7;
    let hr = get_compression_types(
        Some(&mut buf[..]),
        Some(&mut size),
        Some(&mut num),
        flags,
        addr,
        abi,
    );
    (hr, size, num, buf)
}

fn name_pointer(buf: &[u8], abi: Abi) -> u64 {
    let at = abi.name_offset();
    let mut bytes = [0u8; 8];
    bytes[..abi.pointer_size()].copy_from_slice(&buf[at..at + abi.pointer_size()]);
    u64::from_le_bytes(bytes)
}

#[test]
fn layout_matches_oracle_sizeof_and_offsetof() {
    // Oracle-derived: the `// layout` lines printed by the harness (sizeof, offsetof).
    assert_eq!(Abi::Win32.pointer_size(), 4);
    assert_eq!(Abi::Win32.struct_size(), 36);
    assert_eq!(Abi::Win32.name_offset(), 20);
    assert_eq!(Abi::Win32.description_offset(), 24);
    assert_eq!(Abi::Win32.flags_offset(), 28);
    assert_eq!(Abi::Win32.max_bits_offset(), 32);
    assert_eq!(Abi::Win64.pointer_size(), 8);
    assert_eq!(Abi::Win64.struct_size(), 48);
    assert_eq!(Abi::Win64.name_offset(), 24);
    assert_eq!(Abi::Win64.description_offset(), 32);
    assert_eq!(Abi::Win64.flags_offset(), 40);
    assert_eq!(Abi::Win64.max_bits_offset(), 44);
}

#[test]
fn error_codes_match_dvoice_macros() {
    // Oracle-derived: the `hr` column. Values are MAKE_HRESULT(1, 0x15, code) from dvoice.h.
    assert_eq!(DV_OK, 0);
    assert_eq!(DVERR_INVALIDPOINTER as u32, 0x8000_4003);
    assert_eq!(DVERR_INVALIDFLAGS as u32, 0x8015_0078);
    assert_eq!(DVERR_BUFFERTOOSMALL as u32, 0x8015_001e);
    // The consts are evaluated at compile time, so call the helper at run time as well.
    assert_eq!(make_dvhresult(0x0078), DVERR_INVALIDFLAGS);
}

#[test]
fn pcm_name_is_17_utf16_units_including_nul() {
    // Oracle-derived: the harness sizes buffers with sizeof(L"MS-PCM 64 kbit/s") = 34.
    assert_eq!(PCM_NAME_BYTES, 34);
    assert_eq!(PCM_NAME.encode_utf16().count() + 1, 17);
}

#[test]
fn struct_size_is_padded_to_pointer_alignment() {
    // Not oracle-derived: an invariant of natural alignment.
    for abi in [Abi::Win32, Abi::Win64] {
        let pointer = abi.pointer_size();
        assert_eq!(abi.struct_size() % pointer, 0);
        assert_eq!(abi.name_offset() % pointer, 0);
        assert!(abi.struct_size() >= abi.max_bits_offset() + 4);
    }
}

#[test]
fn win64_name_pointer_is_data_addr_plus_struct_size() {
    // Not oracle-derived: the oracle stores offsets, so the base address is checked by hand.
    let (hr, _, _, buf) = call_with_buffer(required(Abi::Win64), 0, 0x1000, Abi::Win64);
    assert_eq!(hr, DV_OK);
    assert_eq!(name_pointer(&buf, Abi::Win64), 0x1000 + 48);
}

#[test]
fn win32_name_pointer_wraps_at_32_bits() {
    // Not oracle-derived: 0xffff_fff0 + 36 wraps to 0x14 in 32-bit pointer arithmetic.
    let (hr, _, _, buf) = call_with_buffer(required(Abi::Win32), 0, 0xffff_fff0, Abi::Win32);
    assert_eq!(hr, DV_OK);
    assert_eq!(name_pointer(&buf, Abi::Win32), 0x14);
}

#[test]
fn win64_name_pointer_wraps_at_64_bits() {
    // Not oracle-derived: u64::MAX - 15 + 48 wraps to 0x20.
    let (hr, _, _, buf) = call_with_buffer(required(Abi::Win64), 0, u64::MAX - 15, Abi::Win64);
    assert_eq!(hr, DV_OK);
    assert_eq!(name_pointer(&buf, Abi::Win64), 0x20);
}

#[test]
fn one_byte_past_required_leaves_tail_untouched() {
    // Not oracle-derived: the oracle's larger_write row has a 10-byte tail, this checks one byte.
    let (hr, size, num, buf) = call_with_buffer(required(Abi::Win64) + 1, 0, 0, Abi::Win64);
    assert_eq!(hr, DV_OK);
    assert_eq!(size, required(Abi::Win64) as u32);
    assert_eq!(num, 1);
    assert_eq!(buf[required(Abi::Win64)], 0xaa);
}

#[test]
fn small_size_takes_precedence_over_null_buffer() {
    // Not oracle-derived: the oracle covers size 0 with NULL pData, not size required-1.
    let mut size = required(Abi::Win64) as u32 - 1;
    let mut num = 7;
    let hr = get_compression_types(None, Some(&mut size), Some(&mut num), 0, 0, Abi::Win64);
    assert_eq!(hr, DVERR_BUFFERTOOSMALL);
    assert_eq!(size, required(Abi::Win64) as u32);
    assert_eq!(num, 1);
}

#[test]
fn flags_check_precedes_size_query() {
    // Not oracle-derived: the oracle's flags rows use a full-size buffer.
    let (hr, size, num, buf) = call_with_buffer(3, 1, 0, Abi::Win64);
    assert_eq!(hr, DVERR_INVALIDFLAGS);
    assert_eq!(size, 3);
    assert_eq!(num, 7);
    assert_eq!(buf, vec![0xaa; 3]);
}
