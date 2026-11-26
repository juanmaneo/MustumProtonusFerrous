use crate::stub::{wine_spec_unimplemented_stub, StubException};

/// The module name `winebuild` passes to stubs: `MODULE` in `dlls/adsldpc/Makefile.in`.
const ADSLDPC_DLL_NAME: &str = "adsldpc.dll";

/// `??0CLexer@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn clexer_ctor_arm() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "??0CLexer@@QAA@XZ")
}

/// `??0CLexer@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn clexer_ctor_i386() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "??0CLexer@@QAE@XZ")
}

/// `??0CLexer@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn clexer_ctor_win64() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "??0CLexer@@QEAA@XZ")
}

/// `??1CLexer@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn clexer_dtor_arm() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "??1CLexer@@QAA@XZ")
}

/// `??1CLexer@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn clexer_dtor_i386() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "??1CLexer@@QAE@XZ")
}

/// `??1CLexer@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn clexer_dtor_win64() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "??1CLexer@@QEAA@XZ")
}

/// `ADSIPrint`, an `@ stub` export.
pub fn adsi_print() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIPrint")
}

/// `ADsAbandonSearch`, an `@ stub` export.
pub fn ads_abandon_search() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsAbandonSearch")
}

/// `ADsCloseSearchHandle`, an `@ stub` export.
pub fn ads_close_search_handle() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsCloseSearchHandle")
}

/// `ADsCreateAttributeDefinition`, an `@ stub` export.
pub fn ads_create_attribute_definition() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsCreateAttributeDefinition")
}

/// `ADsCreateClassDefinition`, an `@ stub` export.
pub fn ads_create_class_definition() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsCreateClassDefinition")
}

/// `ADsCreateDSObject`, an `@ stub` export.
pub fn ads_create_ds_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsCreateDSObject")
}

/// `ADsCreateDSObjectExt`, an `@ stub` export.
pub fn ads_create_ds_object_ext() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsCreateDSObjectExt")
}

/// `ADsDeleteAttributeDefinition`, an `@ stub` export.
pub fn ads_delete_attribute_definition() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsDeleteAttributeDefinition")
}

/// `ADsDeleteClassDefinition`, an `@ stub` export.
pub fn ads_delete_class_definition() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsDeleteClassDefinition")
}

/// `ADsDeleteDSObject`, an `@ stub` export.
pub fn ads_delete_ds_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsDeleteDSObject")
}

/// `ADsEnumAttributes`, an `@ stub` export.
pub fn ads_enum_attributes() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsEnumAttributes")
}

/// `ADsEnumClasses`, an `@ stub` export.
pub fn ads_enum_classes() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsEnumClasses")
}

/// `ADsExecuteSearch`, an `@ stub` export.
pub fn ads_execute_search() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsExecuteSearch")
}

/// `ADsFreeColumn`, an `@ stub` export.
pub fn ads_free_column() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsFreeColumn")
}

/// `ADsGetColumn`, an `@ stub` export.
pub fn ads_get_column() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsGetColumn")
}

/// `ADsGetFirstRow`, an `@ stub` export.
pub fn ads_get_first_row() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsGetFirstRow")
}

/// `ADsGetNextColumnName`, an `@ stub` export.
pub fn ads_get_next_column_name() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsGetNextColumnName")
}

/// `ADsGetNextRow`, an `@ stub` export.
pub fn ads_get_next_row() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsGetNextRow")
}

/// `ADsGetObjectAttributes`, an `@ stub` export.
pub fn ads_get_object_attributes() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsGetObjectAttributes")
}

/// `ADsGetPreviousRow`, an `@ stub` export.
pub fn ads_get_previous_row() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsGetPreviousRow")
}

/// `ADsHelperGetCurrentRowMessage`, an `@ stub` export.
pub fn ads_helper_get_current_row_message() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsHelperGetCurrentRowMessage")
}

/// `ADsObject`, an `@ stub` export.
pub fn ads_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsObject")
}

/// `ADsSetObjectAttributes`, an `@ stub` export.
pub fn ads_set_object_attributes() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsSetObjectAttributes")
}

/// `ADsSetSearchPreference`, an `@ stub` export.
pub fn ads_set_search_preference() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsSetSearchPreference")
}

/// `ADsWriteAttributeDefinition`, an `@ stub` export.
pub fn ads_write_attribute_definition() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsWriteAttributeDefinition")
}

/// `ADsWriteClassDefinition`, an `@ stub` export.
pub fn ads_write_class_definition() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsWriteClassDefinition")
}

/// `AdsTypeToLdapTypeCopyConstruct`, an `@ stub` export.
pub fn ads_type_to_ldap_type_copy_construct() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "AdsTypeToLdapTypeCopyConstruct")
}

/// `AdsTypeToLdapTypeCopyDNWithBinary`, an `@ stub` export.
pub fn ads_type_to_ldap_type_copy_dn_with_binary() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "AdsTypeToLdapTypeCopyDNWithBinary")
}

/// `AdsTypeToLdapTypeCopyDNWithString`, an `@ stub` export.
pub fn ads_type_to_ldap_type_copy_dn_with_string() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "AdsTypeToLdapTypeCopyDNWithString")
}

/// `AdsTypeToLdapTypeCopyGeneralizedTime`, an `@ stub` export.
pub fn ads_type_to_ldap_type_copy_generalized_time() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "AdsTypeToLdapTypeCopyGeneralizedTime")
}

/// `AdsTypeToLdapTypeCopyTime`, an `@ stub` export.
pub fn ads_type_to_ldap_type_copy_time() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "AdsTypeToLdapTypeCopyTime")
}

/// `BerBvFree`, an `@ stub` export.
pub fn ber_bv_free() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BerBvFree")
}

/// `BerEncodingQuotaControl`, an `@ stub` export.
pub fn ber_encoding_quota_control() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BerEncodingQuotaControl")
}

/// `BuildADsParentPath`, an `@ stub` export.
pub fn build_ads_parent_path() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BuildADsParentPath")
}

/// `BuildADsParentPathFromObjectInfo2`, an `@ stub` export.
pub fn build_ads_parent_path_from_object_info2() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BuildADsParentPathFromObjectInfo2")
}

/// `BuildADsParentPathFromObjectInfo`, an `@ stub` export.
pub fn build_ads_parent_path_from_object_info() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BuildADsParentPathFromObjectInfo")
}

/// `BuildADsPathFromLDAPPath2`, an `@ stub` export.
pub fn build_ads_path_from_ldap_path2() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BuildADsPathFromLDAPPath2")
}

/// `BuildADsPathFromLDAPPath`, an `@ stub` export.
pub fn build_ads_path_from_ldap_path() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BuildADsPathFromLDAPPath")
}

/// `BuildADsPathFromParent`, an `@ stub` export.
pub fn build_ads_path_from_parent() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BuildADsPathFromParent")
}

/// `BuildLDAPPathFromADsPath2`, an `@ stub` export.
pub fn build_ldap_path_from_ads_path2() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BuildLDAPPathFromADsPath2")
}

/// `BuildLDAPPathFromADsPath`, an `@ stub` export.
pub fn build_ldap_path_from_ads_path() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "BuildLDAPPathFromADsPath")
}

/// `ChangeSeparator`, an `@ stub` export.
pub fn change_separator() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ChangeSeparator")
}

/// `Component`, an `@ stub` export.
pub fn component() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "Component")
}

/// `ConvertSidToString`, an `@ stub` export.
pub fn convert_sid_to_string() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ConvertSidToString")
}

/// `ConvertSidToU2Trustee`, an `@ stub` export.
pub fn convert_sid_to_u2_trustee() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ConvertSidToU2Trustee")
}

/// `ConvertU2TrusteeToSid`, an `@ stub` export.
pub fn convert_u2_trustee_to_sid() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ConvertU2TrusteeToSid")
}

/// `FindEntryInSearchTable`, an `@ stub` export.
pub fn find_entry_in_search_table() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "FindEntryInSearchTable")
}

/// `FindSearchTableIndex`, an `@ stub` export.
pub fn find_search_table_index() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "FindSearchTableIndex")
}

/// `FreeObjectInfo`, an `@ stub` export.
pub fn free_object_info() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "FreeObjectInfo")
}

/// `GetDefaultServer`, an `@ stub` export.
pub fn get_default_server() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "GetDefaultServer")
}

/// `GetDisplayName`, an `@ stub` export.
pub fn get_display_name() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "GetDisplayName")
}

/// `GetDomainDNSNameForDomain`, an `@ stub` export.
pub fn get_domain_dns_name_for_domain() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "GetDomainDNSNameForDomain")
}

/// `GetLDAPTypeName`, an `@ stub` export.
pub fn get_ldap_type_name() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "GetLDAPTypeName")
}

/// `?GetNextToken@CLexer@@QAAJPAGPAK@Z`, an `@ stub` export for `-arch=arm`.
pub fn clexer_get_next_token_arm() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?GetNextToken@CLexer@@QAAJPAGPAK@Z")
}

/// `?GetNextToken@CLexer@@QAEJPAGPAK@Z`, an `@ stub` export for `-arch=i386`.
pub fn clexer_get_next_token_i386() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?GetNextToken@CLexer@@QAEJPAGPAK@Z")
}

/// `?GetNextToken@CLexer@@QEAAJPEAGPEAK@Z`, an `@ stub` export for `-arch=win64`.
pub fn clexer_get_next_token_win64() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?GetNextToken@CLexer@@QEAAJPEAGPEAK@Z")
}

/// `GetServerAndPort`, an `@ stub` export.
pub fn get_server_and_port() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "GetServerAndPort")
}

/// `GetSyntaxOfAttribute`, an `@ stub` export.
pub fn get_syntax_of_attribute() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "GetSyntaxOfAttribute")
}

/// `InitObjectInfo`, an `@ stub` export.
pub fn init_object_info() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "InitObjectInfo")
}

/// `?InitializePath@CLexer@@QAAJPAG@Z`, an `@ stub` export for `-arch=arm`.
pub fn clexer_initialize_path_arm() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?InitializePath@CLexer@@QAAJPAG@Z")
}

/// `?InitializePath@CLexer@@QAEJPAG@Z`, an `@ stub` export for `-arch=i386`.
pub fn clexer_initialize_path_i386() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?InitializePath@CLexer@@QAEJPAG@Z")
}

/// `?InitializePath@CLexer@@QEAAJPEAG@Z`, an `@ stub` export for `-arch=win64`.
pub fn clexer_initialize_path_win64() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?InitializePath@CLexer@@QEAAJPEAG@Z")
}

/// `IsGCNamespace`, an `@ stub` export.
pub fn is_gc_namespace() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "IsGCNamespace")
}

/// `LdapAddExtS`, an `@ stub` export.
pub fn ldap_add_ext_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapAddExtS")
}

/// `LdapAddS`, an `@ stub` export.
pub fn ldap_add_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapAddS")
}

/// `LdapAttributeFree`, an `@ stub` export.
pub fn ldap_attribute_free() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapAttributeFree")
}

/// `LdapCacheAddRef`, an `@ stub` export.
pub fn ldap_cache_add_ref() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapCacheAddRef")
}

/// `LdapCloseObject`, an `@ stub` export.
pub fn ldap_close_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapCloseObject")
}

/// `LdapCompareExt`, an `@ stub` export.
pub fn ldap_compare_ext() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapCompareExt")
}

/// `LdapControlFree`, an `@ stub` export.
pub fn ldap_control_free() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapControlFree")
}

/// `LdapControlsFree`, an `@ stub` export.
pub fn ldap_controls_free() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapControlsFree")
}

/// `LdapCountEntries`, an `@ stub` export.
pub fn ldap_count_entries() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapCountEntries")
}

/// `LdapCrackUserDNtoNTLMUser2`, an `@ stub` export.
pub fn ldap_crack_user_dn_to_ntlm_user2() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapCrackUserDNtoNTLMUser2")
}

/// `LdapCreatePageControl`, an `@ stub` export.
pub fn ldap_create_page_control() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapCreatePageControl")
}

/// `LdapDeleteExtS`, an `@ stub` export.
pub fn ldap_delete_ext_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapDeleteExtS")
}

/// `LdapDeleteS`, an `@ stub` export.
pub fn ldap_delete_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapDeleteS")
}

/// `LdapFirstAttribute`, an `@ stub` export.
pub fn ldap_first_attribute() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapFirstAttribute")
}

/// `LdapFirstEntry`, an `@ stub` export.
pub fn ldap_first_entry() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapFirstEntry")
}

/// `LdapGetDn`, an `@ stub` export.
pub fn ldap_get_dn() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapGetDn")
}

/// `LdapGetNextPageS`, an `@ stub` export.
pub fn ldap_get_next_page_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapGetNextPageS")
}

/// `LdapGetSchemaObjectCount`, an `@ stub` export.
pub fn ldap_get_schema_object_count() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapGetSchemaObjectCount")
}

/// `LdapGetSubSchemaSubEntryPath`, an `@ stub` export.
pub fn ldap_get_sub_schema_sub_entry_path() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapGetSubSchemaSubEntryPath")
}

/// `LdapGetSyntaxIdOfAttribute`, an `@ stub` export.
pub fn ldap_get_syntax_id_of_attribute() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapGetSyntaxIdOfAttribute")
}

/// `LdapGetSyntaxOfAttributeOnServer`, an `@ stub` export.
pub fn ldap_get_syntax_of_attribute_on_server() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapGetSyntaxOfAttributeOnServer")
}

/// `LdapGetValues`, an `@ stub` export.
pub fn ldap_get_values() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapGetValues")
}

/// `LdapGetValuesLen`, an `@ stub` export.
pub fn ldap_get_values_len() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapGetValuesLen")
}

/// `LdapInitializeSearchPreferences`, an `@ stub` export.
pub fn ldap_initialize_search_preferences() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapInitializeSearchPreferences")
}

/// `LdapIsClassNameValidOnServer`, an `@ stub` export.
pub fn ldap_is_class_name_valid_on_server() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapIsClassNameValidOnServer")
}

/// `LdapMakeSchemaCacheObsolete`, an `@ stub` export.
pub fn ldap_make_schema_cache_obsolete() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapMakeSchemaCacheObsolete")
}

/// `LdapMemFree`, an `@ stub` export.
pub fn ldap_mem_free() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapMemFree")
}

/// `LdapModDnS`, an `@ stub` export.
pub fn ldap_mod_dn_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapModDnS")
}

/// `LdapModifyExtS`, an `@ stub` export.
pub fn ldap_modify_ext_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapModifyExtS")
}

/// `LdapModifyS`, an `@ stub` export.
pub fn ldap_modify_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapModifyS")
}

/// `LdapMsgFree`, an `@ stub` export.
pub fn ldap_msg_free() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapMsgFree")
}

/// `LdapNextAttribute`, an `@ stub` export.
pub fn ldap_next_attribute() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapNextAttribute")
}

/// `LdapNextEntry`, an `@ stub` export.
pub fn ldap_next_entry() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapNextEntry")
}

/// `LdapOpenObject2`, an `@ stub` export.
pub fn ldap_open_object2() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapOpenObject2")
}

/// `LdapOpenObject`, an `@ stub` export.
pub fn ldap_open_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapOpenObject")
}

/// `LdapParsePageControl`, an `@ stub` export.
pub fn ldap_parse_page_control() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapParsePageControl")
}

/// `LdapParseResult`, an `@ stub` export.
pub fn ldap_parse_result() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapParseResult")
}

/// `LdapReadAttribute2`, an `@ stub` export.
pub fn ldap_read_attribute2() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapReadAttribute2")
}

/// `LdapReadAttribute`, an `@ stub` export.
pub fn ldap_read_attribute() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapReadAttribute")
}

/// `LdapReadAttributeFast`, an `@ stub` export.
pub fn ldap_read_attribute_fast() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapReadAttributeFast")
}

/// `LdapRenameExtS`, an `@ stub` export.
pub fn ldap_rename_ext_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapRenameExtS")
}

/// `LdapResult`, an `@ stub` export.
pub fn ldap_result() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapResult")
}

/// `LdapSearch`, an `@ stub` export.
pub fn ldap_search() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapSearch")
}

/// `LdapSearchAbandonPage`, an `@ stub` export.
pub fn ldap_search_abandon_page() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapSearchAbandonPage")
}

/// `LdapSearchExtS`, an `@ stub` export.
pub fn ldap_search_ext_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapSearchExtS")
}

/// `LdapSearchInitPage`, an `@ stub` export.
pub fn ldap_search_init_page() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapSearchInitPage")
}

/// `LdapSearchS`, an `@ stub` export.
pub fn ldap_search_s() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapSearchS")
}

/// `LdapSearchST`, an `@ stub` export.
pub fn ldap_search_st() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapSearchST")
}

/// `LdapTypeBinaryToString`, an `@ stub` export.
pub fn ldap_type_binary_to_string() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeBinaryToString")
}

/// `LdapTypeCopyConstruct`, an `@ stub` export.
pub fn ldap_type_copy_construct() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeCopyConstruct")
}

/// `LdapTypeFreeLdapModList`, an `@ stub` export.
pub fn ldap_type_free_ldap_mod_list() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeFreeLdapModList")
}

/// `LdapTypeFreeLdapModObject`, an `@ stub` export.
pub fn ldap_type_free_ldap_mod_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeFreeLdapModObject")
}

/// `LdapTypeFreeLdapObjects`, an `@ stub` export.
pub fn ldap_type_free_ldap_objects() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeFreeLdapObjects")
}

/// `LdapTypeToAdsTypeDNWithBinary`, an `@ stub` export.
pub fn ldap_type_to_ads_type_dn_with_binary() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeToAdsTypeDNWithBinary")
}

/// `LdapTypeToAdsTypeDNWithString`, an `@ stub` export.
pub fn ldap_type_to_ads_type_dn_with_string() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeToAdsTypeDNWithString")
}

/// `LdapTypeToAdsTypeGeneralizedTime`, an `@ stub` export.
pub fn ldap_type_to_ads_type_generalized_time() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeToAdsTypeGeneralizedTime")
}

/// `LdapTypeToAdsTypeUTCTime`, an `@ stub` export.
pub fn ldap_type_to_ads_type_utc_time() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeToAdsTypeUTCTime")
}

/// `LdapValueFree`, an `@ stub` export.
pub fn ldap_value_free() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapValueFree")
}

/// `LdapValueFreeLen`, an `@ stub` export.
pub fn ldap_value_free_len() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapValueFreeLen")
}

/// `LdapcKeepHandleAround`, an `@ stub` export.
pub fn ldapc_keep_handle_around() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapcKeepHandleAround")
}

/// `LdapcSetStickyServer`, an `@ stub` export.
pub fn ldapc_set_sticky_server() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapcSetStickyServer")
}

/// `PathName`, an `@ stub` export.
pub fn path_name() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "PathName")
}

/// `ReadPagingSupportedAttr`, an `@ stub` export.
pub fn read_paging_supported_attr() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ReadPagingSupportedAttr")
}

/// `ReadSecurityDescriptorControlType`, an `@ stub` export.
pub fn read_security_descriptor_control_type() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ReadSecurityDescriptorControlType")
}

/// `ReadServerSupportsIsADAMControl`, an `@ stub` export.
pub fn read_server_supports_is_adam_control() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ReadServerSupportsIsADAMControl")
}

/// `ReadServerSupportsIsADControl`, an `@ stub` export.
pub fn read_server_supports_is_ad_control() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ReadServerSupportsIsADControl")
}

/// `SchemaAddRef`, an `@ stub` export.
pub fn schema_add_ref() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaAddRef")
}

/// `SchemaClose`, an `@ stub` export.
pub fn schema_close() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaClose")
}

/// `SchemaGetClassInfo`, an `@ stub` export.
pub fn schema_get_class_info() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaGetClassInfo")
}

/// `SchemaGetClassInfoByIndex`, an `@ stub` export.
pub fn schema_get_class_info_by_index() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaGetClassInfoByIndex")
}

/// `SchemaGetObjectCount`, an `@ stub` export.
pub fn schema_get_object_count() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaGetObjectCount")
}

/// `SchemaGetPropertyInfo`, an `@ stub` export.
pub fn schema_get_property_info() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaGetPropertyInfo")
}

/// `SchemaGetPropertyInfoByIndex`, an `@ stub` export.
pub fn schema_get_property_info_by_index() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaGetPropertyInfoByIndex")
}

/// `SchemaGetStringsFromStringTable`, an `@ stub` export.
pub fn schema_get_strings_from_string_table() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaGetStringsFromStringTable")
}

/// `SchemaGetSyntaxOfAttribute`, an `@ stub` export.
pub fn schema_get_syntax_of_attribute() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaGetSyntaxOfAttribute")
}

/// `SchemaIsClassAContainer`, an `@ stub` export.
pub fn schema_is_class_a_container() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaIsClassAContainer")
}

/// `SchemaOpen`, an `@ stub` export.
pub fn schema_open() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SchemaOpen")
}

/// `?SetAtDisabler@CLexer@@QAAXH@Z`, an `@ stub` export for `-arch=arm`.
pub fn clexer_set_at_disabler_arm() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?SetAtDisabler@CLexer@@QAAXH@Z")
}

/// `?SetAtDisabler@CLexer@@QAEXH@Z`, an `@ stub` export for `-arch=i386`.
pub fn clexer_set_at_disabler_i386() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?SetAtDisabler@CLexer@@QAEXH@Z")
}

/// `?SetAtDisabler@CLexer@@QEAAXH@Z`, an `@ stub` export for `-arch=win64`.
pub fn clexer_set_at_disabler_win64() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?SetAtDisabler@CLexer@@QEAAXH@Z")
}

/// `?SetExclaimnationDisabler@CLexer@@QAAXH@Z`, an `@ stub` export for `-arch=arm`.
pub fn clexer_set_exclaimnation_disabler_arm() -> StubException {
    wine_spec_unimplemented_stub(
        ADSLDPC_DLL_NAME,
        "?SetExclaimnationDisabler@CLexer@@QAAXH@Z",
    )
}

/// `?SetExclaimnationDisabler@CLexer@@QAEXH@Z`, an `@ stub` export for `-arch=i386`.
pub fn clexer_set_exclaimnation_disabler_i386() -> StubException {
    wine_spec_unimplemented_stub(
        ADSLDPC_DLL_NAME,
        "?SetExclaimnationDisabler@CLexer@@QAEXH@Z",
    )
}

/// `?SetExclaimnationDisabler@CLexer@@QEAAXH@Z`, an `@ stub` export for `-arch=win64`.
pub fn clexer_set_exclaimnation_disabler_win64() -> StubException {
    wine_spec_unimplemented_stub(
        ADSLDPC_DLL_NAME,
        "?SetExclaimnationDisabler@CLexer@@QEAAXH@Z",
    )
}

/// `?SetFSlashDisabler@CLexer@@QAAXH@Z`, an `@ stub` export for `-arch=arm`.
pub fn clexer_set_fslash_disabler_arm() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?SetFSlashDisabler@CLexer@@QAAXH@Z")
}

/// `?SetFSlashDisabler@CLexer@@QAEXH@Z`, an `@ stub` export for `-arch=i386`.
pub fn clexer_set_fslash_disabler_i386() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?SetFSlashDisabler@CLexer@@QAEXH@Z")
}

/// `?SetFSlashDisabler@CLexer@@QEAAXH@Z`, an `@ stub` export for `-arch=win64`.
pub fn clexer_set_fslash_disabler_win64() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "?SetFSlashDisabler@CLexer@@QEAAXH@Z")
}

/// `SortAndRemoveDuplicateOIDs`, an `@ stub` export.
pub fn sort_and_remove_duplicate_oids() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "SortAndRemoveDuplicateOIDs")
}

/// `UnMarshallLDAPToLDAPSynID`, an `@ stub` export.
pub fn un_marshall_ldap_to_ldap_syn_id() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "UnMarshallLDAPToLDAPSynID")
}

/// `intcmp`, an `@ stub` export.
pub fn intcmp() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "intcmp")
}

/// `ADSIAbandonSearch`, an `@ stub` export.
pub fn adsi_abandon_search() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIAbandonSearch")
}

/// `ADSICloseDSObject`, an `@ stub` export.
pub fn adsi_close_ds_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSICloseDSObject")
}

/// `ADSICloseSearchHandle`, an `@ stub` export.
pub fn adsi_close_search_handle() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSICloseSearchHandle")
}

/// `ADSICreateDSObject`, an `@ stub` export.
pub fn adsi_create_ds_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSICreateDSObject")
}

/// `ADSIDeleteDSObject`, an `@ stub` export.
pub fn adsi_delete_ds_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIDeleteDSObject")
}

/// `ADSIExecuteSearch`, an `@ stub` export.
pub fn adsi_execute_search() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIExecuteSearch")
}

/// `ADSIFreeColumn`, an `@ stub` export.
pub fn adsi_free_column() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIFreeColumn")
}

/// `ADSIGetColumn`, an `@ stub` export.
pub fn adsi_get_column() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIGetColumn")
}

/// `ADSIGetFirstRow`, an `@ stub` export.
pub fn adsi_get_first_row() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIGetFirstRow")
}

/// `ADSIGetNextColumnName`, an `@ stub` export.
pub fn adsi_get_next_column_name() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIGetNextColumnName")
}

/// `ADSIGetNextRow`, an `@ stub` export.
pub fn adsi_get_next_row() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIGetNextRow")
}

/// `ADSIGetObjectAttributes`, an `@ stub` export.
pub fn adsi_get_object_attributes() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIGetObjectAttributes")
}

/// `ADSIGetPreviousRow`, an `@ stub` export.
pub fn adsi_get_previous_row() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIGetPreviousRow")
}

/// `ADSIModifyRdn`, an `@ stub` export.
pub fn adsi_modify_rdn() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIModifyRdn")
}

/// `ADSIOpenDSObject`, an `@ stub` export.
pub fn adsi_open_ds_object() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSIOpenDSObject")
}

/// `ADSISetObjectAttributes`, an `@ stub` export.
pub fn adsi_set_object_attributes() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSISetObjectAttributes")
}

/// `ADSISetSearchPreference`, an `@ stub` export.
pub fn adsi_set_search_preference() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADSISetSearchPreference")
}

/// `ADsDecodeBinaryData`, an `@ stub` export.
pub fn ads_decode_binary_data() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsDecodeBinaryData")
}

/// `ADsEncodeBinaryData`, an `@ stub` export.
pub fn ads_encode_binary_data() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsEncodeBinaryData")
}

/// `ADsGetLastError`, an `@ stub` export.
pub fn ads_get_last_error() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsGetLastError")
}

/// `ADsSetLastError`, an `@ stub` export.
pub fn ads_set_last_error() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ADsSetLastError")
}

/// `AdsTypeFreeAdsObjects`, an `@ stub` export.
pub fn ads_type_free_ads_objects() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "AdsTypeFreeAdsObjects")
}

/// `AllocADsMem`, an `@ stub` export.
pub fn alloc_ads_mem() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "AllocADsMem")
}

/// `AllocADsStr`, an `@ stub` export.
pub fn alloc_ads_str() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "AllocADsStr")
}

/// `FreeADsMem`, an `@ stub` export.
pub fn free_ads_mem() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "FreeADsMem")
}

/// `FreeADsStr`, an `@ stub` export.
pub fn free_ads_str() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "FreeADsStr")
}

/// `LdapTypeToAdsTypeCopyConstruct`, an `@ stub` export.
pub fn ldap_type_to_ads_type_copy_construct() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "LdapTypeToAdsTypeCopyConstruct")
}

/// `MapADSTypeToLDAPType`, an `@ stub` export.
pub fn map_ads_type_to_ldap_type() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "MapADSTypeToLDAPType")
}

/// `MapLDAPTypeToADSType`, an `@ stub` export.
pub fn map_ldap_type_to_ads_type() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "MapLDAPTypeToADSType")
}

/// `ReallocADsMem`, an `@ stub` export.
pub fn realloc_ads_mem() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ReallocADsMem")
}

/// `ReallocADsStr`, an `@ stub` export.
pub fn realloc_ads_str() -> StubException {
    wine_spec_unimplemented_stub(ADSLDPC_DLL_NAME, "ReallocADsStr")
}
