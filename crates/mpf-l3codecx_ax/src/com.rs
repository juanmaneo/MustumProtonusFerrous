//! COM status codes, GUIDs, and the host services that `l3codecx.c` calls out to.

use crate::registration::RegFilter2;

/// `HRESULT`: a signed 32-bit status code. Negative values are failures.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_FAIL: Hresult = 0x8000_4005_u32 as Hresult;
pub const E_NOINTERFACE: Hresult = 0x8000_4002_u32 as Hresult;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111_u32 as Hresult;

/// `GUID`. Equality compares every field, as `IsEqualGUID` does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// The `FAILED` macro.
pub fn is_failed(hr: Hresult) -> bool {
    hr < 0
}

/// The calls `l3codecx.c` makes to COM, the registry and Wine's resource registration.
///
/// The port decides the order of the calls and what to do with each result. The host performs the
/// calls and holds the `IFilterMapper2` pointer that `co_create_instance` produces for
/// `CLSID_FilterMapper2`; `release_filter_mapper` releases it. `co_create_instance` always uses
/// `CLSCTX_INPROC_SERVER`, and the `out` pointer is host-side.
pub trait ComHost {
    /// `__wine_register_resources()`.
    fn register_resources(&mut self) -> Hresult;
    /// `__wine_unregister_resources()`.
    fn unregister_resources(&mut self) -> Hresult;
    /// `CoCreateInstance(clsid, outer, CLSCTX_INPROC_SERVER, iid, out)`. `outer` is true when the
    /// C caller passed a non-NULL outer `IUnknown`.
    fn co_create_instance(&mut self, clsid: &Guid, outer: bool, iid: &Guid) -> Hresult;
    /// `IFilterMapper2::RegisterFilter(clsid, name, NULL, NULL, NULL, desc)` on the held mapper.
    fn register_filter(&mut self, clsid: &Guid, name: &str, desc: &RegFilter2) -> Hresult;
    /// `IFilterMapper2::UnregisterFilter(NULL, NULL, clsid)` on the held mapper.
    fn unregister_filter(&mut self, clsid: &Guid) -> Hresult;
    /// `IFilterMapper2::Release` on the held mapper.
    fn release_filter_mapper(&mut self);
}
