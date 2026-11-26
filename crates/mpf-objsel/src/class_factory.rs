//! The static class factory in `dlls/objsel/objsel.c`: `struct class_factory` and its `IClassFactory` methods.

use crate::guid::{is_equal_guid, Guid, IID_CLASS_FACTORY, IID_DS_OBJECT_PICKER, IID_UNKNOWN};
use crate::hresult::{
    Hresult, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_NOINTERFACE, E_POINTER, S_OK,
};
use crate::object_picker::{object_picker_create, Interface, ObjectPicker};

/// `struct class_factory`. `refs` is the `LONG ref` field. The C keeps one static instance, which
/// `DllGetClassObject` shares, so the caller passes that instance in. `Default` is the static's initial
/// state, a count of zero.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ClassFactory {
    pub refs: i32,
}

impl ClassFactory {
    /// `class_factory_QueryInterface`. `out` is `out`, and `None` is a NULL pointer. `IUnknown` and
    /// `IClassFactory` succeed and take a reference, as `IClassFactory_AddRef` does. Any other IID fails
    /// and leaves `*out` alone.
    pub fn query_interface(&mut self, riid: &Guid, out: Option<&mut Option<Interface>>) -> Hresult {
        let Some(out) = out else {
            return E_POINTER;
        };
        if is_equal_guid(riid, &IID_UNKNOWN) || is_equal_guid(riid, &IID_CLASS_FACTORY) {
            *out = Some(Interface::ClassFactory);
            self.add_ref();
            return S_OK;
        }
        E_NOINTERFACE
    }

    /// `class_factory_AddRef`: `InterlockedIncrement`, returned as a `ULONG`.
    pub fn add_ref(&mut self) -> u32 {
        self.refs = self.refs.wrapping_add(1);
        self.refs as u32
    }

    /// `class_factory_Release`: `InterlockedDecrement`, returned as a `ULONG`. The static factory is never
    /// destroyed, so the count can go below zero.
    pub fn release(&mut self) -> u32 {
        self.refs = self.refs.wrapping_sub(1);
        self.refs as u32
    }

    /// `class_factory_CreateInstance`. `out` is `out`, and `None` is a NULL pointer. `outer` is true when
    /// the C `outer` pointer is non-NULL. Only `IDsObjectPicker` can be created. `heap_alloc` is passed to
    /// [`object_picker_create`].
    pub fn create_instance(
        &self,
        outer: bool,
        riid: &Guid,
        out: Option<&mut Option<ObjectPicker>>,
        heap_alloc: impl FnOnce() -> bool,
    ) -> Hresult {
        let Some(out) = out else {
            return E_POINTER;
        };
        if outer {
            return CLASS_E_NOAGGREGATION;
        }
        if is_equal_guid(&IID_DS_OBJECT_PICKER, riid) {
            return object_picker_create(out, heap_alloc);
        }
        CLASS_E_CLASSNOTAVAILABLE
    }

    /// `class_factory_LockServer`: `lock` takes a reference, and `!lock` drops one. Always `S_OK`.
    pub fn lock_server(&mut self, lock: bool) -> Hresult {
        if lock {
            self.add_ref();
        } else {
            self.release();
        }
        S_OK
    }
}
