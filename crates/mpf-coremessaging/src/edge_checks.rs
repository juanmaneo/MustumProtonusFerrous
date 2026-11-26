//! Boundary and branch tests for the ported coremessaging logic.
//!
//! The expectations come from Wine's `async.c` and `main.c`, or from the port's own
//! branches. Each expectation that no oracle row prints is marked "non-oracle".

use super::*;
use crate::oracle_checks::{pending_then_ok, queue_options, sync_fail, sync_ok, wide, E_FAIL};
use crate::AsyncStatus::{Canceled, Closed, Completed, Error, Started};

/// Always `STATUS_PENDING`, including on the threadpool.
fn pending_always(_called_async: bool) -> CallbackResult {
    CallbackResult {
        hr: STATUS_PENDING,
        value: None,
    }
}

/// Fails, but still writes `VARIANT_TRUE`.
fn fail_with_value(_called_async: bool) -> CallbackResult {
    CallbackResult {
        hr: E_FAIL,
        value: Some(true),
    }
}

#[test]
fn failed_sign_boundary() {
    // Non-oracle: FAILED is a sign test, so only negative HRESULTs fail.
    assert!(!failed(S_OK));
    assert!(!failed(STATUS_PENDING));
    assert!(!failed(i32::MAX));
    assert!(failed(-1));
    assert!(failed(i32::MIN));
    assert!(failed(E_POINTER));
}

#[test]
fn complete_pending_only_on_synchronous_call() {
    // Non-oracle: a threadpool call that returns STATUS_PENDING completes with it as hr.
    let mut op = AsyncInfo::new(pending_always);
    assert_eq!(op.start(), Completion::Pending);
    assert_eq!(op.get_status(), (S_OK, Started));
    assert_eq!(op.complete(true), Completion::Done { invoked: None });
    assert_eq!(op.get_status(), (S_OK, Completed));
    assert_eq!(op.get_error_code(), (S_OK, STATUS_PENDING));
    assert_eq!(op.get_results(), (STATUS_PENDING, false));
}

#[test]
fn complete_invokes_only_a_set_handler() {
    // Non-oracle: a Null handler is never invoked, and a Set handler is invoked once.
    let mut op = AsyncInfo::new(sync_ok);
    assert_eq!(op.put_completed(Handler::Null), Ok(None));
    assert_eq!(op.start(), Completion::Done { invoked: None });
    assert_eq!(op.get_completed(), (S_OK, false));

    let mut op = AsyncInfo::new(sync_ok);
    assert_eq!(op.put_completed(Handler::Set), Ok(None));
    assert_eq!(op.get_completed(), (S_OK, true));
    assert_eq!(
        op.start(),
        Completion::Done {
            invoked: Some(Completed)
        }
    );
    assert_eq!(op.get_completed(), (S_OK, false));
    assert_eq!(op.complete(true), Completion::Done { invoked: None });
}

#[test]
fn put_completed_null_after_done_is_stored_without_invoke() {
    // Non-oracle: a Null handler is stored, so a later put is a delegate assignment.
    let mut op = AsyncInfo::new(sync_ok);
    op.start();
    assert_eq!(op.put_completed(Handler::Null), Ok(None));
    assert_eq!(op.get_completed(), (S_OK, false));
    assert_eq!(
        op.put_completed(Handler::Set),
        Err(E_ILLEGAL_DELEGATE_ASSIGNMENT)
    );
}

#[test]
fn put_completed_after_close_is_illegal_method_call() {
    // Non-oracle: the Closed check runs before the handler check (async.c line 120).
    let mut op = AsyncInfo::new(pending_then_ok);
    op.start();
    assert_eq!(op.put_completed(Handler::Set), Ok(None));
    assert_eq!(op.cancel(), S_OK);
    assert_eq!(op.close(), S_OK);
    assert_eq!(op.put_completed(Handler::Set), Err(E_ILLEGAL_METHOD_CALL));
    assert_eq!(op.put_completed(Handler::Null), Err(E_ILLEGAL_METHOD_CALL));
}

#[test]
fn closed_while_handler_pending_invokes_with_closed() {
    // Non-oracle: bug-compatible with async.c. Closing leaves the handler in place, so
    // the completion still invokes it, with status Closed.
    let mut op = AsyncInfo::new(pending_then_ok);
    assert_eq!(op.start(), Completion::Pending);
    assert_eq!(op.put_completed(Handler::Set), Ok(None));
    assert_eq!(op.cancel(), S_OK);
    assert_eq!(op.close(), S_OK);
    assert_eq!(
        op.complete(true),
        Completion::Done {
            invoked: Some(Closed)
        }
    );
    assert_eq!(op.get_status(), (E_ILLEGAL_METHOD_CALL, Closed));
}

#[test]
fn complete_after_close_keeps_closed_status() {
    // Non-oracle: a closed operation keeps Closed, even when the callback fails.
    let mut op = AsyncInfo::new(sync_fail);
    op.start();
    assert_eq!(op.close(), S_OK);
    assert_eq!(op.complete(true), Completion::Done { invoked: None });
    assert_eq!(op.get_status(), (E_ILLEGAL_METHOD_CALL, Closed));
    assert_eq!(
        op.get_error_code(),
        (E_ILLEGAL_METHOD_CALL, E_ILLEGAL_METHOD_CALL)
    );
}

#[test]
fn get_results_requires_completed_or_error() {
    // Non-oracle: only Completed and Error expose a result. Started, Canceled and Closed
    // do not.
    let mut op = AsyncInfo::new(pending_then_ok);
    op.start();
    assert_eq!(op.get_results(), (E_ILLEGAL_METHOD_CALL, false));
    assert_eq!(op.cancel(), S_OK);
    assert_eq!(op.get_results(), (E_ILLEGAL_METHOD_CALL, false));

    let mut op = AsyncInfo::new(sync_ok);
    op.start();
    assert_eq!(op.get_results(), (S_OK, true));
    assert_eq!(op.close(), S_OK);
    assert_eq!(op.get_results(), (E_ILLEGAL_METHOD_CALL, false));
}

#[test]
fn get_results_keeps_value_on_failure() {
    // Non-oracle: an Error result still reports the value the callback wrote.
    let mut op = AsyncInfo::new(fail_with_value);
    assert_eq!(op.start(), Completion::Done { invoked: None });
    assert_eq!(op.get_results(), (E_FAIL, true));
}

#[test]
fn get_results_without_value_is_false() {
    // Non-oracle: a Completed result with no value written reads as false.
    let mut op = AsyncInfo::new(shutdown_queue_callback);
    assert_eq!(op.start(), Completion::Pending);
    assert_eq!(op.complete(true), Completion::Done { invoked: None });
    assert_eq!(op.get_results(), (S_OK, false));
}

#[test]
fn cancel_and_close_only_change_a_started_operation() {
    // Non-oracle: cancel changes only Started. close succeeds from every status except
    // Started, and a closed operation refuses cancel.
    let mut op = AsyncInfo::new(sync_ok);
    op.start();
    assert_eq!(op.cancel(), S_OK);
    assert_eq!(op.get_status(), (S_OK, Completed));
    assert_eq!(op.close(), S_OK);
    assert_eq!(op.cancel(), E_ILLEGAL_METHOD_CALL);

    let mut op = AsyncInfo::new(sync_fail);
    op.start();
    assert_eq!(op.cancel(), S_OK);
    assert_eq!(op.get_status(), (S_OK, Error));
    assert_eq!(op.close(), S_OK);
    assert_eq!(op.get_status(), (E_ILLEGAL_METHOD_CALL, Closed));

    let mut op = AsyncInfo::new(pending_then_ok);
    op.start();
    assert_eq!(op.cancel(), S_OK);
    assert_eq!(op.cancel(), S_OK);
    assert_eq!(op.get_status(), (S_OK, Canceled));
    assert_eq!(op.close(), S_OK);
    assert_eq!(op.close(), S_OK);
}

#[test]
fn create_checks_out_pointer_before_options() {
    // Non-oracle: main.c checks the out pointer before any option (lines 276 to 278).
    assert_eq!(
        create_dispatcher_queue_controller(queue_options(0, 0, 0), false),
        Err(E_POINTER)
    );
}

#[test]
fn create_size_boundaries() {
    // Non-oracle: dw_size must equal sizeof(DispatcherQueueOptions) exactly.
    for dw_size in [0, 11, 13, u32::MAX] {
        assert_eq!(
            create_dispatcher_queue_controller(
                queue_options(dw_size, DQTYPE_THREAD_CURRENT, 0),
                true
            ),
            Err(E_INVALIDARG)
        );
    }
    assert_eq!(
        create_dispatcher_queue_controller(
            queue_options(DISPATCHER_QUEUE_OPTIONS_SIZE, DQTYPE_THREAD_CURRENT, 0),
            true
        ),
        Ok(())
    );
}

#[test]
fn create_thread_type_boundaries() {
    // Non-oracle: only DQTYPE_THREAD_DEDICATED (1) and DQTYPE_THREAD_CURRENT (2) pass.
    // R12 covers CURRENT. DEDICATED is not oracle-derived, because R9 returns early.
    for thread_type in [-1, 0, 3, i32::MAX] {
        assert_eq!(
            create_dispatcher_queue_controller(
                queue_options(DISPATCHER_QUEUE_OPTIONS_SIZE, thread_type, 0),
                true
            ),
            Err(E_INVALIDARG)
        );
    }
    assert_eq!(
        create_dispatcher_queue_controller(
            queue_options(DISPATCHER_QUEUE_OPTIONS_SIZE, DQTYPE_THREAD_DEDICATED, 0),
            true
        ),
        Ok(())
    );
    // The apartment type is never checked, so a negative value is accepted.
    assert_eq!(
        create_dispatcher_queue_controller(
            queue_options(DISPATCHER_QUEUE_OPTIONS_SIZE, DQTYPE_THREAD_DEDICATED, -1),
            true
        ),
        Ok(())
    );
}

#[test]
fn shutdown_callback_is_pending_only_synchronously() {
    // Non-oracle: the callback behind R14. The synchronous call is pending, and the
    // threadpool call succeeds without writing a value.
    assert_eq!(
        shutdown_queue_callback(false),
        CallbackResult {
            hr: STATUS_PENDING,
            value: None
        }
    );
    assert_eq!(
        shutdown_queue_callback(true),
        CallbackResult {
            hr: S_OK,
            value: None
        }
    );
}

#[test]
fn activation_name_boundaries() {
    // Non-oracle: the name ends at the first NUL (wcscmp), and the comparison is exact
    // and case-sensitive.
    assert_eq!(
        dll_get_activation_factory(&wide("Windows.System.DispatcherQueueController\0ignored")),
        Ok(ActivationClass::DispatcherQueueController)
    );
    for name in [
        "",
        "\0Windows.System.DispatcherQueueController",
        "Windows.System.DispatcherQueueControlle",
        "Windows.System.DispatcherQueueController ",
        "Windows.System.DispatcherQueueControllerX",
        "windows.system.dispatcherqueuecontroller",
    ] {
        assert_eq!(
            dll_get_activation_factory(&wide(name)),
            Err(CLASS_E_CLASSNOTAVAILABLE),
            "{name:?}"
        );
    }
    assert_eq!(
        dll_get_activation_factory(&[0]),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(
        dll_get_activation_factory(&[]),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn derived_impls_are_exercised() {
    // Non-oracle: calls the derived impls directly. Each variant is formatted, so every
    // generated match arm counts toward coverage.
    for (status, name) in [
        (Started, "Started"),
        (Completed, "Completed"),
        (Canceled, "Canceled"),
        (Error, "Error"),
        (Closed, "Closed"),
    ] {
        assert_eq!(Clone::clone(&status), status);
        assert_eq!(format!("{status:?}"), name);
    }
    assert_ne!(Started, Closed);

    let result = CallbackResult {
        hr: E_POINTER,
        value: Some(false),
    };
    assert_eq!(Clone::clone(&result), result);
    assert_ne!(
        result,
        CallbackResult {
            hr: S_OK,
            value: Some(false)
        }
    );
    assert_ne!(
        result,
        CallbackResult {
            hr: E_POINTER,
            value: None
        }
    );
    assert!(format!("{result:?}").starts_with("CallbackResult"));

    for (handler, name) in [(Handler::Null, "Null"), (Handler::Set, "Set")] {
        assert_eq!(Clone::clone(&handler), handler);
        assert_eq!(format!("{handler:?}"), name);
    }
    assert_ne!(Handler::Null, Handler::Set);

    for (completion, name) in [
        (Completion::Pending, "Pending"),
        (Completion::Done { invoked: None }, "Done { invoked: None }"),
        (
            Completion::Done {
                invoked: Some(Error),
            },
            "Done { invoked: Some(Error) }",
        ),
    ] {
        assert_eq!(Clone::clone(&completion), completion);
        assert_eq!(format!("{completion:?}"), name);
    }
    assert_ne!(Completion::Pending, Completion::Done { invoked: None });
    assert_ne!(
        Completion::Done { invoked: None },
        Completion::Done {
            invoked: Some(Completed)
        }
    );

    let options = queue_options(DISPATCHER_QUEUE_OPTIONS_SIZE, DQTYPE_THREAD_CURRENT, 1);
    assert_eq!(Clone::clone(&options), options);
    assert_eq!(
        format!("{options:?}"),
        "DispatcherQueueOptions { dw_size: 12, thread_type: 2, apartment_type: 1 }"
    );
    assert_ne!(options, queue_options(0, DQTYPE_THREAD_CURRENT, 1));
    assert_ne!(
        options,
        queue_options(DISPATCHER_QUEUE_OPTIONS_SIZE, DQTYPE_THREAD_DEDICATED, 1)
    );
    assert_ne!(
        options,
        queue_options(DISPATCHER_QUEUE_OPTIONS_SIZE, DQTYPE_THREAD_CURRENT, 0)
    );

    let class = ActivationClass::DispatcherQueueController;
    assert_eq!(Clone::clone(&class), class);
    assert_eq!(format!("{class:?}"), "DispatcherQueueController");

    for (method, name) in [
        (StubMethod::FactoryGetIids, "FactoryGetIids"),
        (
            StubMethod::FactoryGetRuntimeClassName,
            "FactoryGetRuntimeClassName",
        ),
        (StubMethod::FactoryGetTrustLevel, "FactoryGetTrustLevel"),
        (
            StubMethod::FactoryActivateInstance,
            "FactoryActivateInstance",
        ),
        (StubMethod::ControllerGetIids, "ControllerGetIids"),
        (
            StubMethod::ControllerGetRuntimeClassName,
            "ControllerGetRuntimeClassName",
        ),
        (
            StubMethod::ControllerGetTrustLevel,
            "ControllerGetTrustLevel",
        ),
        (
            StubMethod::ControllerGetDispatcherQueue,
            "ControllerGetDispatcherQueue",
        ),
        (
            StubMethod::StaticsCreateOnDedicatedThread,
            "StaticsCreateOnDedicatedThread",
        ),
    ] {
        assert_eq!(Clone::clone(&method), method);
        assert_eq!(format!("{method:?}"), name);
    }
    assert_ne!(StubMethod::FactoryGetIids, StubMethod::ControllerGetIids);
}
