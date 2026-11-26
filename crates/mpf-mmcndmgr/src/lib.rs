//! Port of the portable logic in Wine's `dlls/mmcndmgr/mmcndmgr.c` and `dlls/mmcndmgr/mmcndmgr.spec`.
//!
//! Ported:
//! - The `IMMCVersionInfo` object (`mmcversion_*`): [`mmcversion_query_interface`],
//!   [`mmcversion_add_ref`], [`mmcversion_release`], [`mmcversion_get_mmc_version`]
//! - The class factory (`ClassFactory_*`): [`class_factory_query_interface`],
//!   [`class_factory_add_ref`], [`class_factory_release`], [`class_factory_create_instance`],
//!   [`class_factory_lock_server`]
//! - `DllGetClassObject` class-id dispatch: [`dll_get_class_object`]
//! - The `@ stub` exports `CreateExecutivePlatform` and `GetAllocCounters` (both arch spellings),
//!   and the `__wine_spec_unimplemented_stub` record they raise: [`create_executive_platform`],
//!   [`get_alloc_counters_win32`], [`get_alloc_counters_win64`], [`wine_spec_unimplemented_stub`],
//!   [`StubException`]
//! - The GUIDs, `HRESULT` values and out-pointer targets the dispatch uses: [`Guid`], [`Hresult`],
//!   [`Object`], [`is_equal_guid`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: private in the spec, with no
//!   body in `dlls/mmcndmgr`. Their default bodies come from `libs/winecrt0`, outside this port.
//! - The `this` pointer of each method, and the `outer` argument of `CreateInstance` and the
//!   `fLock` argument of `LockServer`, which the C bodies never read.
//! - Reference counting: the C returns fixed counts (2 and 1) from static objects, so nothing is
//!   tracked. The `TRACE`, `WARN` and `FIXME` output and `debugstr_guid` are also not ported.

mod class_factory;
mod com;
mod stub;
mod version_info;

pub use class_factory::{
    class_factory_add_ref, class_factory_create_instance, class_factory_lock_server,
    class_factory_query_interface, class_factory_release, dll_get_class_object,
};
pub use com::{
    is_equal_guid, Guid, Hresult, Object, CLASS_E_CLASSNOTAVAILABLE, CLSID_MMCVERSIONINFO,
    E_NOINTERFACE, IID_ICLASSFACTORY, IID_IMMCVERSIONINFO, IID_IUNKNOWN, S_OK,
};
pub use stub::{
    create_executive_platform, get_alloc_counters_win32, get_alloc_counters_win64,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use version_info::{
    mmcversion_add_ref, mmcversion_get_mmc_version, mmcversion_query_interface, mmcversion_release,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
