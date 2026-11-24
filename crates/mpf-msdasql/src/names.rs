//! Debug names for SQL and OLE DB type codes. The C code builds them with `SQLTYPE_TO_STR`, which is `#x`, so the
//! names here are the identifiers themselves.

use crate::types::*;

/// Ports `debugstr_sqltype`.
pub fn debugstr_sqltype(sql_type: i16) -> &'static str {
    match sql_type {
        SQL_DECIMAL => stringify!(SQL_DECIMAL),
        SQL_CHAR => stringify!(SQL_CHAR),
        SQL_VARCHAR => stringify!(SQL_VARCHAR),
        SQL_LONGVARCHAR => stringify!(SQL_LONGVARCHAR),
        SQL_NUMERIC => stringify!(SQL_NUMERIC),
        SQL_GUID => stringify!(SQL_GUID),
        SQL_TINYINT => stringify!(SQL_TINYINT),
        SQL_SMALLINT => stringify!(SQL_SMALLINT),
        SQL_INTEGER => stringify!(SQL_INTEGER),
        SQL_REAL => stringify!(SQL_REAL),
        SQL_FLOAT => stringify!(SQL_FLOAT),
        SQL_DOUBLE => stringify!(SQL_DOUBLE),
        SQL_BINARY => stringify!(SQL_BINARY),
        SQL_VARBINARY => stringify!(SQL_VARBINARY),
        SQL_LONGVARBINARY => stringify!(SQL_LONGVARBINARY),
        SQL_TYPE_DATE => stringify!(SQL_TYPE_DATE),
        SQL_DATE => stringify!(SQL_DATE),
        SQL_TIME => stringify!(SQL_TIME),
        SQL_TYPE_TIMESTAMP => stringify!(SQL_TYPE_TIMESTAMP),
        SQL_TIMESTAMP => stringify!(SQL_TIMESTAMP),
        SQL_TYPE_TIME => stringify!(SQL_TYPE_TIME),
        SQL_BIGINT => stringify!(SQL_BIGINT),
        SQL_C_SBIGINT => stringify!(SQL_C_SBIGINT),
        SQL_C_SLONG => stringify!(SQL_C_SLONG),
        SQL_C_ULONG => stringify!(SQL_C_ULONG),
        SQL_WLONGVARCHAR => stringify!(SQL_WLONGVARCHAR),
        SQL_WCHAR => stringify!(SQL_WCHAR),
        SQL_WVARCHAR => stringify!(SQL_WVARCHAR),
        _ => "Unknown",
    }
}

/// Ports `debugstr_dbtype`.
pub fn debugstr_dbtype(db_type: u16) -> &'static str {
    match db_type {
        DBTYPE_NUMERIC => stringify!(DBTYPE_NUMERIC),
        DBTYPE_STR => stringify!(DBTYPE_STR),
        DBTYPE_GUID => stringify!(DBTYPE_GUID),
        DBTYPE_I1 => stringify!(DBTYPE_I1),
        DBTYPE_I2 => stringify!(DBTYPE_I2),
        DBTYPE_UI2 => stringify!(DBTYPE_UI2),
        DBTYPE_I4 => stringify!(DBTYPE_I4),
        DBTYPE_I8 => stringify!(DBTYPE_I8),
        DBTYPE_UI4 => stringify!(DBTYPE_UI4),
        DBTYPE_R4 => stringify!(DBTYPE_R4),
        DBTYPE_R8 => stringify!(DBTYPE_R8),
        DBTYPE_BYTES => stringify!(DBTYPE_BYTES),
        DBTYPE_DBDATE => stringify!(DBTYPE_DBDATE),
        DBTYPE_DBTIME => stringify!(DBTYPE_DBTIME),
        DBTYPE_DATE => stringify!(DBTYPE_DATE),
        DBTYPE_DBTIMESTAMP => stringify!(DBTYPE_DBTIMESTAMP),
        DBTYPE_WSTR => stringify!(DBTYPE_WSTR),
        _ => "Unknown",
    }
}
