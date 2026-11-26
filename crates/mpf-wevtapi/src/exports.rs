/// How `dlls/wevtapi/wevtapi.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported in the modules of this crate.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `wevtapi.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 46] = [
    ("EvtArchiveExportedLog", ExportKind::Stub),
    ("EvtCancel", ExportKind::Stub),
    ("EvtClearLog", ExportKind::Stub),
    ("EvtClose", ExportKind::Stdcall),
    ("EvtCreateBookmark", ExportKind::Stdcall),
    ("EvtCreateRenderContext", ExportKind::Stdcall),
    ("EvtExportLog", ExportKind::Stdcall),
    ("EvtFormatMessage", ExportKind::Stub),
    ("EvtGetChannelConfigProperty", ExportKind::Stdcall),
    ("EvtGetEventInfo", ExportKind::Stub),
    ("EvtGetEventMetadataProperty", ExportKind::Stub),
    ("EvtGetExtendedStatus", ExportKind::Stub),
    ("EvtGetLogInfo", ExportKind::Stub),
    ("EvtGetObjectArrayProperty", ExportKind::Stub),
    ("EvtGetObjectArraySize", ExportKind::Stub),
    ("EvtGetPublisherMetadataProperty", ExportKind::Stub),
    ("EvtGetQueryInfo", ExportKind::Stub),
    ("EvtIntAssertConfig", ExportKind::Stub),
    ("EvtIntCreateBinXMLFromCustomXML", ExportKind::Stub),
    ("EvtIntCreateLocalLogfile", ExportKind::Stub),
    ("EvtIntGetClassicLogDisplayName", ExportKind::Stub),
    ("EvtIntRenderResourceEventTemplate", ExportKind::Stub),
    ("EvtIntReportAuthzEventAndSourceAsync", ExportKind::Stub),
    ("EvtIntReportEventAndSourceAsync", ExportKind::Stub),
    ("EvtIntRetractConfig", ExportKind::Stub),
    ("EvtIntSysprepCleanup", ExportKind::Stub),
    ("EvtIntWriteXmlEventToLocalLogfile", ExportKind::Stub),
    ("EvtNext", ExportKind::Stdcall),
    ("EvtNextChannelPath", ExportKind::Stdcall),
    ("EvtNextEventMetadata", ExportKind::Stub),
    ("EvtNextPublisherId", ExportKind::Stub),
    ("EvtOpenChannelConfig", ExportKind::Stdcall),
    ("EvtOpenChannelEnum", ExportKind::Stdcall),
    ("EvtOpenEventMetadataEnum", ExportKind::Stub),
    ("EvtOpenLog", ExportKind::Stdcall),
    ("EvtOpenPublisherEnum", ExportKind::Stub),
    ("EvtOpenPublisherMetadata", ExportKind::Stub),
    ("EvtOpenSession", ExportKind::Stdcall),
    ("EvtQuery", ExportKind::Stdcall),
    ("EvtRender", ExportKind::Stdcall),
    ("EvtSaveChannelConfig", ExportKind::Stdcall),
    ("EvtSeek", ExportKind::Stub),
    ("EvtSetChannelConfigProperty", ExportKind::Stdcall),
    ("EvtSetObjectArrayProperty", ExportKind::Stub),
    ("EvtSubscribe", ExportKind::Stdcall),
    ("EvtUpdateBookmark", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
