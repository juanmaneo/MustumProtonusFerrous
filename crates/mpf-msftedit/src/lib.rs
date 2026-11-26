//! Port of the portable logic in Wine's `dlls/msftedit/msftedit_main.c` and `dlls/msftedit/msftedit.spec`,
//! plus the `DllGetVersion` default in `libs/winecrt0/dll_version.c` that the spec's `-private` export uses.
//!
//! Ported:
//! - `DllMain`: the `DLL_PROCESS_ATTACH` and `DLL_PROCESS_DETACH` handling of the `riched20.dll`
//!   handle, always returning `TRUE`: [`DllMainState`], [`DllHost`]
//! - `msftedit.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports (`SetCustomTextOutHandlerEx`, `RichEditWndProc`, `RichListBoxWndProc`,
//!   `RichComboBoxWndProc`) and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export
//! - `DllGetVersion` default: [`dll_get_version`], [`DllVersionInfo`]
//!
//! Skipped:
//! - The forwarded exports (`riched20.*`, including the `IID_*` data) and the `extern IID_IRichEditOle`
//!   and `IID_IRichEditOleCallback` GUID data: this DLL has no code for them, so the table records them only.
//! - The export ordinals (the numbers before each spec line), which winebuild assigns.
//! - The `LoadLibraryW`, `DisableThreadLibraryCalls`, `FreeLibrary`, `FindResourceW` and `LoadResource`
//!   calls: the OS side of the code. They enter through [`DllHost`] or as the resource bytes.
//! - The `WINAPI` and `__cdecl` calling conventions, and the `reserved` parameter, which is never read.
//!
//! Deviation: the C `DllGetVersion` default reads the resource without a length check, which is undefined
//! behaviour on a truncated resource. The port returns `E_INVALIDARG` there instead.

mod dll_main;
mod dll_version;
mod exports;
mod stub;

pub use dll_main::{
    DllHost, DllMainState, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH,
    DLL_THREAD_DETACH, RICHEDIT_DLL_NAME,
};
pub use dll_version::{
    dll_get_version, DllVersionInfo, DLLVERSIONINFO2_SIZE, DLLVERSIONINFO_SIZE, DLLVER_PLATFORM_NT,
    E_INVALIDARG, S_OK,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    rich_combo_box_wnd_proc, rich_edit_wnd_proc, rich_list_box_wnd_proc,
    set_custom_text_out_handler_ex, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
