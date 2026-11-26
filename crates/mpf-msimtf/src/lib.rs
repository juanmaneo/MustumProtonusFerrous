//! Port of the portable logic in Wine's `dlls/msimtf/activeimmapp.c`, `dlls/msimtf/main.c` and
//! `dlls/msimtf/msimtf.spec`, and of the `DllCanUnloadNow` default in `libs/winecrt0/dll_canunload.c`
//! that the spec exports.
//!
//! Ported:
//! - The `ActiveIMMApp` reference count, `QueryInterface` and `ActiveIMMApp_Constructor`:
//!   [`ActiveImmApp`], [`ActiveImmInterface`], [`active_imm_app_constructor`],
//!   [`active_imm_app_query_interface`], [`active_imm_app_add_ref`], [`active_imm_app_release`]
//! - The `ActiveIMMApp` explicit functions that return a fixed value: [`active_imm_app_enum_register_word_a`],
//!   [`active_imm_app_enum_register_word_w`], [`active_imm_app_activate`], [`active_imm_app_deactivate`],
//!   [`active_imm_app_on_def_window_proc`], [`active_imm_app_filter_client_windows`],
//!   [`active_imm_app_get_code_page_a`], [`active_imm_app_get_lang_id`], [`active_imm_app_enum_input_context`]
//! - The five explicit `ActiveIMMMessagePumpOwner` methods, which return `E_NOTIMPL`:
//!   [`active_imm_message_pump_owner_start`], [`active_imm_message_pump_owner_end`],
//!   [`active_imm_message_pump_owner_on_translate_message`], [`active_imm_message_pump_owner_pause`],
//!   [`active_imm_message_pump_owner_resume`]
//! - The `ClassFactory` object of `main.c`: [`ClassFactory`], [`class_factory_query_interface`],
//!   [`class_factory_add_ref`], [`class_factory_release`], [`class_factory_lock_server`],
//!   [`class_factory_create_instance`]
//! - `DllGetClassObject` and `DllCanUnloadNow`: [`dll_get_class_object`], [`dll_can_unload_now`]
//! - The `msimtf.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The `@ stub` exports `MsimtfIsGuidMapEnable` and `MsimtfIsWindowFiltered`, and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`msimtf_is_guid_map_enable`],
//!   [`msimtf_is_window_filtered`], [`wine_spec_unimplemented_stub`], [`StubException`]
//! - The GUIDs and `HRESULT` values these use: [`Guid`], [`is_equal_guid`], [`IID_UNKNOWN`],
//!   [`IID_CLASS_FACTORY`], [`IID_ACTIVE_IMM_APP`], [`IID_ACTIVE_IMM_MESSAGE_PUMP_OWNER`],
//!   [`CLSID_CACTIVE_IMM`], [`Hresult`], [`failed`]
//!
//! Skipped:
//! - `TRACE`, `FIXME` and `WARN` logging, and `WINE_DEFAULT_DEBUG_CHANNEL`.
//! - The `ActiveIMMApp` methods that forward to `imm32` (59 `Imm*` call sites in
//!   `activeimmapp.c`, such as `ImmEscapeA` and `ImmGetCandidateWindow`). Their results come from the
//!   OS, and their `BOOL`-to-`HRESULT` mapping is not ported.
//! - `ActiveIMMApp_Destructor` and its `HeapFree`: the port reports the release to zero through the
//!   return value, and the owner drops the value.
//! - The `HeapAlloc` call, which is the `alloc_succeeds` argument of the constructor.
//! - The COM vtables (`ActiveIMMAppVtbl`, `ActiveIMMMessagePumpOwnerVtbl` and `ClassFactoryVtbl`),
//!   the `CONTAINING_RECORD` pointer casts, and the `ClassFactory` `cf` member. The port has no
//!   pointers, and the factory always calls the constructor.
//! - The `ActiveIMMMessagePumpOwner` `QueryInterface`, `AddRef` and `Release`, which forward to the
//!   `ActiveIMMApp` object.
//! - `DllRegisterServer` and `DllUnregisterServer`. The default in `libs/winecrt0` forwards them to
//!   `__wine_register_resources` and `__wine_unregister_resources`, which register resources with the OS.
//! - The arguments of the explicit functions and of `ClassFactory_LockServer`, which the C bodies never read.

mod active_imm_app;
mod class_factory;
mod dll;
mod exports;
mod guid;
mod hresult;
mod message_pump_owner;
mod stub;

pub use active_imm_app::{
    active_imm_app_activate, active_imm_app_add_ref, active_imm_app_constructor,
    active_imm_app_deactivate, active_imm_app_enum_input_context,
    active_imm_app_enum_register_word_a, active_imm_app_enum_register_word_w,
    active_imm_app_filter_client_windows, active_imm_app_get_code_page_a,
    active_imm_app_get_lang_id, active_imm_app_on_def_window_proc, active_imm_app_query_interface,
    active_imm_app_release, ActiveImmApp, ActiveImmInterface,
};
pub use class_factory::{
    class_factory_add_ref, class_factory_create_instance, class_factory_lock_server,
    class_factory_query_interface, class_factory_release, ClassFactory,
};
pub use dll::{dll_can_unload_now, dll_get_class_object};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use guid::{
    is_equal_guid, Guid, CLSID_CACTIVE_IMM, IID_ACTIVE_IMM_APP, IID_ACTIVE_IMM_MESSAGE_PUMP_OWNER,
    IID_CLASS_FACTORY, IID_UNKNOWN,
};
pub use hresult::{
    failed, Hresult, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_FAIL, E_NOINTERFACE,
    E_NOTIMPL, E_OUTOFMEMORY, S_FALSE, S_OK,
};
pub use message_pump_owner::{
    active_imm_message_pump_owner_end, active_imm_message_pump_owner_on_translate_message,
    active_imm_message_pump_owner_pause, active_imm_message_pump_owner_resume,
    active_imm_message_pump_owner_start,
};
pub use stub::{
    msimtf_is_guid_map_enable, msimtf_is_window_filtered, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
