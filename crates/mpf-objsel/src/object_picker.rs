//! The `IDsObjectPicker` object in `dlls/objsel/objsel.c`: `IDsObjectPickerImpl`, its vtable methods,
//! and `object_picker_create`.

use crate::guid::{is_equal_guid, Guid, IID_DS_OBJECT_PICKER, IID_UNKNOWN};
use crate::hresult::{Hresult, E_NOINTERFACE, E_OUTOFMEMORY, E_POINTER, S_FALSE, S_OK};

/// The interface pointer that a `QueryInterface` or `IClassFactory` call hands out. `IUnknown` queries on
/// the picker return the same `IDsObjectPicker` pointer as `IDsObjectPicker` queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    DsObjectPicker,
    ClassFactory,
}

/// `IDsObjectPickerImpl`. `refs` is the `LONG ref` field. `destroyed` records the `HeapFree` in
/// `OBJSEL_IDsObjectPicker_Destroy`, which runs when the count reaches zero.
#[derive(Debug, PartialEq, Eq)]
pub struct ObjectPicker {
    pub refs: i32,
    pub destroyed: bool,
}

/// `object_picker_create`. `heap_alloc` stands in for `HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, ...)`,
/// and `false` is a NULL return. That gives `E_OUTOFMEMORY` and leaves `*ppvObj` alone. On success the new
/// object holds the one reference that the C takes with `AddRef`.
pub fn object_picker_create(
    out: &mut Option<ObjectPicker>,
    heap_alloc: impl FnOnce() -> bool,
) -> Hresult {
    if !heap_alloc() {
        return E_OUTOFMEMORY;
    }
    let mut picker = ObjectPicker {
        refs: 0,
        destroyed: false,
    };
    picker.add_ref();
    *out = Some(picker);
    S_OK
}

impl ObjectPicker {
    /// `OBJSEL_IDsObjectPicker_AddRef`: `InterlockedIncrement`, with the 32-bit wrap, returned as a `ULONG`.
    pub fn add_ref(&mut self) -> u32 {
        self.refs = self.refs.wrapping_add(1);
        self.refs as u32
    }

    /// `OBJSEL_IDsObjectPicker_Release`: `InterlockedDecrement`, returned as a `ULONG`. Only a count of
    /// exactly zero destroys the object, so a count that wraps below zero does not.
    pub fn release(&mut self) -> u32 {
        self.refs = self.refs.wrapping_sub(1);
        if self.refs == 0 {
            self.destroyed = true;
        }
        self.refs as u32
    }

    /// `OBJSEL_IDsObjectPicker_QueryInterface`. `out` is `ppvObj`, and `None` is a NULL pointer. On success
    /// `*ppvObj` is the picker, and the count goes up by one, as the C `AddRef` does.
    pub fn query_interface(&mut self, riid: &Guid, out: Option<&mut Option<Interface>>) -> Hresult {
        let Some(out) = out else {
            return E_POINTER;
        };
        if is_equal_guid(riid, &IID_UNKNOWN) || is_equal_guid(riid, &IID_DS_OBJECT_PICKER) {
            *out = Some(Interface::DsObjectPicker);
            self.add_ref();
            return S_OK;
        }
        E_NOINTERFACE
    }

    /// `OBJSEL_IDsObjectPicker_Initialize`: `FIXME("stub!")`, then `S_OK`. `pInitInfo` is not read.
    pub fn initialize(&self) -> Hresult {
        S_OK
    }

    /// `OBJSEL_IDsObjectPicker_InvokeDialog`: `FIXME("stub!")`, then `S_FALSE`. The parent window and
    /// `ppdoSelections` are not read or written.
    pub fn invoke_dialog(&self) -> Hresult {
        S_FALSE
    }
}
