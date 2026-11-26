/// How `dlls/netutils/netutils.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported under its snake_case name.
    Stdcall,
    /// `@ stdcall NAME TARGET`: the same function as `TARGET`.
    Alias(&'static str),
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `netutils.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 22] = [
    ("NetApiBufferAllocate", ExportKind::Stdcall),
    ("NetApiBufferFree", ExportKind::Stdcall),
    ("NetApiBufferReallocate", ExportKind::Stdcall),
    ("NetApiBufferSize", ExportKind::Stdcall),
    ("NetRemoteComputerSupports", ExportKind::Stub),
    (
        "NetapipBufferAllocate",
        ExportKind::Alias("NetApiBufferAllocate"),
    ),
    ("NetpIsComputerNameValid", ExportKind::Stub),
    ("NetpIsDomainNameValid", ExportKind::Stub),
    ("NetpIsGroupNameValid", ExportKind::Stub),
    ("NetpIsRemote", ExportKind::Stub),
    ("NetpIsRemoteNameValid", ExportKind::Stub),
    ("NetpIsShareNameValid", ExportKind::Stub),
    ("NetpIsUncComputerNameValid", ExportKind::Stub),
    ("NetpIsUserNameValid", ExportKind::Stub),
    ("NetpwListCanonicalize", ExportKind::Stub),
    ("NetpwListTraverse", ExportKind::Stub),
    ("NetpwNameCanonicalize", ExportKind::Stub),
    ("NetpwNameCompare", ExportKind::Stub),
    ("NetpwNameValidate", ExportKind::Stub),
    ("NetpwPathCanonicalize", ExportKind::Stub),
    ("NetpwPathCompare", ExportKind::Stub),
    ("NetpwPathType", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
