//! Edge-case tests for the ported crtdll logic.
//!
//! These cover the export table, the stub helpers, the integer and bit boundaries, the float
//! classification and the `swab` buffer contract. Not every expectation is oracle-derived; the ones
//! that aren't say so in a comment.

use super::*;

#[test]
fn stub_constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_the_spec() {
    // Not oracle-derived: the counts are the `@ <kind>` lines of dlls/crtdll/crtdll.spec, counted
    // with grep. The first and last entries are the first and last `@` lines of that file.
    assert_eq!(EXPORTS.len(), 523);
    let count = |kind| EXPORTS.iter().filter(|(_, k)| *k == kind).count();
    assert_eq!(count(ExportKind::Cdecl), 457);
    assert_eq!(count(ExportKind::Extern), 31);
    assert_eq!(count(ExportKind::Stub), 9);
    assert_eq!(count(ExportKind::Varargs), 26);
    assert_eq!(EXPORTS[0], ("??2@YAPAXI@Z", ExportKind::Cdecl));
    assert_eq!(EXPORTS[522], ("wscanf", ExportKind::Varargs));
}

#[test]
fn export_kind_finds_every_entry_and_rejects_others() {
    for &(name, kind) in EXPORTS.iter() {
        assert_eq!(export_kind(name), Some(kind));
    }
    // Not oracle-derived: spot checks against crtdll.spec.
    assert_eq!(export_kind("_getsystime"), Some(ExportKind::Stub));
    assert_eq!(export_kind("_HUGE_dll"), Some(ExportKind::Extern));
    assert_eq!(export_kind("printf"), Some(ExportKind::Varargs));
    // Names are case-sensitive, and names from other DLLs are not exported here.
    assert_eq!(export_kind("_GETSYSTIME"), None);
    assert_eq!(export_kind("MergeFontPackage"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_helpers_match_the_spec_stub_exports() {
    // Not oracle-derived: every stub helper names one of the spec's `@ stub` exports, and together
    // they name all nine of them.
    let helpers = [
        getsystime(),
        setsystime(),
        ismbbalnum(),
        ismbbalpha(),
        ismbbgraph(),
        ismbbkalnum(),
        ismbbkpunct(),
        ismbbprint(),
        ismbbpunct(),
    ];
    let mut helper_names: Vec<&str> = helpers.iter().map(|h| h.function).collect();
    let mut stub_names: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    helper_names.sort_unstable();
    stub_names.sort_unstable();
    assert_eq!(helper_names, stub_names);
    for helper in helpers {
        assert_eq!(export_kind(helper.function), Some(ExportKind::Stub));
        assert_eq!(helper.module, "crtdll.dll");
    }
}

#[test]
fn stub_record_echoes_module_and_function() {
    // Not oracle-derived: the record carries whatever names it is given, as the C function does.
    assert_eq!(
        wine_spec_unimplemented_stub("fontsub.dll", "MergeFontPackage"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "fontsub.dll",
            function: "MergeFontPackage",
        }
    );
}

#[test]
fn abs_and_labs_wrap_at_int_min() {
    // Not oracle-derived: abs(INT_MIN) is undefined in the C body, and UBSan flags it (oracle row
    // 24, excluded). The port gives the x86 result, which is INT_MIN.
    assert_eq!(abs(i32::MIN), i32::MIN);
    assert_eq!(labs(i32::MIN), i32::MIN);
    assert_eq!(abs(i32::MAX), i32::MAX);
    assert_eq!(abs(0), 0);
    assert_eq!(labs(-i32::MAX), i32::MAX);
}

#[test]
fn div_and_ldiv_truncate_toward_zero() {
    // Not oracle-derived beyond rows 12 and 13: the quotient truncates toward zero and the
    // remainder takes the sign of the dividend.
    assert_eq!(div(7, -2), DivResult { quot: -3, rem: 1 });
    assert_eq!(ldiv(-7, -2), DivResult { quot: 3, rem: -1 });
    assert_eq!(
        div(i32::MIN, 1),
        DivResult {
            quot: i32::MIN,
            rem: 0
        }
    );
    assert_eq!(ldiv(i32::MAX, i32::MAX), DivResult { quot: 1, rem: 0 });
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn div_by_zero_panics_where_c_traps() {
    // Not oracle-derived: the C division traps with SIGFPE, which the harness cannot record.
    div(1, std::hint::black_box(0));
}

#[test]
#[should_panic(expected = "attempt to divide with overflow")]
fn ldiv_int_min_by_minus_one_panics_where_c_traps() {
    // Not oracle-derived: the x86 idiv traps on this quotient, which the harness cannot record.
    ldiv(i32::MIN, std::hint::black_box(-1));
}

#[test]
fn rotations_mask_the_shift_to_five_bits() {
    // Not oracle-derived: the masked shift `shift & 31` maps negative shifts into 0..32.
    assert_eq!(rotl(1, -1), 0x8000_0000);
    assert_eq!(rotl(1, -31), 2);
    assert_eq!(rotr(1, -31), 0x8000_0000);
    assert_eq!(rotl(0x8000_0001, 32), 0x8000_0001);
    // The masked shift of 0 is undefined in the C body (oracle row 25, excluded). The port gives
    // the x86 result, which is the value unchanged.
    assert_eq!(rotl(0x8000_0001, 0), 0x8000_0001);
    assert_eq!(rotr(0x8000_0001, 0), 0x8000_0001);
}

#[test]
fn rotations_invert_each_other() {
    // Not oracle-derived: rotating left and then right by the same shift returns the value.
    for shift in 0..32 {
        assert_eq!(rotr(rotl(0x1234_5678, shift), shift), 0x1234_5678);
        assert_eq!(lrotr(lrotl(0x1234_5678, shift), shift), 0x1234_5678);
    }
}

#[test]
fn long_rotations_are_the_32_bit_rotations() {
    // Not oracle-derived: unsigned long is 32 bits on the target, so _lrotl and _lrotr equal _rotl
    // and _rotr for every shift.
    for shift in [-33, -1, 0, 1, 31, 33] {
        assert_eq!(lrotl(0xdead_beef, shift), rotl(0xdead_beef, shift));
        assert_eq!(lrotr(0xdead_beef, shift), rotr(0xdead_beef, shift));
    }
}

#[test]
fn chgsign_flips_only_the_sign_bit() {
    // Not oracle-derived beyond row 18: zeros, infinities and a NaN payload.
    assert_eq!(chgsign(-0.0).to_bits(), 0);
    assert_eq!(chgsign(0.0).to_bits(), 0x8000_0000_0000_0000);
    assert_eq!(chgsign(f64::INFINITY), f64::NEG_INFINITY);
    let nan = f64::from_bits(0x7ff8_0000_0000_0001);
    assert_eq!(chgsign(nan).to_bits(), 0xfff8_0000_0000_0001);
}

#[test]
fn finite_and_isnan_classify_the_exponent_bits() {
    // Not oracle-derived beyond rows 19 and 20: zeros, subnormals, extremes, infinities and NaN
    // payloads, whatever the sign.
    for value in [0.0, -0.0, f64::MIN_POSITIVE / 2.0, 1.0, f64::MAX, f64::MIN] {
        assert_eq!(finite(value), 1);
        assert_eq!(isnan(value), 0);
    }
    for value in [f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(finite(value), 0);
        assert_eq!(isnan(value), 0);
    }
    for value in [f64::NAN, -f64::NAN, f64::from_bits(0x7ff0_0000_0000_0001)] {
        assert_eq!(finite(value), 0);
        assert_eq!(isnan(value), 1);
    }
}

#[test]
fn toupper_and_tolower_shift_every_int() {
    // Not oracle-derived beyond rows 21 and 22: the `/* sic */` shift applies to every int, not
    // only to letters.
    assert_eq!(toupper(b'A' as i32), 0x21);
    assert_eq!(toupper(0), -32);
    assert_eq!(tolower(b'z' as i32), 0x9a);
    assert_eq!(tolower(-1), 31);
    // The C shift overflows at the ends of the int range (undefined behaviour). The port wraps,
    // as the x86 instruction does.
    assert_eq!(toupper(i32::MIN), i32::MAX - 31);
    assert_eq!(tolower(i32::MAX), i32::MIN + 31);
}

#[test]
fn swab_copies_pairs_and_skips_short_lengths() {
    // Not oracle-derived beyond row 23: lengths of 1 or less, including every negative length, copy
    // nothing, and an odd length leaves the last byte alone.
    let src = *b"ABCDE";
    for len in [i32::MIN, -2, 0, 1] {
        let mut dst = [b'?'; 5];
        swab(&src, &mut dst, len);
        assert_eq!(dst, [b'?'; 5]);
    }
    let mut dst = [b'?'; 5];
    swab(&src, &mut dst, 2);
    assert_eq!(&dst, b"BA???");
    let mut dst = [b'?'; 5];
    swab(&src, &mut dst, 3);
    assert_eq!(&dst, b"BA???");

    // Not oracle-derived: a full pair run.
    let mut dst = [0u8; 6];
    swab(b"ABCDEF", &mut dst, 6);
    assert_eq!(&dst, b"BADCFE");
}

#[test]
#[should_panic(expected = "range end index")]
fn swab_panics_on_a_short_destination() {
    // Not oracle-derived: the C function writes past the end of dst, and the port panics instead.
    let mut dst = [0u8; 3];
    swab(b"ABCD", &mut dst, 4);
}

#[test]
#[should_panic(expected = "range end index")]
fn swab_panics_on_a_short_source() {
    // Not oracle-derived: the C function reads past the end of src, and the port panics instead.
    let mut dst = [0u8; 4];
    swab(b"AB", &mut dst, 4);
}
