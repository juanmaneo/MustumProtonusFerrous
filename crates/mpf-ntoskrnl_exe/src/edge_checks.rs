use super::{rtl_ulong_byte_swap, rtl_ulonglong_byte_swap, rtl_ushort_byte_swap};

#[test]
fn byte_swap_is_an_involution_at_each_width() {
    for value in [0, 1, u16::MAX, 0x1234, 0x8001] {
        assert_eq!(rtl_ushort_byte_swap(rtl_ushort_byte_swap(value)), value);
    }
    for value in [0, 1, u32::MAX, 0x12345678, 0x80000001] {
        assert_eq!(rtl_ulong_byte_swap(rtl_ulong_byte_swap(value)), value);
    }
    for value in [0, 1, u64::MAX, 0x0123456789abcdef, 0x8000000000000001] {
        assert_eq!(
            rtl_ulonglong_byte_swap(rtl_ulonglong_byte_swap(value)),
            value
        );
    }
}

#[test]
fn byte_swap_preserves_zero_and_all_ones() {
    assert_eq!(rtl_ushort_byte_swap(0), 0);
    assert_eq!(rtl_ushort_byte_swap(u16::MAX), u16::MAX);
    assert_eq!(rtl_ulong_byte_swap(0), 0);
    assert_eq!(rtl_ulong_byte_swap(u32::MAX), u32::MAX);
    assert_eq!(rtl_ulonglong_byte_swap(0), 0);
    assert_eq!(rtl_ulonglong_byte_swap(u64::MAX), u64::MAX);
}
