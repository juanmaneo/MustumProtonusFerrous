//! Edge checks for every ported function, at both widths. The oracle rows in `oracle_checks` reach
//! the common paths. These checks reach the rest: the invalid-parameter, throw and empty-input
//! paths, and the functions that the rows do not call (`flags_get`, `setf` and the `copy` family).
//!
//! The generic checks run at `u8` (`char`) and `u16` (`wchar_t`), so every instantiation of a
//! generic function is exercised.

use std::fmt::Debug;

use crate::basic_string;
use crate::char_traits::{self, Unit};
use crate::ios_base;
use crate::NPOS;

/// A unit type that the edge checks can build from byte values and print.
trait Width: Unit + From<u8> + Debug {}

impl<T: Unit + From<u8> + Debug> Width for T {}

fn units<T: Width>(bytes: &[u8]) -> Vec<T> {
    bytes.iter().map(|&b| T::from(b)).collect()
}

/// Runs `copy_s` on a copy of `dest`, and returns whether it succeeded with the resulting buffer.
fn copy_s_on<T: Width>(dest: &[T], src: Option<&[T]>, count: usize) -> (bool, Vec<T>) {
    let mut d = dest.to_vec();
    let ok = char_traits::copy_s(Some(d.as_mut_slice()), src, count).is_ok();
    (ok, d)
}

/// Runs `copy` on a copy of `dest`, and returns whether it succeeded with the resulting buffer.
fn copy_on<T: Width>(dest: &[T], src: Option<&[T]>, count: usize) -> (bool, Vec<T>) {
    let mut d = dest.to_vec();
    let ok = char_traits::copy(Some(d.as_mut_slice()), src, count).is_ok();
    (ok, d)
}

fn six_bb<T: Width>(left: &[T], right: &[T]) -> [bool; 6] {
    [
        basic_string::equal(left, right),
        basic_string::not_equal(left, right),
        basic_string::lower(left, right),
        basic_string::leq(left, right),
        basic_string::greater(left, right),
        basic_string::geq(left, right),
    ]
}

fn six_bc<T: Width>(left: &[T], right: &[T]) -> [bool; 6] {
    [
        basic_string::equal_str_cstr(left, right),
        basic_string::not_equal_str_cstr(left, right),
        basic_string::lower_bstr_cstr(left, right),
        basic_string::leq_bstr_cstr(left, right),
        basic_string::greater_bstr_cstr(left, right),
        basic_string::geq_bstr_cstr(left, right),
    ]
}

fn six_cb<T: Width>(left: &[T], right: &[T]) -> [bool; 6] {
    [
        basic_string::equal_cstr_str(left, right),
        basic_string::not_equal_cstr_str(left, right),
        basic_string::lower_cstr_bstr(left, right),
        basic_string::leq_cstr_bstr(left, right),
        basic_string::greater_cstr_bstr(left, right),
        basic_string::geq_cstr_bstr(left, right),
    ]
}

fn check_compare<T: Width>() {
    let abc = units::<T>(&[1, 2, 3]);
    let abd = units::<T>(&[1, 2, 4]);
    assert_eq!(char_traits::compare(&abc, &abd, 3), -1);
    assert_eq!(char_traits::compare(&abd, &abc, 3), 1);
    assert_eq!(char_traits::compare(&abc, &abd, 2), 0);
    assert_eq!(char_traits::compare(&abc, &abd, 0), 0);
    let high = units::<T>(&[0xff]);
    let one = units::<T>(&[1]);
    assert_eq!(char_traits::compare(&high, &one, 1), 1);
    assert_eq!(char_traits::compare(&one, &high, 1), -1);
}

fn check_length<T: Width>() {
    assert_eq!(char_traits::length::<T>(&[]), 0);
    assert_eq!(char_traits::length(&units::<T>(&[1, 2])), 2);
    assert_eq!(char_traits::length(&units::<T>(&[1, 2, 0, 5])), 2);
    assert_eq!(char_traits::length(&units::<T>(&[0, 1])), 0);
}

fn check_cstr<T: Width>() {
    let with_zero = units::<T>(&[1, 0, 2]);
    assert_eq!(char_traits::cstr(&with_zero), units::<T>(&[1]).as_slice());
    let no_zero = units::<T>(&[1, 2]);
    assert_eq!(char_traits::cstr(&no_zero), no_zero.as_slice());
}

fn check_copy_s<T: Width>() {
    let src = units::<T>(&[1, 2, 3]);
    assert_eq!(
        copy_s_on(&units::<T>(&[9, 9, 9]), Some(&src), 2),
        (true, units::<T>(&[1, 2, 9]))
    );
    assert_eq!(
        copy_s_on(&units::<T>(&[]), Some(&units::<T>(&[])), 0),
        (true, units::<T>(&[]))
    );
    assert_eq!(
        copy_s_on(&units::<T>(&[5]), None, 0),
        (false, units::<T>(&[0]))
    );
    assert_eq!(
        copy_s_on(&units::<T>(&[]), None, 1),
        (false, units::<T>(&[]))
    );
    assert_eq!(
        copy_s_on(&units::<T>(&[7]), Some(&units::<T>(&[1, 2])), 2),
        (false, units::<T>(&[0]))
    );
    assert_eq!(
        copy_s_on(&units::<T>(&[]), Some(&src), 1),
        (false, units::<T>(&[]))
    );
    assert_eq!(
        copy_s_on(&units::<T>(&[7, 7]), Some(&units::<T>(&[1])), 2),
        (false, units::<T>(&[0, 7]))
    );
    assert!(char_traits::copy_s::<T>(None, Some(&src), 1).is_err());
}

fn check_copy<T: Width>() {
    let src = units::<T>(&[1, 2, 3]);
    assert_eq!(
        copy_on(&units::<T>(&[9, 9, 9]), Some(&src), 2),
        (true, units::<T>(&[1, 2, 9]))
    );
    assert_eq!(
        copy_on(&units::<T>(&[5]), None, 0),
        (false, units::<T>(&[5]))
    );
    assert_eq!(
        copy_on(&units::<T>(&[5]), None, 1),
        (false, units::<T>(&[0]))
    );
    assert_eq!(
        copy_on(&units::<T>(&[5]), Some(&units::<T>(&[1, 2])), 2),
        (false, units::<T>(&[0]))
    );
    assert!(char_traits::copy::<T>(None, Some(&src), 1).is_err());
}

fn check_char_find<T: Width>() {
    let s = units::<T>(&[1, 2, 3]);
    assert_eq!(char_traits::find(&s, T::from(2u8)), Some(1));
    assert_eq!(char_traits::find(&s, T::from(9u8)), None);
    assert_eq!(char_traits::find::<T>(&[], T::from(1u8)), None);
}

fn check_assign<T: Width>() {
    let mut s = units::<T>(&[1, 2, 3]);
    char_traits::assign(&mut s, 2, T::from(9u8));
    assert_eq!(s, units::<T>(&[9, 9, 3]));
    char_traits::assign(&mut s, 0, T::from(7u8));
    assert_eq!(s, units::<T>(&[9, 9, 3]));
}

fn check_find<T: Width>() {
    let s = units::<T>(&[1, 2, 1, 2]);
    let ab = units::<T>(&[1, 2]);
    assert_eq!(basic_string::find(&s, &[], 0), 0);
    assert_eq!(basic_string::find(&s, &[], 4), 4);
    assert_eq!(basic_string::find(&s, &[], 5), NPOS);
    assert_eq!(basic_string::find(&s, &ab, 1), 2);
    assert_eq!(basic_string::find(&s, &ab, 3), NPOS);
    assert_eq!(basic_string::find(&s, &ab, 4), NPOS);
    assert_eq!(basic_string::find(&units::<T>(&[1]), &ab, 0), NPOS);
    assert_eq!(basic_string::find(&s, &units::<T>(&[3]), 0), NPOS);
}

fn check_rfind<T: Width>() {
    let s = units::<T>(&[1, 2, 1, 2]);
    let ab = units::<T>(&[1, 2]);
    assert_eq!(basic_string::rfind(&s, &[], 1), 1);
    assert_eq!(basic_string::rfind(&s, &[], 9), 4);
    assert_eq!(basic_string::rfind(&units::<T>(&[1]), &ab, 0), NPOS);
    assert_eq!(basic_string::rfind(&s, &ab, 9), 2);
    assert_eq!(basic_string::rfind(&s, &ab, 1), 0);
    assert_eq!(
        basic_string::rfind(&units::<T>(&[1, 2]), &units::<T>(&[3]), 1),
        NPOS
    );
}

fn check_find_first_of<T: Width>() {
    let s = units::<T>(&[1, 2, 3, 2]);
    assert_eq!(basic_string::find_first_of(&s, &[], 0), NPOS);
    assert_eq!(basic_string::find_first_of(&s, &units::<T>(&[2]), 4), NPOS);
    assert_eq!(basic_string::find_first_of(&s, &units::<T>(&[3, 2]), 0), 1);
    assert_eq!(basic_string::find_first_of(&s, &units::<T>(&[2]), 2), 3);
    assert_eq!(basic_string::find_first_of(&s, &units::<T>(&[9]), 0), NPOS);
}

fn check_find_first_not_of<T: Width>() {
    let s = units::<T>(&[1, 2, 3]);
    assert_eq!(
        basic_string::find_first_not_of(&s, &units::<T>(&[9]), 3),
        NPOS
    );
    assert_eq!(basic_string::find_first_not_of(&s, &units::<T>(&[1]), 0), 1);
    assert_eq!(
        basic_string::find_first_not_of(&units::<T>(&[1, 2]), &units::<T>(&[1, 2]), 0),
        NPOS
    );
    assert_eq!(
        basic_string::find_first_not_of(&units::<T>(&[1, 2]), &[], 1),
        1
    );
}

fn check_find_last<T: Width>() {
    let s = units::<T>(&[1, 2, 1]);
    let empty = units::<T>(&[]);
    let one = units::<T>(&[1]);
    assert_eq!(basic_string::find_last_of(&s, &[], 9), NPOS);
    assert_eq!(basic_string::find_last_not_of(&s, &[], 9), NPOS);
    assert_eq!(basic_string::find_last_of(&empty, &one, 0), NPOS);
    assert_eq!(basic_string::find_last_not_of(&empty, &one, 0), NPOS);
    assert_eq!(basic_string::find_last_of(&s, &one, 9), 2);
    assert_eq!(basic_string::find_last_of(&s, &one, 1), 0);
    assert_eq!(
        basic_string::find_last_of(&units::<T>(&[1, 2]), &units::<T>(&[9]), 1),
        NPOS
    );
    assert_eq!(
        basic_string::find_last_not_of(&units::<T>(&[1, 2, 2]), &units::<T>(&[2]), 9),
        0
    );
    assert_eq!(
        basic_string::find_last_not_of(&units::<T>(&[1, 2, 2]), &units::<T>(&[1, 2]), 9),
        NPOS
    );
}

fn check_compare_strings<T: Width>() {
    let ab = units::<T>(&[1, 2]);
    assert_eq!(basic_string::compare(&ab, &units::<T>(&[1, 2])), 0);
    assert_eq!(basic_string::compare(&units::<T>(&[1]), &ab), -1);
    assert_eq!(basic_string::compare(&ab, &units::<T>(&[1])), 1);
    assert_eq!(
        basic_string::compare(&units::<T>(&[1, 2]), &units::<T>(&[1, 3])),
        -1
    );
    assert_eq!(basic_string::compare(&units::<T>(&[]), &units::<T>(&[])), 0);
    assert_eq!(
        basic_string::compare_cstr(&ab, &units::<T>(&[1, 2, 0, 9])),
        0
    );
    assert_eq!(basic_string::compare_cstr(&ab, &units::<T>(&[1, 0, 2])), 1);
    assert_eq!(
        basic_string::compare_cstr(&units::<T>(&[1]), &units::<T>(&[])),
        1
    );
    assert_eq!(
        basic_string::compare_cstr(&units::<T>(&[]), &units::<T>(&[0])),
        0
    );
}

fn check_compare_substr<T: Width>() {
    let s = units::<T>(&[1, 2, 3]);
    let empty = units::<T>(&[]);
    assert_eq!(
        basic_string::compare_substr(&units::<T>(&[1, 2]), 3, 1, &units::<T>(&[1])),
        None
    );
    assert_eq!(
        basic_string::compare_substr(&units::<T>(&[1, 2]), 2, 5, &empty),
        Some(0)
    );
    assert_eq!(
        basic_string::compare_substr(&s, 1, 9, &units::<T>(&[2, 3])),
        Some(0)
    );
    assert_eq!(basic_string::compare_substr(&s, 0, 2, &s), Some(-1));
    assert_eq!(
        basic_string::compare_substr(&s, 0, 3, &units::<T>(&[1, 2])),
        Some(1)
    );
    assert_eq!(
        basic_string::compare_substr(&s, 1, 2, &units::<T>(&[9, 9])),
        Some(-1)
    );
}

fn check_compare_substr_substr<T: Width>() {
    let one = units::<T>(&[1]);
    assert_eq!(
        basic_string::compare_substr_substr(&one, 0, 1, &units::<T>(&[1, 2]), 3, 1),
        None
    );
    assert_eq!(
        basic_string::compare_substr_substr(
            &units::<T>(&[1, 2]),
            0,
            2,
            &units::<T>(&[9, 1, 2]),
            1,
            9
        ),
        Some(0)
    );
    assert_eq!(
        basic_string::compare_substr_substr(&one, 0, 1, &one, 1, 5),
        Some(1)
    );
}

fn check_operators<T: Width>() {
    let (lo, hi) = (units::<T>(&[1]), units::<T>(&[2]));
    assert_eq!(six_bb(&lo, &hi), [false, true, true, true, false, false]);
    assert_eq!(six_bb(&hi, &lo), [false, true, false, false, true, true]);
    assert_eq!(six_bb(&lo, &lo), [true, false, false, true, false, true]);
    assert_eq!(
        six_bc(&lo, &units::<T>(&[2, 0, 9])),
        [false, true, true, true, false, false]
    );
    assert_eq!(
        six_bc(&hi, &units::<T>(&[1, 0])),
        [false, true, false, false, true, true]
    );
    assert_eq!(
        six_bc(&lo, &units::<T>(&[1, 0, 9])),
        [true, false, false, true, false, true]
    );
    assert_eq!(
        six_cb(&units::<T>(&[1, 0, 9]), &hi),
        [false, true, true, true, false, false]
    );
    assert_eq!(
        six_cb(&units::<T>(&[2, 0]), &lo),
        [false, true, false, false, true, true]
    );
    assert_eq!(
        six_cb(&units::<T>(&[1, 0]), &lo),
        [true, false, false, true, false, true]
    );
}

macro_rules! both_widths {
    ($($name:ident => $check:ident;)*) => {
        $(
            #[test]
            fn $name() {
                $check::<u8>();
                $check::<u16>();
            }
        )*
    };
}

both_widths! {
    char_traits_compare_edges => check_compare;
    char_traits_length_edges => check_length;
    char_traits_cstr_edges => check_cstr;
    char_traits_copy_s_edges => check_copy_s;
    char_traits_copy_edges => check_copy;
    char_traits_find_edges => check_char_find;
    char_traits_assign_edges => check_assign;
    basic_string_find_edges => check_find;
    basic_string_rfind_edges => check_rfind;
    basic_string_find_first_of_edges => check_find_first_of;
    basic_string_find_first_not_of_edges => check_find_first_not_of;
    basic_string_find_last_edges => check_find_last;
    basic_string_compare_edges => check_compare_strings;
    basic_string_compare_substr_edges => check_compare_substr;
    basic_string_compare_substr_substr_edges => check_compare_substr_substr;
    basic_string_operator_edges => check_operators;
}

#[test]
fn ios_base_fmtflags_edges() {
    let mut f = 0x1_0003;
    assert_eq!(ios_base::flags_set(&mut f, 0x1_0005), 0x1_0003);
    assert_eq!(f, 0x5);
    assert_eq!(ios_base::flags_get(&f), 0x5);
    assert_eq!(ios_base::setf(&mut f, 0x1_0006), 0x5);
    assert_eq!(f, 0x7);
    let mut g = 0x0f;
    assert_eq!(ios_base::setf_mask(&mut g, 0x1_0030, 0x3c), 0x0f);
    assert_eq!(g, 0x33);
    let mut h = 0x1_0001;
    assert_eq!(ios_base::setf_mask(&mut h, 0x2, 0x1_0002), 0x1_0001);
    assert_eq!(h, 0x3);
    let mut k = 0x1_0003;
    ios_base::unsetf(&mut k, 0x1_0001);
    assert_eq!(k, 0x2);
}
