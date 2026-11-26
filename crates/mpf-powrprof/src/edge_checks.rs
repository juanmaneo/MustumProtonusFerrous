//! Edge-case tests for the ported powrprof logic.
//!
//! These cover branch and boundary behaviour. Expectations marked "not
//! oracle-derived" come from reading the C source or from the msvcrt `wcstol`
//! contract, not from a harness row.

use super::*;

fn caps(s: [u8; 6]) -> SystemPowerCapabilities {
    SystemPowerCapabilities {
        system_s1: s[0],
        system_s2: s[1],
        system_s3: s[2],
        system_s4: s[3],
        system_s5: s[4],
        hiber_file_present: s[5],
    }
}

/// UTF-16LE bytes of `text` followed by a NUL code unit.
fn wide(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .chain(std::iter::once(0))
        .flat_map(u16::to_le_bytes)
        .collect()
}

/// UTF-16LE bytes of raw code units, with no terminator added.
fn units(values: &[u16]) -> Vec<u8> {
    values.iter().flat_map(|u| u.to_le_bytes()).collect()
}

fn parse(text: &str) -> i32 {
    wcstol_decimal(&wide(text))
}

#[test]
fn hibernate_requires_both_operands() {
    // Not oracle-derived: branch table read from the C `&&`.
    assert_eq!(is_pwr_hibernate_allowed(&caps([0, 0, 0, 0, 0, 1])), 0);
    assert_eq!(is_pwr_hibernate_allowed(&caps([0, 0, 0, 1, 0, 0])), 0);
    assert_eq!(is_pwr_hibernate_allowed(&caps([0, 0, 0, 1, 0, 1])), 1);
    // Not oracle-derived: any nonzero byte counts as set.
    assert_eq!(is_pwr_hibernate_allowed(&caps([0, 0, 0, 0xff, 0, 0x80])), 1);
}

#[test]
fn suspend_requires_all_three_states() {
    // Not oracle-derived: branch table read from the C `&&`.
    assert_eq!(is_pwr_suspend_allowed(&caps([0, 1, 1, 0, 0, 0])), 0);
    assert_eq!(is_pwr_suspend_allowed(&caps([1, 0, 1, 0, 0, 0])), 0);
    assert_eq!(is_pwr_suspend_allowed(&caps([1, 1, 0, 0, 0, 0])), 0);
    assert_eq!(is_pwr_suspend_allowed(&caps([1, 1, 1, 0, 0, 0])), 1);
    // Not oracle-derived: nonzero bytes other than 1 still count as set.
    assert_eq!(
        is_pwr_suspend_allowed(&caps([0x80, 0xff, 0x02, 0, 0, 0])),
        1
    );
}

#[test]
fn shutdown_returns_the_raw_byte() {
    // 0x02, 0x80 and 0xff are oracle rows. The other values are not oracle-derived.
    for raw in [0u8, 1, 0x02, 0x7f, 0x80, 0xff] {
        assert_eq!(is_pwr_shutdown_allowed(&caps([0, 0, 0, 0, raw, 0])), raw);
    }
}

#[test]
fn capabilities_default_clone_and_debug() {
    // Not oracle-derived: derived trait plumbing only.
    let default = SystemPowerCapabilities::default();
    assert_eq!(default, caps([0; 6]));
    let set = caps([1, 0, 0, 1, 1, 1]);
    assert_eq!(Clone::clone(&set), set);
    assert!(format!("{set:?}").contains("system_s4: 1"));
}

#[test]
fn wcstol_skips_only_c_locale_space() {
    // Not oracle-derived: the C-locale space set is 0x09..=0x0d and 0x20.
    for unit in 0x09u16..=0x0d {
        assert_eq!(wcstol_decimal(&units(&[unit, 0x37])), 7);
    }
    assert_eq!(wcstol_decimal(&units(&[0x20, 0x20, 0x37])), 7);
    // 0x08, 0x0e and 0x21 are not space, so parsing stops before the digit.
    assert_eq!(wcstol_decimal(&units(&[0x08, 0x37])), 0);
    assert_eq!(wcstol_decimal(&units(&[0x0e, 0x37])), 0);
    assert_eq!(wcstol_decimal(&units(&[0x21, 0x37])), 0);
}

#[test]
fn wcstol_sign_rules() {
    // Not oracle-derived: a sign must come first and be followed by a digit.
    assert_eq!(parse("-7"), -7);
    assert_eq!(parse("+7"), 7);
    assert_eq!(parse("  +042"), 42);
    assert_eq!(parse("-"), 0);
    assert_eq!(parse("+"), 0);
    assert_eq!(parse("--7"), 0);
    assert_eq!(parse("+-7"), 0);
    assert_eq!(parse("- 7"), 0);
    assert_eq!(parse("-x"), 0);
}

#[test]
fn wcstol_clamps_at_32_bit_long_limits() {
    // Not oracle-derived: the oracle covers 2147483647, 2147483648, -2147483649,
    // 99999999999 and -99999999999. The rest are boundary values.
    assert_eq!(parse("2147483647"), i32::MAX);
    assert_eq!(parse("2147483648"), i32::MAX);
    assert_eq!(parse("-2147483648"), i32::MIN);
    assert_eq!(parse("-2147483649"), i32::MIN);
    // 2^32 is the accumulator cap. The result still saturates.
    assert_eq!(parse("4294967296"), i32::MAX);
    assert_eq!(parse("-4294967296"), i32::MIN);
    // More digits than fit in a u64.
    assert_eq!(parse("99999999999999999999999999999"), i32::MAX);
    assert_eq!(parse("-99999999999999999999999999999"), i32::MIN);
}

#[test]
fn wcstol_stops_at_nul_or_slice_end() {
    // Not oracle-derived: the NUL inside the slice ends the string.
    assert_eq!(wcstol_decimal(&units(&[0x31, 0x32, 0, 0x39])), 12);
    // Not oracle-derived: no NUL at all. The port stops at the slice end, where the C code would read past the buffer.
    assert_eq!(wcstol_decimal(&units(&[0x33, 0x34])), 34);
    // Not oracle-derived: an odd trailing byte is ignored.
    assert_eq!(wcstol_decimal(&[0x37, 0x00, 0x38]), 7);
    // Not oracle-derived: an empty slice and a leading NUL both give 0.
    assert_eq!(wcstol_decimal(&[]), 0);
    assert_eq!(wcstol_decimal(&units(&[0])), 0);
}

#[test]
fn spindown_value_buffer_boundary() {
    // Not oracle-derived: 40 bytes is the largest value that still parses.
    let mut exact = wide("5");
    exact.resize(40, 0);
    assert_eq!(disk_spindown_range(Some(&exact), None), (5, 3));
    // Not oracle-derived: 41 bytes exceeds the buffer, so ERROR_MORE_DATA takes the default.
    let mut over = wide("5");
    over.resize(41, 0);
    assert_eq!(disk_spindown_range(Some(&over), Some(&over)), (3600, 3));
}

#[test]
fn spindown_empty_value_parses_to_zero() {
    // Not oracle-derived: an empty slice and a bare NUL both have no digits, so they give 0, not the default.
    // The C code reads an uninitialized buffer on the empty-data path, so 0 is the port's defined choice.
    assert_eq!(disk_spindown_range(Some(&[]), Some(&[0, 0])), (0, 0));
}
