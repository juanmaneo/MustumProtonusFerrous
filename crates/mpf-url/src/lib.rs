//! Port of the portable logic in Wine's `dlls/url/url_main.c` and `dlls/url/url.spec`.
//!
//! Ported:
//! - `HRESULT` values from `include/winerror.h`: [`HResult`], [`S_OK`], [`E_NOTIMPL`]
//! - the explicit C functions that return a fixed value or have no effect:
//!   [`add_mime_file_types_ps`], [`inet_is_offline`], [`open_url_a`], [`telnet_protocol_handler_a`]
//! - the target choice in `FileProtocolHandlerA`: [`file_protocol_handler_a_target`]
//! - `url.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports, modelled as the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`] and one helper per stub export
//!
//! Skipped:
//! - The `FIXME` and `TRACE` output and `debugstr_a`.
//! - The parameters of the explicit functions, which their C bodies never read.
//! - `PathCreateFromUrlA` and `ShellExecuteA`. They are OS calls. The port takes the
//!   `PathCreateFromUrlA` result and the path it produced as inputs, and returns the string
//!   that `ShellExecuteA` would receive.
//! - `__cdecl`, `WINAPI`, and the window and instance handles.

mod exports;
mod handlers;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use handlers::{
    add_mime_file_types_ps, file_protocol_handler_a_target, inet_is_offline, open_url_a,
    telnet_protocol_handler_a, HResult, E_NOTIMPL, S_OK,
};
pub use stub::{
    autodial_hook_callback, dummy_entry_point, dummy_entry_point_a, mail_to_protocol_handler,
    mail_to_protocol_handler_a, mime_association_dialog_a, mime_association_dialog_w,
    news_protocol_handler, news_protocol_handler_a, translate_url_a, translate_url_w,
    url_association_dialog_a, url_association_dialog_w, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
