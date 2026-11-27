//! Port of the portable logic in Wine's `dlls/mapistub/mapistub.spec`.
//!
//! `dlls/mapistub/` has no C sources: its `Makefile.in` sets `MODULE = mapistub.dll` and the spec
//! defines every export. The port therefore covers the export table and the stub records, and its
//! verdict is partial.
//!
//! Ported:
//! - `mapistub.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` records: [`StubException`], [`StubFunction`] and [`wine_spec_unimplemented_stub`],
//!   plus one helper per stub export, such as [`pr_provider_init`] and [`ordinal_8`]
//!
//! Skipped:
//! - The 90 `stdcall` forwards (`module.function` targets). Winebuild emits no relay for a forward
//!   (`tools/winebuild/spec32.c`), so the table records each target and no code is ported.
//! - The `-ret64` flag on five of those forwards. On i386 it only changes the return-type letter
//!   in the relay argument string (`tools/winebuild/spec32.c`), and forwards have no relay.
//! - `stdcall -private DllCanUnloadNow()`. It has no forward target and no C body in the
//!   reference, so it is recorded as [`ExportKind::Unbound`] and no result is modelled.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    __cpp_validate_parameters_at_8, __validate_parameters_at_8, bmapi_address, bmapi_details,
    bmapi_find_next, bmapi_get_address, bmapi_get_read_mail, bmapi_read_mail, bmapi_resolve_name,
    bmapi_save_mail, bmapi_send_mail, build_display_table_at_40, cch_of_encoding_at_4,
    change_idle_routine_at_28, close_i_msg_session_at_4, cmc_act_on, cmc_free, cmc_list,
    cmc_logoff, cmc_logon, cmc_look_up, cmc_read, cmc_send, cmc_send_documents, create_table_at_36,
    deregister_idle_routine_at_4, dll_get_class_object, enable_idle_routine_at_8, encode_id_at_12,
    f_bad_restriction_at_4, f_bad_rglp_name_id_at_8, f_bad_sort_order_set_at_4, f_decode_id_at_12,
    fix_mapi, fix_mapi_at_0, ft_adc_ft_at_20, ft_div_ft_bogus_at_20,
    ftg_register_idle_routine_at_20, get_attrib_i_msg_on_i_stg_at_12, get_outlook_version,
    get_outlook_version_at_0, get_tnef_stream_codepage, get_tnef_stream_codepage_at_12,
    hr_add_columns_at_16, hr_add_columns_ex_at_20, hr_alloc_advise_sink_at_12,
    hr_compose_eid_at_28, hr_compose_msg_id_at_24, hr_decompose_eid_at_28,
    hr_decompose_msg_id_at_24, hr_dispatch_notifications_at_4, hr_entry_id_from_sz_at_12,
    hr_get_omi_providers_flags, hr_get_omi_providers_flags_at_8, hr_i_storage_from_stream_at_16,
    hr_set_omi_providers_flags_invalid, hr_set_omi_providers_flags_invalid_at_4,
    hr_sz_from_entry_id_at_12, hr_validate_ipm_subtree_at_20, hr_validate_parameters_at_8,
    install_filter_hook_at_4, launch_wizard_at_20, launchwizard, map_storage_s_code_at_4,
    mapi_deinit_idle_at_0, mapi_init_idle_at_4, mapi_open_form_mgr, mapi_open_form_mgr_at_8,
    open_i_msg_on_i_stg_at_44, open_i_msg_session_at_12, open_tnef_stream, open_tnef_stream_at_28,
    open_tnef_stream_ex, open_tnef_stream_ex_at_32, ordinal_8, pr_provider_init, rtf_sync,
    rtf_sync_at_12, sc_bin_from_hex_bounded_at_12, sc_copy_notifications_at_16,
    sc_count_notifications_at_12, sc_create_conversation_index_at_16, sc_generate_muid_at_4,
    sc_local_path_from_unc_at_12, sc_mapix_from_cmc, sc_mapix_from_smapi,
    sc_reloc_notifications_at_20, sc_unc_from_local_path_at_12, set_attrib_i_msg_on_i_stg_at_16,
    unkobj_co_free_at_8, unkobj_free_at_8, unkobj_free_rows_at_8, unkobj_sc_allocate_at_12,
    unkobj_sc_allocate_more_at_16, unkobj_sc_co_allocate_at_12, unkobj_sc_co_reallocate_at_12,
    unkobj_sc_sz_from_ids_alloc_at_20, wine_spec_unimplemented_stub, wrap_compressed_rtf_stream,
    wrap_compressed_rtf_stream_at_12, wrap_store_entry_id_at_24, StubException, StubFunction,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
