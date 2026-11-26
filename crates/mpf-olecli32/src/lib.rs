//! Port of the portable logic in Wine's `dlls/olecli32/olecli_main.c` and `dlls/olecli32/olecli32.spec`.
//!
//! Ported:
//! - `OleRegisterClientDoc` handle counter: [`ole_register_client_doc`], [`ClientDocRegistry`],
//!   [`LhClientDoc`]
//! - `OleQueryType` NULL-object status and object dispatch: [`ole_query_type`], [`OleObject`]
//! - `OLESTATUS` values: [`OleStatus`], [`OLE_OK`], [`OLE_NULL_OBJECT_STATUS`]
//! - `OleIsDcMeta` comparison with `OBJ_METADC`: [`ole_is_dc_meta`], [`OBJ_METADC`]
//! - Explicit C functions that return `OLE_OK`: [`ole_rename_client_doc`],
//!   [`ole_revoke_client_doc`], [`ole_saved_client_doc`], [`ole_create_link_from_clip`],
//!   [`ole_query_link_from_clip`], [`ole_query_create_from_clip`], [`ole_create_from_clip`],
//!   [`ole_set_host_names`]
//! - `@ stub` exports (41): [`wine_spec_unimplemented_stub`], [`StubException`], the
//!   `EXCEPTION_*` values, and one helper per export named by the snake_case of the C name
//!   (`wep`, `ole_delete`, ..., `ole_query_client_version`)
//!
//! Skipped:
//! - The `FIXME` and `TRACE` output, and the `OLEOBJECTVTBL` slots other than `QueryType`.
//! - Arguments the C bodies never read: all arguments of the `OLE_OK` functions, and the
//!   `classname`, `docname` and `reserved` of `OleRegisterClientDoc`.
//! - NULL pointers: the port takes references, so a NULL `hRet` or `xlong` has no equivalent.
//!   The C code writes through `hRet` unconditionally.
//! - The calling conventions (`WINAPI`, `CALLBACK`, `__cdecl`), and the OS calls `GetObjectType`
//!   (its result is the `object_type` argument) and `RtlRaiseException`.
//! - The `OLE_current_handle` static, which is the explicit [`ClientDocRegistry`] state.
//! - Forwarded exports `OleSaveToStream`, `OleLoadFromStream`, `OleCreate` and
//!   `OleCreateFromFile`. They forward to ole32 and have no code in this DLL.

mod client_doc;
mod clip;
mod dc;
mod object;
mod stub;

pub use client_doc::{
    ole_register_client_doc, ole_rename_client_doc, ole_revoke_client_doc, ole_saved_client_doc,
    ClientDocRegistry, LhClientDoc,
};
pub use clip::{
    ole_create_from_clip, ole_create_link_from_clip, ole_query_create_from_clip,
    ole_query_link_from_clip,
};
pub use dc::{ole_is_dc_meta, OBJ_METADC};
pub use object::{
    ole_query_type, ole_set_host_names, OleObject, OleStatus, OLE_NULL_OBJECT_STATUS, OLE_OK,
};
pub use stub::{
    ole_activate, ole_clone, ole_close, ole_copy_from_link, ole_copy_to_clipboard,
    ole_create_from_template, ole_create_invisible, ole_create_link_from_file, ole_delete,
    ole_draw, ole_enum_formats, ole_enum_objects, ole_equal, ole_execute, ole_get_data,
    ole_get_link_update_options, ole_lock_server, ole_object_convert, ole_query_bounds,
    ole_query_client_version, ole_query_name, ole_query_open, ole_query_out_of_date,
    ole_query_protocol, ole_query_release_error, ole_query_release_method,
    ole_query_release_status, ole_query_size, ole_reconnect, ole_release, ole_rename,
    ole_request_data, ole_revert_client_doc, ole_set_bounds, ole_set_color_scheme, ole_set_data,
    ole_set_link_update_options, ole_set_target_device, ole_unlock_server, ole_update, wep,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
