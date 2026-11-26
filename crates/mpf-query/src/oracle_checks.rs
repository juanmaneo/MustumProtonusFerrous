//! Oracle-derived test vectors for the ported mpf-query logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`, `CIState`,
//! `LocateCatalogsA`, `LocateCatalogsW` and `LoadIFilter` (`dlls/query/query_main.c`) and of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/query/`. The harness printed 49 rows:
//! rows 01 to 11 for the explicit functions, and rows 12 to 49 for the 38 `@ stub` exports in spec
//! order. The verbatim stub loops on `RtlRaiseException`, so the harness replaces that function
//! with a recorder that longjmps back to the harness. ASan and UBSan reported nothing, so no row
//! was excluded.

use super::*;

/// One row of the stub oracle: the helper, then the code, flags, number of parameters, module and
/// function of the `StubException` that the C function raised.
type StubRow = (
    fn() -> StubException,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

/// The rows for each `@ stub` export, in spec order. These are rows 12 to 49 of the oracle.
pub(crate) const STUB_ROWS: [StubRow; 38] = [
    (
        abort_merges,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "_AbortMerges@16",
    ),
    (
        begin_cache_transaction,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "BeginCacheTransaction",
    ),
    (
        bind_i_filter_from_storage,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "BindIFilterFromStorage",
    ),
    (
        bind_i_filter_from_stream,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "BindIFilterFromStream",
    ),
    (
        ci_build_query_node,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CIBuildQueryNode",
    ),
    (
        ci_build_query_tree,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CIBuildQueryTree",
    ),
    (
        ci_create_command,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CICreateCommand",
    ),
    (
        ci_get_global_property_list,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CIGetGlobalPropertyList",
    ),
    (
        ci_make_i_command,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CIMakeICommand",
    ),
    (
        ci_restriction_to_full_tree,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CIRestrictionToFullTree",
    ),
    (
        ci_text_to_full_tree,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CITextToFullTree",
    ),
    (
        ci_text_to_full_tree_ex,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CITextToFullTreeEx",
    ),
    (
        ci_text_to_select_tree,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CITextToSelectTree",
    ),
    (
        ci_text_to_select_tree_ex,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CITextToSelectTreeEx",
    ),
    (ci_svc_main, 0x8000_0100, 0x1, 2, "query.dll", "CiSvcMain"),
    (
        collect_ciisapi_performance_data,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CollectCIISAPIPerformanceData",
    ),
    (
        collect_ci_performance_data,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CollectCIPerformanceData",
    ),
    (
        collect_filter_performance_data,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CollectFILTERPerformanceData",
    ),
    (
        create_security_store,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "CreateSecurityStore",
    ),
    (
        done_ciisapi_performance_data,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "DoneCIISAPIPerformanceData",
    ),
    (
        done_ci_performance_data,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "DoneCIPerformanceData",
    ),
    (
        done_filter_performance_data,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "DoneFILTERPerformanceData",
    ),
    (
        end_cache_transaction,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "EndCacheTransaction",
    ),
    (
        force_master_merge,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "_ForceMasterMerge@16",
    ),
    (
        fs_ci_shutdown,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "FsCiShutdown",
    ),
    (
        initialize_ciisapi_performance_data,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "InitializeCIISAPIPerformanceData",
    ),
    (
        initialize_ci_performance_data,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "InitializeCIPerformanceData",
    ),
    (
        initialize_filter_performance_data,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "InitializeFILTERPerformanceData",
    ),
    (
        load_bhi_filter,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "_LoadBHIFilter@16",
    ),
    (
        load_binary_filter,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "LoadBinaryFilter",
    ),
    (
        load_text_filter,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "LoadTextFilter",
    ),
    (
        locate_catalogs,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "LocateCatalogs",
    ),
    (
        set_catalog_state,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "SetCatalogState",
    ),
    (setup_cache, 0x8000_0100, 0x1, 2, "query.dll", "SetupCache"),
    (
        setup_cache_ex,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "SetupCacheEx",
    ),
    (
        start_fw_ci_svc_work,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "_StartFWCiSvcWork@12",
    ),
    (
        stop_fw_ci_svc_work,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "_StopFWCiSvcWork@16",
    ),
    (
        svc_entry_ci_svc,
        0x8000_0100,
        0x1,
        2,
        "query.dll",
        "SvcEntry_CiSvc",
    ),
];

#[test]
fn oracle_stub_rows() {
    for (helper, code, flags, number_parameters, module, function) in STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // Row 01: DllGetClassObject(NULL, NULL, NULL) -> 0x80040111.
    // Row 02: DllGetClassObject(&clsid, &iid, &ppv) -> 0x80040111, ppv stays NULL.
    assert_eq!(dll_get_class_object() as u32, 0x8004_0111);
}

#[test]
fn oracle_ci_state_rows() {
    // Row 03: CIState(NULL, NULL, NULL) -> 0x80041820.
    // Row 04: CIState(cat, machine, &state) -> 0x80041820.
    assert_eq!(ci_state() as u32, 0x8004_1820);
}

#[test]
fn oracle_locate_catalogs_rows() {
    // Rows 05 and 06: LocateCatalogsA with NULL, then with a scope and buffers -> 0x80041820.
    assert_eq!(locate_catalogs_a() as u32, 0x8004_1820);
    // Rows 07 and 08: LocateCatalogsW with NULL, then with a scope and buffers -> 0x80041820.
    assert_eq!(locate_catalogs_w() as u32, 0x8004_1820);
}

#[test]
fn oracle_load_ifilter_rows() {
    // Rows 09 and 10: *ppIUnk starts non-NULL, the call returns 0x80004001 and leaves it NULL.
    let mut out = Some(1_u8);
    assert_eq!(load_ifilter(&mut out) as u32, 0x8000_4001);
    assert_eq!(out, None);
    let mut out = Some(1_u8);
    assert_eq!(load_ifilter(&mut out) as u32, 0x8000_4001);
    assert_eq!(out, None);
    // Row 11: *ppIUnk starts NULL, the call returns 0x80004001 and leaves it NULL.
    let mut out: Option<u8> = None;
    assert_eq!(load_ifilter(&mut out) as u32, 0x8000_4001);
    assert_eq!(out, None);
}
