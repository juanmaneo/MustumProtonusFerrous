//! Edge cases the oracle rows do not reach: identity and zero operands, the operand order that
//! Wine's field convention fixes, and the value traits of [`Matrix4x4`].

use super::*;

const IDENTITY: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

const SAMPLE: [f32; 16] = [
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
];

fn from_array(v: [f32; 16]) -> Matrix4x4 {
    Matrix4x4 {
        _11: v[0],
        _12: v[1],
        _13: v[2],
        _14: v[3],
        _21: v[4],
        _22: v[5],
        _23: v[6],
        _24: v[7],
        _31: v[8],
        _32: v[9],
        _33: v[10],
        _34: v[11],
        _41: v[12],
        _42: v[13],
        _43: v[14],
        _44: v[15],
    }
}

fn product(a: &Matrix4x4, b: &Matrix4x4) -> Matrix4x4 {
    let mut out = Matrix4x4::default();
    multiply_matrix(&mut out, a, b);
    out
}

fn unit(row: usize, col: usize) -> Matrix4x4 {
    let mut v = [0.0; 16];
    v[row * 4 + col] = 1.0;
    from_array(v)
}

#[test]
fn identity_is_neutral_on_both_sides() {
    let m = from_array(SAMPLE);
    let id = from_array(IDENTITY);
    assert_eq!(product(&id, &m), m);
    assert_eq!(product(&m, &id), m);
}

#[test]
fn zero_matrix_annihilates_both_sides() {
    let m = from_array(SAMPLE);
    let zero = Matrix4x4::default();
    assert_eq!(product(&zero, &m), zero);
    assert_eq!(product(&m, &zero), zero);
}

#[test]
fn operand_order_follows_wine_field_convention() {
    // The result is `src2 * src1` in row-major algebra, so the order of the two arguments matters.
    assert_eq!(product(&unit(0, 1), &unit(1, 0)), unit(1, 1));
    assert_eq!(product(&unit(1, 0), &unit(0, 1)), unit(0, 0));
}

#[test]
#[allow(clippy::clone_on_copy)]
fn value_traits_copy_compare_and_print() {
    let m = from_array(SAMPLE);
    let copy = Clone::clone(&m);
    let mut changed = copy;
    changed._44 = 0.5;
    assert_eq!(copy, m);
    assert_ne!(changed, m);
    assert_eq!(Matrix4x4::default()._11.to_bits(), 0);
    assert!(format!("{m:?}").starts_with("Matrix4x4 {"));
}
