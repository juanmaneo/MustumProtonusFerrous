//! COM values from `include/winerror.h`, `include/objidl.h`, `include/wia_lh.idl` and
//! `dlls/wiaservc/wiaservc.idl`, plus the reference count the COM objects keep.

use std::cell::Cell;

/// `HRESULT`.
pub type Hresult = i32;

pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as Hresult;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as Hresult;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000Eu32 as Hresult;
pub const CLASS_E_NOAGGREGATION: Hresult = 0x8004_0110u32 as Hresult;

/// `FAILED(hr)`.
pub fn failed(hr: Hresult) -> bool {
    hr < 0
}

/// `GUID`, compared field by field as `IsEqualGUID` does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

pub const IID_IWIADEVMGR: Guid = Guid {
    data1: 0x5eb2_502a,
    data2: 0x8cf1,
    data3: 0x11d1,
    data4: [0xbf, 0x92, 0x00, 0x60, 0x08, 0x1e, 0xd8, 0x11],
};

pub const IID_IENUMWIA_DEV_INFO: Guid = Guid {
    data1: 0x5e38_b83c,
    data2: 0x8cf1,
    data3: 0x11d1,
    data4: [0xbf, 0x92, 0x00, 0x60, 0x08, 0x1e, 0xd8, 0x11],
};

pub const CLSID_WIADEVMGR: Guid = Guid {
    data1: 0xa1f4_e726,
    data2: 0x8cf1,
    data3: 0x11d1,
    data4: [0xbf, 0x92, 0x00, 0x60, 0x08, 0x1e, 0xd8, 0x11],
};

/// The `ULONG` reference count of a COM object.
#[derive(Debug)]
pub(crate) struct RefCount(Cell<u32>);

impl RefCount {
    pub(crate) fn new(count: u32) -> Self {
        Self(Cell::new(count))
    }

    pub(crate) fn get(&self) -> u32 {
        self.0.get()
    }

    /// `InterlockedIncrement`: returns the new count.
    pub(crate) fn add_ref(&self) -> u32 {
        let count = self.0.get() + 1;
        self.0.set(count);
        count
    }

    /// `InterlockedDecrement`: returns the new count.
    pub(crate) fn release(&self) -> u32 {
        let count = self.0.get() - 1;
        self.0.set(count);
        count
    }
}
