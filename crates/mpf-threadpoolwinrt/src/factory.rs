//! Port of `struct threadpool_factory` and the `IActivationFactory` and `IThreadPoolStatics`
//! `QueryInterface`, `AddRef` and `Release` bodies in Wine's `dlls/threadpoolwinrt/main.c`.

use crate::hresult::{E_NOINTERFACE, HRESULT};
use crate::query::{factory_view, FactoryView, Iid};
use crate::refcount::RefCount;

/// The static factory of `Windows.System.Threading.ThreadPool`. It starts with `refcount = 1` and is
/// never freed: `threadpool_factory_Release` does not call `free`. The statics object shares its count.
#[derive(Debug)]
pub struct ThreadPoolFactory {
    ref_count: RefCount,
}

impl Default for ThreadPoolFactory {
    fn default() -> Self {
        Self {
            ref_count: RefCount::initial(),
        }
    }
}

impl ThreadPoolFactory {
    /// `threadpool_factory_QueryInterface`. A successful query adds a reference and returns the view.
    pub fn query_interface(&mut self, iid: Iid) -> Result<FactoryView, HRESULT> {
        let view = factory_view(iid).ok_or(E_NOINTERFACE)?;
        self.add_ref();
        Ok(view)
    }

    /// `threadpool_factory_AddRef`: returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count.add_ref()
    }

    /// `threadpool_factory_Release`: returns the new count. The static object is never freed.
    pub fn release(&mut self) -> u32 {
        self.ref_count.release()
    }

    pub fn ref_count(&self) -> i32 {
        self.ref_count.get()
    }
}
