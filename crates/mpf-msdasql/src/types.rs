//! SQL type codes, and the mapping from them to OLE DB bind types (`sqltype_to_bindtype`) and length classes
//! (`is_variable_length`, `is_fixed_length`).

pub const SQL_CHAR: i16 = 1;
pub const SQL_NUMERIC: i16 = 2;
pub const SQL_DECIMAL: i16 = 3;
pub const SQL_INTEGER: i16 = 4;
pub const SQL_SMALLINT: i16 = 5;
pub const SQL_FLOAT: i16 = 6;
pub const SQL_REAL: i16 = 7;
pub const SQL_DOUBLE: i16 = 8;
pub const SQL_DATE: i16 = 9;
pub const SQL_TIME: i16 = 10;
pub const SQL_TIMESTAMP: i16 = 11;
pub const SQL_VARCHAR: i16 = 12;
pub const SQL_TYPE_DATE: i16 = 91;
pub const SQL_TYPE_TIME: i16 = 92;
pub const SQL_TYPE_TIMESTAMP: i16 = 93;
pub const SQL_LONGVARCHAR: i16 = -1;
pub const SQL_BINARY: i16 = -2;
pub const SQL_VARBINARY: i16 = -3;
pub const SQL_LONGVARBINARY: i16 = -4;
pub const SQL_BIGINT: i16 = -5;
pub const SQL_TINYINT: i16 = -6;
pub const SQL_WCHAR: i16 = -8;
pub const SQL_WVARCHAR: i16 = -9;
pub const SQL_WLONGVARCHAR: i16 = -10;
pub const SQL_GUID: i16 = -11;
pub const SQL_SIGNED_OFFSET: i16 = -20;
pub const SQL_UNSIGNED_OFFSET: i16 = -22;
pub const SQL_C_SBIGINT: i16 = SQL_BIGINT + SQL_SIGNED_OFFSET;
pub const SQL_C_SLONG: i16 = SQL_INTEGER + SQL_SIGNED_OFFSET;
pub const SQL_C_ULONG: i16 = SQL_INTEGER + SQL_UNSIGNED_OFFSET;

pub const DBTYPE_I1: u16 = 16;
pub const DBTYPE_I2: u16 = 2;
pub const DBTYPE_I4: u16 = 3;
pub const DBTYPE_I8: u16 = 20;
pub const DBTYPE_UI2: u16 = 18;
pub const DBTYPE_UI4: u16 = 19;
pub const DBTYPE_R4: u16 = 4;
pub const DBTYPE_R8: u16 = 5;
pub const DBTYPE_DATE: u16 = 7;
pub const DBTYPE_GUID: u16 = 72;
pub const DBTYPE_BYTES: u16 = 128;
pub const DBTYPE_STR: u16 = 129;
pub const DBTYPE_WSTR: u16 = 130;
pub const DBTYPE_NUMERIC: u16 = 131;
pub const DBTYPE_DBDATE: u16 = 133;
pub const DBTYPE_DBTIME: u16 = 134;
pub const DBTYPE_DBTIMESTAMP: u16 = 135;

/// Ports `sqltype_to_bindtype`. The C function returns `SQLSMALLINT`, but every value it returns is a `DBTYPE`,
/// so the port returns the `DBTYPE` as `u16`. Unknown codes map to `DBTYPE_I4`, as in the C code.
pub fn sqltype_to_bindtype(sql_type: i16, sign: bool) -> u16 {
    match sql_type {
        SQL_DECIMAL => DBTYPE_NUMERIC,
        SQL_CHAR | SQL_VARCHAR | SQL_LONGVARCHAR | SQL_NUMERIC => DBTYPE_STR,
        SQL_GUID => DBTYPE_GUID,
        SQL_TINYINT => DBTYPE_I1,
        SQL_SMALLINT => {
            if sign {
                DBTYPE_I2
            } else {
                DBTYPE_UI2
            }
        }
        SQL_INTEGER => {
            if sign {
                DBTYPE_I4
            } else {
                DBTYPE_UI4
            }
        }
        SQL_REAL => DBTYPE_R4,
        SQL_FLOAT | SQL_DOUBLE => DBTYPE_R8,
        SQL_BINARY | SQL_VARBINARY | SQL_LONGVARBINARY => DBTYPE_BYTES,
        SQL_TYPE_DATE => DBTYPE_DBDATE,
        SQL_DATE => DBTYPE_DBTIME,
        SQL_TIME => DBTYPE_DATE,
        SQL_TYPE_TIMESTAMP | SQL_TIMESTAMP => DBTYPE_DBTIMESTAMP,
        SQL_TYPE_TIME => DBTYPE_DBTIME,
        SQL_BIGINT | SQL_C_SBIGINT => DBTYPE_I8,
        SQL_C_SLONG => DBTYPE_I4,
        SQL_C_ULONG => DBTYPE_UI4,
        SQL_WLONGVARCHAR | SQL_WCHAR | SQL_WVARCHAR => DBTYPE_WSTR,
        _ => DBTYPE_I4,
    }
}

/// Ports `is_variable_length`.
pub fn is_variable_length(sql_type: i16) -> bool {
    matches!(
        sql_type,
        SQL_LONGVARCHAR | SQL_WLONGVARCHAR | SQL_LONGVARBINARY
    )
}

/// Ports `is_fixed_length`. It is false for the long and wide variable-width types, and true for every other code,
/// including unknown ones.
pub fn is_fixed_length(sql_type: i16) -> bool {
    !matches!(
        sql_type,
        SQL_LONGVARCHAR | SQL_WLONGVARCHAR | SQL_WVARCHAR | SQL_LONGVARBINARY | SQL_VARBINARY
    )
}
