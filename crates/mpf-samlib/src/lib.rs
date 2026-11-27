//! Port of the portable logic in Wine's `dlls/samlib/samlib.spec` and `dlls/samlib/Makefile.in`.
//!
//! `dlls/samlib` has no C, `.idl`, `.rc` or `.tlb` sources. Its `samlib.spec` lists 65 exports,
//! and every one is `@ stub`, so this crate ports the export table and models each stub.
//!
//! Ported:
//! - `samlib.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `__wine_spec_unimplemented_stub` and the record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//! - One helper per `@ stub` export, which raises the record with the export's name, for example
//!   [`sam_add_member_to_alias`]
//!
//! Skipped:
//! - The SAM function bodies. The reference defines none: each export is `@ stub`, so winebuild
//!   points it at `__wine_spec_unimplemented_stub`.
//! - The `for (;;)` loop around `RtlRaiseException`. The port returns the record instead.
//! - The `__cdecl` calling convention, and the ordinal form of the function argument, since every
//!   export in the spec is named.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    sam_add_member_to_alias, sam_add_member_to_group, sam_add_multiple_members_to_alias,
    sam_change_password_user, sam_change_password_user2, sam_change_password_user3,
    sam_close_handle, sam_connect, sam_connect_with_creds, sam_create_alias_in_domain,
    sam_create_group_in_domain, sam_create_user2_in_domain, sam_create_user_in_domain,
    sam_delete_alias, sam_delete_group, sam_delete_user, sam_enumerate_aliases_in_domain,
    sam_enumerate_domains_in_sam_server, sam_enumerate_groups_in_domain,
    sam_enumerate_users_in_domain, sam_free_memory, sam_get_alias_membership,
    sam_get_compatibility_mode, sam_get_display_enumeration_index, sam_get_groups_for_user,
    sam_get_members_in_alias, sam_get_members_in_group, sam_lookup_domain_in_sam_server,
    sam_lookup_ids_in_domain, sam_lookup_names_in_domain, sam_open_alias, sam_open_domain,
    sam_open_group, sam_open_user, sam_query_display_information, sam_query_information_alias,
    sam_query_information_domain, sam_query_information_group, sam_query_information_user,
    sam_query_security_object, sam_remove_member_from_alias, sam_remove_member_from_foreign_domain,
    sam_remove_member_from_group, sam_remove_multiple_members_from_alias, sam_rid_to_sid,
    sam_set_information_alias, sam_set_information_domain, sam_set_information_group,
    sam_set_information_user, sam_set_member_attributes_of_group, sam_set_security_object,
    sam_shutdown_sam_server, sam_test_private_functions_domain, sam_test_private_functions_user,
    sami_change_keys, sami_change_password_user, sami_change_password_user2,
    sami_change_password_user3, sami_encrypt_passwords, sami_get_boot_key_information,
    sami_lm_change_password_user, sami_oem_change_password_user2, sami_set_boot_key_information,
    sami_set_dsrm_password, sami_set_dsrm_password_owf, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
