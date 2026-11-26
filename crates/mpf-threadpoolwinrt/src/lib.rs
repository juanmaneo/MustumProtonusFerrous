//! Port of the portable logic in Wine's `dlls/threadpoolwinrt/main.c`, and of the `DllCanUnloadNow`
//! default that the DLL links from `libs/winecrt0/dll_canunload.c`.
//!
//! Ported:
//! - `DllGetActivationFactory` class-id match (`wcscmp`, so the id ends at the first NUL) and the
//!   factory reference it hands out: [`dll_get_activation_factory`], [`THREAD_POOL_CLASS`]
//! - `DllGetClassObject` and `DllCanUnloadNow`: [`dll_get_class_object`], [`dll_can_unload_now`]
//! - `threadpool_factory_QueryInterface` and `threadpool_statics_QueryInterface` IID table:
//!   [`Iid`], [`FactoryView`], [`ThreadPoolFactory::query_interface`]
//! - `async_action_QueryInterface` IID table (`async_info_QueryInterface` forwards to it):
//!   [`ActionView`], [`AsyncAction::query_interface`]
//! - `AddRef` and `Release` counts of the factory and of the async action:
//!   [`ThreadPoolFactory::add_ref`], [`ThreadPoolFactory::release`], [`AsyncAction::add_ref`],
//!   [`AsyncAction::release`]
//! - The checks, routing and allocation outcome at the top of `run_async`, and the pool index of
//!   `submit_threadpool_work`: [`plan_run_async`], [`Route`], [`Priority`], [`POOL_MAX_THREADS`]
//! - The creation flags and thread priority of `submit_standalone_thread_work`:
//!   [`standalone_thread_plan`], [`ThreadPlan`]
//! - The 18 explicit `E_NOTIMPL` methods of the factory, statics and async objects: [`StubMethod`]
//! - `HRESULT` values from `winerror.h`, and `SUCCEEDED`: [`succeeded`] and the `HRESULT` constants
//!
//! Skipped:
//! - Thread pool and thread creation (`CreateThreadpool`, `CreateThreadpoolWork`,
//!   `SubmitThreadpoolWork`, `InitOnceExecuteOnce`, `CreateThread`, `SetThreadPriority`,
//!   `ResumeThread`, `CloseHandle`) and their failure results (`E_FAIL`, `HRESULT_FROM_WIN32`).
//!   [`plan_run_async`] returns the decision that is made before those calls.
//! - Running the work item (`sliced_thread_proc`, `pool_work_callback`, `work_item_invoke_release`)
//!   and the `IWorkItemHandler` `AddRef`, `Release` and `Invoke` calls.
//! - The `assert` in `submit_threadpool_work`: the checks in [`plan_run_async`] reject every
//!   priority outside `Low..=High` first.
//! - `WindowsGetStringRawBuffer` HSTRING storage: the class id is a UTF-16 slice, and `None` is a
//!   NULL HSTRING, which reads as the empty string.
//! - COM vtables and interface pointers: the Rust methods are called directly. The `IAsyncInfo`
//!   and `IThreadPoolStatics` objects are views of the counted object that owns them.
//! - `calloc` and `free`: the allocation outcome is an input, and dropping the object replaces
//!   `free`.
//! - The `RunAsync`, `RunWithPriorityAsync` and `RunWithPriorityAndOptionsAsync` wrappers. They
//!   pass a fixed priority and options to `run_async`, so they map to [`plan_run_async`] with
//!   `Normal` and `None`, the given priority with `None`, or the given priority and options.
//! - `IAsyncInfo` `GetIids`, `GetRuntimeClassName` and `GetTrustLevel`, which forward to
//!   `IAsyncAction`. They return `E_NOTIMPL` through that forwarding, so they are not listed in
//!   [`StubMethod`] on their own.
//! - `FIXME`, `TRACE` and `WARN` output, and the `debugstr_*` helpers.
//! - The arguments of `DllGetClassObject`, which the C body ignores.

mod async_action;
mod dll;
mod factory;
mod hresult;
mod query;
mod refcount;
mod stub;
mod thread;
mod work_item;

pub use async_action::AsyncAction;
pub use dll::{
    dll_can_unload_now, dll_get_activation_factory, dll_get_class_object, THREAD_POOL_CLASS,
};
pub use factory::ThreadPoolFactory;
pub use hresult::{
    succeeded, CLASS_E_CLASSNOTAVAILABLE, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY,
    HRESULT, S_FALSE, S_OK,
};
pub use query::{ActionView, FactoryView, Iid};
pub use stub::StubMethod;
pub use thread::{
    standalone_thread_plan, ThreadPlan, CREATE_SUSPENDED, THREAD_PRIORITY_HIGHEST,
    THREAD_PRIORITY_LOWEST,
};
pub use work_item::{plan_run_async, Priority, Route, POOL_MAX_THREADS};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
