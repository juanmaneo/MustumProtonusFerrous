//! `IDirectManipulationViewport2` object (`struct directviewport`).

use crate::guid::{classify_iid, Guid, Iid};
use crate::hresult::{HResult, E_NOTIMPL};
use crate::primary::PrimaryContext;
use crate::query::{query_counted, Iface, Object};
use crate::refcount::RefCount;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Viewport {
    refs: RefCount,
}

impl Viewport {
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
        query_counted(Object::Viewport, iid, &mut self.refs)
    }

    /// Returns a new primary content object with its own count of 1. The viewport's count is unchanged.
    pub fn get_primary_content(&self, iid: &Guid) -> Result<PrimaryContext, HResult> {
        match classify_iid(iid) {
            Some(Iid::PrimaryContent) => Ok(PrimaryContext::default()),
            _ => Err(E_NOTIMPL),
        }
    }
}
