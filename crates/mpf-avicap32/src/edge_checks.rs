//! Edge-case tests for the ported avicap32 logic.
//!
//! Boundary and branch checks. Expectations marked "not oracle-derived" come from the
//! buffer sizes and the C semantics of `%s` and `%u`, and are not captured rows.

use super::*;

#[test]
fn device_node_path_fits_16_byte_buffer() {
    // Not oracle-derived: the longest u16 index gives 15 bytes plus NUL, exactly the 16-byte buffer.
    assert_eq!(device_node_path(u16::MAX).len(), 15);
    assert_eq!(device_node_path(u16::MAX).len() + 1, 16);
    assert_eq!(device_node_path(10000), "/dev/video10000");
}

#[test]
fn capture_bit_boundaries() {
    // Not oracle-derived: bit 0 alone captures; bit 1 alone does not; bit 31 alone does not
    // because device_caps is then zero.
    assert!(is_capture_device(V4L2_CAP_VIDEO_CAPTURE, 0));
    assert!(!is_capture_device(0x0000_0002, 0));
    assert!(!is_capture_device(V4L2_CAP_DEVICE_CAPS, 0));
    // With DEVICE_CAPS set, only device_caps bit 0 counts.
    assert!(is_capture_device(
        V4L2_CAP_DEVICE_CAPS | V4L2_CAP_VIDEO_CAPTURE,
        V4L2_CAP_VIDEO_CAPTURE
    ));
    assert!(!is_capture_device(V4L2_CAP_DEVICE_CAPS, 0x0000_0002));
}

#[test]
fn driver_version_keeps_31_bytes() {
    // Not oracle-derived: snprintf into char[32] stores 31 bytes plus NUL. A 31-byte
    // driver fills the buffer and drops the whole tail.
    let driver = [b'd'; 31];
    assert_eq!(format_driver_version(&driver, 0x0001_0203), driver.to_vec());

    // A 30-byte driver keeps only the first byte of the tail, the space.
    let driver = [b'd'; 30];
    let mut expected = driver.to_vec();
    expected.push(b' ');
    assert_eq!(format_driver_version(&driver, 0x0001_0203), expected);
}

#[test]
fn driver_version_stops_at_nul_and_uses_whole_slice_without_one() {
    // Not oracle-derived: a leading NUL makes the driver name empty, as `%s` would.
    assert_eq!(format_driver_version(b"\0abc", 0), b" v0.0.0".to_vec());
    // Not oracle-derived: with no NUL, the whole slice is the C string.
    assert_eq!(format_driver_version(b"abc", 0), b"abc v0.0.0".to_vec());
}

#[test]
fn driver_version_splits_version_into_bytes() {
    // Not oracle-derived: each byte of `version` is printed separately, and bits 24-31 are dropped.
    assert_eq!(
        format_driver_version(b"x", 0x0000_00ff),
        b"x v0.0.255".to_vec()
    );
    assert_eq!(
        format_driver_version(b"x", 0x0000_0100),
        b"x v0.1.0".to_vec()
    );
    assert_eq!(
        format_driver_version(b"x", 0x0001_0000),
        b"x v1.0.0".to_vec()
    );
    assert_eq!(
        format_driver_version(b"x", 0x0100_0000),
        b"x v0.0.0".to_vec()
    );
}
