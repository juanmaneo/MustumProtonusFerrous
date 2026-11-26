//! COM class routing and the `IPropertyBag` checks from `dlls/shdocvw/shdocvw_main.c` and
//! `dlls/shdocvw/shlinstobj.c`.

use crate::fixed::{E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, S_FALSE, S_OK};

/// A `GUID`, laid out as in `include/guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `CLSID_WebBrowser` from `include/exdisp.idl`.
pub const CLSID_WEB_BROWSER: Guid = Guid {
    data1: 0x8856_f961,
    data2: 0x340a,
    data3: 0x11d0,
    data4: [0xa9, 0x6b, 0x00, 0xc0, 0x4f, 0xd7, 0x05, 0xa2],
};

/// `CLSID_WebBrowser_V1` from `include/exdisp.idl`.
pub const CLSID_WEB_BROWSER_V1: Guid = Guid {
    data1: 0xeab2_2ac3,
    data2: 0x30c1,
    data3: 0x11cf,
    data4: [0xa7, 0xeb, 0x00, 0x00, 0xc0, 0x5b, 0xae, 0x0b],
};

/// `CLSID_InternetShortcut` from `include/isguids.h`.
pub const CLSID_INTERNET_SHORTCUT: Guid = Guid {
    data1: 0xfbf2_3b40,
    data2: 0xe3f0,
    data3: 0x101b,
    data4: [0x84, 0x88, 0x00, 0xaa, 0x00, 0x3e, 0x56, 0xf8],
};

/// `CLSID_CUrlHistory` from `include/shlguid.h`.
pub const CLSID_CURL_HISTORY: Guid = Guid {
    data1: 0x3c37_4a40,
    data2: 0xbae4,
    data3: 0x11cf,
    data4: [0xbf, 0x7d, 0x00, 0xaa, 0x00, 0x69, 0x46, 0xee],
};

/// `CLSID_TaskbarList` from `include/shobjidl.idl`.
pub const CLSID_TASKBAR_LIST: Guid = Guid {
    data1: 0x56fd_f344,
    data2: 0xfd6d,
    data3: 0x11d0,
    data4: [0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90],
};

/// `IID_IUnknown` from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IPropertyBag` from `include/oaidl.idl`.
pub const IID_IPROPERTY_BAG: Guid = Guid {
    data1: 0x5527_2a00,
    data2: 0x42cb,
    data3: 0x11ce,
    data4: [0x81, 0x35, 0x00, 0xaa, 0x00, 0x4b, 0xb8, 0x51],
};

/// Where `DllGetClassObject` sends a class: to ieframe, or to the shell instance object lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassRoute {
    IeFrame,
    ShellInstance,
}

/// The routing in `DllGetClassObject`: the five browser classes go to ieframe, and every other
/// class is looked up as a shell instance object.
pub fn dll_get_class_object_route(clsid: &Guid) -> ClassRoute {
    let ieframe_classes = [
        CLSID_WEB_BROWSER,
        CLSID_WEB_BROWSER_V1,
        CLSID_INTERNET_SHORTCUT,
        CLSID_CURL_HISTORY,
        CLSID_TASKBAR_LIST,
    ];
    if ieframe_classes.contains(clsid) {
        ClassRoute::IeFrame
    } else {
        ClassRoute::ShellInstance
    }
}

/// `DllCanUnloadNow`: returns `S_FALSE` while the reference count is non-zero, and `S_OK` once it is zero.
pub fn dll_can_unload_now(ref_count: i32) -> u32 {
    if ref_count != 0 {
        S_FALSE
    } else {
        S_OK
    }
}

/// `DllInstall`: returns `S_OK`. The `bInstall` and `cmdline` arguments are never read.
pub fn dll_install() -> u32 {
    S_OK
}

/// `RegistryPropertyBag::QueryInterface`: fails with `E_INVALIDARG` when there is no out-pointer,
/// returns `S_OK` for `IUnknown` and `IPropertyBag`, and `E_NOINTERFACE` otherwise.
pub fn property_bag_query_interface(riid: &Guid, has_ppv: bool) -> u32 {
    if !has_ppv {
        return E_INVALIDARG;
    }
    if *riid == IID_IUNKNOWN || *riid == IID_IPROPERTY_BAG {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// `RegistryPropertyBag::Write`: returns `E_NOTIMPL`.
pub fn property_bag_write() -> u32 {
    E_NOTIMPL
}
