/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = samlib.dll` in `dlls/samlib/Makefile.in`.
const SAMLIB_DLL_NAME: &str = "samlib.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `SamAddMemberToAlias`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_add_member_to_alias() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamAddMemberToAlias")
}

/// `SamAddMemberToGroup`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_add_member_to_group() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamAddMemberToGroup")
}

/// `SamAddMultipleMembersToAlias`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_add_multiple_members_to_alias() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamAddMultipleMembersToAlias")
}

/// `SamChangePasswordUser2`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_change_password_user2() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamChangePasswordUser2")
}

/// `SamChangePasswordUser3`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_change_password_user3() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamChangePasswordUser3")
}

/// `SamChangePasswordUser`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_change_password_user() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamChangePasswordUser")
}

/// `SamCloseHandle`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_close_handle() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamCloseHandle")
}

/// `SamConnect`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_connect() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamConnect")
}

/// `SamConnectWithCreds`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_connect_with_creds() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamConnectWithCreds")
}

/// `SamCreateAliasInDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_create_alias_in_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamCreateAliasInDomain")
}

/// `SamCreateGroupInDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_create_group_in_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamCreateGroupInDomain")
}

/// `SamCreateUser2InDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_create_user2_in_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamCreateUser2InDomain")
}

/// `SamCreateUserInDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_create_user_in_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamCreateUserInDomain")
}

/// `SamDeleteAlias`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_delete_alias() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamDeleteAlias")
}

/// `SamDeleteGroup`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_delete_group() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamDeleteGroup")
}

/// `SamDeleteUser`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_delete_user() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamDeleteUser")
}

/// `SamEnumerateAliasesInDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_enumerate_aliases_in_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamEnumerateAliasesInDomain")
}

/// `SamEnumerateDomainsInSamServer`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_enumerate_domains_in_sam_server() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamEnumerateDomainsInSamServer")
}

/// `SamEnumerateGroupsInDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_enumerate_groups_in_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamEnumerateGroupsInDomain")
}

/// `SamEnumerateUsersInDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_enumerate_users_in_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamEnumerateUsersInDomain")
}

/// `SamFreeMemory`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_free_memory() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamFreeMemory")
}

/// `SamGetAliasMembership`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_get_alias_membership() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamGetAliasMembership")
}

/// `SamGetCompatibilityMode`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_get_compatibility_mode() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamGetCompatibilityMode")
}

/// `SamGetDisplayEnumerationIndex`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_get_display_enumeration_index() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamGetDisplayEnumerationIndex")
}

/// `SamGetGroupsForUser`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_get_groups_for_user() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamGetGroupsForUser")
}

/// `SamGetMembersInAlias`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_get_members_in_alias() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamGetMembersInAlias")
}

/// `SamGetMembersInGroup`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_get_members_in_group() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamGetMembersInGroup")
}

/// `SamLookupDomainInSamServer`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_lookup_domain_in_sam_server() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamLookupDomainInSamServer")
}

/// `SamLookupIdsInDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_lookup_ids_in_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamLookupIdsInDomain")
}

/// `SamLookupNamesInDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_lookup_names_in_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamLookupNamesInDomain")
}

/// `SamOpenAlias`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_open_alias() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamOpenAlias")
}

/// `SamOpenDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_open_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamOpenDomain")
}

/// `SamOpenGroup`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_open_group() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamOpenGroup")
}

/// `SamOpenUser`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_open_user() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamOpenUser")
}

/// `SamQueryDisplayInformation`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_query_display_information() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamQueryDisplayInformation")
}

/// `SamQueryInformationAlias`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_query_information_alias() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamQueryInformationAlias")
}

/// `SamQueryInformationDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_query_information_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamQueryInformationDomain")
}

/// `SamQueryInformationGroup`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_query_information_group() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamQueryInformationGroup")
}

/// `SamQueryInformationUser`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_query_information_user() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamQueryInformationUser")
}

/// `SamQuerySecurityObject`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_query_security_object() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamQuerySecurityObject")
}

/// `SamRemoveMemberFromAlias`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_remove_member_from_alias() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamRemoveMemberFromAlias")
}

/// `SamRemoveMemberFromForeignDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_remove_member_from_foreign_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamRemoveMemberFromForeignDomain")
}

/// `SamRemoveMemberFromGroup`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_remove_member_from_group() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamRemoveMemberFromGroup")
}

/// `SamRemoveMultipleMembersFromAlias`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_remove_multiple_members_from_alias() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamRemoveMultipleMembersFromAlias")
}

/// `SamRidToSid`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_rid_to_sid() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamRidToSid")
}

/// `SamSetInformationAlias`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_set_information_alias() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamSetInformationAlias")
}

/// `SamSetInformationDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_set_information_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamSetInformationDomain")
}

/// `SamSetInformationGroup`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_set_information_group() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamSetInformationGroup")
}

/// `SamSetInformationUser`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_set_information_user() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamSetInformationUser")
}

/// `SamSetMemberAttributesOfGroup`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_set_member_attributes_of_group() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamSetMemberAttributesOfGroup")
}

/// `SamSetSecurityObject`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_set_security_object() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamSetSecurityObject")
}

/// `SamShutdownSamServer`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_shutdown_sam_server() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamShutdownSamServer")
}

/// `SamTestPrivateFunctionsDomain`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_test_private_functions_domain() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamTestPrivateFunctionsDomain")
}

/// `SamTestPrivateFunctionsUser`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sam_test_private_functions_user() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamTestPrivateFunctionsUser")
}

/// `SamiChangeKeys`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_change_keys() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiChangeKeys")
}

/// `SamiChangePasswordUser2`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_change_password_user2() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiChangePasswordUser2")
}

/// `SamiChangePasswordUser3`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_change_password_user3() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiChangePasswordUser3")
}

/// `SamiChangePasswordUser`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_change_password_user() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiChangePasswordUser")
}

/// `SamiEncryptPasswords`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_encrypt_passwords() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiEncryptPasswords")
}

/// `SamiGetBootKeyInformation`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_get_boot_key_information() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiGetBootKeyInformation")
}

/// `SamiLmChangePasswordUser`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_lm_change_password_user() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiLmChangePasswordUser")
}

/// `SamiOemChangePasswordUser2`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_oem_change_password_user2() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiOemChangePasswordUser2")
}

/// `SamiSetBootKeyInformation`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_set_boot_key_information() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiSetBootKeyInformation")
}

/// `SamiSetDSRMPassword`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_set_dsrm_password() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiSetDSRMPassword")
}

/// `SamiSetDSRMPasswordOWF`, exported as `@ stub` in `dlls/samlib/samlib.spec`.
pub fn sami_set_dsrm_password_owf() -> StubException {
    wine_spec_unimplemented_stub(SAMLIB_DLL_NAME, "SamiSetDSRMPasswordOWF")
}
