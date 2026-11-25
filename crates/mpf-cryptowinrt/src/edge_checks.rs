//! Edge-case and branch tests for the ported cryptowinrt logic.
//!
//! A comment marks each expectation that is not one of the printed oracle rows.
//! "External" marks RFC 4648, Wine's own test, or coreutils `base64` vectors.
//! "Not oracle-derived" marks expectations derived by reading Wine's source. Those
//! marked "scratch C driver" were also run through the verbatim C bodies in a
//! temporary driver, which is not part of the crate.

use super::*;
use crate::oracle_checks::{run_script, wide};
use crate::AsyncStatus::{Canceled, Closed, Completed, Error, Started};

fn fail_with_value(_called_async: bool) -> CallbackResult {
    CallbackResult {
        hr: E_FAIL,
        value: Some(true),
    }
}

fn pending_always(_called_async: bool) -> CallbackResult {
    CallbackResult {
        hr: STATUS_PENDING,
        value: None,
    }
}

#[test]
fn base64_rfc4648_vectors() {
    // External: RFC 4648 section 10.
    let vectors: [(&[u8], &str); 7] = [
        (b"", ""),
        (b"f", "Zg=="),
        (b"fo", "Zm8="),
        (b"foo", "Zm9v"),
        (b"foob", "Zm9vYg=="),
        (b"fooba", "Zm9vYmE="),
        (b"foobar", "Zm9vYmFy"),
    ];
    for (input, expected) in vectors {
        assert_eq!(encode_base64(input), expected);
    }
}

#[test]
fn base64_wine_and_coreutils_vectors() {
    // External: Wine's tests/crypto.c encodes bytes 0..16 to this string.
    let bytes: Vec<u8> = (0..16).collect();
    assert_eq!(encode_base64(&bytes), "AAECAwQFBgcICQoLDA0ODw==");
    // External: coreutils `base64` output for these inputs, which exercise the
    // sextets that map to `/`.
    assert_eq!(encode_base64(&[0xff]), "/w==");
    assert_eq!(encode_base64(&[0xff, 0xff, 0xff]), "////");
}

#[test]
fn activation_class_names_must_match_exactly() {
    // Not oracle-derived: near misses of the two class names. Wine compares with
    // wcscmp, so case, suffixes, prefixes and empty names all fail.
    let misses = [
        "windows.security.cryptography.cryptographicbuffer",
        "Windows.Security.Cryptography.CryptographicBufferX",
        "Windows.Security.Credentials",
        "Windows.Security.Credentials.KeyCredentialManage",
    ];
    for name in misses {
        assert_eq!(
            dll_get_activation_factory(&wide(name)),
            Err(CLASS_E_CLASSNOTAVAILABLE),
            "{name}"
        );
    }
    assert_eq!(
        dll_get_activation_factory(&[]),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn activation_nul_truncates_either_class() {
    // Not oracle-derived: the `cbuf_nul_junk` oracle row, applied to the other class,
    // and a NUL with nothing after it.
    assert_eq!(
        dll_get_activation_factory(&wide(
            "Windows.Security.Credentials.KeyCredentialManager\0junk"
        )),
        Ok(ActivationClass::KeyCredentialManager)
    );
    assert_eq!(
        dll_get_activation_factory(&wide("Windows.Security.Cryptography.CryptographicBuffer\0")),
        Ok(ActivationClass::CryptographicBuffer)
    );
}

#[test]
fn run_script_completed_operation() {
    // Not oracle-derived (scratch C driver): scripts outside the printed rows.
    // Cancel is a no-op once the operation has completed or errored.
    assert_eq!(run_script("Tcs"), (S_OK, 1, None));
    assert_eq!(run_script("Fcs"), (S_OK, 3, None));
    assert_eq!(run_script("Tg"), (S_OK, 0, None));
    // A null handler is stored without invoking anything.
    assert_eq!(run_script("Tn"), (S_OK, 0, None));
    // Setting a handler again fails, even after the null handler was stored.
    assert_eq!(run_script("Tnh"), (E_ILLEGAL_DELEGATE_ASSIGNMENT, 0, None));
    // Setting a handler on an errored operation invokes it at once.
    assert_eq!(run_script("Fh"), (S_OK, 0, Some(Error)));
}

#[test]
fn run_script_started_operation() {
    // Not oracle-derived (scratch C driver): scripts outside the printed rows.
    assert_eq!(run_script("Pcs"), (S_OK, 2, None));
    assert_eq!(run_script("Pcw"), (S_OK, 0, None));
    assert_eq!(run_script("Pi"), (S_OK, 1, None));
    assert_eq!(run_script("Pnw"), (S_OK, 0, None));
    assert_eq!(run_script("Png"), (S_OK, 0, None));
    // Close on a started operation fails, but the operation can still complete.
    assert_eq!(run_script("Pxh"), (S_OK, 0, None));
    assert_eq!(run_script("Pxws"), (S_OK, 1, None));
    // Setting a handler after completion invokes it at once.
    assert_eq!(run_script("Pwh"), (S_OK, 0, Some(Completed)));
}

#[test]
fn run_script_closed_operation() {
    // Not oracle-derived (scratch C driver): scripts outside the printed rows. Once
    // closed, every call except a second Close fails.
    assert_eq!(run_script("Txc"), (E_ILLEGAL_METHOD_CALL, 0, None));
    assert_eq!(run_script("Txh"), (E_ILLEGAL_METHOD_CALL, 0, None));
    assert_eq!(run_script("Txg"), (E_ILLEGAL_METHOD_CALL, 0, None));
    assert_eq!(run_script("Txi"), (E_ILLEGAL_METHOD_CALL, 1, None));
    assert_eq!(run_script("Txx"), (S_OK, 0, None));
}

#[test]
fn error_keeps_hr_and_value() {
    // Not oracle-derived (scratch C driver): the oracle's failing callback writes no
    // value. This case, a failure that still sets a boolean, comes from
    // `async_info_complete`.
    let mut op = AsyncInfo::new(fail_with_value);
    assert_eq!(op.start(), Completion::Done { invoked: None });
    assert_eq!(op.get_results(), (E_FAIL, true));
    assert_eq!(op.get_error_code(), (S_OK, E_FAIL));
    assert_eq!(op.get_status(), (S_OK, Error));
}

#[test]
fn threadpool_run_keeps_pending_as_result() {
    // Not oracle-derived (scratch C driver): Wine treats STATUS_PENDING as pending
    // only on the synchronous call. A threadpool run completes with it as the hr, and
    // GetResults reports it.
    let mut op = AsyncInfo::new(pending_always);
    assert_eq!(op.start(), Completion::Pending);
    assert_eq!(op.get_status(), (S_OK, Started));
    assert_eq!(op.complete(true), Completion::Done { invoked: None });
    assert_eq!(op.get_status(), (S_OK, Completed));
    assert_eq!(op.get_error_code(), (S_OK, STATUS_PENDING));
    assert_eq!(op.get_results(), (STATUS_PENDING, false));
}

#[test]
fn failed_is_a_sign_test() {
    // Not oracle-derived: FAILED(hr) is hr < 0, so STATUS_PENDING (0x103) is not a failure.
    assert!(!failed(S_OK));
    assert!(!failed(STATUS_PENDING));
    assert!(!failed(i32::MAX));
    assert!(failed(-1));
    assert!(failed(i32::MIN));
    assert!(failed(E_FAIL));
}

#[test]
#[should_panic(expected = "unknown recipe")]
fn run_script_rejects_unknown_recipe() {
    run_script("Z");
}

#[test]
#[should_panic(expected = "unknown op")]
fn run_script_rejects_unknown_op() {
    run_script("Pz");
}

#[test]
fn derived_impls_name_every_variant() {
    // Not oracle-derived: Rust-side derives, checked for every variant.
    let statuses = [Started, Completed, Canceled, Error, Closed];
    let names = ["Started", "Completed", "Canceled", "Error", "Closed"];
    for (status, name) in statuses.into_iter().zip(names) {
        assert_eq!(Clone::clone(&status), status);
        assert_eq!(format!("{status:?}"), name);
    }
    assert_ne!(Started, Completed);

    for (handler, name) in [(Handler::Null, "Null"), (Handler::Set, "Set")] {
        assert_eq!(Clone::clone(&handler), handler);
        assert_eq!(format!("{handler:?}"), name);
    }
    assert_ne!(Handler::Null, Handler::Set);

    let pending = Completion::Pending;
    let done = Completion::Done {
        invoked: Some(Completed),
    };
    assert_eq!(Clone::clone(&pending), pending);
    assert_eq!(Clone::clone(&done), done);
    assert_eq!(format!("{pending:?}"), "Pending");
    assert_eq!(format!("{done:?}"), "Done { invoked: Some(Completed) }");
    assert_eq!(pending, Completion::Pending);
    assert_ne!(pending, done);
    assert_ne!(done, Completion::Done { invoked: None });

    for (class, name) in [
        (ActivationClass::CryptographicBuffer, "CryptographicBuffer"),
        (
            ActivationClass::KeyCredentialManager,
            "KeyCredentialManager",
        ),
    ] {
        assert_eq!(Clone::clone(&class), class);
        assert_eq!(format!("{class:?}"), name);
    }
    assert_ne!(
        ActivationClass::CryptographicBuffer,
        ActivationClass::KeyCredentialManager
    );
}
