//! CIM and VARIANT type codes from `wbemcli.idl` and `wtypes.idl`, and the conversions of `query.c`:
//! [to_vartype] and [to_cimtype]. The type predicates [crate::is_int] and [crate::resolve_type]
//! live in the crate root.

/// A `CIMTYPE` value.
pub type CimType = u32;

/// A `VARTYPE` value.
pub type VarType = u16;

pub const CIM_EMPTY: CimType = 0;
pub const CIM_SINT16: CimType = 2;
pub const CIM_SINT32: CimType = 3;
pub const CIM_REAL32: CimType = 4;
pub const CIM_REAL64: CimType = 5;
pub const CIM_STRING: CimType = 8;
pub const CIM_BOOLEAN: CimType = 11;
pub const CIM_SINT8: CimType = 16;
pub const CIM_UINT8: CimType = 17;
pub const CIM_UINT16: CimType = 18;
pub const CIM_UINT32: CimType = 19;
pub const CIM_SINT64: CimType = 20;
pub const CIM_UINT64: CimType = 21;
pub const CIM_DATETIME: CimType = 101;
pub const CIM_REFERENCE: CimType = 102;
pub const CIM_CHAR16: CimType = 103;
pub const CIM_ILLEGAL: CimType = 0xfff;

pub const VT_I2: VarType = 2;
pub const VT_I4: VarType = 3;
pub const VT_R4: VarType = 4;
pub const VT_BSTR: VarType = 8;
pub const VT_BOOL: VarType = 11;
pub const VT_I1: VarType = 16;
pub const VT_UI1: VarType = 17;
pub const VT_UI2: VarType = 18;
pub const VT_UI4: VarType = 19;
pub const VT_I8: VarType = 20;
pub const VT_UI8: VarType = 21;

/// The VARIANT type for a CIM type, or 0 when `query.c` has no mapping for it.
pub fn to_vartype(ty: CimType) -> VarType {
    match ty {
        CIM_BOOLEAN => VT_BOOL,
        CIM_STRING | CIM_REFERENCE | CIM_DATETIME => VT_BSTR,
        CIM_SINT8 => VT_I1,
        CIM_UINT8 => VT_UI1,
        CIM_SINT16 => VT_I2,
        CIM_UINT16 | CIM_SINT32 | CIM_UINT32 => VT_I4,
        CIM_SINT64 => VT_I8,
        CIM_UINT64 => VT_UI8,
        CIM_REAL32 => VT_R4,
        _ => 0,
    }
}

/// The CIM type for a VARIANT type, or `CIM_EMPTY` when `query.c` has no mapping for it.
pub fn to_cimtype(vt: VarType) -> CimType {
    match vt {
        VT_BOOL => CIM_BOOLEAN,
        VT_BSTR => CIM_STRING,
        VT_I1 => CIM_SINT8,
        VT_UI1 => CIM_UINT8,
        VT_I2 => CIM_SINT16,
        VT_UI2 => CIM_UINT16,
        VT_I4 => CIM_SINT32,
        VT_UI4 => CIM_UINT32,
        VT_I8 => CIM_SINT64,
        VT_UI8 => CIM_UINT64,
        _ => CIM_EMPTY,
    }
}
