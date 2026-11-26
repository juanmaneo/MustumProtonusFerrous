//! The DBTYPE helpers in `dlls/oledb32/convert.c`: `get_length`, `get_dbtype_class`,
//! `common_class` and `array_type`. Values are from `include/dbs.idl`.

pub const DBTYPE_EMPTY: u16 = 0;
pub const DBTYPE_NULL: u16 = 1;
pub const DBTYPE_I2: u16 = 2;
pub const DBTYPE_I4: u16 = 3;
pub const DBTYPE_R4: u16 = 4;
pub const DBTYPE_R8: u16 = 5;
pub const DBTYPE_CY: u16 = 6;
pub const DBTYPE_DATE: u16 = 7;
pub const DBTYPE_BSTR: u16 = 8;
pub const DBTYPE_BOOL: u16 = 11;
pub const DBTYPE_VARIANT: u16 = 12;
pub const DBTYPE_DECIMAL: u16 = 14;
pub const DBTYPE_I1: u16 = 16;
pub const DBTYPE_UI1: u16 = 17;
pub const DBTYPE_UI2: u16 = 18;
pub const DBTYPE_UI4: u16 = 19;
pub const DBTYPE_I8: u16 = 20;
pub const DBTYPE_UI8: u16 = 21;
pub const DBTYPE_FILETIME: u16 = 64;
pub const DBTYPE_GUID: u16 = 72;
pub const DBTYPE_BYTES: u16 = 128;
pub const DBTYPE_STR: u16 = 129;
pub const DBTYPE_WSTR: u16 = 130;
pub const DBTYPE_NUMERIC: u16 = 131;
pub const DBTYPE_DBDATE: u16 = 133;
pub const DBTYPE_DBTIME: u16 = 134;
pub const DBTYPE_DBTIMESTAMP: u16 = 135;
pub const DBTYPE_BYREF_WSTR: u16 = 0x4000 | DBTYPE_WSTR;

/// `get_length`: the x86-64 `sizeof` of a fixed-size DBTYPE value. Variable-length types and
/// unknown types give 0. The C `FIXME` on the default arm is not modelled.
pub fn dbtype_length(ty: u16) -> usize {
    match ty {
        DBTYPE_I1 | DBTYPE_UI1 => 1,
        DBTYPE_I2 | DBTYPE_UI2 => 2,
        DBTYPE_BOOL => 2, // sizeof(VARIANT_BOOL)
        DBTYPE_I4 | DBTYPE_UI4 | DBTYPE_R4 => 4,
        DBTYPE_I8 | DBTYPE_UI8 | DBTYPE_R8 | DBTYPE_DATE => 8,
        DBTYPE_DBDATE => 6,       // sizeof(DBDATE)
        DBTYPE_DBTIMESTAMP => 16, // sizeof(DBTIMESTAMP)
        DBTYPE_CY => 8,
        DBTYPE_BSTR => 8,
        DBTYPE_FILETIME => 8,
        DBTYPE_GUID => 16,
        DBTYPE_NUMERIC => 19, // sizeof(DB_NUMERIC)
        DBTYPE_BYTES | DBTYPE_WSTR | DBTYPE_STR | DBTYPE_BYREF_WSTR => 0,
        DBTYPE_VARIANT => 24, // sizeof(VARIANT) on x86-64
        _ => 0,
    }
}

/// `get_dbtype_class`: the class a DBTYPE converts through. Types with no class map to themselves.
pub fn dbtype_class(ty: u16) -> u16 {
    match ty {
        DBTYPE_I2 | DBTYPE_R4 | DBTYPE_R8 | DBTYPE_I1 | DBTYPE_UI1 | DBTYPE_UI2 => DBTYPE_I2,
        DBTYPE_I4 | DBTYPE_UI4 => DBTYPE_I4,
        DBTYPE_I8 | DBTYPE_UI8 => DBTYPE_I8,
        DBTYPE_BSTR | DBTYPE_STR | DBTYPE_WSTR => DBTYPE_BSTR,
        DBTYPE_DBDATE | DBTYPE_DBTIME | DBTYPE_DBTIMESTAMP => DBTYPE_DBDATE,
        _ => ty,
    }
}

/// `common_class`: whether `dst_class` is in the group that many source types convert to.
pub fn common_class(dst_class: u16) -> bool {
    matches!(
        dst_class,
        DBTYPE_EMPTY
            | DBTYPE_NULL
            | DBTYPE_I2
            | DBTYPE_I4
            | DBTYPE_BSTR
            | DBTYPE_BOOL
            | DBTYPE_VARIANT
            | DBTYPE_I8
            | DBTYPE_CY
            | DBTYPE_DECIMAL
            | DBTYPE_NUMERIC
    )
}

/// `array_type`: whether `ty` lies in the `DBTYPE_I2` to `DBTYPE_UI4` range.
pub fn array_type(ty: u16) -> bool {
    (DBTYPE_I2..=DBTYPE_UI4).contains(&ty)
}
