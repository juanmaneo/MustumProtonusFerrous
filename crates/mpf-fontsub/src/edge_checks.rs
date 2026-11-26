//! Edge-case tests for the ported fontsub logic.
//!
//! These cover the format boundary, the allocator contract, the out-parameter writes, and the
//! stub and export tables. Not every expectation is oracle-derived; the ones that aren't say so
//! in a comment.

use super::*;
use std::cell::Cell;

fn sentinel() -> FontPackageOut {
    FontPackageOut {
        dest: Some(vec![0xaa]),
        dest_len: 0xffff_ffff,
        written: 0xffff_fffe,
    }
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/fontsub.h, include/winerror.h, include/winternl.h
    // and include/winnt.h.
    assert_eq!(TTFCFP_SUBSET, 0);
    assert_eq!(TTFCFP_SUBSET1, 1);
    assert_eq!(TTFCFP_DELTA, 2);
    assert_eq!(ERR_GENERIC, 1000);
    assert_eq!(ERR_MEM, 1005);
    assert_eq!(NO_ERROR, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn only_subset_format_reaches_the_allocator() {
    // Not oracle-derived: the format boundary around TTFCFP_SUBSET. The C code checks the format
    // before it calls allocproc. One counting allocator serves every call, so the count is 1
    // after the accepted format and stays 1 after each rejected format.
    let calls = Cell::new(0);
    let alloc = |size: usize| {
        calls.set(calls.get() + 1);
        Some(vec![0; size])
    };
    let mut out = sentinel();
    assert_eq!(
        create_font_package(b"x", TTFCFP_SUBSET, &mut out, alloc),
        NO_ERROR
    );
    assert_eq!(calls.get(), 1);

    for format in [TTFCFP_SUBSET1, TTFCFP_DELTA, 3, u16::MAX] {
        let mut out = sentinel();
        assert_eq!(
            create_font_package(b"x", format, &mut out, alloc),
            ERR_GENERIC
        );
        assert_eq!(out, sentinel());
    }
    assert_eq!(calls.get(), 1);
}

#[test]
fn oversized_block_keeps_its_tail_and_reports_source_length() {
    // Not oracle-derived: the C memcpy copies src_len bytes into any block at least that large,
    // and the rest of the block stays as allocated. dest_len and written are the source length.
    let mut out = sentinel();
    let ret = create_font_package(b"abc", TTFCFP_SUBSET, &mut out, |_| {
        Some(b"xxxxxx".to_vec())
    });
    assert_eq!(ret, NO_ERROR);
    assert_eq!(
        out,
        FontPackageOut {
            dest: Some(b"abcxxx".to_vec()),
            dest_len: 3,
            written: 3,
        }
    );
}

#[test]
#[should_panic(expected = "out of range")]
fn short_block_panics_where_c_would_overflow() {
    // Not oracle-derived: the C memcpy would write past a short block (undefined behaviour).
    // The port panics instead.
    let mut out = sentinel();
    create_font_package(b"abc", TTFCFP_SUBSET, &mut out, |_| Some(vec![0; 2]));
}

#[test]
fn alloc_failure_clears_only_dest() {
    // Not oracle-derived: the oracle only used sentinel values. The C code writes only *dest on
    // ERR_MEM, so the other fields keep whatever the caller stored.
    let mut out = FontPackageOut {
        dest: Some(vec![1, 2]),
        dest_len: 7,
        written: 9,
    };
    let ret = create_font_package(b"abc", TTFCFP_SUBSET, &mut out, |_| None);
    assert_eq!(ret, ERR_MEM);
    assert_eq!(
        out,
        FontPackageOut {
            dest: None,
            dest_len: 7,
            written: 9,
        }
    );
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of fontsub.spec. The code and
    // flags are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the two lines of dlls/fontsub/fontsub.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("CreateFontPackage", ExportKind::Cdecl),
            ("MergeFontPackage", ExportKind::Stub),
        ]
    );
    assert_eq!(export_kind("CreateFontPackage"), Some(ExportKind::Cdecl));
    assert_eq!(export_kind("MergeFontPackage"), Some(ExportKind::Stub));
    assert_eq!(export_kind("createfontpackage"), None);
    assert_eq!(export_kind(""), None);
}
