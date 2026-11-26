//! `struct primarycontext`: the PrimaryContent interface and the Content interface share one count.

use crate::guid::Guid;
use crate::hresult::HResult;
use crate::query::{query_counted, Iface, Object};
use crate::refcount::RefCount;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PrimaryContext {
    refs: RefCount,
}

impl PrimaryContext {
    pub fn ref_count(&self) -> u32 {
        self.refs.count()
    }

    pub fn add_ref(&mut self) -> u32 {
        self.refs.add_ref()
    }

    pub fn release(&mut self) -> u32 {
        self.refs.release()
    }

    /// Shared by the PrimaryContent and Content interfaces, which route to the same QueryInterface.
    pub fn query_interface(&mut self, iid: &Guid) -> Result<Iface, HResult> {
        query_counted(Object::PrimaryContext, iid, &mut self.refs)
    }
}
