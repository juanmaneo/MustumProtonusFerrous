/// How `dlls/samlib/samlib.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `samlib.spec` export table, in spec order. Every export is `@ stub`.
pub const EXPORTS: [(&str, ExportKind); 65] = [
    ("SamAddMemberToAlias", ExportKind::Stub),
    ("SamAddMemberToGroup", ExportKind::Stub),
    ("SamAddMultipleMembersToAlias", ExportKind::Stub),
    ("SamChangePasswordUser2", ExportKind::Stub),
    ("SamChangePasswordUser3", ExportKind::Stub),
    ("SamChangePasswordUser", ExportKind::Stub),
    ("SamCloseHandle", ExportKind::Stub),
    ("SamConnect", ExportKind::Stub),
    ("SamConnectWithCreds", ExportKind::Stub),
    ("SamCreateAliasInDomain", ExportKind::Stub),
    ("SamCreateGroupInDomain", ExportKind::Stub),
    ("SamCreateUser2InDomain", ExportKind::Stub),
    ("SamCreateUserInDomain", ExportKind::Stub),
    ("SamDeleteAlias", ExportKind::Stub),
    ("SamDeleteGroup", ExportKind::Stub),
    ("SamDeleteUser", ExportKind::Stub),
    ("SamEnumerateAliasesInDomain", ExportKind::Stub),
    ("SamEnumerateDomainsInSamServer", ExportKind::Stub),
    ("SamEnumerateGroupsInDomain", ExportKind::Stub),
    ("SamEnumerateUsersInDomain", ExportKind::Stub),
    ("SamFreeMemory", ExportKind::Stub),
    ("SamGetAliasMembership", ExportKind::Stub),
    ("SamGetCompatibilityMode", ExportKind::Stub),
    ("SamGetDisplayEnumerationIndex", ExportKind::Stub),
    ("SamGetGroupsForUser", ExportKind::Stub),
    ("SamGetMembersInAlias", ExportKind::Stub),
    ("SamGetMembersInGroup", ExportKind::Stub),
    ("SamLookupDomainInSamServer", ExportKind::Stub),
    ("SamLookupIdsInDomain", ExportKind::Stub),
    ("SamLookupNamesInDomain", ExportKind::Stub),
    ("SamOpenAlias", ExportKind::Stub),
    ("SamOpenDomain", ExportKind::Stub),
    ("SamOpenGroup", ExportKind::Stub),
    ("SamOpenUser", ExportKind::Stub),
    ("SamQueryDisplayInformation", ExportKind::Stub),
    ("SamQueryInformationAlias", ExportKind::Stub),
    ("SamQueryInformationDomain", ExportKind::Stub),
    ("SamQueryInformationGroup", ExportKind::Stub),
    ("SamQueryInformationUser", ExportKind::Stub),
    ("SamQuerySecurityObject", ExportKind::Stub),
    ("SamRemoveMemberFromAlias", ExportKind::Stub),
    ("SamRemoveMemberFromForeignDomain", ExportKind::Stub),
    ("SamRemoveMemberFromGroup", ExportKind::Stub),
    ("SamRemoveMultipleMembersFromAlias", ExportKind::Stub),
    ("SamRidToSid", ExportKind::Stub),
    ("SamSetInformationAlias", ExportKind::Stub),
    ("SamSetInformationDomain", ExportKind::Stub),
    ("SamSetInformationGroup", ExportKind::Stub),
    ("SamSetInformationUser", ExportKind::Stub),
    ("SamSetMemberAttributesOfGroup", ExportKind::Stub),
    ("SamSetSecurityObject", ExportKind::Stub),
    ("SamShutdownSamServer", ExportKind::Stub),
    ("SamTestPrivateFunctionsDomain", ExportKind::Stub),
    ("SamTestPrivateFunctionsUser", ExportKind::Stub),
    ("SamiChangeKeys", ExportKind::Stub),
    ("SamiChangePasswordUser2", ExportKind::Stub),
    ("SamiChangePasswordUser3", ExportKind::Stub),
    ("SamiChangePasswordUser", ExportKind::Stub),
    ("SamiEncryptPasswords", ExportKind::Stub),
    ("SamiGetBootKeyInformation", ExportKind::Stub),
    ("SamiLmChangePasswordUser", ExportKind::Stub),
    ("SamiOemChangePasswordUser2", ExportKind::Stub),
    ("SamiSetBootKeyInformation", ExportKind::Stub),
    ("SamiSetDSRMPassword", ExportKind::Stub),
    ("SamiSetDSRMPasswordOWF", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
