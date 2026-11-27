//! Oracle-derived test vectors for the ported odbccu32 export table and stub.
//!
//! `dlls/odbccu32` has no C sources, so the export rows come from the reference tool. Wine's
//! `tools/winebuild` was built from the reference checkout with a minimal `config.h`, then ran as
//! `winebuild -b x86_64-windows --def -F odbccu32.dll -E odbccu32.spec`. The harness
//! `target/canon/odbccu32/harness.c`, built with `gcc -fsanitize=address,undefined -g`, reads that
//! output and prints one row per export. The stub row calls the verbatim
//! `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`, with `RtlRaiseException`
//! replaced by a recorder that longjmps back to the harness, because the verbatim loop never
//! returns. ASan and UBSan reported nothing, so no row is excluded. The harness printed 37 rows
//! (R01-R37).

use super::*;

/// `(row, export name, ordinal, kind)` for each of the 37 export lines of `odbccu32.spec`, in spec order.
const EXPORT_ROWS: [(&str, &str, u16, ExportKind); 37] = [
    (
        "R01",
        "SQLBindCol",
        4,
        ExportKind::Forward("odbc32.SQLBindCol"),
    ),
    (
        "R02",
        "SQLCancel",
        5,
        ExportKind::Forward("odbc32.SQLCancel"),
    ),
    ("R03", "ReleaseCLStmtResources", 6, ExportKind::Stub),
    (
        "R04",
        "SQLExecDirect",
        11,
        ExportKind::Forward("odbc32.SQLExecDirect"),
    ),
    (
        "R05",
        "SQLExecute",
        12,
        ExportKind::Forward("odbc32.SQLExecute"),
    ),
    (
        "R06",
        "SQLFetch",
        13,
        ExportKind::Forward("odbc32.SQLFetch"),
    ),
    (
        "R07",
        "SQLFreeStmt",
        16,
        ExportKind::Forward("odbc32.SQLFreeStmt"),
    ),
    (
        "R08",
        "SQLPrepare",
        19,
        ExportKind::Forward("odbc32.SQLPrepare"),
    ),
    (
        "R09",
        "SQLRowCount",
        20,
        ExportKind::Forward("odbc32.SQLRowCount"),
    ),
    (
        "R10",
        "SQLTransact",
        23,
        ExportKind::Forward("odbc32.SQLTransact"),
    ),
    (
        "R11",
        "SQLCloseCursor",
        26,
        ExportKind::Forward("odbc32.SQLCloseCursor"),
    ),
    (
        "R12",
        "SQLEndTran",
        29,
        ExportKind::Forward("odbc32.SQLEndTran"),
    ),
    (
        "R13",
        "SQLFetchScroll",
        30,
        ExportKind::Forward("odbc32.SQLFetchScroll"),
    ),
    (
        "R14",
        "SQLFreeHandle",
        31,
        ExportKind::Forward("odbc32.SQLFreeHandle"),
    ),
    (
        "R15",
        "SQLGetDescField",
        33,
        ExportKind::Forward("odbc32.SQLGetDescField"),
    ),
    (
        "R16",
        "SQLGetDescRec",
        34,
        ExportKind::Forward("odbc32.SQLGetDescRec"),
    ),
    (
        "R17",
        "SQLGetStmtAttr",
        38,
        ExportKind::Forward("odbc32.SQLGetStmtAttr"),
    ),
    (
        "R18",
        "SQLSetConnectAttr",
        39,
        ExportKind::Forward("odbc32.SQLSetConnectAttr"),
    ),
    (
        "R19",
        "SQLGetData",
        43,
        ExportKind::Forward("odbc32.SQLGetData"),
    ),
    (
        "R20",
        "SQLGetInfo",
        45,
        ExportKind::Forward("odbc32.SQLGetInfo"),
    ),
    (
        "R21",
        "SQLGetStmtOption",
        46,
        ExportKind::Forward("odbc32.SQLGetStmtOption"),
    ),
    (
        "R22",
        "SQLParamData",
        48,
        ExportKind::Forward("odbc32.SQLParamData"),
    ),
    (
        "R23",
        "SQLPutData",
        49,
        ExportKind::Forward("odbc32.SQLPutData"),
    ),
    (
        "R24",
        "SQLSetConnectOption",
        50,
        ExportKind::Forward("odbc32.SQLSetConnectOption"),
    ),
    (
        "R25",
        "SQLSetStmtOption",
        51,
        ExportKind::Forward("odbc32.SQLSetStmtOption"),
    ),
    (
        "R26",
        "SQLExtendedFetch",
        59,
        ExportKind::Forward("odbc32.SQLExtendedFetch"),
    ),
    (
        "R27",
        "SQLMoreResults",
        61,
        ExportKind::Forward("odbc32.SQLMoreResults"),
    ),
    (
        "R28",
        "SQLNativeSql",
        62,
        ExportKind::Forward("odbc32.SQLNativeSql"),
    ),
    (
        "R29",
        "SQLNumParams",
        63,
        ExportKind::Forward("odbc32.SQLNumParams"),
    ),
    (
        "R30",
        "SQLParamOptions",
        64,
        ExportKind::Forward("odbc32.SQLParamOptions"),
    ),
    (
        "R31",
        "SQLSetPos",
        68,
        ExportKind::Forward("odbc32.SQLSetPos"),
    ),
    (
        "R32",
        "SQLSetScrollOptions",
        69,
        ExportKind::Forward("odbc32.SQLSetScrollOptions"),
    ),
    (
        "R33",
        "SQLBindParameter",
        72,
        ExportKind::Forward("odbc32.SQLBindParameter"),
    ),
    (
        "R34",
        "SQLSetDescField",
        73,
        ExportKind::Forward("odbc32.SQLSetDescField"),
    ),
    (
        "R35",
        "SQLSetDescRec",
        74,
        ExportKind::Forward("odbc32.SQLSetDescRec"),
    ),
    (
        "R36",
        "SQLSetStmtAttr",
        76,
        ExportKind::Forward("odbc32.SQLSetStmtAttr"),
    ),
    (
        "R37",
        "SQLBulkOperations",
        78,
        ExportKind::Forward("odbc32.SQLBulkOperations"),
    ),
];

#[test]
fn oracle_export_rows() {
    assert_eq!(EXPORTS.len(), EXPORT_ROWS.len());
    for (export, (row, name, ordinal, kind)) in EXPORTS.iter().zip(EXPORT_ROWS) {
        assert_eq!(
            *export,
            Export {
                ordinal,
                name,
                kind
            },
            "{row}"
        );
    }
}

#[test]
fn oracle_release_cl_stmt_resources_row() {
    // row R03 ReleaseCLStmtResources @6 stub -> code=0x80000100 flags=0x01 nparams=2
    // module=odbccu32.dll function=ReleaseCLStmtResources
    assert_eq!(
        release_cl_stmt_resources(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "odbccu32.dll",
            function: "ReleaseCLStmtResources",
        }
    );
}
