//! Oracle-derived test vectors for the ported avicap32 logic.
//!
//! Vectors were captured by compiling the verbatim statements from
//! `dlls/avicap32/v4l.c` (against `<linux/videodev2.h>`) under `target/canon/avicap32/`
//! with `gcc -fsanitize=address,undefined -g` and running the harness. The 24 printed rows
//! all ran clean under ASan and UBSan, so none are excluded.
//!
//! The C return value of `snprintf` (`ret=` in the rows) is the untruncated length. It only
//! feeds `v4l_umbstowcs`, which is not ported, so these tests check the stored string only.

use super::*;

#[test]
fn oracle_device_node_path_rows() {
    // row D1 index=0 => /dev/video0
    assert_eq!(device_node_path(0), "/dev/video0");
    // row D2 index=9 => /dev/video9
    assert_eq!(device_node_path(9), "/dev/video9");
    // row D3 index=10 => /dev/video10
    assert_eq!(device_node_path(10), "/dev/video10");
    // row D4 index=255 => /dev/video255
    assert_eq!(device_node_path(255), "/dev/video255");
    // row D5 index=65535 => /dev/video65535
    assert_eq!(device_node_path(65535), "/dev/video65535");
}

#[test]
fn oracle_is_capture_device_rows() {
    // row C1 caps=0x00000000 dev=0x00000001 => 0
    assert!(!is_capture_device(0x0000_0000, 0x0000_0001));
    // row C2 caps=0x00000001 dev=0x00000000 => 1
    assert!(is_capture_device(0x0000_0001, 0x0000_0000));
    // row C3 caps=0x80000000 dev=0x00000001 => 1
    assert!(is_capture_device(0x8000_0000, 0x0000_0001));
    // row C4 caps=0x80000001 dev=0x00000000 => 0
    assert!(!is_capture_device(0x8000_0001, 0x0000_0000));
    // row C5 caps=0x80000000 dev=0x00000000 => 0
    assert!(!is_capture_device(0x8000_0000, 0x0000_0000));
    // row C6 caps=0x00000002 dev=0x00000000 => 0
    assert!(!is_capture_device(0x0000_0002, 0x0000_0000));
    // row C7 caps=0xfffffffe dev=0xffffffff => 1
    assert!(is_capture_device(0xffff_fffe, 0xffff_ffff));
    // row C8 caps=0x7ffffffe dev=0x00000001 => 0
    assert!(!is_capture_device(0x7fff_fffe, 0x0000_0001));
    // row C9 caps=0x7fffffff dev=0x00000000 => 1
    assert!(is_capture_device(0x7fff_ffff, 0x0000_0000));
}

#[test]
fn oracle_format_driver_version_rows() {
    // row V1 driver="uvcvideo" version=0x00050f08 => "uvcvideo v5.15.8"
    assert_eq!(
        format_driver_version(b"uvcvideo", 0x0005_0f08),
        b"uvcvideo v5.15.8".to_vec()
    );
    // row V2 driver="" version=0x00000000 => " v0.0.0"
    assert_eq!(format_driver_version(b"", 0x0000_0000), b" v0.0.0".to_vec());
    // row V3 driver="drv" version=0xffffffff => "drv v255.255.255"
    assert_eq!(
        format_driver_version(b"drv", 0xffff_ffff),
        b"drv v255.255.255".to_vec()
    );
    // row V4 driver="drv" version=0x01000000 => "drv v0.0.0"
    assert_eq!(
        format_driver_version(b"drv", 0x0100_0000),
        b"drv v0.0.0".to_vec()
    );
    // row V5 driver="drv" version=0x00ff0000 => "drv v255.0.0"
    assert_eq!(
        format_driver_version(b"drv", 0x00ff_0000),
        b"drv v255.0.0".to_vec()
    );
    // row V6 driver="drv" version=0x0000ff00 => "drv v0.255.0"
    assert_eq!(
        format_driver_version(b"drv", 0x0000_ff00),
        b"drv v0.255.0".to_vec()
    );
    // row V7 driver="ab" (C string with an embedded NUL) version=0x00010203 => "ab v1.2.3"
    assert_eq!(
        format_driver_version(b"ab\0cd", 0x0001_0203),
        b"ab v1.2.3".to_vec()
    );
    // row V8 driver="abcdefghijklmno" version=0x00010203 => "abcdefghijklmno v1.2.3"
    assert_eq!(
        format_driver_version(b"abcdefghijklmno", 0x0001_0203),
        b"abcdefghijklmno v1.2.3".to_vec()
    );
    // row V9 driver=40 x's version=0x00010203 => 31 x's (ret=47 in C, untruncated)
    assert_eq!(
        format_driver_version(&[b'x'; 40], 0x0001_0203),
        vec![b'x'; 31]
    );
    // row V10 driver="\xff\x80" version=0x00000001 => "\xff\x80 v0.0.1"
    assert_eq!(
        format_driver_version(b"\xff\x80", 0x0000_0001),
        b"\xff\x80 v0.0.1".to_vec()
    );
}
