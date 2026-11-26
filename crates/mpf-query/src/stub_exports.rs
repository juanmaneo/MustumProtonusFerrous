//! Helpers for the `@ stub` exports of `dlls/query/query.spec`, one per export line, in spec order.
//!
//! Each helper is named after its export: the C name in snake_case, without the leading `_` and
//! the `@N` stdcall suffix. Each one passes the export name to [`wine_spec_unimplemented_stub`]
//! exactly as the spec writes it, which is the string `winebuild` emits for the `function` field.

use super::stub::{wine_spec_unimplemented_stub, StubException, QUERY_DLL_NAME};

/// `@ stub _AbortMerges@16`.
pub fn abort_merges() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "_AbortMerges@16")
}

/// `@ stub BeginCacheTransaction`.
pub fn begin_cache_transaction() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "BeginCacheTransaction")
}

/// `@ stub BindIFilterFromStorage`.
pub fn bind_i_filter_from_storage() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "BindIFilterFromStorage")
}

/// `@ stub BindIFilterFromStream`.
pub fn bind_i_filter_from_stream() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "BindIFilterFromStream")
}

/// `@ stub CIBuildQueryNode`.
pub fn ci_build_query_node() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CIBuildQueryNode")
}

/// `@ stub CIBuildQueryTree`.
pub fn ci_build_query_tree() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CIBuildQueryTree")
}

/// `@ stub CICreateCommand`.
pub fn ci_create_command() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CICreateCommand")
}

/// `@ stub CIGetGlobalPropertyList`.
pub fn ci_get_global_property_list() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CIGetGlobalPropertyList")
}

/// `@ stub CIMakeICommand`.
pub fn ci_make_i_command() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CIMakeICommand")
}

/// `@ stub CIRestrictionToFullTree`.
pub fn ci_restriction_to_full_tree() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CIRestrictionToFullTree")
}

/// `@ stub CITextToFullTree`.
pub fn ci_text_to_full_tree() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CITextToFullTree")
}

/// `@ stub CITextToFullTreeEx`.
pub fn ci_text_to_full_tree_ex() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CITextToFullTreeEx")
}

/// `@ stub CITextToSelectTree`.
pub fn ci_text_to_select_tree() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CITextToSelectTree")
}

/// `@ stub CITextToSelectTreeEx`.
pub fn ci_text_to_select_tree_ex() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CITextToSelectTreeEx")
}

/// `@ stub CiSvcMain`.
pub fn ci_svc_main() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CiSvcMain")
}

/// `@ stub CollectCIISAPIPerformanceData`.
pub fn collect_ciisapi_performance_data() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CollectCIISAPIPerformanceData")
}

/// `@ stub CollectCIPerformanceData`.
pub fn collect_ci_performance_data() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CollectCIPerformanceData")
}

/// `@ stub CollectFILTERPerformanceData`.
pub fn collect_filter_performance_data() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CollectFILTERPerformanceData")
}

/// `@ stub CreateSecurityStore`.
pub fn create_security_store() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "CreateSecurityStore")
}

/// `@ stub DoneCIISAPIPerformanceData`.
pub fn done_ciisapi_performance_data() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "DoneCIISAPIPerformanceData")
}

/// `@ stub DoneCIPerformanceData`.
pub fn done_ci_performance_data() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "DoneCIPerformanceData")
}

/// `@ stub DoneFILTERPerformanceData`.
pub fn done_filter_performance_data() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "DoneFILTERPerformanceData")
}

/// `@ stub EndCacheTransaction`.
pub fn end_cache_transaction() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "EndCacheTransaction")
}

/// `@ stub _ForceMasterMerge@16`.
pub fn force_master_merge() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "_ForceMasterMerge@16")
}

/// `@ stub FsCiShutdown`.
pub fn fs_ci_shutdown() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "FsCiShutdown")
}

/// `@ stub InitializeCIISAPIPerformanceData`.
pub fn initialize_ciisapi_performance_data() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "InitializeCIISAPIPerformanceData")
}

/// `@ stub InitializeCIPerformanceData`.
pub fn initialize_ci_performance_data() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "InitializeCIPerformanceData")
}

/// `@ stub InitializeFILTERPerformanceData`.
pub fn initialize_filter_performance_data() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "InitializeFILTERPerformanceData")
}

/// `@ stub _LoadBHIFilter@16`.
pub fn load_bhi_filter() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "_LoadBHIFilter@16")
}

/// `@ stub LoadBinaryFilter`.
pub fn load_binary_filter() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "LoadBinaryFilter")
}

/// `@ stub LoadTextFilter`.
pub fn load_text_filter() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "LoadTextFilter")
}

/// `@ stub LocateCatalogs`.
pub fn locate_catalogs() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "LocateCatalogs")
}

/// `@ stub SetCatalogState`.
pub fn set_catalog_state() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "SetCatalogState")
}

/// `@ stub SetupCache`.
pub fn setup_cache() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "SetupCache")
}

/// `@ stub SetupCacheEx`.
pub fn setup_cache_ex() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "SetupCacheEx")
}

/// `@ stub _StartFWCiSvcWork@12`.
pub fn start_fw_ci_svc_work() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "_StartFWCiSvcWork@12")
}

/// `@ stub _StopFWCiSvcWork@16`.
pub fn stop_fw_ci_svc_work() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "_StopFWCiSvcWork@16")
}

/// `@ stub SvcEntry_CiSvc`.
pub fn svc_entry_ci_svc() -> StubException {
    wine_spec_unimplemented_stub(QUERY_DLL_NAME, "SvcEntry_CiSvc")
}
