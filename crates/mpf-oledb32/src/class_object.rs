//! `DllGetClassObject` and the class factory in `dlls/oledb32/main.c`.

/// An `HRESULT`. The C values are signed 32-bit; the port keeps the bit pattern.
pub type Hresult = u32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_NOINTERFACE: Hresult = 0x8000_4002;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111;

/// A `GUID` laid out as in `include/guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const CLSID_EXTENDEDERRORINFO: Guid = Guid {
    data1: 0xc8b5_22cf,
    data2: 0x5cf3,
    data3: 0x11ce,
    data4: [0xad, 0xe5, 0x00, 0xaa, 0x00, 0x44, 0x77, 0x3d],
};
pub const CLSID_OLEDB_CONVERSIONLIBRARY: Guid = Guid {
    data1: 0xc8b5_22d1,
    data2: 0x5cf3,
    data3: 0x11ce,
    data4: [0xad, 0xe5, 0x00, 0xaa, 0x00, 0x44, 0x77, 0x3d],
};
pub const CLSID_OLEDB_ROWPOSITIONLIBRARY: Guid = Guid {
    data1: 0x2048_eee6,
    data2: 0x7fa2,
    data3: 0x11d0,
    data4: [0x9e, 0x6a, 0x00, 0xa0, 0xc9, 0x13, 0x8c, 0x29],
};
pub const CLSID_MSDAINITIALIZE: Guid = Guid {
    data1: 0x2206_cdb0,
    data2: 0x19c1,
    data3: 0x11d1,
    data4: [0x89, 0xe0, 0x00, 0xc0, 0x4f, 0xd7, 0xa8, 0x29],
};
pub const CLSID_DATALINKS: Guid = Guid {
    data1: 0x2206_cdb2,
    data2: 0x19c1,
    data3: 0x11d1,
    data4: [0x89, 0xe0, 0x00, 0xc0, 0x4f, 0xd7, 0xa8, 0x29],
};
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// The factory `DllGetClassObject` selects. Each one is a `cf` in `main.c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FactoryKind {
    Convert,
    DataInit,
    ErrorInfo,
    RowPos,
    DataLinks,
}

/// What `*obj` holds after `DllGetClassObject` returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjOut {
    /// `*obj = NULL`: the CLSID is not one of the five.
    Null,
    /// `*obj` is the factory's `IClassFactory` pointer, written by `CF_QueryInterface`.
    Factory(FactoryKind),
    /// `*obj` is not written: the factory refused the IID.
    Untouched,
}

/// The factory `DllGetClassObject` selects for `clsid`, or `None` for an unknown CLSID.
pub fn class_for_clsid(clsid: &Guid) -> Option<FactoryKind> {
    match *clsid {
        CLSID_OLEDB_CONVERSIONLIBRARY => Some(FactoryKind::Convert),
        CLSID_MSDAINITIALIZE => Some(FactoryKind::DataInit),
        CLSID_EXTENDEDERRORINFO => Some(FactoryKind::ErrorInfo),
        CLSID_OLEDB_ROWPOSITIONLIBRARY => Some(FactoryKind::RowPos),
        CLSID_DATALINKS => Some(FactoryKind::DataLinks),
        _ => None,
    }
}

/// `CF_QueryInterface`: `S_OK` for `IUnknown` and `IClassFactory`, `E_NOINTERFACE` otherwise.
pub fn cf_query_interface(riid: &Guid) -> Hresult {
    if *riid == IID_IUNKNOWN || *riid == IID_ICLASSFACTORY {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// `CF_AddRef`: always 2.
pub fn cf_add_ref() -> u32 {
    2
}

/// `CF_Release`: always 1.
pub fn cf_release() -> u32 {
    1
}

/// `CF_LockServer`: always `S_OK`. The lock flag is ignored.
pub fn cf_lock_server(_dolock: bool) -> Hresult {
    S_OK
}

/// `DllGetClassObject`: the HRESULT and what `*obj` holds afterwards. An unknown CLSID sets
/// `*obj` to NULL. A known CLSID leaves `*obj` alone unless `CF_QueryInterface` accepts the IID.
pub fn dll_get_class_object(clsid: &Guid, riid: &Guid) -> (Hresult, ObjOut) {
    let Some(kind) = class_for_clsid(clsid) else {
        return (CLASS_E_CLASSNOTAVAILABLE, ObjOut::Null);
    };
    let hr = cf_query_interface(riid);
    if hr == S_OK {
        (hr, ObjOut::Factory(kind))
    } else {
        (hr, ObjOut::Untouched)
    }
}
