//! Replays of the oracle rows in `target/canon/msvcp60`. The oracle compiles the verbatim Wine
//! `string.c` and `ios.c` with `gcc -fsanitize=address,undefined -g`. `gen_rows.py run` feeds each
//! input line to the harness and records the output line. `run` replays that input line on this
//! port and returns the same text.
//!
//! Every row runs at both widths. All row values fit in a byte, so the `u8` and `u16` replays must
//! both produce the recorded text.
//!
//! Generated from `oracle_checks.tmpl.rs` by `gen_rows.py emit`. Do not edit by hand.

use crate::basic_string;
use crate::char_traits::{self, InvalidParameter, Unit};
use crate::ios_base;
use crate::NPOS;

/// Test-only conversion for the oracle's `%04x` units.
trait Hex: Unit {
    fn from_hex(digits: &str) -> Self;
    fn to_hex(self) -> String;
}

impl Hex for u8 {
    fn from_hex(digits: &str) -> Self {
        u8::try_from(u16::from_str_radix(digits, 16).unwrap()).unwrap()
    }
    fn to_hex(self) -> String {
        format!("{:04x}", u16::from(self))
    }
}

impl Hex for u16 {
    fn from_hex(digits: &str) -> Self {
        u16::from_str_radix(digits, 16).unwrap()
    }
    fn to_hex(self) -> String {
        format!("{self:04x}")
    }
}

/// Units of an oracle token. `-` is the empty buffer; any other token is 4 hex digits per unit.
fn units<T: Hex>(tok: &str) -> Vec<T> {
    if tok == "-" {
        return Vec::new();
    }
    (0..tok.len())
        .step_by(4)
        .map(|i| T::from_hex(&tok[i..i + 4]))
        .collect()
}

/// Units of a C string token. The oracle appends a zero unit to it.
fn cstr_units<T: Hex>(tok: &str) -> Vec<T> {
    let mut buf = units::<T>(tok);
    buf.push(T::default());
    buf
}

/// A token that may be `null`.
fn pointer<T: Hex>(tok: &str) -> Option<Vec<T>> {
    (tok != "null").then(|| units(tok))
}

fn num(tok: &str) -> usize {
    tok.parse().unwrap()
}

fn hex(tok: &str) -> u32 {
    u32::from_str_radix(tok, 16).unwrap()
}

/// The oracle's buffer text: `-` for an empty buffer, otherwise the units.
fn put<T: Hex>(buf: &[T]) -> String {
    if buf.is_empty() {
        return "-".to_string();
    }
    buf.iter().map(|&c| c.to_hex()).collect()
}

fn verdict(r: Result<(), InvalidParameter>) -> &'static str {
    if r.is_ok() {
        "ok"
    } else {
        "invalid"
    }
}

fn render_index(i: usize) -> String {
    if i == NPOS {
        "npos".to_string()
    } else {
        i.to_string()
    }
}

fn render_compare(r: Option<i32>) -> String {
    match r {
        Some(v) => v.to_string(),
        None => "throw".to_string(),
    }
}

fn six(flags: [bool; 6]) -> String {
    flags.map(|f| u8::from(f).to_string()).join(" ")
}

/// The `bs_find` family. The needle is the first `len` units of its token.
fn search<T: Hex>(f: fn(&[T], &[T], usize) -> usize, [s, needle, pos, len]: [&str; 4]) -> String {
    let needle = units::<T>(needle);
    render_index(f(&units::<T>(s), &needle[..num(len)], num(pos)))
}

fn run<T: Hex>(line: &str) -> String {
    let w: Vec<&str> = line.split_whitespace().collect();
    match w.as_slice() {
        ["ct_length", _, s] => char_traits::length(&cstr_units::<T>(s)).to_string(),
        ["ct_copy_s", _, dest, src, count] => {
            let mut d = units::<T>(dest);
            let s = pointer::<T>(src);
            let r = char_traits::copy_s(Some(d.as_mut_slice()), s.as_deref(), num(count));
            format!("{} {}", verdict(r), put(&d))
        }
        ["ct_find", _, s, c, range] => {
            let s = units::<T>(s);
            match char_traits::find(&s[..num(range)], units::<T>(c)[0]) {
                Some(i) => i.to_string(),
                None => "none".to_string(),
            }
        }
        ["ct_assignn", _, s, count, c] => {
            let mut buf = units::<T>(s);
            char_traits::assign(&mut buf, num(count), units::<T>(c)[0]);
            put(&buf)
        }
        ["bs_find", _, s, needle, pos, len] => {
            search::<T>(basic_string::find, [s, needle, pos, len])
        }
        ["bs_rfind", _, s, needle, pos, len] => {
            search::<T>(basic_string::rfind, [s, needle, pos, len])
        }
        ["bs_find_first_of", _, s, needle, pos, len] => {
            search::<T>(basic_string::find_first_of, [s, needle, pos, len])
        }
        ["bs_find_first_not_of", _, s, needle, pos, len] => {
            search::<T>(basic_string::find_first_not_of, [s, needle, pos, len])
        }
        ["bs_find_last_of", _, s, needle, pos, len] => {
            search::<T>(basic_string::find_last_of, [s, needle, pos, len])
        }
        ["bs_find_last_not_of", _, s, needle, pos, len] => {
            search::<T>(basic_string::find_last_not_of, [s, needle, pos, len])
        }
        ["bs_compare", _, a, b] => {
            basic_string::compare(&units::<T>(a), &units::<T>(b)).to_string()
        }
        ["bs_compare_cstr", _, a, b] => {
            basic_string::compare_cstr(&units::<T>(a), &cstr_units::<T>(b)).to_string()
        }
        ["bs_compare_substr", _, a, pos, n, b] => render_compare(basic_string::compare_substr(
            &units::<T>(a),
            num(pos),
            num(n),
            &units::<T>(b),
        )),
        ["bs_compare_substr_substr", _, a, pos, n, b, off, count] => {
            render_compare(basic_string::compare_substr_substr(
                &units::<T>(a),
                num(pos),
                num(n),
                &units::<T>(b),
                num(off),
                num(count),
            ))
        }
        ["bs_compare_substr_cstr_len", _, a, pos, n, b, count] => {
            let other = units::<T>(b);
            render_compare(basic_string::compare_substr(
                &units::<T>(a),
                num(pos),
                num(n),
                &other[..num(count)],
            ))
        }
        ["ops_bb", _, a, b] => {
            let (x, y) = (units::<T>(a), units::<T>(b));
            six([
                basic_string::equal(&x, &y),
                basic_string::not_equal(&x, &y),
                basic_string::lower(&x, &y),
                basic_string::leq(&x, &y),
                basic_string::greater(&x, &y),
                basic_string::geq(&x, &y),
            ])
        }
        ["ops_bc", _, a, b] => {
            let (x, c) = (units::<T>(a), cstr_units::<T>(b));
            six([
                basic_string::equal_str_cstr(&x, &c),
                basic_string::not_equal_str_cstr(&x, &c),
                basic_string::lower_bstr_cstr(&x, &c),
                basic_string::leq_bstr_cstr(&x, &c),
                basic_string::greater_bstr_cstr(&x, &c),
                basic_string::geq_bstr_cstr(&x, &c),
            ])
        }
        ["ops_cb", _, a, b] => {
            let (c, x) = (cstr_units::<T>(a), units::<T>(b));
            six([
                basic_string::equal_cstr_str(&c, &x),
                basic_string::not_equal_cstr_str(&c, &x),
                basic_string::lower_cstr_bstr(&c, &x),
                basic_string::leq_cstr_bstr(&c, &x),
                basic_string::greater_cstr_bstr(&c, &x),
                basic_string::geq_cstr_bstr(&c, &x),
            ])
        }
        ["ios_flags_set", a, b] => {
            let mut f = hex(a);
            let old = ios_base::flags_set(&mut f, hex(b));
            format!("{old:x} {f:x}")
        }
        ["ios_setf_mask", a, b, c] => {
            let mut f = hex(a);
            let old = ios_base::setf_mask(&mut f, hex(b), hex(c));
            format!("{old:x} {f:x}")
        }
        ["ios_unsetf", a, b] => {
            let mut f = hex(a);
            ios_base::unsetf(&mut f, hex(b));
            format!("{f:x}")
        }
        _ => format!("unknown {}", w[0]),
    }
}

#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    ("ct_copy_s c - null 0", "invalid -"),
    ("ct_length c 004100000042", "1"),
    ("ct_copy_s c 0000 0041 1", "ok 0041"),
    ("ct_copy_s w 0041 null 1", "invalid 0000"),
    ("ct_find c 00410042 0042 2", "1"),
    ("ct_find w 00410042 0043 2", "none"),
    ("ct_assignn c 00000000 2 0041", "00410041"),
    ("bs_find c 00410042 0042 0 1", "1"),
    ("bs_find w 00410042 0042 3 0", "npos"),
    ("bs_rfind c 00410042 0041 5 1", "0"),
    ("bs_find_first_of w 00410042 0042 0 1", "1"),
    ("bs_find_first_not_of c 00410042 0041 0 1", "1"),
    ("bs_find_last_of w 00410042 0041 9 1", "0"),
    ("bs_find_last_not_of c 00410042 0042 1 0", "npos"),
    ("bs_compare w 00410042 00410043", "-1"),
    ("bs_compare_cstr c 00410042 00410042", "0"),
    ("bs_compare_substr w 00410042 0 2 00410043", "-1"),
    ("bs_compare_substr_substr c 00410042 1 5 00410042 0 1", "1"),
    ("bs_compare_substr_cstr_len w 00410042 3 1 00410042 1", "throw"),
    ("ops_bb c 0041 0042", "0 1 1 1 0 0"),
    ("ops_bc w 0041 0041", "1 0 0 1 0 1"),
    ("ops_cb c 0042 0041", "0 1 0 0 1 1"),
    ("ios_flags_set 1 2", "1 2"),
    ("ios_setf_mask 1 2 3", "1 2"),
    ("ios_unsetf 7 1", "6"),
];

#[test]
fn oracle_rows_match_wine() {
    for &(input, output) in ROWS {
        assert_eq!(run::<u8>(input), output, "{input} at u8");
        assert_eq!(run::<u16>(input), output, "{input} at u16");
    }
}

#[test]
fn unknown_op_is_reported() {
    assert_eq!(run::<u8>("nope c"), "unknown nope");
    assert_eq!(run::<u16>("nope c"), "unknown nope");
}
