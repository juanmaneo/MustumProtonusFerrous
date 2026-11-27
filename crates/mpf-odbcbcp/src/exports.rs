use crate::stub::{wine_spec_unimplemented_stub, StubException};

/// `MODULE = odbcbcp.dll` in `dlls/odbcbcp/Makefile.in`. winebuild passes it as the module of every stub.
const ODBCBCP_DLL_NAME: &str = "odbcbcp.dll";

/// `bcp_batch`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_batch() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_batch")
}

/// `bcp_bind`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_bind() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_bind")
}

/// `bcp_colfmt`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_colfmt() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_colfmt")
}

/// `bcp_collen`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_collen() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_collen")
}

/// `bcp_colptr`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_colptr() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_colptr")
}

/// `bcp_columns`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_columns() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_columns")
}

/// `bcp_control`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_control() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_control")
}

/// `bcp_done`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_done() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_done")
}

/// `bcp_exec`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_exec() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_exec")
}

/// `bcp_getcolfmt`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_getcolfmt() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_getcolfmt")
}

/// `bcp_initA`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_init_a() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_initA")
}

/// `bcp_initW`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_init_w() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_initW")
}

/// `bcp_moretext`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_moretext() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_moretext")
}

/// `bcp_readfmtA`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_readfmt_a() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_readfmtA")
}

/// `bcp_readfmtW`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_readfmt_w() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_readfmtW")
}

/// `bcp_sendrow`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_sendrow() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_sendrow")
}

/// `bcp_setcolfmt`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_setcolfmt() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_setcolfmt")
}

/// `bcp_writefmtA`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_writefmt_a() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_writefmtA")
}

/// `bcp_writefmtW`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn bcp_writefmt_w() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "bcp_writefmtW")
}

/// `dbprtypeA`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn dbprtype_a() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "dbprtypeA")
}

/// `dbprtypeW`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn dbprtype_w() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "dbprtypeW")
}

/// `LibMain`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn lib_main() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "LibMain")
}

/// `SQLCloseEnumServers`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn sql_close_enum_servers() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "SQLCloseEnumServers")
}

/// `SQLGetNextEnumeration`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn sql_get_next_enumeration() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "SQLGetNextEnumeration")
}

/// `SQLInitEnumServers`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn sql_init_enum_servers() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "SQLInitEnumServers")
}

/// `SQLLinkedCatalogsA`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn sql_linked_catalogs_a() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "SQLLinkedCatalogsA")
}

/// `SQLLinkedCatalogsW`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn sql_linked_catalogs_w() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "SQLLinkedCatalogsW")
}

/// `SQLLinkedServers`, exported as `@ stub` in `dlls/odbcbcp/odbcbcp.spec`.
pub fn sql_linked_servers() -> StubException {
    wine_spec_unimplemented_stub(ODBCBCP_DLL_NAME, "SQLLinkedServers")
}

/// The signature of each helper in [`EXPORTS`].
type ExportFn = fn() -> StubException;

/// The `odbcbcp.spec` export table, in spec order. Every export is `@ stub`, so each entry
/// pairs the export name with the helper that raises its record.
pub const EXPORTS: [(&str, ExportFn); 28] = [
    ("bcp_batch", bcp_batch),
    ("bcp_bind", bcp_bind),
    ("bcp_colfmt", bcp_colfmt),
    ("bcp_collen", bcp_collen),
    ("bcp_colptr", bcp_colptr),
    ("bcp_columns", bcp_columns),
    ("bcp_control", bcp_control),
    ("bcp_done", bcp_done),
    ("bcp_exec", bcp_exec),
    ("bcp_getcolfmt", bcp_getcolfmt),
    ("bcp_initA", bcp_init_a),
    ("bcp_initW", bcp_init_w),
    ("bcp_moretext", bcp_moretext),
    ("bcp_readfmtA", bcp_readfmt_a),
    ("bcp_readfmtW", bcp_readfmt_w),
    ("bcp_sendrow", bcp_sendrow),
    ("bcp_setcolfmt", bcp_setcolfmt),
    ("bcp_writefmtA", bcp_writefmt_a),
    ("bcp_writefmtW", bcp_writefmt_w),
    ("dbprtypeA", dbprtype_a),
    ("dbprtypeW", dbprtype_w),
    ("LibMain", lib_main),
    ("SQLCloseEnumServers", sql_close_enum_servers),
    ("SQLGetNextEnumeration", sql_get_next_enumeration),
    ("SQLInitEnumServers", sql_init_enum_servers),
    ("SQLLinkedCatalogsA", sql_linked_catalogs_a),
    ("SQLLinkedCatalogsW", sql_linked_catalogs_w),
    ("SQLLinkedServers", sql_linked_servers),
];

/// The stub record for `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, stub)| stub())
}
