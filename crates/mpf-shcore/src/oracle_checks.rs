//! Replays of Wine's oracle rows. The oracle compiles the verbatim `dlls/shcore/main.c` in
//! `target/canon/shcore/` with `-fsanitize=address,undefined`. It mocks only the outside world: the
//! msvcrt heap, the registry, `CreateFileW`, the version queries and `wcstol`. Each test names the
//! row of `oracle.txt` that it reproduces, and the expected values are that row's `out=` field.
//! Two vectors are left out because the sanitizers reject them: `SHCreateMemStream(NULL, n)`, a NULL
//! `memcpy`, and a `Seek` with an `INT64_MAX` offset, a signed overflow.

use super::*;

pub(crate) const STGM_READ: u32 = 0x0000;
pub(crate) const STGM_WRITE: u32 = 0x0001;
pub(crate) const STGM_READWRITE: u32 = 0x0002;
pub(crate) const STGM_SHARE_DENY_NONE: u32 = 0x0040;
pub(crate) const STGM_SHARE_DENY_READ: u32 = 0x0030;
pub(crate) const STGM_SHARE_DENY_WRITE: u32 = 0x0020;
pub(crate) const STGM_SHARE_EXCLUSIVE: u32 = 0x0010;
pub(crate) const STGM_CREATE: u32 = 0x1000;
pub(crate) const STGM_TRANSACTED: u32 = 0x0001_0000;
pub(crate) const STGM_CONVERT: u32 = 0x0002_0000;
pub(crate) const STGM_DELETEONRELEASE: u32 = 0x0400_0000;

pub(crate) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

pub(crate) fn argv(parts: &[&str]) -> Vec<Vec<u16>> {
    parts.iter().map(|&part| wide(part)).collect()
}

pub(crate) fn version(platform: u32, major: u32, minor: u32) -> OsVersion {
    OsVersion {
        platform,
        major,
        minor,
        wow64: false,
        tablet_pc: false,
        media_center: false,
    }
}

/// The features `0..=0x25` for which `IsOS` is true, as the oracle's `mask` bits.
pub(crate) fn feature_mask(os: &OsVersion) -> u64 {
    (0..=0x25u32)
        .filter(|&feature| is_os(feature, os))
        .fold(0, |mask, feature| mask | (1u64 << feature))
}

pub(crate) fn open(access: u32, share: u32, disposition: u32) -> Result<OpenFlags, Hresult> {
    Ok(OpenFlags {
        access,
        share,
        disposition,
    })
}

#[test]
fn oracle_cmd1_empty_line_gives_the_module_path() {
    assert_eq!(
        command_line_to_argv(&[], &wide(r"C:\test\app.exe")),
        argv(&[r"C:\test\app.exe"])
    );
}

#[test]
fn oracle_cmd2_quoted_exe_then_quoted_and_trailing_blanks() {
    let line = wide(r#""C:\Program Files\app.exe" -a "b c" d   "#);
    assert_eq!(
        command_line_to_argv(&line, &[]),
        argv(&[r"C:\Program Files\app.exe", "-a", "b c", "d"])
    );
}

#[test]
fn oracle_cmd3_escaped_quote_and_literal_backslashes() {
    let line = wide(r#"app a\"b c\\ d"#);
    assert_eq!(
        command_line_to_argv(&line, &[]),
        argv(&["app", r#"a"b"#, r"c\\", "d"])
    );
}

#[test]
fn oracle_cmd4_doubled_quotes_inside_an_argument() {
    let line = wide(r#"app "x""y" "" z"#);
    assert_eq!(
        command_line_to_argv(&line, &[]),
        argv(&["app", r#"x"y ""#, "z"])
    );
}

#[test]
fn oracle_ansi1_copies_a_short_source_with_its_terminator() {
    let mut dest = [b'#'; 8];
    assert_eq!(sh_ansi_to_ansi(Some(b"hello\0"), &mut dest), 6);
    assert_eq!(&dest[..6], b"hello\0");
}

#[test]
fn oracle_ansi2_truncates_and_returns_zero() {
    let mut dest = [b'#'; 8];
    assert_eq!(sh_ansi_to_ansi(Some(b"hello\0"), &mut dest[..4]), 0);
    assert_eq!(&dest[..4], b"hel\0");
}

#[test]
fn oracle_uni1_truncates_a_wide_source_and_returns_zero() {
    let src = wide("abcdef\0");
    let mut dest = [u16::from(b'#'); 8];
    assert_eq!(sh_unicode_to_unicode(Some(&src), &mut dest[..4]), 0);
    assert_eq!(dest[..4], wide("abc\0")[..]);
}

#[test]
fn oracle_mem1_reads_in_chunks_then_reports_the_end() {
    let mut s = MemStream::new(Some(b"abcdef"));
    let mut buf = [0u8; 64];
    assert_eq!(s.read(&mut buf[..4]), (S_OK, 4));
    assert_eq!(&buf[..4], b"abcd");
    assert_eq!(s.read(&mut buf[..4]), (S_OK, 2));
    assert_eq!(&buf[..2], b"ef");
    assert_eq!(s.read(&mut buf[..4]), (S_FALSE, 0));
    assert_eq!(s.stat().size, 6);
}

#[test]
fn oracle_mem2_seek_from_end_then_overwrite() {
    let mut s = MemStream::new(Some(b"abcdef"));
    assert_eq!(s.seek(-2, STREAM_SEEK_END), Ok(4));
    assert_eq!(s.write(b"XY"), Ok(()));
    assert_eq!(s.seek(0, STREAM_SEEK_SET), Ok(0));
    let mut buf = [0u8; 10];
    assert_eq!(s.read(&mut buf), (S_OK, 6));
    assert_eq!(&buf[..6], b"abcdXY");
    assert_eq!(s.stat().size, 6);
}

#[test]
fn oracle_mem3_write_past_the_end_zero_fills_the_gap() {
    let mut s = MemStream::new(Some(b"ab"));
    assert_eq!(s.seek(5, STREAM_SEEK_SET), Ok(5));
    assert_eq!(s.write(b"Z"), Ok(()));
    assert_eq!(s.seek(0, STREAM_SEEK_SET), Ok(0));
    let mut buf = [0u8; 10];
    assert_eq!(s.read(&mut buf), (S_OK, 6));
    assert_eq!(&buf[..6], b"ab\0\0\0Z");
    assert_eq!(s.stat().size, 6);
}

#[test]
fn oracle_mem4_set_size_then_seek_errors() {
    let mut s = MemStream::new(Some(b"abc"));
    assert_eq!(s.set_size(1), Ok(()));
    assert_eq!(s.set_size(4), Ok(()));
    assert_eq!(s.seek(0, STREAM_SEEK_SET), Ok(0));
    let mut buf = [0u8; 8];
    assert_eq!(s.read(&mut buf), (S_OK, 4));
    assert_eq!(&buf[..4], b"a\0\0\0");
    assert_eq!(s.seek(-10, STREAM_SEEK_CUR), Err(STG_E_INVALIDFUNCTION));
    assert_eq!(s.seek(0, 9), Err(STG_E_INVALIDPARAMETER));
    assert_eq!(s.stat().size, 4);
}

#[test]
fn oracle_mem5_set_size_zero_fails_and_keeps_the_stream() {
    let mut s = MemStream::new(Some(b"abc"));
    assert_eq!(s.set_size(0), Err(STG_E_INSUFFICIENTMEMORY));
    assert_eq!(s.stat().size, 3);
}

#[test]
fn oracle_open1_read_write_existing() {
    assert_eq!(
        stream_on_file_ex_flags(STGM_READWRITE, false, false),
        open(0xc000_0000, 3, 3)
    );
}

#[test]
fn oracle_open2_create_always_with_deny_read() {
    assert_eq!(
        stream_on_file_ex_flags(STGM_WRITE | STGM_CREATE | STGM_SHARE_DENY_READ, true, false),
        open(0xc000_0000, 2, 2)
    );
}

#[test]
fn oracle_open3_read_create_new_with_deny_write() {
    assert_eq!(
        stream_on_file_ex_flags(STGM_READ | STGM_SHARE_DENY_WRITE, true, false),
        open(0x8000_0000, 1, 1)
    );
}

#[test]
fn oracle_open4_wide_form_rejects_convert() {
    assert_eq!(
        stream_on_file_w_flags(STGM_READ | STGM_CONVERT),
        Err(E_INVALIDARG)
    );
}

#[test]
fn oracle_reg1_parses_a_decimal_value() {
    assert_eq!(reg_int_value(Some(&wide("42")), 7), 42);
}

#[test]
fn oracle_reg2_saturates_an_overflowing_value() {
    assert_eq!(reg_int_value(Some(&wide("99999999999")), 7), 2_147_483_647);
}

#[test]
fn oracle_os1_windows_95() {
    assert_eq!(feature_mask(&version(1, 4, 0)), 0x1401_0005);
}

#[test]
fn oracle_os2_windows_98() {
    assert_eq!(feature_mask(&version(1, 4, 10)), 0x1400_0065);
}

#[test]
fn oracle_os3_nt4() {
    assert_eq!(feature_mask(&version(2, 4, 0)), 0x1_b5b0_1e0a);
}

#[test]
fn oracle_os4_windows_2000() {
    assert_eq!(feature_mask(&version(2, 5, 0)), 0x1_b5f0_1f9a);
}

#[test]
fn oracle_os5_windows_xp_under_wow64_with_media_center() {
    let os = OsVersion {
        wow64: true,
        media_center: true,
        ..version(2, 5, 1)
    };
    assert_eq!(feature_mask(&os), 0x9_f7fc_1f9a);
}

#[test]
fn oracle_os6_vista_tablet() {
    let os = OsVersion {
        tablet_pc: true,
        ..version(2, 6, 0)
    };
    assert_eq!(feature_mask(&os), 0x23_b5f0_1f9a);
}
