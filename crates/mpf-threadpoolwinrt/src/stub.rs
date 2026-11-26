//! The explicit `E_NOTIMPL` methods of the factory, statics and async action objects in Wine's
//! `dlls/threadpoolwinrt/main.c`. Each body only emits a FIXME and returns `E_NOTIMPL`.

use crate::hresult::{E_NOTIMPL, HRESULT};

/// One stub method. The `IAsyncInfo` forwarders are not listed; see the crate documentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StubMethod {
    AsyncActionGetIids,
    AsyncActionGetRuntimeClassName,
    AsyncActionGetTrustLevel,
    AsyncActionPutCompleted,
    AsyncActionGetCompleted,
    AsyncActionGetResults,
    AsyncInfoGetId,
    AsyncInfoGetStatus,
    AsyncInfoGetErrorCode,
    AsyncInfoCancel,
    AsyncInfoClose,
    ThreadPoolFactoryGetIids,
    ThreadPoolFactoryGetRuntimeClassName,
    ThreadPoolFactoryGetTrustLevel,
    ThreadPoolFactoryActivateInstance,
    ThreadPoolStaticsGetIids,
    ThreadPoolStaticsGetRuntimeClassName,
    ThreadPoolStaticsGetTrustLevel,
}

impl StubMethod {
    /// The value the C body returns.
    pub fn result(self) -> HRESULT {
        E_NOTIMPL
    }
}
