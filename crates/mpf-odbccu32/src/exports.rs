/// What an `odbccu32.spec` line says its export does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `stdcall <name>(...) odbc32.<name>`: the export is forwarded to that DLL, so
    /// `odbccu32` has no code for it.
    Forward(&'static str),
    /// `stub <name>`: the export calls `__wine_spec_unimplemented_stub`, see
    /// [`crate::release_cl_stmt_resources`].
    Stub,
}

/// One line of `odbccu32.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The number at the start of the spec line.
    pub ordinal: u16,
    pub name: &'static str,
    pub kind: ExportKind,
}

/// Every entry of `odbccu32.spec`, in spec order.
pub const EXPORTS: &[Export] = &[
    Export {
        ordinal: 4,
        name: "SQLBindCol",
        kind: ExportKind::Forward("odbc32.SQLBindCol"),
    },
    Export {
        ordinal: 5,
        name: "SQLCancel",
        kind: ExportKind::Forward("odbc32.SQLCancel"),
    },
    Export {
        ordinal: 6,
        name: "ReleaseCLStmtResources",
        kind: ExportKind::Stub,
    },
    Export {
        ordinal: 11,
        name: "SQLExecDirect",
        kind: ExportKind::Forward("odbc32.SQLExecDirect"),
    },
    Export {
        ordinal: 12,
        name: "SQLExecute",
        kind: ExportKind::Forward("odbc32.SQLExecute"),
    },
    Export {
        ordinal: 13,
        name: "SQLFetch",
        kind: ExportKind::Forward("odbc32.SQLFetch"),
    },
    Export {
        ordinal: 16,
        name: "SQLFreeStmt",
        kind: ExportKind::Forward("odbc32.SQLFreeStmt"),
    },
    Export {
        ordinal: 19,
        name: "SQLPrepare",
        kind: ExportKind::Forward("odbc32.SQLPrepare"),
    },
    Export {
        ordinal: 20,
        name: "SQLRowCount",
        kind: ExportKind::Forward("odbc32.SQLRowCount"),
    },
    Export {
        ordinal: 23,
        name: "SQLTransact",
        kind: ExportKind::Forward("odbc32.SQLTransact"),
    },
    Export {
        ordinal: 26,
        name: "SQLCloseCursor",
        kind: ExportKind::Forward("odbc32.SQLCloseCursor"),
    },
    Export {
        ordinal: 29,
        name: "SQLEndTran",
        kind: ExportKind::Forward("odbc32.SQLEndTran"),
    },
    Export {
        ordinal: 30,
        name: "SQLFetchScroll",
        kind: ExportKind::Forward("odbc32.SQLFetchScroll"),
    },
    Export {
        ordinal: 31,
        name: "SQLFreeHandle",
        kind: ExportKind::Forward("odbc32.SQLFreeHandle"),
    },
    Export {
        ordinal: 33,
        name: "SQLGetDescField",
        kind: ExportKind::Forward("odbc32.SQLGetDescField"),
    },
    Export {
        ordinal: 34,
        name: "SQLGetDescRec",
        kind: ExportKind::Forward("odbc32.SQLGetDescRec"),
    },
    Export {
        ordinal: 38,
        name: "SQLGetStmtAttr",
        kind: ExportKind::Forward("odbc32.SQLGetStmtAttr"),
    },
    Export {
        ordinal: 39,
        name: "SQLSetConnectAttr",
        kind: ExportKind::Forward("odbc32.SQLSetConnectAttr"),
    },
    Export {
        ordinal: 43,
        name: "SQLGetData",
        kind: ExportKind::Forward("odbc32.SQLGetData"),
    },
    Export {
        ordinal: 45,
        name: "SQLGetInfo",
        kind: ExportKind::Forward("odbc32.SQLGetInfo"),
    },
    Export {
        ordinal: 46,
        name: "SQLGetStmtOption",
        kind: ExportKind::Forward("odbc32.SQLGetStmtOption"),
    },
    Export {
        ordinal: 48,
        name: "SQLParamData",
        kind: ExportKind::Forward("odbc32.SQLParamData"),
    },
    Export {
        ordinal: 49,
        name: "SQLPutData",
        kind: ExportKind::Forward("odbc32.SQLPutData"),
    },
    Export {
        ordinal: 50,
        name: "SQLSetConnectOption",
        kind: ExportKind::Forward("odbc32.SQLSetConnectOption"),
    },
    Export {
        ordinal: 51,
        name: "SQLSetStmtOption",
        kind: ExportKind::Forward("odbc32.SQLSetStmtOption"),
    },
    Export {
        ordinal: 59,
        name: "SQLExtendedFetch",
        kind: ExportKind::Forward("odbc32.SQLExtendedFetch"),
    },
    Export {
        ordinal: 61,
        name: "SQLMoreResults",
        kind: ExportKind::Forward("odbc32.SQLMoreResults"),
    },
    Export {
        ordinal: 62,
        name: "SQLNativeSql",
        kind: ExportKind::Forward("odbc32.SQLNativeSql"),
    },
    Export {
        ordinal: 63,
        name: "SQLNumParams",
        kind: ExportKind::Forward("odbc32.SQLNumParams"),
    },
    Export {
        ordinal: 64,
        name: "SQLParamOptions",
        kind: ExportKind::Forward("odbc32.SQLParamOptions"),
    },
    Export {
        ordinal: 68,
        name: "SQLSetPos",
        kind: ExportKind::Forward("odbc32.SQLSetPos"),
    },
    Export {
        ordinal: 69,
        name: "SQLSetScrollOptions",
        kind: ExportKind::Forward("odbc32.SQLSetScrollOptions"),
    },
    Export {
        ordinal: 72,
        name: "SQLBindParameter",
        kind: ExportKind::Forward("odbc32.SQLBindParameter"),
    },
    Export {
        ordinal: 73,
        name: "SQLSetDescField",
        kind: ExportKind::Forward("odbc32.SQLSetDescField"),
    },
    Export {
        ordinal: 74,
        name: "SQLSetDescRec",
        kind: ExportKind::Forward("odbc32.SQLSetDescRec"),
    },
    Export {
        ordinal: 76,
        name: "SQLSetStmtAttr",
        kind: ExportKind::Forward("odbc32.SQLSetStmtAttr"),
    },
    Export {
        ordinal: 78,
        name: "SQLBulkOperations",
        kind: ExportKind::Forward("odbc32.SQLBulkOperations"),
    },
];

/// The entry named `name`, or `None` if `odbccu32` does not export it. Matching is case-sensitive.
pub fn find_export(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}
