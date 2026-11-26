//! `CreateInstance` for the manager and the compositor. The factory creates the object with a count of 1,
//! queries the requested IID, then releases its own reference, so the object survives only if the query succeeded.

use crate::compositor::Compositor;
use crate::guid::Guid;
use crate::hresult::HResult;
use crate::manager::Manager;
use crate::query::Iface;

pub fn create_manager_instance(iid: &Guid) -> Result<(Manager, Iface), HResult> {
    let mut manager = Manager::default();
    let result = manager.query_interface(iid);
    manager.release();
    result.map(|iface| (manager, iface))
}

pub fn create_compositor_instance(iid: &Guid) -> Result<(Compositor, Iface), HResult> {
    let mut compositor = Compositor::default();
    let result = compositor.query_interface(iid);
    compositor.release(None);
    result.map(|iface| (compositor, iface))
}
