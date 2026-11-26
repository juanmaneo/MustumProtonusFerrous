//! Edge-case tests for the ported `printproc.c` logic.
//!
//! These cover boundaries and branches that the oracle rows do not reach. Expectations that
//! are not oracle-derived say so in a comment.

use super::*;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn code(r: Result<(), PpError>) -> Result<(), u32> {
    r.map_err(PpError::win32_code)
}

#[test]
fn enum_writes_exact_bytes_x64() {
    // Not oracle-derived: the harness printed the pName offsets and the strings, not the raw
    // bytes. This follows from the C sizes: two 8-byte entries, then RAW and TEXT with NULs.
    let mut buf = [0xAAu8; 40];
    let mut needed = 0;
    let mut no = 0;
    let got = enum_print_processor_datatypes_w(
        Arch::X64,
        1,
        40,
        Some(&mut buf[..]),
        Some(&mut needed),
        Some(&mut no),
    );
    assert_eq!(got, Ok(()));
    assert_eq!((needed, no), (34, 2));
    let want: [u8; 34] = [
        0x10, 0, 0, 0, 0, 0, 0, 0, // info[0].pName = 16
        0x18, 0, 0, 0, 0, 0, 0, 0, // info[1].pName = 24
        0x52, 0, 0x41, 0, 0x57, 0, 0, 0, // "RAW" and NUL
        0x54, 0, 0x45, 0, 0x58, 0, 0x54, 0, 0, 0, // "TEXT" and NUL
    ];
    assert_eq!(&buf[..34], &want[..]);
    // Not oracle-derived: bytes past `needed` are not written.
    assert!(buf[34..].iter().all(|&b| b == 0xAA));
}

#[test]
fn enum_writes_exact_bytes_x86() {
    // Not oracle-derived, as above. The x86 entries are 4 bytes, so the strings start at 8.
    let mut buf = [0xAAu8; 40];
    let mut needed = 0;
    let mut no = 0;
    let got = enum_print_processor_datatypes_w(
        Arch::X86,
        1,
        40,
        Some(&mut buf[..]),
        Some(&mut needed),
        Some(&mut no),
    );
    assert_eq!(got, Ok(()));
    assert_eq!((needed, no), (26, 2));
    let want: [u8; 26] = [
        0x08, 0, 0, 0, // info[0].pName = 8
        0x10, 0, 0, 0, // info[1].pName = 16
        0x52, 0, 0x41, 0, 0x57, 0, 0, 0, // "RAW" and NUL
        0x54, 0, 0x45, 0, 0x58, 0, 0x54, 0, 0, 0, // "TEXT" and NUL
    ];
    assert_eq!(&buf[..26], &want[..]);
    assert!(buf[26..].iter().all(|&b| b == 0xAA));
}

#[test]
fn enum_empty_buffer_with_zero_size_is_insufficient() {
    // Not oracle-derived: the oracle only passed NULL with size 0. An empty slice takes the
    // same insufficient-buffer path.
    let mut needed = 0;
    let mut no = 7;
    let got = enum_print_processor_datatypes_w(
        Arch::X64,
        1,
        0,
        Some(&mut [][..]),
        Some(&mut needed),
        Some(&mut no),
    );
    assert_eq!(code(got), Err(122));
    assert_eq!((needed, no), (34, 0));
}

#[test]
fn enum_both_out_params_null_is_invalid_parameter() {
    // Not oracle-derived: the oracle only passed one NULL out-parameter at a time.
    assert_eq!(
        code(enum_print_processor_datatypes_w(
            Arch::X86,
            1,
            26,
            None,
            None,
            None
        )),
        Err(87)
    );
}

#[test]
fn check_open_port_is_checked_for_presence_only() {
    // Not oracle-derived: the oracle always passed a non-empty port. The C code only tests
    // whether `port` is NULL, so an empty string is accepted.
    let empty: [u16; 0] = [];
    let datatype = wide("RAW");
    let open = OpenData {
        datatype: Some(&datatype),
    };
    assert_eq!(
        code(check_open_print_processor(Some(&empty), Some(&open))),
        Ok(())
    );
}

#[test]
fn check_open_datatype_must_match_exactly() {
    // Not oracle-derived: these prefixes, suffixes and leading NULs are not in the oracle rows.
    // wcscmp stops at the first NUL, so a leading NUL leaves an empty string.
    for bad in ["RA", "TEXT ", "\0RAW", "Text", "RAWRAW"] {
        let datatype = wide(bad);
        let open = OpenData {
            datatype: Some(&datatype),
        };
        let port = wide("LPT1:");
        assert_eq!(
            code(check_open_print_processor(Some(&port), Some(&open))),
            Err(1804),
            "{bad:?}"
        );
    }
}

#[test]
fn check_handle_data_rejects_near_magic_values() {
    // Not oracle-derived: the oracle used magic 0. PP_MAGIC is the #define in printproc.c.
    assert_eq!(PP_MAGIC, 0x9521_73fd);
    for magic in [PP_MAGIC ^ 1, PP_MAGIC + 1, u32::MAX, 0] {
        assert_eq!(code(check_handle_data(Some(magic))), Err(6), "{magic:#x}");
    }
}

#[test]
fn win32_codes_match_winerror_h() {
    // Not an oracle row: the numbers come from Wine's include/winerror.h, and the oracle's
    // err column prints the same values for the paths that reach them.
    assert_eq!(PpError::InvalidHandle.win32_code(), 6);
    assert_eq!(PpError::InvalidParameter.win32_code(), 87);
    assert_eq!(PpError::InsufficientBuffer.win32_code(), 122);
    assert_eq!(PpError::InvalidDatatype.win32_code(), 1804);
}

#[test]
fn pointer_sizes_match_harness() {
    // The harness printed ptr=8 for the x64 build and ptr=4 for the x86 build.
    assert_eq!(Arch::X64.pointer_size(), 8);
    assert_eq!(Arch::X86.pointer_size(), 4);
}
