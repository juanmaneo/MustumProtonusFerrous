/// How `dlls/url/url.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall` with a C body: one of the explicit functions in [`crate::handlers`], or
    /// [`crate::file_protocol_handler_a_target`] for `FileProtocolHandlerA`.
    Stdcall,
    /// `@ stdcall NAME(...) TARGET`: the same function as `TARGET`.
    Alias(&'static str),
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `url.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 21] = [
    ("AddMIMEFileTypesPS", ExportKind::Stdcall),
    ("AutodialHookCallback", ExportKind::Stub),
    ("DummyEntryPoint", ExportKind::Stub),
    ("DummyEntryPointA", ExportKind::Stub),
    (
        "FileProtocolHandler",
        ExportKind::Alias("FileProtocolHandlerA"),
    ),
    ("FileProtocolHandlerA", ExportKind::Stdcall),
    ("InetIsOffline", ExportKind::Stdcall),
    ("MIMEAssociationDialogA", ExportKind::Stub),
    ("MIMEAssociationDialogW", ExportKind::Stub),
    ("MailToProtocolHandler", ExportKind::Stub),
    ("MailToProtocolHandlerA", ExportKind::Stub),
    ("NewsProtocolHandler", ExportKind::Stub),
    ("NewsProtocolHandlerA", ExportKind::Stub),
    ("OpenURL", ExportKind::Alias("OpenURLA")),
    ("OpenURLA", ExportKind::Stdcall),
    (
        "TelnetProtocolHandler",
        ExportKind::Alias("TelnetProtocolHandlerA"),
    ),
    ("TelnetProtocolHandlerA", ExportKind::Stdcall),
    ("TranslateURLA", ExportKind::Stub),
    ("TranslateURLW", ExportKind::Stub),
    ("URLAssociationDialogA", ExportKind::Stub),
    ("URLAssociationDialogW", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
