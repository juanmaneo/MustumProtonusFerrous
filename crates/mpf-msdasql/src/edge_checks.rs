//! Edge checks for the branches the oracle rows do not reach. The tables list every code the C switches name, so each
//! arm of the port is checked, plus the lookup edges of `convert_dbproperty_mode`: case, partial names, and NUL.

use super::*;
use crate::mode::*;
use crate::types::*;

/// Encodes ASCII test text as the UTF-16 units of a Wine `WCHAR` string.
fn w(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn bind_type_for_every_sql_type() {
    // (sql type, bind type without sign, bind type with sign). Only SQL_SMALLINT and SQL_INTEGER depend on the sign.
    let table: &[(i16, u16, u16)] = &[
        (SQL_DECIMAL, DBTYPE_NUMERIC, DBTYPE_NUMERIC),
        (SQL_CHAR, DBTYPE_STR, DBTYPE_STR),
        (SQL_VARCHAR, DBTYPE_STR, DBTYPE_STR),
        (SQL_LONGVARCHAR, DBTYPE_STR, DBTYPE_STR),
        (SQL_NUMERIC, DBTYPE_STR, DBTYPE_STR),
        (SQL_GUID, DBTYPE_GUID, DBTYPE_GUID),
        (SQL_TINYINT, DBTYPE_I1, DBTYPE_I1),
        (SQL_SMALLINT, DBTYPE_UI2, DBTYPE_I2),
        (SQL_INTEGER, DBTYPE_UI4, DBTYPE_I4),
        (SQL_REAL, DBTYPE_R4, DBTYPE_R4),
        (SQL_FLOAT, DBTYPE_R8, DBTYPE_R8),
        (SQL_DOUBLE, DBTYPE_R8, DBTYPE_R8),
        (SQL_BINARY, DBTYPE_BYTES, DBTYPE_BYTES),
        (SQL_VARBINARY, DBTYPE_BYTES, DBTYPE_BYTES),
        (SQL_LONGVARBINARY, DBTYPE_BYTES, DBTYPE_BYTES),
        (SQL_TYPE_DATE, DBTYPE_DBDATE, DBTYPE_DBDATE),
        (SQL_DATE, DBTYPE_DBTIME, DBTYPE_DBTIME),
        (SQL_TIME, DBTYPE_DATE, DBTYPE_DATE),
        (SQL_TYPE_TIMESTAMP, DBTYPE_DBTIMESTAMP, DBTYPE_DBTIMESTAMP),
        (SQL_TIMESTAMP, DBTYPE_DBTIMESTAMP, DBTYPE_DBTIMESTAMP),
        (SQL_TYPE_TIME, DBTYPE_DBTIME, DBTYPE_DBTIME),
        (SQL_BIGINT, DBTYPE_I8, DBTYPE_I8),
        (SQL_C_SBIGINT, DBTYPE_I8, DBTYPE_I8),
        (SQL_C_SLONG, DBTYPE_I4, DBTYPE_I4),
        (SQL_C_ULONG, DBTYPE_UI4, DBTYPE_UI4),
        (SQL_WLONGVARCHAR, DBTYPE_WSTR, DBTYPE_WSTR),
        (SQL_WCHAR, DBTYPE_WSTR, DBTYPE_WSTR),
        (SQL_WVARCHAR, DBTYPE_WSTR, DBTYPE_WSTR),
    ];
    for &(sql_type, unsigned, signed) in table {
        assert_eq!(
            sqltype_to_bindtype(sql_type, false),
            unsigned,
            "sql type {sql_type}, unsigned"
        );
        assert_eq!(
            sqltype_to_bindtype(sql_type, true),
            signed,
            "sql type {sql_type}, signed"
        );
    }
}

#[test]
fn unknown_sql_types_fall_back_to_i4() {
    for sql_type in [0, 13, 14, 99, -7, -12, i16::MIN, i16::MAX] {
        assert_eq!(
            sqltype_to_bindtype(sql_type, false),
            DBTYPE_I4,
            "sql type {sql_type}"
        );
        assert_eq!(
            sqltype_to_bindtype(sql_type, true),
            DBTYPE_I4,
            "sql type {sql_type}"
        );
    }
}

#[test]
fn sign_only_changes_smallint_and_integer() {
    for sql_type in -40..=100 {
        let differs = sqltype_to_bindtype(sql_type, false) != sqltype_to_bindtype(sql_type, true);
        assert_eq!(
            differs,
            sql_type == SQL_SMALLINT || sql_type == SQL_INTEGER,
            "sql type {sql_type}"
        );
    }
}

#[test]
fn sql_c_codes_derive_from_sqlext_offsets() {
    // sqlext.h: SQL_C_LONG is SQL_INTEGER, and the signed and unsigned offsets are -20 and -22.
    assert_eq!(SQL_SIGNED_OFFSET, -20);
    assert_eq!(SQL_UNSIGNED_OFFSET, -22);
    assert_eq!(SQL_C_SBIGINT, -25);
    assert_eq!(SQL_C_SLONG, -16);
    assert_eq!(SQL_C_ULONG, -18);
}

#[test]
fn length_classes_for_each_code() {
    // (sql type, is_variable_length, is_fixed_length)
    let table: &[(i16, bool, bool)] = &[
        (SQL_LONGVARCHAR, true, false),
        (SQL_WLONGVARCHAR, true, false),
        (SQL_LONGVARBINARY, true, false),
        (SQL_VARBINARY, false, false),
        (SQL_WVARCHAR, false, false),
        (SQL_VARCHAR, false, true),
        (SQL_BINARY, false, true),
        (SQL_CHAR, false, true),
        (SQL_WCHAR, false, true),
        (0, false, true),
        (99, false, true),
        (-99, false, true),
    ];
    for &(sql_type, variable, fixed) in table {
        assert_eq!(
            is_variable_length(sql_type),
            variable,
            "variable, sql type {sql_type}"
        );
        assert_eq!(
            is_fixed_length(sql_type),
            fixed,
            "fixed, sql type {sql_type}"
        );
    }
}

#[test]
fn debugstr_sqltype_names_every_code() {
    let table: &[(i16, &str)] = &[
        (SQL_DECIMAL, "SQL_DECIMAL"),
        (SQL_CHAR, "SQL_CHAR"),
        (SQL_VARCHAR, "SQL_VARCHAR"),
        (SQL_LONGVARCHAR, "SQL_LONGVARCHAR"),
        (SQL_NUMERIC, "SQL_NUMERIC"),
        (SQL_GUID, "SQL_GUID"),
        (SQL_TINYINT, "SQL_TINYINT"),
        (SQL_SMALLINT, "SQL_SMALLINT"),
        (SQL_INTEGER, "SQL_INTEGER"),
        (SQL_REAL, "SQL_REAL"),
        (SQL_FLOAT, "SQL_FLOAT"),
        (SQL_DOUBLE, "SQL_DOUBLE"),
        (SQL_BINARY, "SQL_BINARY"),
        (SQL_VARBINARY, "SQL_VARBINARY"),
        (SQL_LONGVARBINARY, "SQL_LONGVARBINARY"),
        (SQL_TYPE_DATE, "SQL_TYPE_DATE"),
        (SQL_DATE, "SQL_DATE"),
        (SQL_TIME, "SQL_TIME"),
        (SQL_TYPE_TIMESTAMP, "SQL_TYPE_TIMESTAMP"),
        (SQL_TIMESTAMP, "SQL_TIMESTAMP"),
        (SQL_TYPE_TIME, "SQL_TYPE_TIME"),
        (SQL_BIGINT, "SQL_BIGINT"),
        (SQL_C_SBIGINT, "SQL_C_SBIGINT"),
        (SQL_C_SLONG, "SQL_C_SLONG"),
        (SQL_C_ULONG, "SQL_C_ULONG"),
        (SQL_WLONGVARCHAR, "SQL_WLONGVARCHAR"),
        (SQL_WCHAR, "SQL_WCHAR"),
        (SQL_WVARCHAR, "SQL_WVARCHAR"),
    ];
    for &(sql_type, name) in table {
        assert_eq!(debugstr_sqltype(sql_type), name);
    }
    for sql_type in [0, 4096, -7, i16::MIN] {
        assert_eq!(debugstr_sqltype(sql_type), "Unknown", "sql type {sql_type}");
    }
}

#[test]
fn debugstr_dbtype_names_every_code() {
    let table: &[(u16, &str)] = &[
        (DBTYPE_NUMERIC, "DBTYPE_NUMERIC"),
        (DBTYPE_STR, "DBTYPE_STR"),
        (DBTYPE_GUID, "DBTYPE_GUID"),
        (DBTYPE_I1, "DBTYPE_I1"),
        (DBTYPE_I2, "DBTYPE_I2"),
        (DBTYPE_UI2, "DBTYPE_UI2"),
        (DBTYPE_I4, "DBTYPE_I4"),
        (DBTYPE_I8, "DBTYPE_I8"),
        (DBTYPE_UI4, "DBTYPE_UI4"),
        (DBTYPE_R4, "DBTYPE_R4"),
        (DBTYPE_R8, "DBTYPE_R8"),
        (DBTYPE_BYTES, "DBTYPE_BYTES"),
        (DBTYPE_DBDATE, "DBTYPE_DBDATE"),
        (DBTYPE_DBTIME, "DBTYPE_DBTIME"),
        (DBTYPE_DATE, "DBTYPE_DATE"),
        (DBTYPE_DBTIMESTAMP, "DBTYPE_DBTIMESTAMP"),
        (DBTYPE_WSTR, "DBTYPE_WSTR"),
    ];
    for &(db_type, name) in table {
        assert_eq!(debugstr_dbtype(db_type), name);
    }
    for db_type in [0, 1, 6, 8, 14, u16::MAX] {
        assert_eq!(debugstr_dbtype(db_type), "Unknown", "dbtype {db_type}");
    }
}

#[test]
fn mode_names_resolve_in_any_ascii_case() {
    let table: &[(&str, u32)] = &[
        ("Read", DB_MODE_READ),
        ("write", DB_MODE_WRITE),
        ("READWRITE", DB_MODE_READWRITE),
        ("share deny none", DB_MODE_SHARE_DENY_NONE),
        ("Share Deny Read", DB_MODE_SHARE_DENY_READ),
        ("SHARE DENY WRITE", DB_MODE_SHARE_DENY_WRITE),
        ("share exclusive", DB_MODE_SHARE_EXCLUSIVE),
    ];
    for &(name, value) in table {
        assert_eq!(convert_dbproperty_mode(&w(name)), Some(value), "{name}");
    }
}

#[test]
fn mode_names_must_match_the_whole_string() {
    for name in [
        "",
        "Rea",
        "Reads",
        "Read ",
        " Read",
        "Share Deny",
        "Share  Exclusive",
        "Mode",
    ] {
        assert_eq!(convert_dbproperty_mode(&w(name)), None, "{name:?}");
    }
}

#[test]
fn mode_reads_up_to_the_first_nul() {
    assert_eq!(
        convert_dbproperty_mode(&w("Write\0junk")),
        Some(DB_MODE_WRITE)
    );
    assert_eq!(convert_dbproperty_mode(&w("\0Read")), None);
    assert_eq!(convert_dbproperty_mode(&[]), None);
}

#[test]
fn property_count_matches_any_shared_bit() {
    assert_eq!(get_property_count(0x40, &[]), 0);
    assert_eq!(get_property_count(0, &[0x40, 0x41, 0xffff_ffff]), 0);
    assert_eq!(get_property_count(0x40, &[0x40, 0x41, 0x80, 0, 0x2040]), 3);
    assert_eq!(get_property_count(0x600, &[0x200, 0x400, 0x600, 0x100]), 3);
}
