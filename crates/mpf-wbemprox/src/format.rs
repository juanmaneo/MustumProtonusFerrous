//! `format_int` from `query.c`: the decimal text of an integer property value. Wine formats the
//! 8, 16 and 32-bit types with `%d` or `%u`, which read the low 32 bits of the 64-bit value, so
//! the port does the same.

use crate::types::{
    CimType, CIM_SINT16, CIM_SINT32, CIM_SINT64, CIM_SINT8, CIM_UINT16, CIM_UINT32, CIM_UINT64,
    CIM_UINT8,
};

/// The decimal text of `val` as a value of type `ty`, or `None` for a type that `query.c` does
/// not format.
pub fn format_int(ty: CimType, val: i64) -> Option<String> {
    match ty {
        CIM_SINT8 | CIM_SINT16 | CIM_SINT32 => Some((val as i32).to_string()),
        CIM_UINT8 | CIM_UINT16 | CIM_UINT32 => Some((val as u32).to_string()),
        CIM_SINT64 => Some(val.to_string()),
        CIM_UINT64 => Some((val as u64).to_string()),
        _ => None,
    }
}
