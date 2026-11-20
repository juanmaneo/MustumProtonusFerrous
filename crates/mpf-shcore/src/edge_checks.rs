//! Edge rows from `edge.c`, which runs the same verbatim C and mocks as the oracle, and Rust-only
//! cases at the limits that the C harness does not reach. Row names follow `edge.txt`.

use super::*;
use crate::oracle_checks::{
    argv, feature_mask, open, version, wide, STGM_CREATE, STGM_DELETEONRELEASE, STGM_READ,
    STGM_READWRITE, STGM_SHARE_DENY_NONE, STGM_SHARE_DENY_WRITE, STGM_SHARE_EXCLUSIVE,
    STGM_TRANSACTED, STGM_WRITE,
};

#[test]
fn edge_cmd_rows_match_the_oracle() {
    let module = wide("module.exe");
    let rows: &[(&str, &[&str])] = &[
        ("\"abc def", &["abc def"]),
        ("\"a\"b", &["a", "b"]),
        ("app\tx", &["app", "x"]),
        ("app   ", &["app"]),
        ("app a \t b", &["app", "a", "b"]),
        ("app a\"\"b", &["app", "ab"]),
        ("app a\"\"\"b", &["app", "a\"b"]),
        (r#"app a\\"b c""#, &["app", r"a\b c"]),
        (r#"app a\\\"b"#, &["app", r#"a\"b"#]),
        ("app \"\"", &["app", ""]),
        ("app a\"", &["app", "a"]),
        ("app\"x\"", &[r#"app"x""#]),
        ("\"", &[""]),
        ("\"a\" \"b\"c d", &["a", "bc", "d"]),
        (r#"app a\\""#, &["app", r"a\"]),
        (r#"app "a\"b" c"#, &["app", "a\"b", "c"]),
        (r#""C:\a b\app.exe"x y"#, &[r"C:\a b\app.exe", "x", "y"]),
        ("app \t\t", &["app"]),
        (r#"app "x""""y z"#, &["app", "x\"y", "z"]),
    ];
    for (line, expected) in rows {
        assert_eq!(
            command_line_to_argv(&wide(line), &module),
            argv(expected),
            "{line}"
        );
    }
}

#[test]
fn edge_cmd_empty_input_gives_the_module_path() {
    let module = wide("module.exe");
    assert_eq!(command_line_to_argv(&[], &module), argv(&["module.exe"]));
    assert_eq!(
        command_line_to_argv(&wide(""), &module),
        argv(&["module.exe"])
    );
    assert_eq!(
        command_line_to_argv(&[0, u16::from(b'x')], &module),
        argv(&["module.exe"])
    );
}

#[test]
fn edge_cmd_single_word_lines() {
    assert_eq!(command_line_to_argv(&wide("app"), &[]), argv(&["app"]));
    assert_eq!(command_line_to_argv(&wide("\"app\""), &[]), argv(&["app"]));
}

#[test]
fn edge_ansi_copy_rows_match_the_oracle() {
    let mut dest = [b'#'; 8];
    assert_eq!(sh_ansi_to_ansi(None, &mut dest), 0);
    assert_eq!(dest, [b'#'; 8]);
    assert_eq!(sh_ansi_to_ansi(Some(b"abc\0"), &mut dest[..0]), 0);
    assert_eq!(sh_ansi_to_ansi(Some(b"\0"), &mut dest[..1]), 1);
    assert_eq!(dest[0], 0);
    assert_eq!(sh_ansi_to_ansi(Some(b"abc\0"), &mut dest[..4]), 4);
    assert_eq!(&dest[..4], b"abc\0");
    assert_eq!(sh_ansi_to_ansi(Some(b"abc\0"), &mut dest[..3]), 0);
    assert_eq!(&dest[..3], b"ab\0");
    assert_eq!(sh_ansi_to_ansi(Some(b"abc\0"), &mut dest[..1]), 0);
    assert_eq!(dest[0], 0);
}

#[test]
fn edge_ansi_copy_stops_at_an_embedded_nul() {
    let mut dest = [b'#'; 8];
    assert_eq!(sh_ansi_to_ansi(Some(b"abc\0"), &mut dest[..4]), 4);
    assert_eq!(sh_ansi_to_ansi(Some(b"ab\0cd"), &mut dest), 3);
    assert_eq!(&dest[..3], b"ab\0");
    assert_eq!(sh_ansi_to_ansi(Some(b"abc"), &mut dest), 0);
    assert_eq!(&dest[..4], b"abc\0");
}

#[test]
fn edge_unicode_copy_rows_match_the_oracle() {
    let abc = wide("abc\0");
    let mut dest = [u16::from(b'#'); 8];
    assert_eq!(sh_unicode_to_unicode(None, &mut dest), 0);
    assert_eq!(dest, [u16::from(b'#'); 8]);
    assert_eq!(sh_unicode_to_unicode(Some(&abc), &mut dest[..0]), 0);
    assert_eq!(sh_unicode_to_unicode(Some(&wide("\0")), &mut dest[..1]), 1);
    assert_eq!(dest[0], 0);
    assert_eq!(sh_unicode_to_unicode(Some(&abc), &mut dest[..4]), 4);
    assert_eq!(dest[..4], wide("abc\0")[..]);
    assert_eq!(sh_unicode_to_unicode(Some(&abc), &mut dest[..3]), 0);
    assert_eq!(dest[..3], wide("ab\0")[..]);
    assert_eq!(sh_unicode_to_unicode(Some(&abc), &mut dest[..1]), 0);
    assert_eq!(dest[0], 0);
}

#[test]
fn edge_unicode_copy_rejects_an_unterminated_source() {
    let mut dest = [u16::from(b'#'); 8];
    assert_eq!(sh_unicode_to_unicode(Some(&wide("ab")), &mut dest), 0);
    assert_eq!(dest[..3], wide("ab\0")[..]);
}

#[test]
fn edge_reg_rows_match_the_oracle() {
    let r5 = format!("{}42", "0".repeat(29));
    let r6 = format!("{}42", "0".repeat(30));
    let r7 = "1".repeat(31);
    let r8 = "1".repeat(32);
    assert_eq!(reg_int_value(Some(&wide("")), 7), 7);
    assert_eq!(reg_int_value(Some(&wide("x42")), 7), 7);
    assert_eq!(reg_int_value(Some(&wide("42x9")), 7), 42);
    assert_eq!(reg_int_value(None, 7), 7);
    assert_eq!(reg_int_value(Some(&wide(&r5)), 7), 42);
    assert_eq!(reg_int_value(Some(&wide(&r6)), 7), 7);
    assert_eq!(reg_int_value(Some(&wide(&r7)), 7), i32::MAX);
    assert_eq!(reg_int_value(Some(&wide(&r8)), 7), 7);
    assert_eq!(reg_int_value(Some(&wide("-5")), 7), 7);
}

#[test]
fn edge_reg_rust_only_inputs() {
    let empty: &[u16] = &[];
    assert_eq!(reg_int_value(Some(empty), 7), 7);
    assert_eq!(reg_int_value(Some(&[0]), 7), 7);
    assert_eq!(
        reg_int_value(Some(&[u16::from(b'4'), 0, u16::from(b'2')]), 7),
        4
    );
}

#[test]
fn edge_memstream_overflow_and_high_bits_match_the_oracle() {
    let mut s = MemStream::new(Some(b"abc"));
    assert_eq!(s.seek(0xFFFF_FFF0, STREAM_SEEK_SET), Ok(0xFFFF_FFF0));
    assert_eq!(s.write(&[b'0'; 32]), Err(STG_E_INSUFFICIENTMEMORY));
    assert_eq!(s.stat().size, 3);
    assert_eq!(s.seek(0x1_0000_0005, STREAM_SEEK_SET), Ok(5));
    assert_eq!(s.seek(-1, STREAM_SEEK_SET), Err(STG_E_INVALIDFUNCTION));
    assert_eq!(s.seek(0, STREAM_SEEK_SET), Ok(0));
    assert_eq!(s.seek(-1, STREAM_SEEK_CUR), Err(STG_E_INVALIDFUNCTION));
    assert_eq!(s.seek(2, STREAM_SEEK_END), Ok(5));
    assert_eq!(s.seek(-4, STREAM_SEEK_END), Err(STG_E_INVALIDFUNCTION));
    assert_eq!(s.seek(0, 3), Err(STG_E_INVALIDPARAMETER));
    let mut buf = [0u8; 16];
    assert_eq!(s.read(&mut buf), (S_FALSE, 0));
}

#[test]
fn edge_memstream_zero_write_grows_and_set_size_keeps_low_bits() {
    let mut s = MemStream::new(Some(b"ab"));
    assert_eq!(s.seek(4, STREAM_SEEK_SET), Ok(4));
    assert_eq!(s.write(&[]), Ok(()));
    assert_eq!(
        s.stat(),
        Stat {
            stream_type: 2,
            size: 4,
            grf_mode: 2
        }
    );
    let mut buf = [0u8; 4];
    assert_eq!(s.read(&mut buf), (S_FALSE, 0));
    assert_eq!(s.seek(10, STREAM_SEEK_SET), Ok(10));
    assert_eq!(s.read(&mut buf), (S_FALSE, 0));

    let mut s = MemStream::new(Some(b"ab"));
    assert_eq!(s.set_size(0x1_0000_0002), Ok(()));
    assert_eq!(s.stat().size, 2);
    assert_eq!(s.set_size(5), Ok(()));
    let mut buf = [0u8; 16];
    assert_eq!(s.read(&mut buf), (S_OK, 5));
    assert_eq!(&buf[..5], b"ab\0\0\0");
}

#[test]
fn edge_memstream_rust_only_limits() {
    assert_eq!(MemStream::new(None).stat().size, 0);
    let mut s = MemStream::new(Some(b"abc"));
    assert_eq!(s.seek(1, STREAM_SEEK_SET), Ok(1));
    assert_eq!(
        s.seek(i64::MAX, STREAM_SEEK_CUR),
        Err(STG_E_INVALIDFUNCTION)
    );
    assert_eq!(
        s.seek(i64::MIN, STREAM_SEEK_SET),
        Err(STG_E_INVALIDFUNCTION)
    );
    assert_eq!(
        s.seek(i64::MIN, STREAM_SEEK_END),
        Err(STG_E_INVALIDFUNCTION)
    );
    assert_eq!(s.set_size(1 << 32), Err(STG_E_INSUFFICIENTMEMORY));
    assert_eq!(s.stat().size, 3);
}

#[test]
fn edge_stat_and_open_flags_format_their_fields() {
    let s = MemStream::new(Some(b"ab"));
    assert_eq!(
        format!("{:?}", s.stat()),
        "Stat { stream_type: 2, size: 2, grf_mode: 2 }"
    );
    let flags = stream_on_file_ex_flags(STGM_READ, false, false);
    assert_eq!(
        format!("{flags:?}"),
        "Ok(OpenFlags { access: 2147483648, share: 3, disposition: 3 })"
    );
}

#[test]
fn edge_open_rows_match_the_oracle() {
    assert_eq!(
        stream_on_file_ex_flags(STGM_READ | STGM_SHARE_EXCLUSIVE, false, false),
        open(0x8000_0000, 0, 3)
    );
    assert_eq!(
        stream_on_file_ex_flags(STGM_READWRITE, false, true),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        stream_on_file_ex_flags(0x3, false, false),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        stream_on_file_ex_flags(STGM_READ | 0x80, false, false),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        stream_on_file_ex_flags(STGM_READ | 0x2000, false, false),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        stream_on_file_ex_flags(STGM_CREATE | STGM_WRITE, false, false),
        open(0xc000_0000, 3, 2)
    );
    assert_eq!(
        stream_on_file_ex_flags(STGM_READ | STGM_SHARE_DENY_NONE, true, false),
        open(0x8000_0000, 3, 1)
    );
    assert_eq!(
        stream_on_file_ex_flags(STGM_READ | STGM_SHARE_DENY_NONE, false, false),
        open(0x8000_0000, 3, 3)
    );
    assert_eq!(
        stream_on_file_w_flags(STGM_READWRITE | STGM_DELETEONRELEASE),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        stream_on_file_w_flags(STGM_READWRITE | STGM_TRANSACTED),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        stream_on_file_w_flags(STGM_READ | 0x10),
        open(0x8000_0000, 0, 3)
    );
    assert_eq!(
        stream_on_file_w_flags(STGM_WRITE | STGM_CREATE | STGM_SHARE_DENY_WRITE),
        open(0xc000_0000, 1, 2)
    );
}

#[test]
fn edge_is_os_rows_match_the_oracle() {
    let configs: [(OsVersion, u64); 9] = [
        (version(0, 3, 11), 0x1400_0001),
        (version(1, 4, 90), 0x1402_0025),
        (version(1, 4, 9), 0x1400_0005),
        (version(2, 3, 51), 0x1_b5b0_0002),
        (version(2, 5, 2), 0x1_b7fc_019a),
        (version(2, 5, 1), 0x1_b7fc_1f9a),
        (version(3, 5, 0), 0x1400_0000),
        (
            OsVersion {
                wow64: true,
                tablet_pc: true,
                media_center: true,
                ..version(2, 6, 1)
            },
            0x2b_f7fc_1f9a,
        ),
        (version(2, 0, 0), 0x1_b5b0_1e02),
    ];
    for (os, mask) in &configs {
        assert_eq!(feature_mask(os), *mask);
        assert!(!is_os(0x26, os));
        assert!(!is_os(0xffff_ffff, os));
    }
}
