//! Port of the portable logic in Wine's `dlls/coremessaging/` (`async.c`, `main.c`).
//!
//! Ported:
//! - The `IAsyncInfo` state machine (`async.c`): [`AsyncInfo`], [`AsyncStatus`],
//!   [`Handler`], [`Completion`], [`Callback`], [`CallbackResult`]
//! - `CreateDispatcherQueueController` argument checks (`main.c`):
//!   [`create_dispatcher_queue_controller`], [`DispatcherQueueOptions`]
//! - `IDispatcherQueueController::ShutdownQueueAsync` (`async_action_create` with
//!   `shutdown_queue_async`): [`shutdown_queue_async`], [`shutdown_queue_callback`]
//! - `DllGetActivationFactory` class-name dispatch: [`dll_get_activation_factory`],
//!   [`ActivationClass`]
//! - The `FIXME ... stub!` methods that return `E_NOTIMPL`: [`StubMethod`], [`stub_method`]
//! - `FAILED`: [`failed`]
//!
//! Skipped:
//! - `@ stub` exports in `coremessaging.spec` (`CoreUI*`, `Msg*`, `ServiceMain`, ...).
//!   Wine's stub raises `EXCEPTION_WINE_STUB` via `__wine_spec_unimplemented_stub`
//!   (`dlls/ntdll/exception.c`) and never returns, so there is no return value to model.
//! - `DllCanUnloadNow`: private in the spec, with no body in this directory.
//! - `CreateDispatcherQueueForCurrentThread`, `GetDispatcherQueueForCurrentThread`, and
//!   `DllGetClassObject`: `@ stub` in the spec and absent from `main.c`.
//! - COM plumbing: vtables, `QueryInterface` IID dispatch, `AddRef`/`Release`, the
//!   `IInspectable` outer object, critical sections, and threadpool submission.
//! - Allocation failures (`E_OUTOFMEMORY`, `CreateThreadpoolWork` failure).
//! - HSTRING handling: the class name is passed in as a NUL-terminated UTF-16 slice.

mod activation;
mod async_info;
mod dispatcher_queue;
mod stubs;

pub use activation::{dll_get_activation_factory, ActivationClass};
pub use async_info::{AsyncInfo, AsyncStatus, Callback, CallbackResult, Completion, Handler};
pub use dispatcher_queue::{
    create_dispatcher_queue_controller, shutdown_queue_async, shutdown_queue_callback,
    DispatcherQueueOptions, DISPATCHER_QUEUE_OPTIONS_SIZE, DQTYPE_THREAD_CURRENT,
    DQTYPE_THREAD_DEDICATED,
};
pub use stubs::{stub_method, StubMethod};

/// `HRESULT`, as a signed 32-bit value so that [`failed`] is a sign test.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const E_INVALIDARG: Hresult = 0x8007_0057u32 as i32;
pub const E_POINTER: Hresult = 0x8000_4003u32 as i32;
pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;
pub const E_ILLEGAL_STATE_CHANGE: Hresult = 0x8000_000Du32 as i32;
pub const E_ILLEGAL_METHOD_CALL: Hresult = 0x8000_000Eu32 as i32;
pub const E_ILLEGAL_DELEGATE_ASSIGNMENT: Hresult = 0x8000_0018u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;
pub const STATUS_PENDING: Hresult = 0x0000_0103;

/// `FAILED(hr)`.
pub fn failed(hr: Hresult) -> bool {
    hr < 0
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
