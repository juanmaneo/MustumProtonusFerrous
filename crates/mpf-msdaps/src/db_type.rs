//! Port of `db_type_size` from Wine's `dlls/msdaps/row_server.c`.

use crate::com::Guid;

/// The `DBTYPE_*` values that `db_type_size` switches on, from `include/dbs.idl`.
pub const DBTYPE_I1: u16 = 16;
pub const DBTYPE_UI1: u16 = 17;
pub const DBTYPE_I2: u16 = 2;
pub const DBTYPE_UI2: u16 = 18;
pub const DBTYPE_I4: u16 = 3;
pub const DBTYPE_UI4: u16 = 19;
pub const DBTYPE_R4: u16 = 4;
pub const DBTYPE_I8: u16 = 20;
pub const DBTYPE_UI8: u16 = 21;
pub const DBTYPE_R8: u16 = 5;
pub const DBTYPE_CY: u16 = 6;
pub const DBTYPE_FILETIME: u16 = 64;
pub const DBTYPE_BSTR: u16 = 8;
pub const DBTYPE_GUID: u16 = 72;
pub const DBTYPE_WSTR: u16 = 130;

/// `db_type_size`: the byte size of a column of type `db_type`. `var_len` is only read for
/// `DBTYPE_WSTR`. Types the C switch does not list return 0; the C code logs a FIXME for them.
///
/// The C code takes these sizes from `sizeof`, so the port takes the Rust size of the same shape.
/// `BSTR` is a pointer, so its size follows the target width.
pub fn db_type_size(db_type: u16, var_len: usize) -> usize {
    match db_type {
        DBTYPE_I1 | DBTYPE_UI1 => 1,
        DBTYPE_I2 | DBTYPE_UI2 => 2,
        DBTYPE_I4 | DBTYPE_UI4 | DBTYPE_R4 => 4,
        DBTYPE_I8 | DBTYPE_UI8 | DBTYPE_R8 => 8,
        // `CY` is a union whose largest member is the 64-bit `int64`.
        DBTYPE_CY => std::mem::size_of::<i64>(),
        // `FILETIME` is two `DWORD`s.
        DBTYPE_FILETIME => std::mem::size_of::<[u32; 2]>(),
        DBTYPE_BSTR => std::mem::size_of::<*const u16>(),
        DBTYPE_GUID => std::mem::size_of::<Guid>(),
        DBTYPE_WSTR => var_len,
        _ => 0,
    }
}
