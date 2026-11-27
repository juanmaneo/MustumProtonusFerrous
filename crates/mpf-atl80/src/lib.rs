//! Port of the portable logic in Wine's `dlls/atl80/atl80.c` and `dlls/atl80/atl80.spec`.
//!
//! Ported:
//! - `AtlComModuleRegisterServer` (ordinal 18): [`atl_com_module_register_server`], with the
//!   object map walk, the CLSID filter, the category-map pass and the optional type library
//!   pass (`register.rs`). Its types: [`ObjMapEntry`], [`CatMapEntry`], [`Guid`], [`Hresult`],
//!   [`S_OK`] and [`failed`].
//! - The eight `@ stub` exports of `atl80.spec`, which winebuild points at
//!   `__wine_spec_unimplemented_stub`: [`atl_set_error_info`], [`atl_dev_mode_w2a`],
//!   [`atl_win_module_register_wnd_class_info_w`], [`atl_win_module_register_wnd_class_info_a`],
//!   [`atl_un_register_type_lib`], [`atl_win_module_register_class_ex_w`],
//!   [`atl_win_module_register_class_ex_a`] and [`atl_win_module_term`], with
//!   [`wine_spec_unimplemented_stub`] and [`StubException`] (`stub.rs`).
//!
//! Skipped:
//! - `TRACE` output and `debugstr_guid`: diagnostics only.
//! - The bodies of `AtlRegisterClassCategoriesHelper` and `AtlRegisterTypeLib`. They are in
//!   `dlls/atl`, which atl80.dll compiles in through `PARENTSRC = ../atl`. The caller supplies
//!   them as `fn` pointers.
//! - The other exports of `atl80.spec` that are not stubs (`AtlAdvise` through `AtlGetVersion`,
//!   `AtlAx*`, `AtlIPersist*`, and so on). Their bodies are in `dlls/atl`, not in this directory.
//! - The NULL cases that the C code dereferences without checking: `pclsid`,
//!   `pfnUpdateRegistry` and `pfnGetCategoryMap` are non-null by type, and the object map
//!   is a slice in place of the `first`/`last` pointer pair.

mod register;
mod stub;

pub use register::{
    atl_com_module_register_server, failed, CatMapEntry, Guid, HInstance, Hresult, ObjMapEntry,
    S_OK,
};
pub use stub::{
    atl_dev_mode_w2a, atl_set_error_info, atl_un_register_type_lib,
    atl_win_module_register_class_ex_a, atl_win_module_register_class_ex_w,
    atl_win_module_register_wnd_class_info_a, atl_win_module_register_wnd_class_info_w,
    atl_win_module_term, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
