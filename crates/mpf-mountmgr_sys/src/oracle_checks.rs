//! Rows printed by the verbatim Wine oracle (`target/canon/mountmgr.sys`), copied
//! unchanged. The sanitizer-flagged vectors are not here: see `edge_checks.rs`.

use super::*;
use crate::errno::{
    STATUS_ACCESS_DENIED, STATUS_DEVICE_BUSY, STATUS_IO_TIMEOUT, STATUS_NO_MEDIA_IN_DEVICE,
    STATUS_OBJECT_NAME_NOT_FOUND, STATUS_SHARING_VIOLATION, STATUS_UNSUCCESSFUL,
};

fn guid(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Guid {
    Guid {
        data1,
        data2,
        data3,
        data4,
    }
}

fn toc(first_track: u8, last_track: u8, addresses: &[[u8; 4]]) -> Toc {
    Toc {
        first_track,
        last_track,
        addresses: addresses.to_vec(),
    }
}

#[test]
fn errno_rows() {
    let rows = [
        (1, STATUS_ACCESS_DENIED),         // EPERM
        (11, STATUS_SHARING_VIOLATION),    // EAGAIN
        (16, STATUS_DEVICE_BUSY),          // EBUSY
        (2, STATUS_OBJECT_NAME_NOT_FOUND), // ENOENT
        (8, STATUS_UNSUCCESSFUL),          // ENOEXEC, unmapped
        (123, STATUS_NO_MEDIA_IN_DEVICE),  // ENOMEDIUM
        (62, STATUS_IO_TIMEOUT),           // ETIME
    ];
    for (err, status) in rows {
        assert_eq!(errno_to_status(err), status, "errno {err}");
    }
}

#[test]
fn uuid_rows() {
    let tail = [0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc];
    assert_eq!(
        parse_uuid(b"12345678-1234-1234-1234-123456789abc"),
        Some(guid(0x1234_5678, 0x1234, 0x1234, tail))
    );
    assert_eq!(
        parse_uuid(b"ABCDEF01-2345-6789-abcd-EF0123456789"),
        Some(guid(
            0xabcd_ef01,
            0x2345,
            0x6789,
            [0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89]
        ))
    );
    assert_eq!(parse_uuid(b"12345678-1234-1234-1234-123456789abg"), None);
    assert_eq!(
        parse_uuid(b" 1234567-1234-1234-1234-123456789abc"),
        Some(guid(0x0123_4567, 0x1234, 0x1234, tail))
    );
    assert_eq!(
        parse_uuid(b"-0000001-1234-1234-1234-123456789abc"),
        Some(guid(0xffff_ffff, 0x1234, 0x1234, tail))
    );
    assert_eq!(
        parse_uuid(b"12345678--abc-1234-1234-123456789abc"),
        Some(guid(0x1234_5678, 0xf544, 0x1234, tail))
    );
    assert_eq!(
        parse_uuid(b"1234-567g"),
        Some(guid(0, 0x1234, 0x0567, [0; 8]))
    );
}

#[test]
fn msf_rows() {
    assert_eq!(frame_to_msf(4501), [1, 0, 1]);
    assert_eq!(frame_to_msf(2_000_000), [188, 26, 50]);
}

#[test]
fn track_rows() {
    let disc = toc(
        1,
        3,
        &[[0, 0, 2, 0], [0, 3, 10, 45], [0, 7, 20, 0], [0, 15, 0, 0]],
    );
    assert_eq!(track_to_frame(&disc, 2), Some(14295));
    assert_eq!(track_to_frame(&disc, 4), Some(67500));
}

#[test]
fn serial_rows() {
    let one_track = toc(1, 1, &[[0, 0, 2, 0], [0, 5, 30, 12]]);
    assert_eq!(audio_cd_serial(&one_track), Some(25124));

    let two_tracks = toc(2, 3, &[[0, 0, 2, 0], [0, 3, 10, 45], [0, 15, 0, 0]]);
    assert_eq!(audio_cd_serial(&two_tracks), Some(267_075));

    let three_tracks = toc(
        1,
        3,
        &[[0, 0, 2, 0], [0, 3, 10, 45], [0, 7, 20, 0], [0, 15, 0, 0]],
    );
    assert_eq!(audio_cd_serial(&three_tracks), Some(663_597));
}

#[test]
fn credential_rows() {
    let mut wide = [0u8; 16];
    wide[..4].copy_from_slice(&[b'A', 0, b'B', 0]);
    assert!(check_credential_string(&wide, 6, 0));
    assert!(!check_credential_string(&wide, 4, 0));
    assert!(!check_credential_string(&wide, 6, 12));
}
