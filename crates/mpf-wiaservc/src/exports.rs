//! The export table of `dlls/wiaservc/wiaservc.spec`.

/// How `wiaservc.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall -private`: a function in the C sources.
    Private,
    /// `@ stub`: the name raises the `__wine_spec_unimplemented_stub` record.
    Stub,
}

/// The exports of `dlls/wiaservc/wiaservc.spec`, in spec order.
pub const EXPORTS: [(&str, ExportKind); 57] = [
    ("ServiceMain", ExportKind::Private),
    ("DllRegisterServer", ExportKind::Private),
    ("DllUnregisterServer", ExportKind::Private),
    ("wiasCreateChildAppItem", ExportKind::Stub),
    ("wiasCreateDrvItem", ExportKind::Stub),
    ("wiasCreateLogInstance", ExportKind::Stub),
    ("wiasCreatePropContext", ExportKind::Stub),
    ("wiasDebugError", ExportKind::Stub),
    ("wiasDebugTrace", ExportKind::Stub),
    ("wiasDownSampleBuffer", ExportKind::Stub),
    ("wiasFormatArgs", ExportKind::Stub),
    ("wiasFreePropContext", ExportKind::Stub),
    ("wiasGetChangedValueFloat", ExportKind::Stub),
    ("wiasGetChangedValueGuid", ExportKind::Stub),
    ("wiasGetChangedValueLong", ExportKind::Stub),
    ("wiasGetChangedValueStr", ExportKind::Stub),
    ("wiasGetChildrenContexts", ExportKind::Stub),
    ("wiasGetContextFromName", ExportKind::Stub),
    ("wiasGetDrvItem", ExportKind::Stub),
    ("wiasGetImageInformation", ExportKind::Stub),
    ("wiasGetItemType", ExportKind::Stub),
    ("wiasGetPropertyAttributes", ExportKind::Stub),
    ("wiasGetRootItem", ExportKind::Stub),
    ("wiasIsPropChanged", ExportKind::Stub),
    ("wiasParseEndorserString", ExportKind::Stub),
    ("wiasPrintDebugHResult", ExportKind::Stub),
    ("wiasQueueEvent", ExportKind::Stub),
    ("wiasReadMultiple", ExportKind::Stub),
    ("wiasReadPropBin", ExportKind::Stub),
    ("wiasReadPropFloat", ExportKind::Stub),
    ("wiasReadPropGuid", ExportKind::Stub),
    ("wiasReadPropLong", ExportKind::Stub),
    ("wiasReadPropStr", ExportKind::Stub),
    ("wiasSendEndOfPage", ExportKind::Stub),
    ("wiasSetItemPropAttribs", ExportKind::Stub),
    ("wiasSetItemPropNames", ExportKind::Stub),
    ("wiasSetPropChanged", ExportKind::Stub),
    ("wiasSetPropertyAttributes", ExportKind::Stub),
    ("wiasSetValidFlag", ExportKind::Stub),
    ("wiasSetValidListFloat", ExportKind::Stub),
    ("wiasSetValidListGuid", ExportKind::Stub),
    ("wiasSetValidListLong", ExportKind::Stub),
    ("wiasSetValidListStr", ExportKind::Stub),
    ("wiasSetValidRangeFloat", ExportKind::Stub),
    ("wiasSetValidRangeLong", ExportKind::Stub),
    ("wiasUpdateScanRect", ExportKind::Stub),
    ("wiasUpdateValidFormat", ExportKind::Stub),
    ("wiasValidateItemProperties", ExportKind::Stub),
    ("wiasWriteBufToFile", ExportKind::Stub),
    ("wiasWriteMultiple", ExportKind::Stub),
    ("wiasWritePageBufToFile", ExportKind::Stub),
    ("wiasWritePageBufToStream", ExportKind::Stub),
    ("wiasWritePropBin", ExportKind::Stub),
    ("wiasWritePropFloat", ExportKind::Stub),
    ("wiasWritePropGuid", ExportKind::Stub),
    ("wiasWritePropLong", ExportKind::Stub),
    ("wiasWritePropStr", ExportKind::Stub),
];

/// The kind of the export named `name`, or `None` if `wiaservc.spec` does not export it.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(export, _)| *export == name)
        .map(|(_, kind)| *kind)
}
