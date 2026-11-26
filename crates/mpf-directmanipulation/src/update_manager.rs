//! `IDirectManipulationUpdateManager` object (`struct directupdatemanager`).

use crate::guid::Guid;
use crate::hresult::HResult;
use crate::query::{query_counted, Iface, Object};
use crate::refcount::RefCount;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UpdateManager {
    refs: RefCount,
}

impl UpdateManager {
    pub fn ref_count(&self) -> u32 {
        self.refs.count()
    }

    pub fn add_ref(&mut self) -> u32 {
        self.refs.add_ref()
    }

    pub fn release(&mut self) -> u32 {
        self.refs.release()
    }

    pub fn query_interface(&mut self, iid: &Guid) -> Result<Iface, HResult> {
        query_counted(Object::UpdateManager, iid, &mut self.refs)
    }
}
