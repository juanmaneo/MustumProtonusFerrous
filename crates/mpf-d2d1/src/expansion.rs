//! Shewchuk floating-point expansion arithmetic over `f32`, as used by `d2d_point_ccw`.
//! Expansions are stored least significant component first; each `[f32; 2]` pair is `[low, high]`.

/// `(1 << (FLT_MANT_DIG / 2)) + 1`, the Dekker splitter for `f32`.
const SPLITTER: f32 = 4097.0;

pub(crate) fn two_sum(a: f32, b: f32) -> [f32; 2] {
    let x = a + b;
    let b_virt = x - a;
    let a_virt = x - b_virt;
    let b_round = b - b_virt;
    let a_round = a - a_virt;
    [a_round + b_round, x]
}

pub(crate) fn fast_two_sum(a: f32, b: f32) -> [f32; 2] {
    let x = a + b;
    let b_virt = x - a;
    [b - b_virt, x]
}

pub(crate) fn two_diff_tail(a: f32, b: f32, x: f32) -> f32 {
    let b_virt = a - x;
    let a_virt = x + b_virt;
    let b_round = b_virt - b;
    let a_round = a - a_virt;
    a_round + b_round
}

pub(crate) fn two_two_diff(a: [f32; 2], b: [f32; 2]) -> [f32; 4] {
    let diff = a[0] - b[0];
    let tail0 = two_diff_tail(a[0], b[0], diff);
    let sum = two_sum(a[1], diff);
    let diff = sum[0] - b[1];
    let tail1 = two_diff_tail(sum[0], b[1], diff);
    let top = two_sum(sum[1], diff);
    [tail0, tail1, top[0], top[1]]
}

pub(crate) fn split(a: f32) -> [f32; 2] {
    let c = a * SPLITTER;
    let a_big = c - a;
    let hi = c - a_big;
    [a - hi, hi]
}

pub(crate) fn two_product_presplit(a: f32, b: f32, b_split: [f32; 2]) -> [f32; 2] {
    let hi = a * b;
    let a_split = split(a);
    let err1 = hi - a_split[1] * b_split[1];
    let err2 = err1 - a_split[0] * b_split[1];
    let err3 = err2 - a_split[1] * b_split[0];
    [a_split[0] * b_split[0] - err3, hi]
}

pub(crate) fn two_product(a: f32, b: f32) -> [f32; 2] {
    two_product_presplit(a, b, split(b))
}

pub(crate) fn estimate(a: &[f32]) -> f32 {
    let mut out = a[0];
    for &x in &a[1..] {
        out += x;
    }
    out
}

/// Takes the next component from `a`, unless `b` lies between `a` and `-a` (Wine's comparison), in
/// which case it is taken from `b`. Advances the chosen cursor.
fn take_next(a: &[f32], b: &[f32], a_idx: &mut usize, b_idx: &mut usize) -> f32 {
    if (b[*b_idx] > a[*a_idx]) == (b[*b_idx] > -a[*a_idx]) {
        *a_idx += 1;
        a[*a_idx - 1]
    } else {
        *b_idx += 1;
        b[*b_idx - 1]
    }
}

/// Appends `x` to `out` unless it is zero.
fn push_nonzero(out: &mut [f32], out_len: &mut usize, x: f32) {
    if x != 0.0 {
        out[*out_len] = x;
        *out_len += 1;
    }
}

/// Merges two expansions into `out`, dropping zero components, and returns the component count.
/// `out` must hold at least `a.len() + b.len()` components.
pub(crate) fn fast_expansion_sum_zeroelim(out: &mut [f32], a: &[f32], b: &[f32]) -> usize {
    let mut a_idx = 0;
    let mut b_idx = 0;
    let mut out_len = 0;
    let mut q = take_next(a, b, &mut a_idx, &mut b_idx);
    if a_idx < a.len() && b_idx < b.len() {
        let sum = fast_two_sum(take_next(a, b, &mut a_idx, &mut b_idx), q);
        push_nonzero(out, &mut out_len, sum[0]);
        q = sum[1];
        while a_idx < a.len() && b_idx < b.len() {
            let sum = two_sum(q, take_next(a, b, &mut a_idx, &mut b_idx));
            push_nonzero(out, &mut out_len, sum[0]);
            q = sum[1];
        }
    }
    while a_idx < a.len() {
        let sum = two_sum(q, a[a_idx]);
        a_idx += 1;
        push_nonzero(out, &mut out_len, sum[0]);
        q = sum[1];
    }
    while b_idx < b.len() {
        let sum = two_sum(q, b[b_idx]);
        b_idx += 1;
        push_nonzero(out, &mut out_len, sum[0]);
        q = sum[1];
    }
    if q != 0.0 || out_len == 0 {
        out[out_len] = q;
        out_len += 1;
    }
    out_len
}
