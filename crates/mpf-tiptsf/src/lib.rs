//! Port of the portable logic in Wine's `dlls/tiptsf/tiptsf_main.c` and `dlls/tiptsf/tiptsf.spec`.
//!
//! Ported:
//! - `ITextInputPanel` object state: [`TextInputPanel`], its `QueryInterface`, `AddRef` and
//!   `Release` reference counting, and [`panelinput_create_instance`]
//! - the 25 `ITextInputPanel` methods that return `E_NOTIMPL`: the methods on [`TextInputPanel`]
//! - the class factory: [`class_factory_query_interface`], [`class_factory_add_ref`],
//!   [`class_factory_release`] and [`class_factory_lock_server`]
//! - `DllGetClassObject`: [`dll_get_class_object`]
//! - the seven `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`StubException`], [`wine_spec_unimplemented_stub`] and one helper per export
//! - the `tiptsf.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//!
//! Skipped:
//! - The `FIXME`, `TRACE` and `WARN` output and the `debugstr_guid` formatting.
//! - The out-pointers of the `E_NOTIMPL` methods and of the `QueryInterface` calls: the C code
//!   either leaves them alone or stores the object pointer, which the port does not model.
//! - The `outer`, `riid` and `ppv` arguments of `panelinput_CreateInstance`: the C code only
//!   traces them. The class factory's `CreateInstance` slot is `panelinput_CreateInstance`.
//! - `free` in `input_panel_Release`: the caller drops the value when the count reaches zero.
//! - `DllRegisterServer`, `DllUnregisterServer` and `DllCanUnloadNow`, which `tiptsf.spec` exports
//!   as `@ stdcall -private`. `dlls/tiptsf` has no body for them: they come from widl's proxy
//!   code (`include/rpcproxy.h`), so there is no reference behaviour to port here.
//! - The `__cdecl` and `WINAPI` calling conventions.

mod class_factory;
mod com;
mod exports;
mod input_panel;
mod stub;

pub use class_factory::{
    class_factory_add_ref, class_factory_lock_server, class_factory_query_interface,
    class_factory_release, dll_get_class_object,
};
pub use com::{
    is_equal_guid, Guid, CLASS_E_CLASSNOTAVAILABLE, CLSID_TEXT_INPUT_PANEL, E_NOINTERFACE,
    E_NOTIMPL, E_OUTOFMEMORY, HRESULT, IID_CLASS_FACTORY, IID_TEXT_INPUT_PANEL, IID_UNKNOWN, S_OK,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use input_panel::{panelinput_create_instance, TextInputPanel};
pub use stub::{
    advise_hook, end_caret_tracking, process_caret_events, process_cicero_caret_event,
    restricted_mode_msg_wnd_proc, start_caret_tracking, unadvise_hook,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
