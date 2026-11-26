//! The `FIXME ... stub!` methods from Wine's `dlls/coremessaging/main.c`, each of which
//! is `FIXME(...); return E_NOTIMPL;`.
//!
//! The statics object forwards its `IInspectable` methods to the factory
//! (`DEFINE_IINSPECTABLE` in `private.h`), so those are listed once, under the factory.

use crate::{Hresult, E_NOTIMPL};

/// A method whose Wine body is a `FIXME` followed by `return E_NOTIMPL`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StubMethod {
    FactoryGetIids,
    FactoryGetRuntimeClassName,
    FactoryGetTrustLevel,
    FactoryActivateInstance,
    ControllerGetIids,
    ControllerGetRuntimeClassName,
    ControllerGetTrustLevel,
    ControllerGetDispatcherQueue,
    StaticsCreateOnDedicatedThread,
}

/// Every stub method returns `E_NOTIMPL` and has no other effect.
pub fn stub_method(_method: StubMethod) -> Hresult {
    E_NOTIMPL
}
