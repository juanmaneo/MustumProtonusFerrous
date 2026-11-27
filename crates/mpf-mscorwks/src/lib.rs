//! Port of the portable logic in Wine's `dlls/mscorwks/mscorwks.spec` and `dlls/mscorwks/Makefile.in`.
//! The DLL has no `.c` sources, so the port covers its export table, its stub records and its
//! constants.
//!
//! Ported:
//! - The export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`export_by_name`]. It has
//!   110 entries in spec order: 13 `@ stdcall` forwards to `mscoree` and 97 `@ stub` exports.
//! - `__wine_spec_unimplemented_stub` and the record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`] and [`EXCEPTION_NONCONTINUABLE`].
//! - One helper per `@ stub` export, named by the snake_case of the export name, that raises its
//!   record, such as [`copy_pdbs`] and [`_axl_get_issuer_public_key_hash`].
//!
//! Skipped:
//! - The code of the forwarded exports, which lives in `mscoree`, and the `@ stdcall` argument
//!   lists, including the i386 `@N` decoration. A forward keeps its name, ordinal and target.
//! - The `ExceptionRecord` (always NULL) and `ExceptionAddress` (the stub's own address) fields,
//!   and the `__cdecl` convention. The record keeps the code, flags, parameter count and the two
//!   parameters.
//! - The stub code that `winebuild` generates for each export. The port passes the module and the
//!   export name directly.

mod exports;
mod stub;

pub use exports::{export_by_name, Export, ExportKind, EXPORTS};
pub use stub::{
    _axl_get_issuer_public_key_hash, _axl_public_key_blob_to_public_key_token,
    _axl_rsa_key_value_to_public_key_token, cert_create_authenticode_license,
    cert_free_authenticode_signer_info, cert_free_authenticode_timestamper_info,
    cert_timestamp_authenticode_license, cert_verify_authenticode_license, clear_download_cache,
    co_initialize_ee, co_uninitialize_cor, co_uninitialize_ee, compare_assembly_identity,
    copy_pdbs, cor_dll_main_for_thunk, cor_launch_application, cor_mark_thread_in_thread_pool,
    create_act_context, create_act_context_internal, create_application_context,
    create_assembly_cache, create_assembly_enum, create_assembly_name_object, create_cms_from_xml,
    create_cms_from_xml_internal, create_history_reader, create_install_reference_enum,
    delete_shadow_cache, dll_can_unload_now_internal, dll_get_class_object_internal,
    dll_register_server_internal, dll_unregister_server_internal, ee_dll_register_server,
    ee_dll_unregister_server, get_addr_of_contract_shutoff_flag, get_app_id_authority,
    get_assembly_identity_from_file, get_cache_path, get_clr_function, get_clr_identity_manager,
    get_hash_from_assembly_file, get_hash_from_assembly_file_w, get_hash_from_blob,
    get_hash_from_file, get_hash_from_file_w, get_hash_from_handle, get_history_file_directory,
    get_identity_authority, get_meta_data_internal_interface,
    get_meta_data_internal_interface_from_public, get_meta_data_public_interface_from_internal,
    get_permission_requests, get_private_contexts_perf_counters, get_user_state_manager,
    get_user_state_manager_internal, get_user_store, get_user_store_internal, iee,
    initialize_fusion, install_custom_assembly, install_custom_module, legacy_n_gen_compile,
    legacy_n_gen_create_zapper, legacy_n_gen_free_zapper, legacy_n_gen_try_enumerate_fusion_cache,
    log_help_log_assert, log_help_no_gui_on_assert, log_help_terminate_on_assert,
    lookup_history_assembly, meta_data_get_dispenser, n_gen_create_n_gen_worker,
    nuke_downloaded_cache, parse_manifest, parse_manifest_internal, post_error_va,
    pre_bind_assembly, pre_bind_assembly_ex, re_open_meta_data_with_memory,
    re_open_meta_data_with_memory_ex, strong_name_compare_assemblies, strong_name_error_info,
    strong_name_free_buffer, strong_name_get_blob, strong_name_get_blob_from_image,
    strong_name_get_public_key, strong_name_hash_size, strong_name_key_delete, strong_name_key_gen,
    strong_name_key_gen_ex, strong_name_key_install, strong_name_signature_generation,
    strong_name_signature_generation_ex, strong_name_signature_size,
    strong_name_signature_verification_from_image, strong_name_token_from_assembly_ex,
    strong_name_token_from_public_key, translate_security_attributes, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
