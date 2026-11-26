//! Port of the portable logic in Wine's `dlls/wevtapi/main.c` and `dlls/wevtapi/wevtapi.spec`.
//!
//! Ported:
//! - `Evt*` exports whose bodies return a fixed value, with the `ERROR_CALL_NOT_IMPLEMENTED`
//!   last error where the C code sets it: [`evt_open_session`], [`evt_open_log`], [`evt_subscribe`],
//!   [`evt_open_channel_enum`], [`evt_open_channel_config`], [`evt_query`], [`evt_create_bookmark`],
//!   [`evt_create_render_context`], [`evt_next_channel_path`], [`evt_set_channel_config_property`],
//!   [`evt_save_channel_config`], [`evt_close`], [`evt_next`], [`evt_export_log`], [`evt_render`]
//! - `EvtGetChannelConfigProperty` for `EvtChannelLoggingConfigLogFilePath`, the buffer-size gate
//!   and the `EVT_VARIANT` string: [`get_channel_config_property`], [`PropertyVariant`]
//! - the `wevtapi.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - the `@ stub` exports: [`wine_spec_unimplemented_stub`], [`StubException`] and one helper per
//!   stub export, such as [`evt_cancel`]
//!
//! Skipped:
//! - the `FIXME` traces, and the arguments the C bodies never read (sessions, paths, queries,
//!   callbacks, and so on)
//! - `PropertyValueBufferUsed == NULL`, which the C code dereferences. The port takes `&mut u32`,
//!   so a NULL pointer cannot be passed.
//! - the `WINAPI` calling convention, real handle values, and the byte layout of `EVT_VARIANT` in
//!   caller memory. The variant is returned as data, and `EvtHandle` is a plain `usize`.
//! - `BOOL` is modelled as `bool`.

mod channel;
mod exports;
mod handles;
mod status;
mod stub;
mod types;

pub use channel::{
    get_channel_config_property, PropertyVariant, EVT_VARIANT_SIZE, EVT_VAR_TYPE_STRING,
    LOG_FILE_PATH_PROPERTY,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use handles::{
    evt_create_bookmark, evt_create_render_context, evt_open_channel_config, evt_open_channel_enum,
    evt_open_log, evt_open_session, evt_query, evt_subscribe, CHANNEL_CONFIG_HANDLE,
};
pub use status::{
    evt_close, evt_export_log, evt_next, evt_next_channel_path, evt_render,
    evt_save_channel_config, evt_set_channel_config_property,
};
pub use stub::{
    evt_archive_exported_log, evt_cancel, evt_clear_log, evt_format_message, evt_get_event_info,
    evt_get_event_metadata_property, evt_get_extended_status, evt_get_log_info,
    evt_get_object_array_property, evt_get_object_array_size, evt_get_publisher_metadata_property,
    evt_get_query_info, evt_int_assert_config, evt_int_create_bin_xml_from_custom_xml,
    evt_int_create_local_logfile, evt_int_get_classic_log_display_name,
    evt_int_render_resource_event_template, evt_int_report_authz_event_and_source_async,
    evt_int_report_event_and_source_async, evt_int_retract_config, evt_int_sysprep_cleanup,
    evt_int_write_xml_event_to_local_logfile, evt_next_event_metadata, evt_next_publisher_id,
    evt_open_event_metadata_enum, evt_open_publisher_enum, evt_open_publisher_metadata, evt_seek,
    evt_set_object_array_property, evt_update_bookmark, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB, WEVTAPI_DLL_NAME,
};
pub use types::{
    EvtHandle, Outcome, ERROR_CALL_NOT_IMPLEMENTED, ERROR_INSUFFICIENT_BUFFER, NULL_HANDLE,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
