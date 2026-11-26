//! `IDirectManipulationManager2` object (`struct directmanipulation`).

use crate::guid::{classify_iid, Guid, Iid};
use crate::hresult::{HResult, E_FAIL, E_NOTIMPL};
use crate::query::{query_counted, Iface, Object};
use crate::refcount::RefCount;
use crate::update_manager::UpdateManager;
use crate::viewport::Viewport;

/// The update manager is created on the first `GetUpdateManager` and released with the manager.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Manager {
    refs: RefCount,
    update_manager: Option<UpdateManager>,
}

impl Manager {
    pub fn ref_count(&self) -> u32 {
        self.refs.count()
    }

    pub fn update_manager(&self) -> Option<&UpdateManager> {
        self.update_manager.as_ref()
    }

    pub fn add_ref(&mut self) -> u32 {
        self.refs.add_ref()
    }

    pub fn release(&mut self) -> u32 {
        let count = self.refs.release();
        if count == 0 {
            if let Some(update_manager) = self.update_manager.as_mut() {
                update_manager.release();
            }
        }
        count
    }

    pub fn query_interface(&mut self, iid: &Guid) -> Result<Iface, HResult> {
        query_counted(Object::Manager, iid, &mut self.refs)
    }

    /// Only the UpdateManager IID is accepted. Each successful call hands out one new reference.
    pub fn get_update_manager(&mut self, iid: &Guid) -> Result<&mut UpdateManager, HResult> {
        if classify_iid(iid) != Some(Iid::UpdateManager) {
            return Err(E_FAIL);
        }
        let update_manager = self
            .update_manager
            .get_or_insert_with(UpdateManager::default);
        update_manager.add_ref();
        Ok(update_manager)
    }

    pub fn create_viewport(&self, iid: &Guid) -> Result<Viewport, HResult> {
        match classify_iid(iid) {
            Some(Iid::Viewport | Iid::Viewport2) => Ok(Viewport::default()),
            _ => Err(E_NOTIMPL),
        }
    }
}
