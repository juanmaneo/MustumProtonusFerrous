//! Port of `struct async_action` and the `IAsyncAction` and `IAsyncInfo` `QueryInterface`,
//! `AddRef` and `Release` bodies in Wine's `dlls/threadpoolwinrt/main.c`.

use crate::hresult::{E_NOINTERFACE, HRESULT};
use crate::query::{action_view, ActionView, Iid};
use crate::refcount::RefCount;

/// An async action from `async_action_create`, which starts with `refcount = 1`.
#[derive(Debug)]
pub struct AsyncAction {
    ref_count: RefCount,
}

impl Default for AsyncAction {
    fn default() -> Self {
        Self {
            ref_count: RefCount::initial(),
        }
    }
}

impl AsyncAction {
    /// `async_action_QueryInterface`. A successful query adds a reference and returns the view.
    pub fn query_interface(&mut self, iid: Iid) -> Result<ActionView, HRESULT> {
        let view = action_view(iid).ok_or(E_NOINTERFACE)?;
        self.add_ref();
        Ok(view)
    }

    /// `async_action_AddRef`: returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count.add_ref()
    }

    /// `async_action_Release`: returns the new count. At zero the C body calls `free`; in Rust the
    /// owner drops the value instead.
    pub fn release(&mut self) -> u32 {
        self.ref_count.release()
    }

    pub fn ref_count(&self) -> i32 {
        self.ref_count.get()
    }
}
