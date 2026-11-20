//! Checks on the edges of each function: every arm of the errno table, each way a
//! UUID can be rejected, the TOC bounds, and the credential string limits.
//! Where the C code is the reference, the expected values come from the oracle.

use super::*;
use crate::errno::{
    STATUS_ACCESS_DENIED, STATUS_ACCESS_VIOLATION, STATUS_DEVICE_NOT_READY,
    STATUS_DIRECTORY_NOT_EMPTY, STATUS_DISK_FULL, STATUS_ILLEGAL_FUNCTION,
    STATUS_INVALID_DEVICE_REQUEST, STATUS_INVALID_HANDLE, STATUS_INVALID_PARAMETER,
    STATUS_NOT_SUPPORTED, STATUS_NO_SUCH_DEVICE, STATUS_OBJECT_PATH_NOT_FOUND,
    STATUS_PIPE_DISCONNECTED, STATUS_REPARSE_POINT_NOT_RESOLVED, STATUS_TOO_MANY_OPENED_FILES,
    STATUS_UNSUCCESSFUL,
};
use std::fmt::Debug;

fn check_traits<T: Clone + Debug + PartialEq>(value: &T) {
    assert_eq!(&value.clone(), value);
    assert!(!format!("{value:?}").is_empty());
}

fn guid(data1: u32, data2: u16, data3: u16) -> Guid {
    Guid {
        data1,
        data2,
        data3,
        ..Guid::default()
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
fn every_errno_arm_maps_as_the_switch_does() {
    let arms = [
        (1, STATUS_ACCESS_DENIED),
        (30, STATUS_ACCESS_DENIED),
        (13, STATUS_ACCESS_DENIED),
        (9, STATUS_INVALID_HANDLE),
        (28, STATUS_DISK_FULL),
        (20, STATUS_OBJECT_PATH_NOT_FOUND),
        (21, STATUS_INVALID_DEVICE_REQUEST),
        (24, STATUS_TOO_MANY_OPENED_FILES),
        (23, STATUS_TOO_MANY_OPENED_FILES),
        (22, STATUS_INVALID_PARAMETER),
        (39, STATUS_DIRECTORY_NOT_EMPTY),
        (32, STATUS_PIPE_DISCONNECTED),
        (104, STATUS_PIPE_DISCONNECTED),
        (5, STATUS_DEVICE_NOT_READY),
        (6, STATUS_NO_SUCH_DEVICE),
        (25, STATUS_NOT_SUPPORTED),
        (95, STATUS_NOT_SUPPORTED),
        (14, STATUS_ACCESS_VIOLATION),
        (29, STATUS_ILLEGAL_FUNCTION),
        (40, STATUS_REPARSE_POINT_NOT_RESOLVED),
    ];
    for (err, status) in arms {
        assert_eq!(errno_to_status(err), status, "errno {err}");
    }
}

#[test]
fn unmapped_errno_is_unsuccessful() {
    assert_eq!(errno_to_status(0), STATUS_UNSUCCESSFUL);
    assert_eq!(errno_to_status(17), STATUS_UNSUCCESSFUL);
    assert_eq!(errno_to_status(-1), STATUS_UNSUCCESSFUL);
    assert_eq!(errno_to_status(i32::MAX), STATUS_UNSUCCESSFUL);
}

#[test]
fn uuid_accepts_uppercase_and_short_forms_as_glibc_scans_them() {
    let tail = [0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc];
    assert_eq!(
        parse_uuid(b"12345678-1234-1234-1234-123456789ABC"),
        Some(Guid {
            data1: 0x1234_5678,
            data2: 0x1234,
            data3: 0x1234,
            data4: tail,
        })
    );
    assert_eq!(parse_uuid(b"0x12-5678"), Some(guid(0, 0x12, 0x5678)));
    assert_eq!(parse_uuid(b"-123-5678"), Some(guid(0, 0xfedd, 0x5678)));
    assert_eq!(parse_uuid(b"1234-zzzz"), None);
}

#[test]
fn uuid_accepts_explicit_sign_and_leading_white_space() {
    assert_eq!(
        parse_uuid(b"+1234567-1234-1234-1234-123456789abc"),
        Some(Guid {
            data1: 0x0123_4567,
            data2: 0x1234,
            data3: 0x1234,
            data4: [0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc],
        })
    );
    assert_eq!(
        parse_uuid(b"12345678-+234-1234-1234-123456789abc"),
        Some(Guid {
            data1: 0x1234_5678,
            data2: 0x0234,
            data3: 0x1234,
            data4: [0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc],
        })
    );
    assert_eq!(
        parse_uuid(b"\t\n\x0b\x0c\r123-1234-1234-1234-123456789abc"),
        Some(Guid {
            data1: 0x123,
            data2: 0x1234,
            data3: 0x1234,
            data4: [0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc],
        })
    );
}

#[test]
fn uuid_rejects_malformed_input() {
    assert_eq!(parse_uuid(b""), None);
    assert_eq!(parse_uuid(b"1234-"), None);
    assert_eq!(parse_uuid(b"12345678-"), None);
    assert_eq!(parse_uuid(b"12345678x1234-1234-1234-123456789abc"), None);
    assert_eq!(parse_uuid(b"1234567g-1234-1234-1234-123456789abc"), None);
    assert_eq!(parse_uuid(b"12345678-1 34-1234-1234-123456789abc"), None);
    assert_eq!(parse_uuid(b"12345678-1234-zzzz-1234-123456789abc"), None);
    assert_eq!(parse_uuid(b"12345678-1234-1234-1234-1234567g9abc"), None);
    assert_eq!(parse_uuid(b"12345678-1234-1234-1234-g23456789abc"), None);
    assert_eq!(parse_uuid(b"12345678-1234-1234-1234-1234567 9abc"), None);
    assert_eq!(parse_uuid(b"12345678-1234-1234-1234-123456789abcd"), None);
    assert_eq!(parse_uuid(b"zzzzzzzz-1234-1234-1234-123456789abc"), None);
    assert_eq!(parse_uuid(b"12345678-zzzz-1234-1234-123456789abc"), None);
    assert_eq!(parse_uuid(b"zzzz-5678"), None);
    assert_eq!(parse_uuid(b"12g4-5678"), None);
}

#[test]
fn uuid_stops_at_the_first_nul() {
    assert_eq!(parse_uuid(b"1234\0-5678"), None);
    assert_eq!(
        parse_uuid(b"12345678-1234-1234-1234-123456789abc\0junk"),
        Some(Guid {
            data1: 0x1234_5678,
            data2: 0x1234,
            data3: 0x1234,
            data4: [0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc],
        })
    );
}

#[test]
fn msf_edges() {
    assert_eq!(frame_to_msf(0), [0, 0, 0]);
    assert_eq!(frame_to_msf(75), [0, 1, 0]);
    assert_eq!(frame_to_msf(4499), [0, 59, 74]);
    assert_eq!(frame_to_msf(u32::MAX), [69, 10, 45]);
}

#[test]
fn track_lookup_stays_within_the_entries() {
    let disc = toc(
        1,
        3,
        &[[0, 0, 2, 0], [0, 3, 10, 45], [0, 7, 20, 0], [0, 15, 0, 0]],
    );
    assert_eq!(track_to_frame(&disc, 1), Some(150));
    assert_eq!(track_to_frame(&disc, 0), None);
    assert_eq!(track_to_frame(&disc, 5), None);
}

#[test]
fn serial_edges() {
    let three_no_lead_out = toc(1, 3, &[[0, 0, 2, 0], [0, 3, 10, 45], [0, 7, 20, 0]]);
    assert_eq!(audio_cd_serial(&three_no_lead_out), Some(663_597));

    let four = toc(1, 4, &[[0, 255, 59, 74]; 5]);
    assert_eq!(audio_cd_serial(&four), Some(66_907_432));

    assert_eq!(audio_cd_serial(&toc(3, 1, &[[0, 0, 2, 0]; 4])), None);
    assert_eq!(audio_cd_serial(&toc(5, 4, &[[0, 0, 2, 0]])), None);
    assert_eq!(
        audio_cd_serial(&toc(1, 3, &[[0, 0, 2, 0], [0, 3, 10, 45]])),
        None
    );
    assert_eq!(audio_cd_serial(&toc(1, 1, &[[0, 0, 2, 0]])), None);
}

#[test]
fn credential_string_limits() {
    let zero = [0u8; 16];
    assert!(!check_credential_string(&zero, 0, 0));
    assert!(!check_credential_string(&zero, 3, 0));
    assert!(check_credential_string(&zero, 4, 12));
    assert!(!check_credential_string(&zero[..15], 4, 12));
    assert!(!check_credential_string(&zero, 2, 16));

    assert!(!check_credential_string(&[0, 0, 0, 1], 4, 0));
    assert!(!check_credential_string(&[0, 0, 1, 0], 4, 0));
    assert!(check_credential_string(&[1, 0, 0, 0], 4, 0));
}

#[test]
fn credential_offset_overflow_is_rejected() {
    let zero = [0u8; 16];
    assert!(!check_credential_string(&zero, 4, 0xFFFF_FFFE));
    assert!(!check_credential_string(&zero, 4, 0xFFFF_FFFC));
}

#[test]
fn value_types_are_usable() {
    check_traits(&guid(1, 2, 3));
    check_traits(&toc(1, 1, &[[0, 0, 2, 0]]));
}
