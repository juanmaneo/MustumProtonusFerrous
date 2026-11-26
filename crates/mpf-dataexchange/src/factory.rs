//! The `dataexchange` activation factory that `DllGetActivationFactory` returns. The statics
//! and interop interfaces are forwarders to the same object, so one count covers all three.

use crate::hresult::{Hresult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_NOTIMPL, S_OK};
use crate::iid::{factory_interface, FactoryInterface, Iid};
use crate::manager::{create_core_dragdrop_manager, CoreDragDropManager, Window};
use crate::refcount::RefCount;

/// The static `dataexchange` object.
#[derive(Debug)]
pub struct DataExchange {
    refs: RefCount,
}

impl DataExchange {
    /// The static object. It starts with one reference, which the DLL never releases.
    pub fn static_object() -> Self {
        Self {
            refs: RefCount::initial(),
        }
    }

    pub fn ref_count(&self) -> u32 {
        self.refs.get()
    }

    /// Returns the interface that `iid` selects and takes a reference on it.
    pub fn query_interface(&mut self, iid: Iid) -> Result<FactoryInterface, Hresult> {
        let interface = factory_interface(iid).ok_or(E_NOINTERFACE)?;
        self.add_ref();
        Ok(interface)
    }

    pub fn add_ref(&mut self) -> u32 {
        self.refs.add_ref()
    }

    pub fn release(&mut self) -> u32 {
        self.refs.release()
    }

    /// `GetIids`, `GetRuntimeClassName`, `GetTrustLevel` and `ActivateInstance` are FIXME stubs.
    pub fn get_iids(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn get_runtime_class_name(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn get_trust_level(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn activate_instance(&self) -> Hresult {
        E_NOTIMPL
    }

    /// `ICoreDragDropManagerStatics::GetForCurrentView`, a FIXME stub.
    pub fn get_for_current_view(&self) -> Hresult {
        E_NOTIMPL
    }

    /// `IDragDropManagerInterop::GetForWindow`. Wine marks it a semi-stub, but it does the
    /// real work, so it forwards to [`create_core_dragdrop_manager`] without touching the count.
    pub fn get_for_window(
        &self,
        window: Window,
        allocation_ok: bool,
        iid: Iid,
    ) -> Result<CoreDragDropManager, Hresult> {
        create_core_dragdrop_manager(window, allocation_ok, iid)
    }
}

/// `DllGetActivationFactory`. Hands out the static factory with one new reference.
pub fn dll_get_activation_factory(factory: &mut DataExchange) -> Hresult {
    factory.add_ref();
    S_OK
}

/// `DllGetClassObject` always fails, whatever the class and interface IDs are.
pub fn dll_get_class_object() -> Hresult {
    CLASS_E_CLASSNOTAVAILABLE
}
