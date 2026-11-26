//! Edge-case tests for the ported winbrand logic.
//!
//! These cover the environment lookup, the expansion boundaries (truncation, the length query and
//! unfinished variables), the allocation contract and the export tables. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;
use crate::format::BRANDING_ENVIRONMENT;
use std::cell::Cell;

fn units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn env() -> Vec<u16> {
    units(BRANDING_ENVIRONMENT)
}

/// `expand_environment_strings` into a sentinel buffer of `count` slots. Returns the status, the
/// length and the buffer.
fn run(src: &str, count: usize) -> (NtStatus, usize, Vec<u16>) {
    let mut dst = vec![0xAAAA; count];
    let (status, len) = expand_environment_strings(&env(), &units(src), &mut dst);
    (status, len, dst)
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/ntstatus.h, include/winternl.h and include/winnt.h.
    assert_eq!(STATUS_SUCCESS, 0);
    assert_eq!(STATUS_BUFFER_TOO_SMALL, 0xC000_0023u32 as i32);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn find_matches_names_case_insensitively_up_to_the_first_equals() {
    // Not oracle-derived: the ENV_FindVariable name and value rules, from dlls/ntdll/env.c.
    let block = units("A=1\0AB=2\0ABC=x=y\0");
    assert_eq!(
        find_environment_variable(&block, &units("A")),
        Some(&units("1")[..])
    );
    assert_eq!(
        find_environment_variable(&block, &units("ab")),
        Some(&units("2")[..])
    );
    assert_eq!(
        find_environment_variable(&block, &units("abc")),
        Some(&units("x=y")[..])
    );
    assert_eq!(find_environment_variable(&block, &units("AB=")), None);
    assert_eq!(
        find_environment_variable(&block, &units("AB")),
        Some(&units("2")[..])
    );
}

#[test]
fn find_rejects_empty_names_and_entries_that_do_not_fit() {
    // Not oracle-derived: the ENV_FindVariable edge cases from dlls/ntdll/env.c.
    let block = units("A=1\0");
    assert_eq!(find_environment_variable(&block, &[]), None);
    assert_eq!(find_environment_variable(&units("=X=1\0"), &[]), None);
    assert_eq!(
        find_environment_variable(&units("WINDOWS\0"), &units("WINDOWS")),
        None
    );
    assert_eq!(
        find_environment_variable(&units("WINDOWS=\0"), &units("WINDOWS")),
        Some(&[][..])
    );
    // An empty entry ends the block, so the entry after it is never seen.
    assert_eq!(
        find_environment_variable(&units("\0A=1\0"), &units("A")),
        None
    );
    assert_eq!(find_environment_variable(&[], &units("A")), None);
}

#[test]
fn find_accepts_a_block_without_its_final_nul() {
    // Not oracle-derived: the split-based walk does not read past the end of the slice.
    assert_eq!(
        find_environment_variable(&units("A=1"), &units("A")),
        Some(&units("1")[..])
    );
}

#[test]
fn length_query_counts_the_variable_value_without_writing() {
    // Not oracle-derived: an empty destination reports the length and writes nothing, here
    // for a variable (the oracle's empty-destination row only covers a literal).
    assert_eq!(
        run("%WINDOWS_SHORT%", 0),
        (STATUS_BUFFER_TOO_SMALL, 11, vec![])
    );
    assert_eq!(run("", 0), (STATUS_BUFFER_TOO_SMALL, 1, vec![]));
}

#[test]
fn empty_source_writes_only_the_terminator() {
    // Not oracle-derived: an empty source expands to the empty string.
    assert_eq!(run("", 3), (STATUS_SUCCESS, 1, vec![0, 0xAAAA, 0xAAAA]));
}

#[test]
fn lone_percent_is_copied_as_an_unfinished_name() {
    // Not oracle-derived: a single '%' has no closing '%', so it is copied as is.
    assert_eq!(
        run("%", 4),
        (STATUS_SUCCESS, 2, vec![u16::from(b'%'), 0, 0xAAAA, 0xAAAA])
    );
}

#[test]
fn truncated_unfinished_name_copies_one_less_and_keeps_the_length() {
    // Not oracle-derived: the literal branch clamps copy to count - 1, and the unfinished name
    // is counted in full.
    assert_eq!(
        run("x%ABC", 2),
        (STATUS_BUFFER_TOO_SMALL, 6, vec![u16::from(b'x'), 0xAAAA])
    );
}

#[test]
fn full_buffer_after_a_literal_gets_no_terminator() {
    // Not oracle-derived: the literal "ab" uses up both slots (copy is clamped to 1), the count
    // reaches 0, and the variable that follows is dropped without a NUL write.
    assert_eq!(
        run("ab%WINDOWS_SHORT%", 2),
        (STATUS_BUFFER_TOO_SMALL, 13, vec![u16::from(b'a'), 0xAAAA])
    );
}

#[test]
fn value_too_long_for_the_remaining_space_is_dropped_whole() {
    // Not oracle-derived: with room for part of the value, the variable is written as a NUL at
    // its position and no characters of the value are copied.
    assert_eq!(
        run("ab%WINDOWS_SHORT%", 3),
        (
            STATUS_BUFFER_TOO_SMALL,
            13,
            vec![u16::from(b'a'), u16::from(b'b'), 0]
        )
    );
}

#[test]
fn value_that_fits_exactly_is_copied_without_a_terminator_write() {
    // Not oracle-derived: a value of exactly count - 1 characters leaves the terminator slot
    // for the final NUL write.
    let (status, len, dst) = run("%WINDOWS_SHORT%", 11);
    assert_eq!((status, len), (STATUS_SUCCESS, 11));
    assert_eq!(dst[10], 0);
}

#[test]
fn branding_format_string_hands_the_allocator_the_character_count() {
    // Not oracle-derived: the count is the expansion length, which oracle row fmt 01 prints as
    // len=11 for "%WINDOWS_SHORT%".
    let asked = Cell::new(0);
    let out = branding_format_string(&units("%WINDOWS_SHORT%"), |size| {
        asked.set(size);
        Some(vec![0; size])
    });
    assert_eq!(asked.get(), 11);
    assert_eq!(out, Some(units("Windows 10\0")));
}

#[test]
fn branding_format_string_truncates_an_oversized_allocation() {
    // Not oracle-derived: an allocator that returns more than asked still yields exactly the
    // expansion and its NUL.
    let out = branding_format_string(&units("%WINDOWS_SHORT%"), |_| Some(vec![0xAAAA; 40]));
    assert_eq!(out, Some(units("Windows 10\0")));
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the stub record only copies its arguments, so other names behave the same.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the fifteen lines of dlls/winbrand/winbrand.spec, in order.
    assert_eq!(EXPORTS.len(), 15);
    assert_eq!(EXPORTS[0], ("BrandingFormatString", ExportKind::Stdcall));
    assert_eq!(
        EXPORTS[1],
        ("BrandingFormatStringForEdition", ExportKind::Stub)
    );
    assert_eq!(EXPORTS[14], ("InstallEULA", ExportKind::Stub));
    assert_eq!(
        export_kind("BrandingFormatString"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("GetEULAFileEx"), Some(ExportKind::Stub));
    assert_eq!(export_kind("branding_format_string"), None);
    assert_eq!(export_kind("installeula"), None);
    assert_eq!(export_kind(""), None);
}
