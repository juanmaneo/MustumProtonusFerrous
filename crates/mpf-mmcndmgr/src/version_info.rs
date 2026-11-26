//! The `IMMCVersionInfo` object from Wine's `dlls/mmcndmgr/mmcndmgr.c` (`mmcversion_*`).

use crate::{
    is_equal_guid, Guid, Hresult, Object, E_NOINTERFACE, IID_IMMCVERSIONINFO, IID_IUNKNOWN, S_OK,
};

const MMC_VERSION_MAJOR: i32 = 3;
const MMC_VERSION_MINOR: i32 = 0;

/// `mmcversion_QueryInterface`. Accepts `IID_IMMCVersionInfo` and `IID_IUnknown`. For any other
/// IID the C returns `E_NOINTERFACE` and sets `*ppv = NULL`, which is the `Err` case.
pub fn mmcversion_query_interface(riid: &Guid) -> Result<Object, Hresult> {
    if is_equal_guid(riid, &IID_IMMCVERSIONINFO) || is_equal_guid(riid, &IID_IUNKNOWN) {
        Ok(Object::MmcVersionInfo)
    } else {
        Err(E_NOINTERFACE)
    }
}

/// `mmcversion_AddRef`. The object is static, so the count is always 2.
pub fn mmcversion_add_ref() -> u32 {
    2
}

/// `mmcversion_Release`. The object is static, so the count is always 1.
pub fn mmcversion_release() -> u32 {
    1
}

/// `mmcversion_GetMMCVersion`. Each out-pointer is written only when it is not NULL, which is
/// modelled as `None`.
pub fn mmcversion_get_mmc_version(major: Option<&mut i32>, minor: Option<&mut i32>) -> Hresult {
    if let Some(major) = major {
        *major = MMC_VERSION_MAJOR;
    }
    if let Some(minor) = minor {
        *minor = MMC_VERSION_MINOR;
    }
    S_OK
}
