//! Edge-case tests for the ported dpnlobby export table.
//!
//! The oracle rows cover the one export under the four `i386` and `kill_at` combinations. These
//! tests cover what the rows do not: the lookup misses, the forward test on the link name, and the
//! `@N` arithmetic. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from dlls/dpnlobby/Makefile.in (MODULE) and dpnlobby.spec.
    assert_eq!(DLL_NAME, "dpnlobby.dll");
    assert_eq!(EXPORTS.len(), 1);
    assert_eq!(EXPORTS[0].name, "DirectPlay8LobbyCreate");
    assert_eq!(EXPORTS[0].ordinal, 1);
    assert_eq!(EXPORTS[0].pointer_args, 3);
    assert_eq!(EXPORTS[0].link_name, "dpnet.DirectPlay8Create");
}

#[test]
fn lookup_matches_names_exactly() {
    // Not oracle-derived: the name match is the model's own. DirectPlay8Create is dpnet's export,
    // not this DLL's, so it is a miss here.
    assert_eq!(export_by_name("DirectPlay8LobbyCreate"), Some(&EXPORTS[0]));
    assert_eq!(export_by_name("directplay8lobbycreate"), None);
    assert_eq!(export_by_name("DirectPlay8Create"), None);
    assert_eq!(export_by_name(""), None);
}

#[test]
fn forward_needs_a_dot_in_the_link_name() {
    // Not oracle-derived: parser.c sets FLAG_FORWARD when the link name contains '.'. A dot at
    // either end counts, since the test is only for its presence.
    let with_link = |link_name: &'static str| Export {
        name: "F",
        ordinal: 1,
        pointer_args: 0,
        link_name,
    };
    assert!(!with_link("DirectPlay8Create").is_forward());
    assert!(with_link(".").is_forward());
    assert!(with_link("dpnet.").is_forward());
    assert!(with_link(".DirectPlay8Create").is_forward());
    assert!(with_link("dpnet.DirectPlay8Create").is_forward());
}

#[test]
fn def_line_decoration_follows_the_argument_bytes() {
    // Not oracle-derived: the reference has one entry, with 3 pointer arguments. These cases vary
    // the count and the ordinal to check the arithmetic. winebuild's get_abi_name writes `name@N`
    // for an i386 stdcall entry, and N is 4 bytes per pointer argument.
    let export = Export {
        name: "Sample",
        ordinal: 300,
        pointer_args: 0,
        link_name: "x.Y",
    };
    assert_eq!(export.def_line(true, false), "Sample@0=x.Y @300");
    let export = Export {
        pointer_args: 5,
        ..export
    };
    assert_eq!(export.def_line(true, false), "Sample@20=x.Y @300");
    assert_eq!(export.def_line(true, true), "Sample=x.Y @300");
    assert_eq!(export.def_line(false, false), "Sample=x.Y @300");
}
