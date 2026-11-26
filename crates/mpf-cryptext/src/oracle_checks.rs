//! Oracle-derived test vectors for the ported cryptext logic.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `dlls/cryptext/cryptext_main.c`
//! (lines 34-107; the only edit is `L"` to `u"` so that `WCHAR` is 2 bytes) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/cryptext/` and running the harness.
//! All 20 printed rows are kept. ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// `MultiByteToWideChar(CP_ACP)` for ASCII input, as modelled by the harness shim.
fn ascii(b: u8) -> u16 {
    u16::from(b)
}

/// Calls `crypt_ext_open_cer_w` with a query that returns `loaded`. Returns the result and the
/// file name the query saw (`None` if it was never called).
fn open_w(file: Option<&[u16]>, loaded: bool) -> (OpenCer, Option<Option<Vec<u16>>>) {
    let mut seen = None;
    let out = crypt_ext_open_cer_w(file, |f| {
        seen = Some(f.map(<[u16]>::to_vec));
        loaded
    });
    (out, seen)
}

/// Calls `crypt_ext_open_cer` with the harness ASCII mapping and a query that returns `loaded`.
fn open_a(file: Option<&[u8]>, loaded: bool) -> (OpenCer, Option<Option<Vec<u16>>>) {
    let mut seen = None;
    let out = crypt_ext_open_cer(file, ascii, |f| {
        seen = Some(f.map(<[u16]>::to_vec));
        loaded
    });
    (out, seen)
}

fn ok(dialog: CerDialog) -> OpenCer {
    OpenCer { hr: S_OK, dialog }
}

fn invalid(caption: Option<&str>) -> CerDialog {
    CerDialog::InvalidCertificate {
        text: INVALID_CERT_TEXT,
        caption: caption.map(wide),
        flags: 0x10,
    }
}

#[test]
fn oracle_strdup_rows() {
    // row 1: strdupAW(NULL) -> NULL
    assert_eq!(strdup_ansi_to_wide(None, ascii), None);
    // row 2: strdupAW("") -> units=[0]
    assert_eq!(
        strdup_ansi_to_wide(Some("".as_bytes()), ascii),
        Some(vec![0])
    );
    // row 3: strdupAW("a") -> units=[97,0]
    assert_eq!(
        strdup_ansi_to_wide(Some("a".as_bytes()), ascii),
        Some(vec![97, 0])
    );
    // row 4: strdupAW("dead.beef") -> units=[100,101,97,100,46,98,101,101,102,0]
    assert_eq!(
        strdup_ansi_to_wide(Some("dead.beef".as_bytes()), ascii),
        Some(vec![100, 101, 97, 100, 46, 98, 101, 101, 102, 0])
    );
    // row 5: strdupAW("ab\0cd") reads the C string "ab" -> units=[97,98,0]
    assert_eq!(
        strdup_ansi_to_wide(Some("ab\0cd".as_bytes()), ascii),
        Some(vec![97, 98, 0])
    );
    // row 6: strdupAW("root.cer") -> units=[114,111,111,116,46,99,101,114,0]
    assert_eq!(
        strdup_ansi_to_wide(Some("root.cer".as_bytes()), ascii),
        Some(vec![114, 111, 111, 116, 46, 99, 101, 114, 0])
    );
}

#[test]
fn oracle_add_pfx_rows() {
    // row 7: CryptExtAddPFX(NULL) -> hr=0x80004001
    assert_eq!(crypt_ext_add_pfx(None) as u32, 0x8000_4001);
    // row 8: CryptExtAddPFX("dead.beef") -> hr=0x80004001
    assert_eq!(
        crypt_ext_add_pfx(Some("dead.beef".as_bytes())) as u32,
        0x8000_4001
    );
    // row 9: CryptExtAddPFXW(NULL) -> hr=0x80004001
    assert_eq!(crypt_ext_add_pfx_w(None) as u32, 0x8000_4001);
    // row 10: CryptExtAddPFXW("x.pfx") -> hr=0x80004001
    assert_eq!(
        crypt_ext_add_pfx_w(Some(&wide("x.pfx"))) as u32,
        0x8000_4001
    );
}

#[test]
fn oracle_open_cer_w_rows() {
    // row 11: CryptExtOpenCERW(NULL, query=0) -> seen=NULL, MSG caption=NULL flags=0x10
    assert_eq!(open_w(None, false), (ok(invalid(None)), Some(None)));
    // row 12: CryptExtOpenCERW("dead.beef", query=0) -> seen="dead.beef", MSG caption="dead.beef"
    assert_eq!(
        open_w(Some(&wide("dead.beef")), false),
        (
            ok(invalid(Some("dead.beef"))),
            Some(Some(wide("dead.beef")))
        )
    );
    // row 13: CryptExtOpenCERW("root.cer", query=1) -> seen="root.cer", VIEW
    assert_eq!(
        open_w(Some(&wide("root.cer")), true),
        (ok(CerDialog::ViewCertificate), Some(Some(wide("root.cer"))))
    );
    // row 14: CryptExtOpenCERW("root.cer", query=0) -> seen="root.cer", MSG caption="root.cer"
    assert_eq!(
        open_w(Some(&wide("root.cer")), false),
        (ok(invalid(Some("root.cer"))), Some(Some(wide("root.cer"))))
    );
    // row 20: CryptExtOpenCERW("", query=1) -> seen="", VIEW
    assert_eq!(
        open_w(Some(&[]), true),
        (ok(CerDialog::ViewCertificate), Some(Some(Vec::new())))
    );
}

#[test]
fn oracle_open_cer_rows() {
    // row 15: CryptExtOpenCER(NULL, query=0) -> seen=NULL, MSG caption=NULL flags=0x10
    assert_eq!(open_a(None, false), (ok(invalid(None)), Some(None)));
    // row 16: CryptExtOpenCER("dead.beef", query=0) -> seen="dead.beef", MSG caption="dead.beef"
    assert_eq!(
        open_a(Some("dead.beef".as_bytes()), false),
        (
            ok(invalid(Some("dead.beef"))),
            Some(Some(wide("dead.beef")))
        )
    );
    // row 17: CryptExtOpenCER("dead.beef", query=1) -> seen="dead.beef", VIEW
    assert_eq!(
        open_a(Some("dead.beef".as_bytes()), true),
        (
            ok(CerDialog::ViewCertificate),
            Some(Some(wide("dead.beef")))
        )
    );
    // row 18: CryptExtOpenCER("", query=0) -> seen="", MSG caption=""
    assert_eq!(
        open_a(Some("".as_bytes()), false),
        (ok(invalid(Some(""))), Some(Some(Vec::new())))
    );
    // row 19: CryptExtOpenCER("ab\0cd", query=1) -> seen="ab", VIEW
    assert_eq!(
        open_a(Some("ab\0cd".as_bytes()), true),
        (ok(CerDialog::ViewCertificate), Some(Some(wide("ab"))))
    );
}
