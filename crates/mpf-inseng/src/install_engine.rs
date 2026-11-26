//! The `InstallEngine` object from `inseng_main.c`: the `QueryInterface`, `AddRef` and `Release`
//! of its `IInstallEngine2` vtable. The other methods are in `not_implemented`.

use crate::com::{
    is_equal_guid, Guid, HResult, E_NOINTERFACE, IID_INSTALL_ENGINE, IID_INSTALL_ENGINE2,
    IID_UNKNOWN,
};

/// `struct InstallEngine`: the reference count, which is `LONG ref` in C.
pub struct InstallEngine {
    ref_count: i32,
}

impl InstallEngine {
    /// The state that `InstallEngineCF_CreateInstance` gives a new object.
    pub(crate) fn created() -> Self {
        Self { ref_count: 1 }
    }

    pub fn ref_count(&self) -> i32 {
        self.ref_count
    }

    /// `InstallEngine_QueryInterface`. Only a successful query adds a reference.
    pub fn query_interface(&mut self, riid: &Guid) -> Result<(), HResult> {
        if is_equal_guid(&IID_UNKNOWN, riid)
            || is_equal_guid(&IID_INSTALL_ENGINE, riid)
            || is_equal_guid(&IID_INSTALL_ENGINE2, riid)
        {
            self.add_ref();
            Ok(())
        } else {
            Err(E_NOINTERFACE)
        }
    }

    /// `InstallEngine_AddRef`: returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `InstallEngine_Release`: returns the new count. The C code frees the object at 0, which
    /// the port leaves to `Drop`.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        self.ref_count as u32
    }
}
