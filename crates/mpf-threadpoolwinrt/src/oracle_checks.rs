//! Oracle-derived test vectors for the ported threadpoolwinrt logic.
//!
//! Vectors were captured from the verbatim Wine C bodies in `dlls/threadpoolwinrt/main.c`
//! (`extract.py` copies them into `body.c`, and `harness.c` includes that file). `DllCanUnloadNow`
//! comes from `libs/winecrt0/dll_canunload.c`. The harness was compiled with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/threadpoolwinrt/` and run with
//! `ASAN_OPTIONS=detect_leaks=0`. LeakSanitizer is off because the harness leaves some async
//! action objects alive at exit; no other sanitizer report appeared.
//!
//! The 55 rows are: 2 for `DllGetClassObject` and `DllCanUnloadNow`, 7 for
//! `DllGetActivationFactory`, 8 for `threadpool_factory_QueryInterface`, 6 for
//! `async_action_QueryInterface`, 14 for `run_async`, and 18 for the `E_NOTIMPL` methods.
//! UBSan and ASan flagged none of them, so 0 rows are excluded.

use super::*;

const GET_ACTIVATION_FACTORY_ROWS: &[(Option<&str>, HRESULT, i32)] = &[
    (Some("Windows.System.Threading.ThreadPool"), S_OK, 2),
    (Some("Windows.System.Threading.ThreadPool\0junk"), S_OK, 2),
    (
        Some("Windows.System.Threading.ThreadPoolX"),
        CLASS_E_CLASSNOTAVAILABLE,
        1,
    ),
    (
        Some("Windows.System.Threading"),
        CLASS_E_CLASSNOTAVAILABLE,
        1,
    ),
    (
        Some("windows.system.threading.threadpool"),
        CLASS_E_CLASSNOTAVAILABLE,
        1,
    ),
    (Some(""), CLASS_E_CLASSNOTAVAILABLE, 1),
    (None, CLASS_E_CLASSNOTAVAILABLE, 1),
];

const FACTORY_QUERY_INTERFACE_ROWS: &[(Iid, Result<FactoryView, HRESULT>, i32)] = &[
    (Iid::Unknown, Ok(FactoryView::ActivationFactory), 2),
    (Iid::Inspectable, Ok(FactoryView::ActivationFactory), 2),
    (Iid::AgileObject, Ok(FactoryView::ActivationFactory), 2),
    (
        Iid::ActivationFactory,
        Ok(FactoryView::ActivationFactory),
        2,
    ),
    (
        Iid::ThreadPoolStatics,
        Ok(FactoryView::ThreadPoolStatics),
        2,
    ),
    (Iid::AsyncAction, Err(E_NOINTERFACE), 1),
    (Iid::AsyncInfo, Err(E_NOINTERFACE), 1),
    (Iid::Other, Err(E_NOINTERFACE), 1),
];

const ACTION_QUERY_INTERFACE_ROWS: &[(Iid, Result<ActionView, HRESULT>, i32)] = &[
    (Iid::AsyncAction, Ok(ActionView::AsyncAction), 2),
    (Iid::Inspectable, Ok(ActionView::AsyncAction), 2),
    (Iid::Unknown, Ok(ActionView::AsyncAction), 2),
    (Iid::AsyncInfo, Ok(ActionView::AsyncInfo), 2),
    (Iid::AgileObject, Err(E_NOINTERFACE), 1),
    (Iid::ActivationFactory, Err(E_NOINTERFACE), 1),
];

type RunAsyncRow = (bool, i32, i32, bool, Result<Route, HRESULT>);

const RUN_ASYNC_ROWS: &[RunAsyncRow] = &[
    (false, 0, 0, true, Err(E_INVALIDARG)),
    (false, -1, 1, true, Err(E_INVALIDARG)),
    (true, -2, 0, true, Err(E_INVALIDARG)),
    (true, 2, 0, true, Err(E_INVALIDARG)),
    (true, -1, 0, true, Ok(Route::ThreadPool { pool_index: 0 })),
    (true, 0, 0, true, Ok(Route::ThreadPool { pool_index: 1 })),
    (true, 1, 0, true, Ok(Route::ThreadPool { pool_index: 2 })),
    (true, 1, 2, true, Ok(Route::ThreadPool { pool_index: 2 })),
    (
        true,
        0,
        1,
        true,
        Ok(Route::Standalone(ThreadPlan {
            creation_flags: 0,
            thread_priority: None,
        })),
    ),
    (
        true,
        1,
        1,
        true,
        Ok(Route::Standalone(ThreadPlan {
            creation_flags: 4,
            thread_priority: Some(2),
        })),
    ),
    (
        true,
        -1,
        1,
        true,
        Ok(Route::Standalone(ThreadPlan {
            creation_flags: 4,
            thread_priority: Some(-2),
        })),
    ),
    (true, 0, 0, false, Err(E_OUTOFMEMORY)),
    (true, 3, 0, false, Err(E_INVALIDARG)),
    (false, 0, 0, false, Err(E_INVALIDARG)),
];

const NOT_IMPLEMENTED_ROWS: &[(StubMethod, HRESULT)] = &[
    (StubMethod::AsyncActionGetIids, E_NOTIMPL),
    (StubMethod::AsyncActionGetRuntimeClassName, E_NOTIMPL),
    (StubMethod::AsyncActionGetTrustLevel, E_NOTIMPL),
    (StubMethod::AsyncActionPutCompleted, E_NOTIMPL),
    (StubMethod::AsyncActionGetCompleted, E_NOTIMPL),
    (StubMethod::AsyncActionGetResults, E_NOTIMPL),
    (StubMethod::AsyncInfoGetId, E_NOTIMPL),
    (StubMethod::AsyncInfoGetStatus, E_NOTIMPL),
    (StubMethod::AsyncInfoGetErrorCode, E_NOTIMPL),
    (StubMethod::AsyncInfoCancel, E_NOTIMPL),
    (StubMethod::AsyncInfoClose, E_NOTIMPL),
    (StubMethod::ThreadPoolFactoryGetIids, E_NOTIMPL),
    (StubMethod::ThreadPoolFactoryGetRuntimeClassName, E_NOTIMPL),
    (StubMethod::ThreadPoolFactoryGetTrustLevel, E_NOTIMPL),
    (StubMethod::ThreadPoolFactoryActivateInstance, E_NOTIMPL),
    (StubMethod::ThreadPoolStaticsGetIids, E_NOTIMPL),
    (StubMethod::ThreadPoolStaticsGetRuntimeClassName, E_NOTIMPL),
    (StubMethod::ThreadPoolStaticsGetTrustLevel, E_NOTIMPL),
];

#[test]
fn oracle_class_return_rows() {
    assert_eq!(dll_get_class_object(), CLASS_E_CLASSNOTAVAILABLE);
    assert_eq!(dll_can_unload_now(), S_FALSE);
}

#[test]
fn oracle_dll_get_activation_factory_rows() {
    for &(class_id, want, refs) in GET_ACTIVATION_FACTORY_ROWS {
        let mut factory = ThreadPoolFactory::default();
        let units: Option<Vec<u16>> = class_id.map(|s| s.encode_utf16().collect());
        let got = dll_get_activation_factory(&mut factory, units.as_deref());
        assert_eq!(
            (got, factory.ref_count()),
            (want, refs),
            "class_id {class_id:?}"
        );
    }
}

#[test]
fn oracle_factory_query_interface_rows() {
    for &(iid, want, refs) in FACTORY_QUERY_INTERFACE_ROWS {
        let mut factory = ThreadPoolFactory::default();
        assert_eq!(factory.query_interface(iid), want, "{iid:?}");
        assert_eq!(factory.ref_count(), refs, "{iid:?}");
    }
}

#[test]
fn oracle_action_query_interface_rows() {
    for &(iid, want, refs) in ACTION_QUERY_INTERFACE_ROWS {
        let mut action = AsyncAction::default();
        assert_eq!(action.query_interface(iid), want, "{iid:?}");
        assert_eq!(action.ref_count(), refs, "{iid:?}");
    }
}

#[test]
fn oracle_run_async_rows() {
    for &(handler, priority, options, alloc, want) in RUN_ASYNC_ROWS {
        let got = plan_run_async(handler, priority, options, alloc);
        assert_eq!(
            got, want,
            "handler {handler} priority {priority} options {options} alloc {alloc}"
        );
    }
}

#[test]
fn oracle_not_implemented_rows() {
    for &(method, want) in NOT_IMPLEMENTED_ROWS {
        assert_eq!(method.result(), want, "{method:?}");
    }
}
