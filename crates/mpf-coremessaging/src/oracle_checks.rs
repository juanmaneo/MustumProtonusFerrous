//! Oracle-derived test vectors for the ported coremessaging logic.
//!
//! The rows were printed by `target/canon/coremessaging/oracle.c`. It compiles the
//! verbatim Wine bodies of `async.c` and `main.c`, with the type shims in `shims.h`,
//! using `gcc -fsanitize=address,undefined -g`. ASan, UBSan and LeakSanitizer reported
//! nothing, so no row is excluded. The 25 printed rows are R1 to R8 (the `IAsyncInfo`
//! state machine), R9 to R12 (`CreateDispatcherQueueController`), R13 and R14
//! (`ShutdownQueueAsync`), R15 and R16 (`DllGetActivationFactory`), and R17 to R25
//! (the nine `FIXME` stubs).
//!
//! How the printed fields map to Rust: `*_hr`, `hr` and `start` are `HRESULT`s, and
//! `res_bool=1` means `VARIANT_TRUE` was written. `out_set` and `factory_set` say
//! whether a C out pointer was written, which Rust expresses through the `Result`.
//! `pre` and `post` are snapshots taken before and after the threadpool callback. C's
//! `start` always returns `S_OK`, so the Rust tests check the `Completion` it returns.

use super::*;
use crate::AsyncStatus::{Canceled, Closed, Completed, Error, Started};

/// `E_FAIL`, which the crate does not otherwise need.
pub(super) const E_FAIL: Hresult = 0x8000_4005u32 as i32;

/// A callback that succeeds and writes `VARIANT_TRUE`.
pub(super) fn sync_ok(_called_async: bool) -> CallbackResult {
    CallbackResult {
        hr: S_OK,
        value: Some(true),
    }
}

/// A callback that fails without writing a value.
pub(super) fn sync_fail(_called_async: bool) -> CallbackResult {
    CallbackResult {
        hr: E_FAIL,
        value: None,
    }
}

/// A callback that returns `STATUS_PENDING` when called synchronously, then succeeds on
/// the threadpool.
pub(super) fn pending_then_ok(called_async: bool) -> CallbackResult {
    if called_async {
        CallbackResult {
            hr: S_OK,
            value: Some(true),
        }
    } else {
        CallbackResult {
            hr: STATUS_PENDING,
            value: None,
        }
    }
}

/// The fields the oracle prints as `status_hr`, `status`, `err_hr`, `err`, `res_hr` and
/// `res_bool`.
pub(super) fn snap(op: &AsyncInfo) -> (Hresult, AsyncStatus, Hresult, Hresult, Hresult, bool) {
    let (status_hr, status) = op.get_status();
    let (err_hr, err) = op.get_error_code();
    let (res_hr, res) = op.get_results();
    (status_hr, status, err_hr, err, res_hr, res)
}

pub(super) fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

pub(super) fn queue_options(
    dw_size: u32,
    thread_type: i32,
    apartment_type: i32,
) -> DispatcherQueueOptions {
    DispatcherQueueOptions {
        dw_size,
        thread_type,
        apartment_type,
    }
}

#[test]
fn oracle_async_state_rows() {
    // R1 async_sync_ok
    let mut op = AsyncInfo::new(sync_ok);
    assert_eq!(op.start(), Completion::Done { invoked: None });
    assert_eq!(snap(&op), (S_OK, Completed, S_OK, S_OK, S_OK, true));

    // R2 async_sync_error
    let mut op = AsyncInfo::new(sync_fail);
    assert_eq!(op.start(), Completion::Done { invoked: None });
    assert_eq!(snap(&op), (S_OK, Error, S_OK, E_FAIL, E_FAIL, false));

    // R3 async_pending_then_threadpool: submit=1, so start leaves the work pending.
    let mut op = AsyncInfo::new(pending_then_ok);
    assert_eq!(op.start(), Completion::Pending);
    assert_eq!(
        snap(&op),
        (S_OK, Started, S_OK, S_OK, E_ILLEGAL_METHOD_CALL, false)
    );
    assert_eq!(op.complete(true), Completion::Done { invoked: None });
    assert_eq!(snap(&op), (S_OK, Completed, S_OK, S_OK, S_OK, true));

    // R4 async_put_after_done: invoke=1 with status Completed, and the handler is cleared.
    let mut op = AsyncInfo::new(sync_ok);
    op.start();
    assert_eq!(op.put_completed(Handler::Set), Ok(Some(Completed)));
    assert_eq!(op.get_completed(), (S_OK, false));
    assert_eq!(snap(&op), (S_OK, Completed, S_OK, S_OK, S_OK, true));

    // R5 async_put_while_started: no invoke, and a second put is a delegate assignment.
    let mut op = AsyncInfo::new(pending_then_ok);
    op.start();
    assert_eq!(op.put_completed(Handler::Set), Ok(None));
    assert_eq!(
        op.put_completed(Handler::Set),
        Err(E_ILLEGAL_DELEGATE_ASSIGNMENT)
    );
    assert_eq!(op.get_completed(), (S_OK, true));
    assert_eq!(
        snap(&op),
        (S_OK, Started, S_OK, S_OK, E_ILLEGAL_METHOD_CALL, false)
    );

    // R6 async_cancel_started
    let mut op = AsyncInfo::new(pending_then_ok);
    op.start();
    assert_eq!(op.cancel(), S_OK);
    assert_eq!(
        snap(&op),
        (S_OK, Canceled, S_OK, S_OK, E_ILLEGAL_METHOD_CALL, false)
    );

    // R7 async_close_started
    let mut op = AsyncInfo::new(pending_then_ok);
    op.start();
    assert_eq!(op.close(), E_ILLEGAL_STATE_CHANGE);
    assert_eq!(
        snap(&op),
        (S_OK, Started, S_OK, S_OK, E_ILLEGAL_METHOD_CALL, false)
    );

    // R8 async_close_then_id: the second close succeeds, and get_Id fails with id still 1.
    let mut op = AsyncInfo::new(sync_ok);
    op.start();
    assert_eq!(op.close(), S_OK);
    assert_eq!(op.close(), S_OK);
    assert_eq!(op.get_id(), (E_ILLEGAL_METHOD_CALL, 1));
    assert_eq!(
        snap(&op),
        (
            E_ILLEGAL_METHOD_CALL,
            Closed,
            E_ILLEGAL_METHOD_CALL,
            E_ILLEGAL_METHOD_CALL,
            E_ILLEGAL_METHOD_CALL,
            false
        )
    );
}

#[test]
fn oracle_create_rows() {
    // R9 create_null_out: the NULL out pointer is checked first.
    assert_eq!(
        create_dispatcher_queue_controller(queue_options(12, DQTYPE_THREAD_DEDICATED, 0), false),
        Err(E_POINTER)
    );
    // R10 create_bad_size: out_set=0 in C, which the Result form already implies.
    assert_eq!(
        create_dispatcher_queue_controller(queue_options(8, DQTYPE_THREAD_DEDICATED, 0), true),
        Err(E_INVALIDARG)
    );
    // R11 create_bad_thread_type: out_set=0 in C.
    assert_eq!(
        create_dispatcher_queue_controller(queue_options(12, 0, 0), true),
        Err(E_INVALIDARG)
    );
    // R12 create_current_thread: out_set=1 in C, and the apartment type is not checked.
    assert_eq!(
        create_dispatcher_queue_controller(queue_options(12, DQTYPE_THREAD_CURRENT, 99), true),
        Ok(())
    );
}

#[test]
fn oracle_shutdown_rows() {
    // R13 shutdown_null_out
    assert_eq!(shutdown_queue_async(false).err(), Some(E_POINTER));

    // R14 shutdown_async_then_threadpool: submit=1, so the action is Started until the
    // threadpool runs it. Its result is never written, so res_bool=0.
    let mut op = shutdown_queue_async(true).unwrap();
    assert_eq!(
        snap(&op),
        (S_OK, Started, S_OK, S_OK, E_ILLEGAL_METHOD_CALL, false)
    );
    assert_eq!(op.complete(true), Completion::Done { invoked: None });
    assert_eq!(snap(&op), (S_OK, Completed, S_OK, S_OK, S_OK, false));
}

#[test]
fn oracle_activation_rows() {
    // R15 dll_match
    assert_eq!(
        dll_get_activation_factory(&wide("Windows.System.DispatcherQueueController")),
        Ok(ActivationClass::DispatcherQueueController)
    );
    // R16 dll_mismatch
    assert_eq!(
        dll_get_activation_factory(&wide("Windows.Security.Cryptography.CryptographicBuffer")),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_stub_rows() {
    // R17 to R25: each FIXME stub returns E_NOTIMPL.
    assert_eq!(stub_method(StubMethod::FactoryGetIids), E_NOTIMPL);
    assert_eq!(
        stub_method(StubMethod::FactoryGetRuntimeClassName),
        E_NOTIMPL
    );
    assert_eq!(stub_method(StubMethod::FactoryGetTrustLevel), E_NOTIMPL);
    assert_eq!(stub_method(StubMethod::FactoryActivateInstance), E_NOTIMPL);
    assert_eq!(stub_method(StubMethod::ControllerGetIids), E_NOTIMPL);
    assert_eq!(
        stub_method(StubMethod::ControllerGetRuntimeClassName),
        E_NOTIMPL
    );
    assert_eq!(stub_method(StubMethod::ControllerGetTrustLevel), E_NOTIMPL);
    assert_eq!(
        stub_method(StubMethod::ControllerGetDispatcherQueue),
        E_NOTIMPL
    );
    assert_eq!(
        stub_method(StubMethod::StaticsCreateOnDedicatedThread),
        E_NOTIMPL
    );
}
