//! The value stored by Wine's `_vcomp_reduction_{i1,i2,i4,i8,r4,r8}` entry points.
//!
//! The operation comes from bits 8..12 of `flags`, clamped to the last table slot. The C
//! functions store the result through a compare-exchange loop, so the port returns that value.

enum IntOp {
    Add,
    Mul,
    And,
    Or,
    Xor,
    BoolAnd,
    BoolOr,
}

enum FloatOp {
    Add,
    Mul,
    BoolAnd,
    BoolOr,
}

fn table_index(flags: u32) -> u32 {
    ((flags >> 8) & 0xf).min(7)
}

fn int_op(flags: u32) -> IntOp {
    match table_index(flags) {
        0 | 1 => IntOp::Add,
        2 => IntOp::Mul,
        3 => IntOp::And,
        4 => IntOp::Or,
        5 => IntOp::Xor,
        6 => IntOp::BoolAnd,
        _ => IntOp::BoolOr,
    }
}

// The float table has no bitwise slots: 3, 4, 5 and 7 all select bool-or.
fn float_op(flags: u32) -> FloatOp {
    match table_index(flags) {
        0 | 1 => FloatOp::Add,
        2 => FloatOp::Mul,
        6 => FloatOp::BoolAnd,
        _ => FloatOp::BoolOr,
    }
}

macro_rules! int_combine {
    ($name:ident, $t:ty) => {
        /// The value `_vcomp_reduction_i*` stores for `old` and `val` under `flags`.
        pub fn $name(flags: u32, old: $t, val: $t) -> $t {
            match int_op(flags) {
                IntOp::Add => old.wrapping_add(val),
                IntOp::Mul => old.wrapping_mul(val),
                IntOp::And => old & val,
                IntOp::Or => old | val,
                IntOp::Xor => old ^ val,
                IntOp::BoolAnd => <$t>::from(old != 0 && val != 0),
                IntOp::BoolOr => {
                    if old != 0 {
                        old
                    } else {
                        <$t>::from(val != 0)
                    }
                }
            }
        }
    };
}

int_combine!(combine_i8, i8);
int_combine!(combine_i16, i16);
int_combine!(combine_i32, i32);
int_combine!(combine_i64, i64);

macro_rules! float_combine {
    ($name:ident, $t:ty) => {
        /// The value `_vcomp_reduction_r*` stores for `old` and `val` under `flags`.
        pub fn $name(flags: u32, old: $t, val: $t) -> $t {
            match float_op(flags) {
                FloatOp::Add => old + val,
                FloatOp::Mul => old * val,
                FloatOp::BoolAnd => {
                    if old != 0.0 && val != 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
                FloatOp::BoolOr => {
                    if old != 0.0 {
                        old
                    } else if val != 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
            }
        }
    };
}

float_combine!(combine_f32, f32);
float_combine!(combine_f64, f64);
