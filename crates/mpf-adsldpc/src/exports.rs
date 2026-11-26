/// The `-arch=` restriction on an `adsldpc.spec` entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arch {
    Arm,
    I386,
    Win64,
}

/// One `@ stub` line of `dlls/adsldpc/adsldpc.spec`: the exported name and its `-arch=`
/// restriction, `None` when the entry applies to every architecture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubExport {
    pub name: &'static str,
    pub arch: Option<Arch>,
}

/// The `adsldpc.spec` export table, in spec order. Every entry is `@ stub`.
pub const EXPORTS: [StubExport; 189] = [
    StubExport {
        name: "??0CLexer@@QAA@XZ",
        arch: Some(Arch::Arm),
    },
    StubExport {
        name: "??0CLexer@@QAE@XZ",
        arch: Some(Arch::I386),
    },
    StubExport {
        name: "??0CLexer@@QEAA@XZ",
        arch: Some(Arch::Win64),
    },
    StubExport {
        name: "??1CLexer@@QAA@XZ",
        arch: Some(Arch::Arm),
    },
    StubExport {
        name: "??1CLexer@@QAE@XZ",
        arch: Some(Arch::I386),
    },
    StubExport {
        name: "??1CLexer@@QEAA@XZ",
        arch: Some(Arch::Win64),
    },
    StubExport {
        name: "ADSIPrint",
        arch: None,
    },
    StubExport {
        name: "ADsAbandonSearch",
        arch: None,
    },
    StubExport {
        name: "ADsCloseSearchHandle",
        arch: None,
    },
    StubExport {
        name: "ADsCreateAttributeDefinition",
        arch: None,
    },
    StubExport {
        name: "ADsCreateClassDefinition",
        arch: None,
    },
    StubExport {
        name: "ADsCreateDSObject",
        arch: None,
    },
    StubExport {
        name: "ADsCreateDSObjectExt",
        arch: None,
    },
    StubExport {
        name: "ADsDeleteAttributeDefinition",
        arch: None,
    },
    StubExport {
        name: "ADsDeleteClassDefinition",
        arch: None,
    },
    StubExport {
        name: "ADsDeleteDSObject",
        arch: None,
    },
    StubExport {
        name: "ADsEnumAttributes",
        arch: None,
    },
    StubExport {
        name: "ADsEnumClasses",
        arch: None,
    },
    StubExport {
        name: "ADsExecuteSearch",
        arch: None,
    },
    StubExport {
        name: "ADsFreeColumn",
        arch: None,
    },
    StubExport {
        name: "ADsGetColumn",
        arch: None,
    },
    StubExport {
        name: "ADsGetFirstRow",
        arch: None,
    },
    StubExport {
        name: "ADsGetNextColumnName",
        arch: None,
    },
    StubExport {
        name: "ADsGetNextRow",
        arch: None,
    },
    StubExport {
        name: "ADsGetObjectAttributes",
        arch: None,
    },
    StubExport {
        name: "ADsGetPreviousRow",
        arch: None,
    },
    StubExport {
        name: "ADsHelperGetCurrentRowMessage",
        arch: None,
    },
    StubExport {
        name: "ADsObject",
        arch: None,
    },
    StubExport {
        name: "ADsSetObjectAttributes",
        arch: None,
    },
    StubExport {
        name: "ADsSetSearchPreference",
        arch: None,
    },
    StubExport {
        name: "ADsWriteAttributeDefinition",
        arch: None,
    },
    StubExport {
        name: "ADsWriteClassDefinition",
        arch: None,
    },
    StubExport {
        name: "AdsTypeToLdapTypeCopyConstruct",
        arch: None,
    },
    StubExport {
        name: "AdsTypeToLdapTypeCopyDNWithBinary",
        arch: None,
    },
    StubExport {
        name: "AdsTypeToLdapTypeCopyDNWithString",
        arch: None,
    },
    StubExport {
        name: "AdsTypeToLdapTypeCopyGeneralizedTime",
        arch: None,
    },
    StubExport {
        name: "AdsTypeToLdapTypeCopyTime",
        arch: None,
    },
    StubExport {
        name: "BerBvFree",
        arch: None,
    },
    StubExport {
        name: "BerEncodingQuotaControl",
        arch: None,
    },
    StubExport {
        name: "BuildADsParentPath",
        arch: None,
    },
    StubExport {
        name: "BuildADsParentPathFromObjectInfo2",
        arch: None,
    },
    StubExport {
        name: "BuildADsParentPathFromObjectInfo",
        arch: None,
    },
    StubExport {
        name: "BuildADsPathFromLDAPPath2",
        arch: None,
    },
    StubExport {
        name: "BuildADsPathFromLDAPPath",
        arch: None,
    },
    StubExport {
        name: "BuildADsPathFromParent",
        arch: None,
    },
    StubExport {
        name: "BuildLDAPPathFromADsPath2",
        arch: None,
    },
    StubExport {
        name: "BuildLDAPPathFromADsPath",
        arch: None,
    },
    StubExport {
        name: "ChangeSeparator",
        arch: None,
    },
    StubExport {
        name: "Component",
        arch: None,
    },
    StubExport {
        name: "ConvertSidToString",
        arch: None,
    },
    StubExport {
        name: "ConvertSidToU2Trustee",
        arch: None,
    },
    StubExport {
        name: "ConvertU2TrusteeToSid",
        arch: None,
    },
    StubExport {
        name: "FindEntryInSearchTable",
        arch: None,
    },
    StubExport {
        name: "FindSearchTableIndex",
        arch: None,
    },
    StubExport {
        name: "FreeObjectInfo",
        arch: None,
    },
    StubExport {
        name: "GetDefaultServer",
        arch: None,
    },
    StubExport {
        name: "GetDisplayName",
        arch: None,
    },
    StubExport {
        name: "GetDomainDNSNameForDomain",
        arch: None,
    },
    StubExport {
        name: "GetLDAPTypeName",
        arch: None,
    },
    StubExport {
        name: "?GetNextToken@CLexer@@QAAJPAGPAK@Z",
        arch: Some(Arch::Arm),
    },
    StubExport {
        name: "?GetNextToken@CLexer@@QAEJPAGPAK@Z",
        arch: Some(Arch::I386),
    },
    StubExport {
        name: "?GetNextToken@CLexer@@QEAAJPEAGPEAK@Z",
        arch: Some(Arch::Win64),
    },
    StubExport {
        name: "GetServerAndPort",
        arch: None,
    },
    StubExport {
        name: "GetSyntaxOfAttribute",
        arch: None,
    },
    StubExport {
        name: "InitObjectInfo",
        arch: None,
    },
    StubExport {
        name: "?InitializePath@CLexer@@QAAJPAG@Z",
        arch: Some(Arch::Arm),
    },
    StubExport {
        name: "?InitializePath@CLexer@@QAEJPAG@Z",
        arch: Some(Arch::I386),
    },
    StubExport {
        name: "?InitializePath@CLexer@@QEAAJPEAG@Z",
        arch: Some(Arch::Win64),
    },
    StubExport {
        name: "IsGCNamespace",
        arch: None,
    },
    StubExport {
        name: "LdapAddExtS",
        arch: None,
    },
    StubExport {
        name: "LdapAddS",
        arch: None,
    },
    StubExport {
        name: "LdapAttributeFree",
        arch: None,
    },
    StubExport {
        name: "LdapCacheAddRef",
        arch: None,
    },
    StubExport {
        name: "LdapCloseObject",
        arch: None,
    },
    StubExport {
        name: "LdapCompareExt",
        arch: None,
    },
    StubExport {
        name: "LdapControlFree",
        arch: None,
    },
    StubExport {
        name: "LdapControlsFree",
        arch: None,
    },
    StubExport {
        name: "LdapCountEntries",
        arch: None,
    },
    StubExport {
        name: "LdapCrackUserDNtoNTLMUser2",
        arch: None,
    },
    StubExport {
        name: "LdapCreatePageControl",
        arch: None,
    },
    StubExport {
        name: "LdapDeleteExtS",
        arch: None,
    },
    StubExport {
        name: "LdapDeleteS",
        arch: None,
    },
    StubExport {
        name: "LdapFirstAttribute",
        arch: None,
    },
    StubExport {
        name: "LdapFirstEntry",
        arch: None,
    },
    StubExport {
        name: "LdapGetDn",
        arch: None,
    },
    StubExport {
        name: "LdapGetNextPageS",
        arch: None,
    },
    StubExport {
        name: "LdapGetSchemaObjectCount",
        arch: None,
    },
    StubExport {
        name: "LdapGetSubSchemaSubEntryPath",
        arch: None,
    },
    StubExport {
        name: "LdapGetSyntaxIdOfAttribute",
        arch: None,
    },
    StubExport {
        name: "LdapGetSyntaxOfAttributeOnServer",
        arch: None,
    },
    StubExport {
        name: "LdapGetValues",
        arch: None,
    },
    StubExport {
        name: "LdapGetValuesLen",
        arch: None,
    },
    StubExport {
        name: "LdapInitializeSearchPreferences",
        arch: None,
    },
    StubExport {
        name: "LdapIsClassNameValidOnServer",
        arch: None,
    },
    StubExport {
        name: "LdapMakeSchemaCacheObsolete",
        arch: None,
    },
    StubExport {
        name: "LdapMemFree",
        arch: None,
    },
    StubExport {
        name: "LdapModDnS",
        arch: None,
    },
    StubExport {
        name: "LdapModifyExtS",
        arch: None,
    },
    StubExport {
        name: "LdapModifyS",
        arch: None,
    },
    StubExport {
        name: "LdapMsgFree",
        arch: None,
    },
    StubExport {
        name: "LdapNextAttribute",
        arch: None,
    },
    StubExport {
        name: "LdapNextEntry",
        arch: None,
    },
    StubExport {
        name: "LdapOpenObject2",
        arch: None,
    },
    StubExport {
        name: "LdapOpenObject",
        arch: None,
    },
    StubExport {
        name: "LdapParsePageControl",
        arch: None,
    },
    StubExport {
        name: "LdapParseResult",
        arch: None,
    },
    StubExport {
        name: "LdapReadAttribute2",
        arch: None,
    },
    StubExport {
        name: "LdapReadAttribute",
        arch: None,
    },
    StubExport {
        name: "LdapReadAttributeFast",
        arch: None,
    },
    StubExport {
        name: "LdapRenameExtS",
        arch: None,
    },
    StubExport {
        name: "LdapResult",
        arch: None,
    },
    StubExport {
        name: "LdapSearch",
        arch: None,
    },
    StubExport {
        name: "LdapSearchAbandonPage",
        arch: None,
    },
    StubExport {
        name: "LdapSearchExtS",
        arch: None,
    },
    StubExport {
        name: "LdapSearchInitPage",
        arch: None,
    },
    StubExport {
        name: "LdapSearchS",
        arch: None,
    },
    StubExport {
        name: "LdapSearchST",
        arch: None,
    },
    StubExport {
        name: "LdapTypeBinaryToString",
        arch: None,
    },
    StubExport {
        name: "LdapTypeCopyConstruct",
        arch: None,
    },
    StubExport {
        name: "LdapTypeFreeLdapModList",
        arch: None,
    },
    StubExport {
        name: "LdapTypeFreeLdapModObject",
        arch: None,
    },
    StubExport {
        name: "LdapTypeFreeLdapObjects",
        arch: None,
    },
    StubExport {
        name: "LdapTypeToAdsTypeDNWithBinary",
        arch: None,
    },
    StubExport {
        name: "LdapTypeToAdsTypeDNWithString",
        arch: None,
    },
    StubExport {
        name: "LdapTypeToAdsTypeGeneralizedTime",
        arch: None,
    },
    StubExport {
        name: "LdapTypeToAdsTypeUTCTime",
        arch: None,
    },
    StubExport {
        name: "LdapValueFree",
        arch: None,
    },
    StubExport {
        name: "LdapValueFreeLen",
        arch: None,
    },
    StubExport {
        name: "LdapcKeepHandleAround",
        arch: None,
    },
    StubExport {
        name: "LdapcSetStickyServer",
        arch: None,
    },
    StubExport {
        name: "PathName",
        arch: None,
    },
    StubExport {
        name: "ReadPagingSupportedAttr",
        arch: None,
    },
    StubExport {
        name: "ReadSecurityDescriptorControlType",
        arch: None,
    },
    StubExport {
        name: "ReadServerSupportsIsADAMControl",
        arch: None,
    },
    StubExport {
        name: "ReadServerSupportsIsADControl",
        arch: None,
    },
    StubExport {
        name: "SchemaAddRef",
        arch: None,
    },
    StubExport {
        name: "SchemaClose",
        arch: None,
    },
    StubExport {
        name: "SchemaGetClassInfo",
        arch: None,
    },
    StubExport {
        name: "SchemaGetClassInfoByIndex",
        arch: None,
    },
    StubExport {
        name: "SchemaGetObjectCount",
        arch: None,
    },
    StubExport {
        name: "SchemaGetPropertyInfo",
        arch: None,
    },
    StubExport {
        name: "SchemaGetPropertyInfoByIndex",
        arch: None,
    },
    StubExport {
        name: "SchemaGetStringsFromStringTable",
        arch: None,
    },
    StubExport {
        name: "SchemaGetSyntaxOfAttribute",
        arch: None,
    },
    StubExport {
        name: "SchemaIsClassAContainer",
        arch: None,
    },
    StubExport {
        name: "SchemaOpen",
        arch: None,
    },
    StubExport {
        name: "?SetAtDisabler@CLexer@@QAAXH@Z",
        arch: Some(Arch::Arm),
    },
    StubExport {
        name: "?SetAtDisabler@CLexer@@QAEXH@Z",
        arch: Some(Arch::I386),
    },
    StubExport {
        name: "?SetAtDisabler@CLexer@@QEAAXH@Z",
        arch: Some(Arch::Win64),
    },
    StubExport {
        name: "?SetExclaimnationDisabler@CLexer@@QAAXH@Z",
        arch: Some(Arch::Arm),
    },
    StubExport {
        name: "?SetExclaimnationDisabler@CLexer@@QAEXH@Z",
        arch: Some(Arch::I386),
    },
    StubExport {
        name: "?SetExclaimnationDisabler@CLexer@@QEAAXH@Z",
        arch: Some(Arch::Win64),
    },
    StubExport {
        name: "?SetFSlashDisabler@CLexer@@QAAXH@Z",
        arch: Some(Arch::Arm),
    },
    StubExport {
        name: "?SetFSlashDisabler@CLexer@@QAEXH@Z",
        arch: Some(Arch::I386),
    },
    StubExport {
        name: "?SetFSlashDisabler@CLexer@@QEAAXH@Z",
        arch: Some(Arch::Win64),
    },
    StubExport {
        name: "SortAndRemoveDuplicateOIDs",
        arch: None,
    },
    StubExport {
        name: "UnMarshallLDAPToLDAPSynID",
        arch: None,
    },
    StubExport {
        name: "intcmp",
        arch: None,
    },
    StubExport {
        name: "ADSIAbandonSearch",
        arch: None,
    },
    StubExport {
        name: "ADSICloseDSObject",
        arch: None,
    },
    StubExport {
        name: "ADSICloseSearchHandle",
        arch: None,
    },
    StubExport {
        name: "ADSICreateDSObject",
        arch: None,
    },
    StubExport {
        name: "ADSIDeleteDSObject",
        arch: None,
    },
    StubExport {
        name: "ADSIExecuteSearch",
        arch: None,
    },
    StubExport {
        name: "ADSIFreeColumn",
        arch: None,
    },
    StubExport {
        name: "ADSIGetColumn",
        arch: None,
    },
    StubExport {
        name: "ADSIGetFirstRow",
        arch: None,
    },
    StubExport {
        name: "ADSIGetNextColumnName",
        arch: None,
    },
    StubExport {
        name: "ADSIGetNextRow",
        arch: None,
    },
    StubExport {
        name: "ADSIGetObjectAttributes",
        arch: None,
    },
    StubExport {
        name: "ADSIGetPreviousRow",
        arch: None,
    },
    StubExport {
        name: "ADSIModifyRdn",
        arch: None,
    },
    StubExport {
        name: "ADSIOpenDSObject",
        arch: None,
    },
    StubExport {
        name: "ADSISetObjectAttributes",
        arch: None,
    },
    StubExport {
        name: "ADSISetSearchPreference",
        arch: None,
    },
    StubExport {
        name: "ADsDecodeBinaryData",
        arch: None,
    },
    StubExport {
        name: "ADsEncodeBinaryData",
        arch: None,
    },
    StubExport {
        name: "ADsGetLastError",
        arch: None,
    },
    StubExport {
        name: "ADsSetLastError",
        arch: None,
    },
    StubExport {
        name: "AdsTypeFreeAdsObjects",
        arch: None,
    },
    StubExport {
        name: "AllocADsMem",
        arch: None,
    },
    StubExport {
        name: "AllocADsStr",
        arch: None,
    },
    StubExport {
        name: "FreeADsMem",
        arch: None,
    },
    StubExport {
        name: "FreeADsStr",
        arch: None,
    },
    StubExport {
        name: "LdapTypeToAdsTypeCopyConstruct",
        arch: None,
    },
    StubExport {
        name: "MapADSTypeToLDAPType",
        arch: None,
    },
    StubExport {
        name: "MapLDAPTypeToADSType",
        arch: None,
    },
    StubExport {
        name: "ReallocADsMem",
        arch: None,
    },
    StubExport {
        name: "ReallocADsStr",
        arch: None,
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn find_export(name: &str) -> Option<StubExport> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}
