//! The predicates of `query.c` that pick the comparison for a WQL expression: [is_strcmp],
//! [is_boolcmp] and [is_refcmp]. Each takes the CIM types of the two property or value operands and
//! the expression kinds (`EXPR_*`) of the two sides, and returns whether the pairing applies.

use crate::is_int;
use crate::types::{CimType, CIM_BOOLEAN, CIM_REFERENCE, CIM_STRING};

pub const EXPR_COMPLEX: u32 = 1;
pub const EXPR_UNARY: u32 = 2;
pub const EXPR_PROPVAL: u32 = 3;
pub const EXPR_SVAL: u32 = 4;
pub const EXPR_IVAL: u32 = 5;
pub const EXPR_BVAL: u32 = 6;

/// A string comparison: a string or integer property against a string literal, in either order.
pub fn is_strcmp(ltype: CimType, rtype: CimType, left: u32, right: u32) -> bool {
    ((ltype == CIM_STRING || is_int(ltype)) && left == EXPR_PROPVAL && right == EXPR_SVAL)
        || ((rtype == CIM_STRING || is_int(rtype)) && right == EXPR_PROPVAL && left == EXPR_SVAL)
}

/// A boolean comparison: a boolean property against a string or boolean literal, in either order.
pub fn is_boolcmp(ltype: CimType, rtype: CimType, left: u32, right: u32) -> bool {
    (ltype == CIM_BOOLEAN && left == EXPR_PROPVAL && (right == EXPR_SVAL || right == EXPR_BVAL))
        || (rtype == CIM_BOOLEAN
            && right == EXPR_PROPVAL
            && (left == EXPR_SVAL || left == EXPR_BVAL))
}

/// A reference comparison: a reference property against a string literal, in either order.
pub fn is_refcmp(ltype: CimType, rtype: CimType, left: u32, right: u32) -> bool {
    (ltype == CIM_REFERENCE && left == EXPR_PROPVAL && right == EXPR_SVAL)
        || (rtype == CIM_REFERENCE && right == EXPR_PROPVAL && left == EXPR_SVAL)
}
