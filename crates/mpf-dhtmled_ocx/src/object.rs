//! `dhtml_edit_create` and the `DHTMLEditImpl` reference count (`dhtml_edit_addref` / `dhtml_edit_release`).

use crate::hresult::{HResult, E_NOINTERFACE};
use crate::qi::{dhtml_edit_qi, Iid, Target};

/// `DHTMLEditImpl::ref`. A new object starts at 1; the object is freed when a release reaches 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectRef {
    count: i32,
}

impl ObjectRef {
    /// Count of a freshly allocated object (`This->ref = 1`).
    pub fn initial() -> Self {
        Self { count: 1 }
    }

    /// `dhtml_edit_addref`.
    pub fn add_ref(&mut self) -> i32 {
        self.count += 1;
        self.count
    }

    /// `dhtml_edit_release`. A result of 0 means the object is freed.
    pub fn release(&mut self) -> i32 {
        self.count -= 1;
        self.count
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

/// `dhtml_edit_create`: queries `iid` on a new object, then drops the creation reference.
/// On success returns the interface and the remaining count. On failure the object is freed.
pub fn dhtml_edit_create(iid: Iid) -> Result<(Target, i32), HResult> {
    let mut refs = ObjectRef::initial();
    let found = dhtml_edit_qi(iid);
    if found.is_some() {
        refs.add_ref();
    }
    refs.release();
    match found {
        Some(target) => Ok((target, refs.count())),
        None => Err(E_NOINTERFACE),
    }
}
