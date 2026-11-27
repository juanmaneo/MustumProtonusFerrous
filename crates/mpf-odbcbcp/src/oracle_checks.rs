//! Oracle-derived test vectors for the odbcbcp export stubs.
//!
//! Vectors were captured by compiling the verbatim `__wine_spec_unimplemented_stub` body
//! (`dlls/ntdll/exception.c` lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/odbcbcp/`. The harness called it once for each of the 28 `@ stub` names in
//! `dlls/odbcbcp/odbcbcp.spec`, with `RtlRaiseException` replaced by a recorder that longjmps back
//! to the harness. All 28 rows ran clean under the sanitizers, so none are excluded.

use super::*;

/// The 28 rows the C harness printed, in spec order: export name, `code`, `flags`,
/// `NumberParameters`, `ExceptionInformation[0]` (module) and `ExceptionInformation[1]`
/// (function). Every run raised the record exactly once (`raised=1`).
const ORACLE_ROWS: [(&str, u32, u32, u32, &str, &str); 28] = [
    (
        "bcp_batch",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_batch",
    ),
    ("bcp_bind", 0x8000_0100, 0x01, 2, "odbcbcp.dll", "bcp_bind"),
    (
        "bcp_colfmt",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_colfmt",
    ),
    (
        "bcp_collen",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_collen",
    ),
    (
        "bcp_colptr",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_colptr",
    ),
    (
        "bcp_columns",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_columns",
    ),
    (
        "bcp_control",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_control",
    ),
    ("bcp_done", 0x8000_0100, 0x01, 2, "odbcbcp.dll", "bcp_done"),
    ("bcp_exec", 0x8000_0100, 0x01, 2, "odbcbcp.dll", "bcp_exec"),
    (
        "bcp_getcolfmt",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_getcolfmt",
    ),
    (
        "bcp_initA",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_initA",
    ),
    (
        "bcp_initW",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_initW",
    ),
    (
        "bcp_moretext",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_moretext",
    ),
    (
        "bcp_readfmtA",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_readfmtA",
    ),
    (
        "bcp_readfmtW",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_readfmtW",
    ),
    (
        "bcp_sendrow",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_sendrow",
    ),
    (
        "bcp_setcolfmt",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_setcolfmt",
    ),
    (
        "bcp_writefmtA",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_writefmtA",
    ),
    (
        "bcp_writefmtW",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "bcp_writefmtW",
    ),
    (
        "dbprtypeA",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "dbprtypeA",
    ),
    (
        "dbprtypeW",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "dbprtypeW",
    ),
    ("LibMain", 0x8000_0100, 0x01, 2, "odbcbcp.dll", "LibMain"),
    (
        "SQLCloseEnumServers",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "SQLCloseEnumServers",
    ),
    (
        "SQLGetNextEnumeration",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "SQLGetNextEnumeration",
    ),
    (
        "SQLInitEnumServers",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "SQLInitEnumServers",
    ),
    (
        "SQLLinkedCatalogsA",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "SQLLinkedCatalogsA",
    ),
    (
        "SQLLinkedCatalogsW",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "SQLLinkedCatalogsW",
    ),
    (
        "SQLLinkedServers",
        0x8000_0100,
        0x01,
        2,
        "odbcbcp.dll",
        "SQLLinkedServers",
    ),
];

#[test]
fn oracle_stub_rows() {
    for (row, &(export, helper)) in ORACLE_ROWS.iter().zip(EXPORTS.iter()) {
        let (name, code, flags, number_parameters, module, function) = *row;
        let expected = StubException {
            code,
            flags,
            number_parameters,
            module,
            function,
        };
        assert_eq!(name, export);
        assert_eq!(helper(), expected);
        assert_eq!(export_stub(name), Some(expected));
    }
}
