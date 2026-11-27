//! Oracle-derived test vectors for the `samlib.spec` stub exports.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/samlib/`. The harness calls the stub the way winebuild's export stubs do, with
//! `samlib.dll` and the export name, for each of the 65 exports. `RtlRaiseException` is replaced
//! by a recorder that saves the record and jumps back, because the C body loops around it. All
//! 65 rows ran with no sanitizer report, so none are excluded.

use super::*;

/// One oracle row: the helper, then the code, flags, number of parameters, module and function.
type OracleRow = (
    fn() -> StubException,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

/// One row per export, in spec order. Kept one row per line, so the table stays readable.
#[rustfmt::skip]
const ORACLE_ROWS: [OracleRow; 65] = [
    (sam_add_member_to_alias, 0x8000_0100, 0x01, 2, "samlib.dll", "SamAddMemberToAlias"),
    (sam_add_member_to_group, 0x8000_0100, 0x01, 2, "samlib.dll", "SamAddMemberToGroup"),
    (sam_add_multiple_members_to_alias, 0x8000_0100, 0x01, 2, "samlib.dll", "SamAddMultipleMembersToAlias"),
    (sam_change_password_user2, 0x8000_0100, 0x01, 2, "samlib.dll", "SamChangePasswordUser2"),
    (sam_change_password_user3, 0x8000_0100, 0x01, 2, "samlib.dll", "SamChangePasswordUser3"),
    (sam_change_password_user, 0x8000_0100, 0x01, 2, "samlib.dll", "SamChangePasswordUser"),
    (sam_close_handle, 0x8000_0100, 0x01, 2, "samlib.dll", "SamCloseHandle"),
    (sam_connect, 0x8000_0100, 0x01, 2, "samlib.dll", "SamConnect"),
    (sam_connect_with_creds, 0x8000_0100, 0x01, 2, "samlib.dll", "SamConnectWithCreds"),
    (sam_create_alias_in_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamCreateAliasInDomain"),
    (sam_create_group_in_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamCreateGroupInDomain"),
    (sam_create_user2_in_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamCreateUser2InDomain"),
    (sam_create_user_in_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamCreateUserInDomain"),
    (sam_delete_alias, 0x8000_0100, 0x01, 2, "samlib.dll", "SamDeleteAlias"),
    (sam_delete_group, 0x8000_0100, 0x01, 2, "samlib.dll", "SamDeleteGroup"),
    (sam_delete_user, 0x8000_0100, 0x01, 2, "samlib.dll", "SamDeleteUser"),
    (sam_enumerate_aliases_in_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamEnumerateAliasesInDomain"),
    (sam_enumerate_domains_in_sam_server, 0x8000_0100, 0x01, 2, "samlib.dll", "SamEnumerateDomainsInSamServer"),
    (sam_enumerate_groups_in_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamEnumerateGroupsInDomain"),
    (sam_enumerate_users_in_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamEnumerateUsersInDomain"),
    (sam_free_memory, 0x8000_0100, 0x01, 2, "samlib.dll", "SamFreeMemory"),
    (sam_get_alias_membership, 0x8000_0100, 0x01, 2, "samlib.dll", "SamGetAliasMembership"),
    (sam_get_compatibility_mode, 0x8000_0100, 0x01, 2, "samlib.dll", "SamGetCompatibilityMode"),
    (sam_get_display_enumeration_index, 0x8000_0100, 0x01, 2, "samlib.dll", "SamGetDisplayEnumerationIndex"),
    (sam_get_groups_for_user, 0x8000_0100, 0x01, 2, "samlib.dll", "SamGetGroupsForUser"),
    (sam_get_members_in_alias, 0x8000_0100, 0x01, 2, "samlib.dll", "SamGetMembersInAlias"),
    (sam_get_members_in_group, 0x8000_0100, 0x01, 2, "samlib.dll", "SamGetMembersInGroup"),
    (sam_lookup_domain_in_sam_server, 0x8000_0100, 0x01, 2, "samlib.dll", "SamLookupDomainInSamServer"),
    (sam_lookup_ids_in_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamLookupIdsInDomain"),
    (sam_lookup_names_in_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamLookupNamesInDomain"),
    (sam_open_alias, 0x8000_0100, 0x01, 2, "samlib.dll", "SamOpenAlias"),
    (sam_open_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamOpenDomain"),
    (sam_open_group, 0x8000_0100, 0x01, 2, "samlib.dll", "SamOpenGroup"),
    (sam_open_user, 0x8000_0100, 0x01, 2, "samlib.dll", "SamOpenUser"),
    (sam_query_display_information, 0x8000_0100, 0x01, 2, "samlib.dll", "SamQueryDisplayInformation"),
    (sam_query_information_alias, 0x8000_0100, 0x01, 2, "samlib.dll", "SamQueryInformationAlias"),
    (sam_query_information_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamQueryInformationDomain"),
    (sam_query_information_group, 0x8000_0100, 0x01, 2, "samlib.dll", "SamQueryInformationGroup"),
    (sam_query_information_user, 0x8000_0100, 0x01, 2, "samlib.dll", "SamQueryInformationUser"),
    (sam_query_security_object, 0x8000_0100, 0x01, 2, "samlib.dll", "SamQuerySecurityObject"),
    (sam_remove_member_from_alias, 0x8000_0100, 0x01, 2, "samlib.dll", "SamRemoveMemberFromAlias"),
    (sam_remove_member_from_foreign_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamRemoveMemberFromForeignDomain"),
    (sam_remove_member_from_group, 0x8000_0100, 0x01, 2, "samlib.dll", "SamRemoveMemberFromGroup"),
    (sam_remove_multiple_members_from_alias, 0x8000_0100, 0x01, 2, "samlib.dll", "SamRemoveMultipleMembersFromAlias"),
    (sam_rid_to_sid, 0x8000_0100, 0x01, 2, "samlib.dll", "SamRidToSid"),
    (sam_set_information_alias, 0x8000_0100, 0x01, 2, "samlib.dll", "SamSetInformationAlias"),
    (sam_set_information_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamSetInformationDomain"),
    (sam_set_information_group, 0x8000_0100, 0x01, 2, "samlib.dll", "SamSetInformationGroup"),
    (sam_set_information_user, 0x8000_0100, 0x01, 2, "samlib.dll", "SamSetInformationUser"),
    (sam_set_member_attributes_of_group, 0x8000_0100, 0x01, 2, "samlib.dll", "SamSetMemberAttributesOfGroup"),
    (sam_set_security_object, 0x8000_0100, 0x01, 2, "samlib.dll", "SamSetSecurityObject"),
    (sam_shutdown_sam_server, 0x8000_0100, 0x01, 2, "samlib.dll", "SamShutdownSamServer"),
    (sam_test_private_functions_domain, 0x8000_0100, 0x01, 2, "samlib.dll", "SamTestPrivateFunctionsDomain"),
    (sam_test_private_functions_user, 0x8000_0100, 0x01, 2, "samlib.dll", "SamTestPrivateFunctionsUser"),
    (sami_change_keys, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiChangeKeys"),
    (sami_change_password_user2, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiChangePasswordUser2"),
    (sami_change_password_user3, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiChangePasswordUser3"),
    (sami_change_password_user, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiChangePasswordUser"),
    (sami_encrypt_passwords, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiEncryptPasswords"),
    (sami_get_boot_key_information, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiGetBootKeyInformation"),
    (sami_lm_change_password_user, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiLmChangePasswordUser"),
    (sami_oem_change_password_user2, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiOemChangePasswordUser2"),
    (sami_set_boot_key_information, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiSetBootKeyInformation"),
    (sami_set_dsrm_password, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiSetDSRMPassword"),
    (sami_set_dsrm_password_owf, 0x8000_0100, 0x01, 2, "samlib.dll", "SamiSetDSRMPasswordOWF"),
];

#[test]
fn oracle_stub_rows() {
    for (index, &(stub, code, flags, number_parameters, module, function)) in
        ORACLE_ROWS.iter().enumerate()
    {
        assert_eq!(
            stub(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            },
            "row {:02}",
            index + 1
        );
    }
}

#[test]
fn oracle_rows_follow_export_table_order() {
    // Not oracle-derived: the row order is the spec order that EXPORTS also uses.
    assert_eq!(ORACLE_ROWS.len(), EXPORTS.len());
    for ((_, _, _, _, _, function), (name, _)) in ORACLE_ROWS.iter().zip(EXPORTS) {
        assert_eq!(*function, name);
    }
}
