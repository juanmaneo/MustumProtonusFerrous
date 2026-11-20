//! Date and time type-code mapping between ODBC 2 and ODBC 3 spellings (`map_type`).

pub const SQL_DATE: i16 = 9;
pub const SQL_TIME: i16 = 10;
pub const SQL_TIMESTAMP: i16 = 11;
pub const SQL_TYPE_DATE: i16 = 91;
pub const SQL_TYPE_TIME: i16 = 92;
pub const SQL_TYPE_TIMESTAMP: i16 = 93;
pub const SQL_OV_ODBC2: u32 = 2;
pub const SQL_OV_ODBC3: u32 = 3;

/// The `driver_odbc_ver` threshold the C code writes as the literal `0x300`.
const DRIVER_ODBC3: u32 = 0x300;

/// Maps a date or time type code as `map_type` does.
///
/// `input` is true for a type the application passes to the driver, and then `driver_odbc_ver` decides.
/// It is false for a type the driver returns to the application, and then `attr_version` decides.
/// Other type codes are returned unchanged.
pub fn map_type(input: bool, driver_odbc_ver: u32, attr_version: u32, sql_type: i16) -> i16 {
    let to_odbc3 = if input {
        driver_odbc_ver >= DRIVER_ODBC3
    } else {
        attr_version >= SQL_OV_ODBC3
    };
    let to_odbc2 = if input {
        driver_odbc_ver < DRIVER_ODBC3
    } else {
        attr_version == SQL_OV_ODBC2
    };
    match sql_type {
        SQL_DATE if to_odbc3 => SQL_TYPE_DATE,
        SQL_TIME if to_odbc3 => SQL_TYPE_TIME,
        SQL_TIMESTAMP if to_odbc3 => SQL_TYPE_TIMESTAMP,
        SQL_TYPE_DATE if to_odbc2 => SQL_DATE,
        SQL_TYPE_TIME if to_odbc2 => SQL_TIME,
        SQL_TYPE_TIMESTAMP if to_odbc2 => SQL_TIMESTAMP,
        _ => sql_type,
    }
}
