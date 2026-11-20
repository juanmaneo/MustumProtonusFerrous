use super::{is_valid_devmode_w, FIELD_SIZES};

fn devmode(fields: u32, dm_size: u16, driver_extra: u16, backing_len: usize) -> Vec<u8> {
    let mut bytes = vec![0; backing_len];
    bytes[68..70].copy_from_slice(&dm_size.to_le_bytes());
    bytes[70..72].copy_from_slice(&driver_extra.to_le_bytes());
    bytes[72..76].copy_from_slice(&fields.to_le_bytes());
    bytes
}

#[test]
fn rejects_missing_or_truncated_fixed_headers() {
    assert!(!is_valid_devmode_w(None, 220));
    assert!(!is_valid_devmode_w(Some(&[]), usize::MAX));
    assert!(!is_valid_devmode_w(Some(&[0; 75]), 220));
    assert!(!is_valid_devmode_w(Some(&[0; 220]), 75));
}

#[test]
fn checks_declared_size_and_driver_extra_boundaries() {
    assert!(!is_valid_devmode_w(Some(&devmode(0, 75, 0, 220)), 220));
    assert!(!is_valid_devmode_w(Some(&devmode(0, 76, 4, 220)), 79));
    assert!(is_valid_devmode_w(Some(&devmode(0, 76, 4, 220)), 80));
    assert!(!is_valid_devmode_w(
        Some(&devmode(0, 76, u16::MAX, 220)),
        65_610
    ));
}

#[test]
fn checks_every_flag_at_its_field_size_boundary() {
    for &(flag, required_size) in &FIELD_SIZES {
        let too_small = devmode(flag, (required_size - 1) as u16, 0, 220);
        assert!(
            !is_valid_devmode_w(Some(&too_small), required_size - 1),
            "flag {flag:#010x} accepted before byte {required_size}"
        );
        let exact = devmode(flag, required_size as u16, 0, 220);
        assert!(
            is_valid_devmode_w(Some(&exact), required_size),
            "flag {flag:#010x} rejected at byte {required_size}"
        );
    }
}

#[test]
fn ignores_unselected_and_unknown_field_bits() {
    assert!(is_valid_devmode_w(Some(&devmode(0, 76, 0, 76)), 76));
    assert!(is_valid_devmode_w(
        Some(&devmode(0x8000_0000, 76, 0, 76)),
        76
    ));
}

#[test]
fn only_requires_the_fixed_header_to_be_backed() {
    let header_only = devmode(0, 100, 0, 76);
    assert!(is_valid_devmode_w(Some(&header_only), 100));
}
