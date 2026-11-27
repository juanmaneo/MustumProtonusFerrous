//! Edge-case tests for the ported `lz32.spec` table.
//!
//! These cover the export lookup, the forward split, the stdcall byte count for each argument
//! kind, and the `Makefile.in` constants. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from dlls/lz32/Makefile.in.
    assert_eq!(MODULE_NAME, "lz32.dll");
    assert_eq!(IMPORT_LIB, "lz32");
    assert_eq!(PRODUCT_VERSION, [5, 1, 2600, 2180]);
}

#[test]
fn table_is_in_spec_order_with_sequential_ordinals() {
    // Ordinals are oracle-derived (rows 02 to 13 print @1 to @12). The names and their order
    // come from dlls/lz32/lz32.spec, and the oracle prints them in the same order.
    let names: Vec<&str> = EXPORTS.iter().map(|export| export.name).collect();
    assert_eq!(
        names,
        [
            "CopyLZFile",
            "GetExpandedNameA",
            "GetExpandedNameW",
            "LZClose",
            "LZCopy",
            "LZDone",
            "LZInit",
            "LZOpenFileA",
            "LZOpenFileW",
            "LZRead",
            "LZSeek",
            "LZStart",
        ]
    );
    for (index, export) in EXPORTS.iter().enumerate() {
        assert_eq!(usize::from(export.ordinal), index + 1);
    }
}

#[test]
fn every_export_forwards_to_kernel32_with_its_own_name() {
    // Oracle-derived: rows 02 to 13 print `kernel32.<name>` for every export.
    for export in EXPORTS {
        assert_eq!(export.forward(), Some(("kernel32", export.name)));
    }
}

#[test]
fn stdcall_bytes_follow_the_argument_kinds() {
    // Oracle-derived for the sums: LZDone@0 and LZStart@0 (no arguments), LZClose@4 and LZInit@4
    // (one long), LZCopy@8 and CopyLZFile@8 (two longs), LZOpenFileA@12 (str, ptr, long).
    // Not oracle-derived: the per-kind size of 4 for ptr, str and wstr. It follows
    // winebuild's get_args_size with get_ptr_size() == 4 on i386.
    let mixed = Export {
        name: "Mixed",
        ordinal: 1,
        args: &[ArgKind::WStr, ArgKind::Ptr, ArgKind::Str, ArgKind::Long],
        target: "kernel32.Mixed",
    };
    assert_eq!(mixed.stdcall_bytes(), 16);

    let none = Export {
        name: "None",
        ordinal: 1,
        args: &[],
        target: "kernel32.None",
    };
    assert_eq!(none.stdcall_bytes(), 0);
}

#[test]
fn forward_split_is_at_the_first_dot() {
    // Not oracle-derived: winebuild marks a target as forwarded when strchr finds a `.`
    // (tools/winebuild/parser.c), so the split point is the first `.`. The oracle row 14 writes
    // the dotted target `a.b.c` back unchanged but does not show where the split falls.
    let dotted = Export {
        name: "Dotted",
        ordinal: 1,
        args: &[],
        target: "a.b.c",
    };
    assert_eq!(dotted.forward(), Some(("a", "b.c")));

    let leading = Export {
        name: "Leading",
        ordinal: 1,
        args: &[],
        target: ".fn",
    };
    assert_eq!(leading.forward(), Some(("", "fn")));

    let trailing = Export {
        name: "Trailing",
        ordinal: 1,
        args: &[],
        target: "mod.",
    };
    assert_eq!(trailing.forward(), Some(("mod", "")));
}

#[test]
fn target_without_a_dot_is_not_a_forward() {
    // Not oracle-derived: winebuild only marks targets that contain a `.` as forwards
    // (tools/winebuild/parser.c). Oracle row 15 writes `LocalImpl` back unchanged, so the
    // target text is the same either way.
    let local = Export {
        name: "Local",
        ordinal: 1,
        args: &[ArgKind::Long],
        target: "LocalImpl",
    };
    assert_eq!(local.forward(), None);
}

#[test]
fn export_lookup_finds_names_and_is_case_sensitive() {
    // Not oracle-derived: the lookup is this crate's helper over the oracle-derived table.
    assert_eq!(export("LZRead").map(|export| export.ordinal), Some(10));
    assert_eq!(export("LZStart").map(|export| export.args.len()), Some(0));
    assert_eq!(export("lzread"), None);
    assert_eq!(export("LZRead "), None);
    assert_eq!(export(""), None);
}

#[test]
fn def_body_has_no_generated_header() {
    // Oracle-derived: the oracle output starts with the `LIBRARY` line. The header comment that
    // winebuild prints first is left out on purpose.
    let body = def_body();
    assert!(body.starts_with("LIBRARY lz32.dll\n\nEXPORTS\n"));
    assert!(!body.contains(';'));
    assert!(body.ends_with("  LZStart@0=kernel32.LZStart @12\n"));
}
