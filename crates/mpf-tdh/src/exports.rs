/// How `dlls/tdh/tdh.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall` with a real port: [`crate::tdh_enumerate_providers`].
    Ported,
    /// `@ stdcall` whose C body returns `STATUS_SUCCESS`: [`crate::tdh_load_manifest`] and
    /// [`crate::tdh_load_manifest_from_binary`].
    StatusSuccess,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `tdh.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 34] = [
    ("TdhAggregatePayloadFilters", ExportKind::Stub),
    ("TdhApplyPayloadFilter", ExportKind::Stub),
    ("TdhCleanupPayloadEventFilterDescriptor", ExportKind::Stub),
    ("TdhCloseDecodingHandle", ExportKind::Stub),
    ("TdhCreatePayloadFilter", ExportKind::Stub),
    ("TdhDeletePayloadFilter", ExportKind::Stub),
    ("TdhEnumerateManifestProviderEvents", ExportKind::Stub),
    ("TdhEnumerateProviderFieldInformation", ExportKind::Stub),
    ("TdhEnumerateProviderFilters", ExportKind::Stub),
    ("TdhEnumerateProviders", ExportKind::Ported),
    (
        "TdhEnumerateRemoteWBEMProviderFieldInformation",
        ExportKind::Stub,
    ),
    ("TdhEnumerateRemoteWBEMProviders", ExportKind::Stub),
    ("TdhFormatProperty", ExportKind::Stub),
    ("TdhGetAllEventsInformation", ExportKind::Stub),
    ("TdhGetDecodingParameter", ExportKind::Stub),
    ("TdhGetEventInformation", ExportKind::Stub),
    ("TdhGetEventMapInformation", ExportKind::Stub),
    ("TdhGetManifestEventInformation", ExportKind::Stub),
    ("TdhGetProperty", ExportKind::Stub),
    ("TdhGetPropertyOffsetAndSize", ExportKind::Stub),
    ("TdhGetPropertySize", ExportKind::Stub),
    ("TdhGetWppMessage", ExportKind::Stub),
    ("TdhGetWppProperty", ExportKind::Stub),
    ("TdhLoadManifest", ExportKind::StatusSuccess),
    ("TdhLoadManifestFromBinary", ExportKind::StatusSuccess),
    ("TdhOpenDecodingHandle", ExportKind::Stub),
    ("TdhQueryProviderFieldInformation", ExportKind::Stub),
    (
        "TdhQueryRemoteWBEMProviderFieldInformation",
        ExportKind::Stub,
    ),
    ("TdhSetDecodingParameter", ExportKind::Stub),
    ("TdhUnloadManifest", ExportKind::Stub),
    ("TdhValidatePayloadFilter", ExportKind::Stub),
    ("TdhpFindMatchClassFromWBEM", ExportKind::Stub),
    ("TdhpGetBestTraceEventInfoWBEM", ExportKind::Stub),
    ("TdhpGetEventMapInfoWBEM", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
