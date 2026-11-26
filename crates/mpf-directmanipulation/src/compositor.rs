//! `struct directcompositor`: the Compositor2 and FrameInfoProvider interfaces share one count.

use crate::guid::Guid;
use crate::hresult::{HResult, E_INVALIDARG, S_OK};
use crate::query::{query_counted, Iface, Object};
use crate::refcount::RefCount;
use crate::update_manager::UpdateManager;

/// `manager` in Wine is a bare pointer. Only whether one reference was taken is kept here, so the
/// caller passes the manager back to `release`.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Compositor {
    refs: RefCount,
    manager_held: bool,
}

impl Compositor {
    pub fn ref_count(&self) -> u32 {
        self.refs.count()
    }

    pub fn add_ref(&mut self) -> u32 {
        self.refs.add_ref()
    }

    /// When the count reaches zero, the update manager reference taken by `set_update_manager` is released
    /// if one is held. Wine does not release a manager replaced by a later `set_update_manager` call, so that
    /// reference is never released.
    pub fn release(&mut self, manager: Option<&mut UpdateManager>) -> u32 {
        let count = self.refs.release();
        if count == 0 && self.manager_held {
            if let Some(manager) = manager {
                manager.release();
            }
        }
        count
    }

    pub fn query_interface(&mut self, iid: &Guid) -> Result<Iface, HResult> {
        query_counted(Object::Compositor, iid, &mut self.refs)
    }

    pub fn set_update_manager(&mut self, manager: Option<&mut UpdateManager>) -> HResult {
        match manager {
            None => E_INVALIDARG,
            Some(manager) => {
                self.manager_held = true;
                manager.add_ref();
                S_OK
            }
        }
    }
}
