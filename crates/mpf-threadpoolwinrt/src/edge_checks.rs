//! Boundary and branch tests for the ported threadpoolwinrt logic.
//!
//! The expectations come from the C source: the `winerror.h` values, `wcscmp` stopping at the
//! first NUL, `InterlockedDecrement` wrapping, and the order of the checks in `run_async`. Each
//! test that is not backed by an oracle row says so in a comment.

use super::*;
use crate::query::{action_view, factory_view};

fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winbase.h, include/winnt.h
    // and include/windows.system.threading.idl.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOINTERFACE, 0x8000_4002_u32 as i32);
    assert_eq!(E_NOTIMPL, 0x8000_4001_u32 as i32);
    assert_eq!(E_INVALIDARG, 0x8007_0057_u32 as i32);
    assert_eq!(E_OUTOFMEMORY, 0x8007_000E_u32 as i32);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111_u32 as i32);
    assert_eq!(CREATE_SUSPENDED, 0x4);
    assert_eq!(THREAD_PRIORITY_HIGHEST, 2);
    assert_eq!(THREAD_PRIORITY_LOWEST, -2);
    assert_eq!(POOL_MAX_THREADS, 10);
    assert_eq!(THREAD_POOL_CLASS, "Windows.System.Threading.ThreadPool");
}

#[test]
fn succeeded_checks_the_severity_bit() {
    // Not oracle-derived: SUCCEEDED(hr) is hr >= 0 in winerror.h.
    assert!(succeeded(S_OK));
    assert!(succeeded(S_FALSE));
    assert!(succeeded(i32::MAX));
    assert!(!succeeded(-1));
    assert!(!succeeded(E_NOTIMPL));
    assert!(!succeeded(i32::MIN));
}

#[test]
fn class_id_nul_edges() {
    // Not oracle-derived: wcscmp stops at the first NUL, so a trailing NUL matches and anything
    // before a NUL must match on its own. An empty id and a NULL HSTRING both read as "".
    let cases: Vec<(Vec<u16>, HRESULT)> = vec![
        (utf16("Windows.System.Threading.ThreadPool\0"), S_OK),
        (
            utf16("\0Windows.System.Threading.ThreadPool"),
            CLASS_E_CLASSNOTAVAILABLE,
        ),
        (
            utf16("Windows.System.Threading.ThreadPoo\0l"),
            CLASS_E_CLASSNOTAVAILABLE,
        ),
        (vec![0], CLASS_E_CLASSNOTAVAILABLE),
        (vec![], CLASS_E_CLASSNOTAVAILABLE),
    ];
    for (class_id, want) in cases {
        let mut factory = ThreadPoolFactory::default();
        let got = dll_get_activation_factory(&mut factory, Some(class_id.as_slice()));
        let refs = if want == S_OK { 2 } else { 1 };
        assert_eq!((got, factory.ref_count()), (want, refs), "{class_id:?}");
    }
}

#[test]
fn factory_refcount_counts_each_successful_activation() {
    // Not oracle-derived for the three repeated calls: the oracle has one call per row. Each match
    // adds one reference, and a non-match adds none.
    let mut factory = ThreadPoolFactory::default();
    let class_id = utf16(THREAD_POOL_CLASS);
    for _ in 0..3 {
        assert_eq!(
            dll_get_activation_factory(&mut factory, Some(class_id.as_slice())),
            S_OK
        );
    }
    assert_eq!(factory.ref_count(), 4);
    let other = utf16("Windows.System.Threading.ThreadPoolX");
    assert_eq!(
        dll_get_activation_factory(&mut factory, Some(other.as_slice())),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(factory.ref_count(), 4);
}

#[test]
fn factory_refcount_wraps_below_zero() {
    // Not oracle-derived: InterlockedDecrement wraps, and the ULONG return reads -1 as u32::MAX.
    let mut factory = ThreadPoolFactory::default();
    assert_eq!(factory.release(), 0);
    assert_eq!(factory.release(), u32::MAX);
    assert_eq!(factory.ref_count(), -1);
    assert_eq!(factory.add_ref(), 0);
    assert_eq!(factory.ref_count(), 0);
}

#[test]
fn async_action_release_reaches_zero() {
    // Not oracle-derived: the C body calls free() at zero. The Rust owner drops the value instead,
    // so the test only checks the count.
    let mut action = AsyncAction::default();
    assert_eq!(
        action.query_interface(Iid::AsyncAction),
        Ok(ActionView::AsyncAction)
    );
    assert_eq!(action.release(), 1);
    assert_eq!(action.release(), 0);
    assert_eq!(action.ref_count(), 0);
}

#[test]
fn standalone_plan_flags_follow_priority() {
    // Not oracle-derived as a direct call. The same three plans appear in the run_async oracle rows.
    assert_eq!(
        standalone_thread_plan(Priority::Normal),
        ThreadPlan {
            creation_flags: 0,
            thread_priority: None,
        }
    );
    assert_eq!(
        standalone_thread_plan(Priority::High),
        ThreadPlan {
            creation_flags: CREATE_SUSPENDED,
            thread_priority: Some(THREAD_PRIORITY_HIGHEST),
        }
    );
    assert_eq!(
        standalone_thread_plan(Priority::Low),
        ThreadPlan {
            creation_flags: CREATE_SUSPENDED,
            thread_priority: Some(THREAD_PRIORITY_LOWEST),
        }
    );
}

#[test]
fn only_time_sliced_options_take_the_standalone_route() {
    // Not oracle-derived for -1, 3, i32::MAX and i32::MIN: the C check is
    // `options == WorkItemOptions_TimeSliced`, so every other value takes the thread pool route.
    for options in [-1, 0, 2, 3, i32::MAX, i32::MIN] {
        assert_eq!(
            plan_run_async(true, 0, options, true),
            Ok(Route::ThreadPool { pool_index: 1 }),
            "options {options}"
        );
    }
    assert_eq!(
        plan_run_async(true, 0, 1, true),
        Ok(Route::Standalone(ThreadPlan {
            creation_flags: 0,
            thread_priority: None,
        }))
    );
}

#[test]
fn priority_outside_the_range_is_rejected() {
    // Not oracle-derived for i32::MIN, i32::MAX and the priority-before-allocation case. The
    // C check is `priority < Low || priority > High`, and it runs before the allocation.
    for priority in [i32::MIN, -2, 2, i32::MAX] {
        assert_eq!(
            plan_run_async(true, priority, 0, true),
            Err(E_INVALIDARG),
            "priority {priority}"
        );
    }
    assert_eq!(plan_run_async(true, i32::MAX, 0, false), Err(E_INVALIDARG));
}

#[test]
fn views_reject_other_interfaces() {
    // Not oracle-derived: the match arms that no oracle row reaches.
    assert_eq!(action_view(Iid::ThreadPoolStatics), None);
    assert_eq!(action_view(Iid::ActivationFactory), None);
    assert_eq!(action_view(Iid::Other), None);
    assert_eq!(factory_view(Iid::AsyncAction), None);
    assert_eq!(factory_view(Iid::AsyncInfo), None);
    assert_eq!(factory_view(Iid::Other), None);
}
