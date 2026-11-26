//! Oracle-derived test vectors for the ported localspl logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/localspl/localmon.c`, `provider.c` and `localspl_private.h` with
//! `gcc -fshort-wchar -fsanitize=address,undefined -g` under `target/canon/localspl/`
//! and running the harness. It printed 24 rows; ASan and UBSan reported nothing,
//! so none are excluded. The host is x86_64, so the default environment row
//! uses `PrintEnv::X64`, which the tests pass explicitly.

use super::*;

fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn text(s: &[u16]) -> String {
    String::from_utf16(s).unwrap()
}

/// Maps a `ServerName` back to the C return value and the text left in `target`.
fn c_view(result: ServerName) -> (i32, String) {
    match result {
        ServerName::Empty => (0, String::new()),
        ServerName::TooLong(len) => (-(len as i32), String::new()),
        ServerName::Local(server) => (0, text(server)),
        ServerName::Remote(server) => (server.len() as i32, text(server)),
    }
}

#[test]
fn oracle_get_type_from_name_rows() {
    // (name, get_type_from_name(name, FALSE)); 0 is PORT_IS_UNKNOWN
    const ROWS: &[(&str, u32)] = &[
        ("LPT1:", 1),
        ("COM2", 2),
        ("FILE:", 3),
        ("/dev/lp0", 5),
        ("|lpr -P x", 6),
        ("CUPS:printer", 7),
        ("LPR:host", 8),
        ("lpt1", 0),
    ];
    for &(name, id) in ROWS {
        let w = utf16(name);
        assert_eq!(classify_port_name(&w).map_or(0, |p| p as u32), id, "{name}");
    }
}

#[test]
fn oracle_multi_sz_lenw_rows() {
    // (buffer, multi_sz_lenW bytes)
    const ROWS: &[(&str, usize)] = &[("a\0b\0\0", 10), ("\0\0", 4), ("abc\0\0", 10)];
    for &(buf, bytes) in ROWS {
        let w = utf16(buf);
        assert_eq!(multi_sz_len_bytes(Some(&w)), Some(bytes), "{buf:?}");
    }
}

#[test]
fn oracle_copy_servername_from_name_rows() {
    // (name, computer name, C return value, text copied into target)
    const ROWS: &[(&str, &str, i32, &str)] = &[
        ("\\\\srv\\share", "WINEBOX", 3, "srv"),
        ("\\\\srv", "WINEBOX", 3, "srv"),
        ("\\\\winebox\\x", "WineBox", 0, "winebox"),
        ("\\\\abcdefghijklmnop\\x", "WINEBOX", -16, ""),
        ("plain\\path", "WINEBOX", 0, ""),
    ];
    for &(name, computer, ret, target) in ROWS {
        let n = utf16(name);
        let c = utf16(computer);
        let got = copy_servername_from_name(Some(&n), Some(&c));
        assert_eq!(
            c_view(got),
            (ret, target.to_string()),
            "{name} on {computer}"
        );
    }
}

#[test]
fn oracle_get_basename_from_name_rows() {
    // (name, get_basename_from_name)
    const ROWS: &[(&str, Option<&str>)] = &[
        ("\\\\srv\\share\\x", Some("share\\x")),
        ("\\\\srv\\", None),
        ("plain", Some("plain")),
    ];
    for &(name, expected) in ROWS {
        let w = utf16(name);
        assert_eq!(
            get_basename_from_name(Some(&w)).map(text),
            expected.map(String::from),
            "{name}"
        );
    }
}

#[test]
fn oracle_get_file_part_rows() {
    // (name, get_file_part)
    const ROWS: &[(&str, &str)] = &[("C:\\dir\\file.txt", "file.txt"), ("C:\\dir\\", "")];
    for &(name, expected) in ROWS {
        let w = utf16(name);
        assert_eq!(text(get_file_part(&w)), expected, "{name}");
    }
}

#[test]
fn oracle_validate_env_rows() {
    // (env, win9x, envname); the default row assumes an x86_64 host
    const ROWS: &[(Option<&str>, bool, Option<&str>)] = &[
        (None, false, Some("Windows x64")),
        (Some(""), true, Some("Windows 4.0")),
        (Some("windows nt X86"), false, Some("Windows NT x86")),
    ];
    for &(env, win9x, expected) in ROWS {
        let w = env.map(utf16);
        assert_eq!(
            validate_env(w.as_deref(), win9x, PrintEnv::X64).map(PrintEnv::name),
            expected,
            "{env:?}"
        );
    }
}
