//! Headless port of the deterministic parts of Wine's `dlls/wbemprox`, the WMI provider.
//!
//! The C reference parses WQL and evaluates it over WMI classes. The parser, the query tables and
//! the COM objects need a live provider. The functions below run here as pure functions over
//! values. The caller supplies each string operand as a slice of UTF-16 code units, with `None`
//! for a NULL pointer, and each integer operand as an `i64`.
//!
//! Ported (names in `dlls/wbemprox/query.c`, with `wbemprox_private.h`):
//! - `eval_like`: [like::eval_like].
//! - `eval_strcmp`, `eval_refcmp` and `eval_boolcmp`: [compare::eval_strcmp],
//!   [compare::eval_refcmp], [compare::eval_boolcmp].
//! - `is_strcmp`, `is_boolcmp` and `is_refcmp`: [expr::is_strcmp], [expr::is_boolcmp],
//!   [expr::is_refcmp].
//! - `is_int` and `resolve_type`: [is_int], [resolve_type].
//! - `format_int`: [format::format_int].
//! - `to_vartype` and `to_cimtype`: [types::to_vartype], [types::to_cimtype].
//! - `towupper`, `towlower` and `_wcsicmp` from the C runtime: [wide].
//!
//! Skipped: the WQL lexer and parser (`wql.y`), the evaluation over WMI tables and properties, the
//! COM interfaces, the `TRACE`, `FIXME` and `ERR` logging, and the `[` character ranges that
//! Wine's `eval_like` leaves unsupported.
//!
//! Deviations from Wine:
//! - Case mapping uses Rust's Unicode tables. A character whose mapping has several characters
//!   keeps its code unit. glibc, which the oracle uses, takes the simple one-character mapping, so
//!   U+0130 lowers to `i` there and U+1F80 uppers to U+1F88. The port keeps both units.
//! - `eval_boolcmp` takes a [compare::Operand] for each side, where C takes a type per side.
//!   C converts only the left side when both are strings. `is_boolcmp` never pairs two strings,
//!   so the port's conversion of both sides is not reached from `query.c`.
//! - A string slice must not contain a NUL unit. The end of the slice is the end of the string.
//! - [format::format_int] returns ASCII text, where C writes a `WCHAR` buffer.

pub mod compare;
pub mod expr;
pub mod format;
pub mod like;
pub mod types;
pub mod wide;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

use types::{
    CimType, CIM_BOOLEAN, CIM_ILLEGAL, CIM_REFERENCE, CIM_SINT16, CIM_SINT32, CIM_SINT64,
    CIM_SINT8, CIM_STRING, CIM_UINT16, CIM_UINT32, CIM_UINT64, CIM_UINT8,
};

/// An `HRESULT` value.
pub type HRESULT = i32;

/// `WBEM_E_INVALID_QUERY`: an operator that a comparison does not support.
pub const WBEM_E_INVALID_QUERY: HRESULT = 0x8004_1017u32 as i32;

/// Whether `ty` is one of the eight integer CIM types.
pub fn is_int(ty: CimType) -> bool {
    matches!(
        ty,
        CIM_SINT8
            | CIM_SINT16
            | CIM_SINT32
            | CIM_SINT64
            | CIM_UINT8
            | CIM_UINT16
            | CIM_UINT32
            | CIM_UINT64
    )
}

/// The type of a comparison between two operands of the given types: `CIM_UINT64` for any pair of
/// integers, the common type for equal string, boolean or reference types, and `CIM_ILLEGAL`
/// otherwise.
pub fn resolve_type(left: CimType, right: CimType) -> CimType {
    if is_int(left) && is_int(right) {
        return CIM_UINT64;
    }
    match (left, right) {
        (CIM_STRING, CIM_STRING) | (CIM_BOOLEAN, CIM_BOOLEAN) | (CIM_REFERENCE, CIM_REFERENCE) => {
            left
        }
        _ => CIM_ILLEGAL,
    }
}
