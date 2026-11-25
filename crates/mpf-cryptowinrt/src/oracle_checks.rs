//! Oracle-derived test vectors for the ported cryptowinrt logic.
//!
//! The rows were printed by `target/canon/cryptowinrt/oracle.c`. It compiles the
//! verbatim Wine bodies of `encodeBase64W`, `BinaryToBase64W`, `DllGetActivationFactory`,
//! `is_supported_async` and the `async.c` state machine with
//! `gcc -std=gnu11 -fsanitize=address,undefined -g`. ASan and UBSan reported nothing,
//! so no row is excluded. The 25 printed rows are 5 `base64`, 5 `activation` and 15
//! `async`. The `diag` lines the harness also prints are not rows.

use super::*;
use crate::AsyncStatus::{Canceled, Closed, Completed};

/// Drives one operation the way the oracle harness's `run_script` does.
///
/// The first character picks the callback: `P` is `is_supported_async`, `T` is a
/// callback that succeeds with `true`, and `F` is one that fails with `E_FAIL`. Each
/// later character is one call: `h` puts a handler, `n` puts a null handler, `g` is
/// `get_Completed`, `r` is `GetResults`, `i` is `get_Id`, `s` is `get_Status`, `e` is
/// `get_ErrorCode`, `c` is `Cancel`, `x` is `Close`, and `w` runs the threadpool
/// callback. Returns the last call's `HRESULT`, its out value, and the status the
/// handler was invoked with.
pub(super) fn run_script(script: &str) -> (Hresult, u32, Option<AsyncStatus>) {
    let (recipe, ops) = script.split_at(1);
    let callback: Callback = match recipe {
        "P" => is_supported_async,
        "T" => sync_true,
        "F" => sync_fail,
        _ => panic!("unknown recipe {recipe}"),
    };
    let mut op = AsyncInfo::new(callback);
    let mut invoked = None;
    note(&mut invoked, op.start());
    let mut hr = S_OK;
    let mut out = 0;
    for c in ops.chars() {
        (hr, out) = match c {
            'h' => (put(&mut op, &mut invoked, Handler::Set), 0),
            'n' => (put(&mut op, &mut invoked, Handler::Null), 0),
            'g' => {
                let (h, set) = op.get_completed();
                (h, u32::from(set))
            }
            'r' => {
                let (h, value) = op.get_results();
                (h, u32::from(value))
            }
            'i' => op.get_id(),
            's' => {
                let (h, status) = op.get_status();
                (h, status as u32)
            }
            'e' => {
                let (h, code) = op.get_error_code();
                (h, code as u32)
            }
            'c' => (op.cancel(), 0),
            'x' => (op.close(), 0),
            'w' => {
                note(&mut invoked, op.complete(true));
                (S_OK, 0)
            }
            _ => panic!("unknown op {c}"),
        };
    }
    (hr, out, invoked)
}

fn sync_true(_called_async: bool) -> CallbackResult {
    CallbackResult {
        hr: S_OK,
        value: Some(true),
    }
}

fn sync_fail(_called_async: bool) -> CallbackResult {
    CallbackResult {
        hr: E_FAIL,
        value: None,
    }
}

fn note(invoked: &mut Option<AsyncStatus>, completion: Completion) {
    if let Completion::Done {
        invoked: Some(status),
    } = completion
    {
        *invoked = Some(status);
    }
}

fn put(op: &mut AsyncInfo, invoked: &mut Option<AsyncStatus>, handler: Handler) -> Hresult {
    match op.put_completed(handler) {
        Ok(status) => {
            *invoked = status.or(*invoked);
            S_OK
        }
        Err(hr) => hr,
    }
}

pub(super) fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn oracle_base64_rows() {
    assert_eq!(encode_base64(&[]), "");
    assert_eq!(encode_base64(&[0x4d, 0x61]), "TWE=");
    assert_eq!(encode_base64(&[0xfb, 0xef, 0xbe]), "++++");
    assert_eq!(encode_base64(&[0xfb, 0xff]), "+/8=");
    let all: Vec<u8> = (0..64).collect();
    assert_eq!(
        encode_base64(&all),
        "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEyMzQ1Njc4OTo7PD0+Pw=="
    );
}

#[test]
fn oracle_activation_rows() {
    assert_eq!(
        dll_get_activation_factory(&wide("Windows.Security.Cryptography.CryptographicBuffer")),
        Ok(ActivationClass::CryptographicBuffer)
    );
    assert_eq!(
        dll_get_activation_factory(&wide("Windows.Security.Credentials.KeyCredentialManager")),
        Ok(ActivationClass::KeyCredentialManager)
    );
    assert_eq!(
        dll_get_activation_factory(&wide(
            "Windows.Security.Cryptography.CryptographicBuffer\0junk"
        )),
        Ok(ActivationClass::CryptographicBuffer)
    );
    assert_eq!(
        dll_get_activation_factory(&wide("Windows.Security.Cryptography.CryptographicBuffer ")),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(
        dll_get_activation_factory(&wide("\0Windows.Security.Cryptography.CryptographicBuffer")),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_async_rows() {
    let rows: [(&str, Hresult, u32, Option<AsyncStatus>); 15] = [
        ("Phws", S_OK, 1, Some(Completed)),
        ("Phwh", E_ILLEGAL_DELEGATE_ASSIGNMENT, 0, Some(Completed)),
        ("Pcws", S_OK, 1, None),
        ("Phcxws", E_ILLEGAL_METHOD_CALL, 4, Some(Closed)),
        ("Px", E_ILLEGAL_STATE_CHANGE, 0, None),
        ("Th", S_OK, 0, Some(Completed)),
        ("Fr", E_FAIL, 0, None),
        ("Fxs", E_ILLEGAL_METHOD_CALL, 4, None),
        ("Phcxg", E_ILLEGAL_METHOD_CALL, 1, None),
        ("Fe", S_OK, E_FAIL as u32, None),
        (
            "Fxe",
            E_ILLEGAL_METHOD_CALL,
            E_ILLEGAL_METHOD_CALL as u32,
            None,
        ),
        ("Pnh", E_ILLEGAL_DELEGATE_ASSIGNMENT, 0, None),
        ("Pr", E_ILLEGAL_METHOD_CALL, 0, None),
        ("Txr", E_ILLEGAL_METHOD_CALL, 0, None),
        ("Pch", S_OK, 0, Some(Canceled)),
    ];
    for (script, hr, out, invoked) in rows {
        assert_eq!(run_script(script), (hr, out, invoked), "script {script}");
    }
}
