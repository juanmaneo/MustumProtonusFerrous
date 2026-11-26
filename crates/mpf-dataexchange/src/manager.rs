//! The `CoreDragDropManager` that `GetForWindow` returns, and the window checks that run first.

use crate::hresult::{
    hresult_from_win32, Hresult, ERROR_INVALID_WINDOW_HANDLE, E_ACCESSDENIED, E_NOINTERFACE,
    E_NOTIMPL, E_OUTOFMEMORY, S_OK,
};
use crate::iid::Iid;
use crate::refcount::RefCount;

/// What `IsWindow` and `GetDesktopWindow` report about the `hwnd` argument. The handle value
/// itself is not modelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Window {
    /// `IsWindow` returns FALSE.
    Invalid,
    /// The handle is the desktop window.
    Desktop,
    /// A valid window that is not the desktop.
    Ordinary,
}

/// The manager that Wine allocates with `calloc` and frees when its count reaches zero.
/// Rust drops it at the owner's end of scope, so `release` only reports the count.
#[derive(Debug)]
pub struct CoreDragDropManager {
    refs: RefCount,
}

impl CoreDragDropManager {
    fn allocate() -> Self {
        Self {
            refs: RefCount::initial(),
        }
    }

    pub fn ref_count(&self) -> u32 {
        self.refs.get()
    }

    /// Unlike the factory, the manager does not answer `IAgileObject`.
    pub fn query_interface(&mut self, iid: Iid) -> Result<(), Hresult> {
        match iid {
            Iid::IUnknown | Iid::IInspectable | Iid::ICoreDragDropManager => {
                self.add_ref();
                Ok(())
            }
            _ => Err(E_NOINTERFACE),
        }
    }

    pub fn add_ref(&mut self) -> u32 {
        self.refs.add_ref()
    }

    pub fn release(&mut self) -> u32 {
        self.refs.release()
    }

    /// `GetIids`, `GetRuntimeClassName` and `GetTrustLevel` are FIXME stubs.
    pub fn get_iids(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn get_runtime_class_name(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn get_trust_level(&self) -> Hresult {
        E_NOTIMPL
    }

    /// Returns a fixed token and does not keep the handler.
    pub fn add_target_requested(&self) -> (Hresult, i64) {
        (S_OK, 0xdead_beef)
    }

    pub fn remove_target_requested(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn get_are_concurrent_operations_enabled(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn put_are_concurrent_operations_enabled(&self) -> Hresult {
        E_NOTIMPL
    }
}

/// `create_core_dragdrop_manager`. The window checks run before any allocation, in Wine's
/// order. The new manager starts at one reference, and the query's reference is dropped
/// before the result is returned, so a failed query leaves no live manager behind.
pub fn create_core_dragdrop_manager(
    window: Window,
    allocation_ok: bool,
    iid: Iid,
) -> Result<CoreDragDropManager, Hresult> {
    if window == Window::Invalid {
        return Err(hresult_from_win32(ERROR_INVALID_WINDOW_HANDLE));
    }
    if window == Window::Desktop {
        return Err(E_ACCESSDENIED);
    }
    if !allocation_ok {
        return Err(E_OUTOFMEMORY);
    }
    let mut manager = CoreDragDropManager::allocate();
    let query = manager.query_interface(iid);
    manager.release();
    query.map(|()| manager)
}
