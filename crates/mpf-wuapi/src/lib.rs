//! Headless subset of Wine's `dlls/wuapi` (`main.c` and the `QueryInterface`
//! methods of the COM objects).
//!
//! The COM classes of wuapi are almost all `E_NOTIMPL` stubs or fixed answers.
//! What is deterministic is the dispatch: which class a CLSID names, which
//! IIDs each factory and object accepts, and the checks of
//! `IClassFactory::CreateInstance`. The COM runtime is not modelled: vtables,
//! reference counts, `HeapAlloc` and the `ppv` stores are skipped.
//!
//! Ported:
//! - `DllGetClassObject`: [`dll_get_class_object`] and its CLSID table, [`class_for_clsid`].
//! - `wucf_QueryInterface`: [`class_factory_query_interface`].
//! - `wucf_CreateInstance`: [`create_instance`], the aggregation check followed by the object's `QueryInterface`.
//! - The `QueryInterface` method of each object: [`object_query_interface`].
//!
//! Skipped:
//! - `wucf_AddRef`, `wucf_Release`, `wucf_LockServer` and the objects' `AddRef` and `Release`: reference counts and the server lock are COM runtime state.
//! - The `E_OUTOFMEMORY` returns of the `*_create` functions: they come from `HeapAlloc`.
//! - Every other method: `E_NOTIMPL` stubs and fixed answers such as `VARIANT_FALSE` and `S_OK` with no state behind them.

/// Success HRESULT.
pub const S_OK: i32 = 0;
/// `E_NOINTERFACE`, returned when a factory or object does not implement the IID.
pub const E_NOINTERFACE: i32 = 0x8000_4002u32 as i32;
/// `CLASS_E_NOAGGREGATION`, returned when `CreateInstance` is given an outer unknown.
pub const CLASS_E_NOAGGREGATION: i32 = 0x8004_0110u32 as i32;
/// `CLASS_E_CLASSNOTAVAILABLE`, returned for a CLSID that this DLL does not register.
pub const CLASS_E_CLASSNOTAVAILABLE: i32 = 0x8004_0111u32 as i32;

/// A `GUID`, with the same four fields in the same layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

const fn guid(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Guid {
    Guid {
        data1,
        data2,
        data3,
        data4,
    }
}

/// The classes that `DllGetClassObject` registers, in the order of `main.c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComClass {
    UpdateSession,
    AutomaticUpdates,
    SystemInformation,
    UpdateInstaller,
}

/// The objects whose `QueryInterface` is ported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComObject {
    UpdateSession,
    UpdateSearcher,
    UpdateDownloader,
    UpdateInstaller,
    AutomaticUpdates,
    SystemInformation,
}

impl ComClass {
    /// The object that the class factory creates.
    pub fn object(self) -> ComObject {
        match self {
            ComClass::UpdateSession => ComObject::UpdateSession,
            ComClass::AutomaticUpdates => ComObject::AutomaticUpdates,
            ComClass::SystemInformation => ComObject::SystemInformation,
            ComClass::UpdateInstaller => ComObject::UpdateInstaller,
        }
    }
}

pub const CLSID_UPDATE_SESSION: Guid = guid(
    0x4cb43d7f,
    0x7eee,
    0x4906,
    [0x86, 0x98, 0x60, 0xda, 0x1c, 0x38, 0xf2, 0xfe],
);
pub const CLSID_AUTOMATIC_UPDATES: Guid = guid(
    0xbfe18e9c,
    0x6d87,
    0x4450,
    [0xb3, 0x7c, 0xe0, 0x2f, 0x0b, 0x37, 0x38, 0x03],
);
pub const CLSID_SYSTEM_INFORMATION: Guid = guid(
    0xc01b9ba0,
    0xbea7,
    0x41ba,
    [0xb6, 0x04, 0xd0, 0xa3, 0x6f, 0x46, 0x91, 0x33],
);
pub const CLSID_UPDATE_INSTALLER: Guid = guid(
    0xd2e0fe7f,
    0xd23e,
    0x48e1,
    [0x93, 0xc0, 0x6f, 0xa8, 0xcc, 0x34, 0x64, 0x74],
);
pub const IID_IUPDATE_SESSION: Guid = guid(
    0x816858a4,
    0x260d,
    0x4260,
    [0x93, 0x3a, 0x25, 0x85, 0xf1, 0xab, 0xc7, 0x6b],
);
pub const IID_IUPDATE_SEARCHER: Guid = guid(
    0x8f45abf1,
    0xf9ae,
    0x4b95,
    [0xa9, 0x33, 0xf0, 0xf6, 0x6e, 0x50, 0x56, 0xea],
);
pub const IID_IUPDATE_DOWNLOADER: Guid = guid(
    0x68f1c6f9,
    0x7ecc,
    0x4666,
    [0xa4, 0x64, 0x24, 0x7f, 0xe1, 0x24, 0x96, 0xc3],
);
pub const IID_IUPDATE_INSTALLER: Guid = guid(
    0x7b929c68,
    0xccdc,
    0x4226,
    [0x96, 0xb1, 0x87, 0x24, 0x60, 0x0b, 0x54, 0xc2],
);
pub const IID_IAUTOMATIC_UPDATES: Guid = guid(
    0x673425bf,
    0xc082,
    0x4c7c,
    [0xbd, 0xfd, 0x56, 0x94, 0x64, 0xb8, 0xe0, 0xce],
);
pub const IID_ISYSTEM_INFORMATION: Guid = guid(
    0xade87bf7,
    0x7b56,
    0x4275,
    [0x8f, 0xab, 0xb9, 0xb0, 0xe5, 0x91, 0x84, 0x4b],
);
pub const IID_IUNKNOWN: Guid = guid(
    0x00000000,
    0x0000,
    0x0000,
    [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
);
pub const IID_ICLASS_FACTORY: Guid = guid(
    0x00000001,
    0x0000,
    0x0000,
    [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
);
pub const IID_IDISPATCH: Guid = guid(
    0x00020400,
    0x0000,
    0x0000,
    [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
);

/// The class that a CLSID names in `DllGetClassObject`, or `None` when the DLL does not register it.
pub fn class_for_clsid(clsid: &Guid) -> Option<ComClass> {
    if *clsid == CLSID_UPDATE_SESSION {
        Some(ComClass::UpdateSession)
    } else if *clsid == CLSID_AUTOMATIC_UPDATES {
        Some(ComClass::AutomaticUpdates)
    } else if *clsid == CLSID_SYSTEM_INFORMATION {
        Some(ComClass::SystemInformation)
    } else if *clsid == CLSID_UPDATE_INSTALLER {
        Some(ComClass::UpdateInstaller)
    } else {
        None
    }
}

/// The `QueryInterface` of a class factory: `IUnknown` and `IClassFactory` only.
pub fn class_factory_query_interface(iid: &Guid) -> i32 {
    if *iid == IID_IUNKNOWN || *iid == IID_ICLASS_FACTORY {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// `DllGetClassObject`: the factory's `QueryInterface` for a registered CLSID.
pub fn dll_get_class_object(clsid: &Guid, iid: &Guid) -> i32 {
    match class_for_clsid(clsid) {
        Some(_) => class_factory_query_interface(iid),
        None => CLASS_E_CLASSNOTAVAILABLE,
    }
}

/// `wucf_CreateInstance`: refuses an outer unknown, otherwise creates the object and queries it for `iid`.
pub fn create_instance(class: ComClass, has_outer: bool, iid: &Guid) -> i32 {
    if has_outer {
        CLASS_E_NOAGGREGATION
    } else {
        object_query_interface(class.object(), iid)
    }
}

/// The `QueryInterface` of an object: its own interface, `IDispatch` and `IUnknown`.
pub fn object_query_interface(object: ComObject, iid: &Guid) -> i32 {
    let own = match object {
        ComObject::UpdateSession => IID_IUPDATE_SESSION,
        ComObject::UpdateSearcher => IID_IUPDATE_SEARCHER,
        ComObject::UpdateDownloader => IID_IUPDATE_DOWNLOADER,
        ComObject::UpdateInstaller => IID_IUPDATE_INSTALLER,
        ComObject::AutomaticUpdates => IID_IAUTOMATIC_UPDATES,
        ComObject::SystemInformation => IID_ISYSTEM_INFORMATION,
    };
    if *iid == own || *iid == IID_IDISPATCH || *iid == IID_IUNKNOWN {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
