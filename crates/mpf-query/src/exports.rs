/// How `dlls/query/query.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    /// Each one has a helper in `stub_exports.rs`.
    Stub,
    /// `@ stdcall` with a C body in `query_main.c`, ported as a function in `query_main.rs`.
    Ported,
    /// `@ stdcall` with no C body in `query_main.c`, so there is nothing to port.
    NoBody,
}

/// The `query.spec` export table, in spec order. Names are the spec names, with any `@N` suffix.
pub const EXPORTS: [(&str, ExportKind); 46] = [
    ("_AbortMerges@16", ExportKind::Stub),
    ("BeginCacheTransaction", ExportKind::Stub),
    ("BindIFilterFromStorage", ExportKind::Stub),
    ("BindIFilterFromStream", ExportKind::Stub),
    ("CIBuildQueryNode", ExportKind::Stub),
    ("CIBuildQueryTree", ExportKind::Stub),
    ("CICreateCommand", ExportKind::Stub),
    ("CIGetGlobalPropertyList", ExportKind::Stub),
    ("CIMakeICommand", ExportKind::Stub),
    ("CIRestrictionToFullTree", ExportKind::Stub),
    ("CIState", ExportKind::Ported),
    ("CITextToFullTree", ExportKind::Stub),
    ("CITextToFullTreeEx", ExportKind::Stub),
    ("CITextToSelectTree", ExportKind::Stub),
    ("CITextToSelectTreeEx", ExportKind::Stub),
    ("CiSvcMain", ExportKind::Stub),
    ("CollectCIISAPIPerformanceData", ExportKind::Stub),
    ("CollectCIPerformanceData", ExportKind::Stub),
    ("CollectFILTERPerformanceData", ExportKind::Stub),
    ("CreateSecurityStore", ExportKind::Stub),
    ("DllCanUnloadNow", ExportKind::NoBody),
    ("DllGetClassObject", ExportKind::Ported),
    ("DllRegisterServer", ExportKind::NoBody),
    ("DllUnregisterServer", ExportKind::NoBody),
    ("DoneCIISAPIPerformanceData", ExportKind::Stub),
    ("DoneCIPerformanceData", ExportKind::Stub),
    ("DoneFILTERPerformanceData", ExportKind::Stub),
    ("EndCacheTransaction", ExportKind::Stub),
    ("_ForceMasterMerge@16", ExportKind::Stub),
    ("FsCiShutdown", ExportKind::Stub),
    ("InitializeCIISAPIPerformanceData", ExportKind::Stub),
    ("InitializeCIPerformanceData", ExportKind::Stub),
    ("InitializeFILTERPerformanceData", ExportKind::Stub),
    ("_LoadBHIFilter@16", ExportKind::Stub),
    ("LoadBinaryFilter", ExportKind::Stub),
    ("LoadIFilter", ExportKind::Ported),
    ("LoadTextFilter", ExportKind::Stub),
    ("LocateCatalogs", ExportKind::Stub),
    ("LocateCatalogsA", ExportKind::Ported),
    ("LocateCatalogsW", ExportKind::Ported),
    ("SetCatalogState", ExportKind::Stub),
    ("SetupCache", ExportKind::Stub),
    ("SetupCacheEx", ExportKind::Stub),
    ("_StartFWCiSvcWork@12", ExportKind::Stub),
    ("_StopFWCiSvcWork@16", ExportKind::Stub),
    ("SvcEntry_CiSvc", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
