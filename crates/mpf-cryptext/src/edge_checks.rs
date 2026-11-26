//! Edge-case tests for the ported cryptext logic.
//!
//! These cover the NUL scan boundaries, the byte mapping, and both dialog branches with NULL and
//! empty names. Expectations that are not oracle-derived are marked as such in a comment.

use super::*;
use crate::strings::until_nul;

/// Not oracle-derived: a non-identity mapping, to show that bytes of 0x80 and above go through
/// the caller's `ansi_unit` closure.
fn shifted(b: u8) -> u16 {
    u16::from(b) + 0x3000
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn open_w(file: Option<&[u16]>, loaded: bool) -> (OpenCer, Option<Option<Vec<u16>>>) {
    let mut seen = None;
    let out = crypt_ext_open_cer_w(file, |f| {
        seen = Some(f.map(<[u16]>::to_vec));
        loaded
    });
    (out, seen)
}

#[test]
fn until_nul_boundaries() {
    // Not oracle-derived: the NUL scan on its own, for both widths used by the port.
    assert_eq!(until_nul::<u8>(&[]), &[] as &[u8]);
    assert_eq!(until_nul::<u8>(&[1, 2]), &[1, 2]);
    assert_eq!(until_nul::<u8>(&[0, 1]), &[] as &[u8]);
    assert_eq!(until_nul::<u8>(&[1, 0, 2]), &[1]);
    assert_eq!(until_nul::<u8>(&[1, 0]), &[1]);
    assert_eq!(until_nul::<u16>(&[]), &[] as &[u16]);
    assert_eq!(until_nul::<u16>(&[0x61, 0x62]), &[0x61, 0x62]);
    assert_eq!(until_nul::<u16>(&[0x61, 0]), &[0x61]);
}

#[test]
fn strdup_none_and_nul_only() {
    // Not oracle-derived: NULL input never calls the mapping, and a lone NUL gives one unit.
    assert_eq!(strdup_ansi_to_wide(None, shifted), None);
    assert_eq!(strdup_ansi_to_wide(Some(&[0]), shifted), Some(vec![0]));
    assert_eq!(
        strdup_ansi_to_wide(Some(&[0, 0x61]), shifted),
        Some(vec![0])
    );
}

#[test]
fn strdup_maps_high_bytes_through_closure() {
    // Not oracle-derived: the harness only feeds ASCII. The mapping closure decides bytes >= 0x80.
    assert_eq!(
        strdup_ansi_to_wide(Some(&[0x41, 0x80, 0xff]), shifted),
        Some(vec![0x3041, 0x3080, 0x30ff, 0])
    );
}

#[test]
fn stubs_ignore_their_argument() {
    // Not oracle-derived: NULL and a non-NULL name give the same stub return, and E_NOTIMPL is
    // the winerror.h value. The oracle rows already cover these inputs.
    assert_eq!(crypt_ext_add_pfx(Some(&[0x78])), E_NOTIMPL);
    assert_eq!(crypt_ext_add_pfx_w(Some(&wide("x"))), E_NOTIMPL);
    assert_eq!(E_NOTIMPL, -2_147_467_263);
    assert_eq!(S_OK, 0);
}

#[test]
fn open_w_embedded_nul_is_c_string() {
    // Not oracle-derived: the harness never passes an embedded NUL in a wide name. The C code
    // reads only up to the NUL, so the query and the caption both see "a".
    assert_eq!(
        open_w(Some(&[0x61, 0, 0x62]), true),
        (
            OpenCer {
                hr: S_OK,
                dialog: CerDialog::ViewCertificate
            },
            Some(Some(wide("a")))
        )
    );
    assert_eq!(
        open_w(Some(&[0x61, 0, 0x62]), false),
        (
            OpenCer {
                hr: S_OK,
                dialog: CerDialog::InvalidCertificate {
                    text: INVALID_CERT_TEXT,
                    caption: Some(wide("a")),
                    flags: 0x10,
                },
            },
            Some(Some(wide("a")))
        )
    );
}

#[test]
fn open_w_null_name_has_no_caption() {
    // Not oracle-derived: a NULL name with a valid certificate still takes the viewer branch, so
    // the caption is not used there. The message box caption is NULL, not "".
    let (out, seen) = open_w(None, true);
    assert_eq!(out.dialog, CerDialog::ViewCertificate);
    assert_eq!(seen, Some(None));
    let (out, _) = open_w(None, false);
    assert_eq!(
        out.dialog,
        CerDialog::InvalidCertificate {
            text: INVALID_CERT_TEXT,
            caption: None,
            flags: 0x10,
        }
    );
}

#[test]
fn open_cer_non_ascii_caption_uses_mapping() {
    // Not oracle-derived: the mapping applies before the query and before the caption copy.
    let mut seen = None;
    let out = crypt_ext_open_cer(Some(&[0x41, 0x80]), shifted, |f| {
        seen = Some(f.map(<[u16]>::to_vec));
        false
    });
    assert_eq!(seen, Some(Some(vec![0x3041, 0x3080])));
    assert_eq!(
        out.dialog,
        CerDialog::InvalidCertificate {
            text: INVALID_CERT_TEXT,
            caption: Some(vec![0x3041, 0x3080]),
            flags: 0x10,
        }
    );
}

#[test]
fn dialog_debug_names_the_branch() {
    // Not oracle-derived: checks the derived Debug output names each branch.
    assert!(format!("{:?}", CerDialog::ViewCertificate).contains("ViewCertificate"));
    assert!(format!(
        "{:?}",
        CerDialog::InvalidCertificate {
            text: INVALID_CERT_TEXT,
            caption: None,
            flags: 0x10,
        }
    )
    .contains("InvalidCertificate"));
}
