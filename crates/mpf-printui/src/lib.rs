//! Port of the portable logic in Wine's `dlls/printui/printui.c` and `dlls/printui/printui.spec`.
//!
//! Ported:
//! - The `rundll32` command-line parser `parse_rundll`: [`parse_rundll`] and [`RunDll`], with the
//!   option letters `abcfhjlmntrv` and flag letters `qwyzZ` from `printui.c`, sized by [`OPT_MAX`]
//!   and [`FLAG_MAX`]
//! - The `PrintUIEntryW` decision whether a parsed command is reported as not implemented:
//!   [`print_ui_entry_w`] and [`Dispatch`]
//! - The `printui.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per stub export, such as
//!   [`connect_to_printer_dlg`]
//!
//! Skipped:
//! - `CommandLineToArgvW` and `LocalFree`: the caller passes the split arguments to
//!   [`print_ui_entry_w`].
//! - The `hWnd`, `hInst` and `nCmdShow` parameters of `PrintUIEntryW`, which the C stores in its
//!   context but never reads.
//! - The `FIXME` and `TRACE` output. The `@` redirect is not followed, so its file is never read.
//! - The command handlers. Every command is reported as not implemented, and none is run.
//! - `DllCanUnloadNow`: `printui.spec` exports it with `-private`, but `printui.c` has no body for
//!   it, so it is listed in [`EXPORTS`] without a stub helper.
//! - The `ExceptionAddress` and `ExceptionRecord` fields of the raised record, which
//!   [`StubException`] does not carry.

mod entry;
mod exports;
mod rundll;
mod stub;

pub use entry::{print_ui_entry_w, Dispatch};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use rundll::{parse_rundll, RunDll, FLAG_MAX, OPT_MAX};
pub use stub::{
    b_folder_enum_printers, b_folder_get_printer, b_folder_refresh, b_printer_setup,
    connect_to_printer_dlg, connect_to_printer_property_page, construct_printer_friendly_name,
    dll_get_class_object, document_properties_wrap, pnp_interface, print_notify_tray_exit,
    print_notify_tray_init, printer_prop_page_provider, register_print_notify,
    show_error_message_hr, show_error_message_sc, unregister_print_notify, v_document_defaults,
    v_printer_prop_pages, v_queue_create, v_server_prop_pages, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
