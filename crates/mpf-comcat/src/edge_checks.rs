//! Edge-case tests for the ported comcat logic.
//!
//! Each expectation is either oracle-derived (its value is one of the winebuild rows in
//! `oracle_checks.rs`) or marked "Not oracle-derived" with the reason.

use super::*;

#[test]
fn export_lookup_is_exact_and_case_sensitive() {
    // Not oracle-derived: names the spec does not export, and near-miss spellings.
    assert_eq!(export("dllcanunloadnow"), None);
    assert_eq!(export("DllCanUnloadNow "), None);
    assert_eq!(export("DllGetClassObjectW"), None);
    assert_eq!(export(""), None);
}

#[test]
fn export_lookup_returns_the_table_record() {
    // Oracle-derived: the fields of each record match the rows 01-08 spec entries.
    assert_eq!(export("DllCanUnloadNow"), Some(EXPORTS[0]));
    assert_eq!(export("DllGetClassObject"), Some(EXPORTS[1]));
    assert_eq!(export("DllRegisterServer"), Some(EXPORTS[2]));
    assert_eq!(export("DllUnregisterServer"), Some(EXPORTS[3]));
    assert_eq!(EXPORTS[1].forward, Some("ole32.DllGetClassObject"));
}

#[test]
fn ordinals_follow_spec_order() {
    // Oracle-derived: winebuild numbers the four entries @1 to @4 in spec order (rows 01-04).
    for (index, export) in EXPORTS.iter().enumerate() {
        assert_eq!(export.ordinal, index as u32 + 1);
    }
}

#[test]
fn makefile_values_are_pinned() {
    // Not oracle-derived: values from dlls/comcat/Makefile.in.
    assert_eq!(MODULE, "comcat.dll");
    assert_eq!(IMPORTS, "ole32");
    assert_eq!(VER_OLESELFREGISTER, "1");
    assert_eq!(makefile_var("MODULE"), Some("comcat.dll"));
    assert_eq!(makefile_var("IMPORTS"), Some("ole32"));
    assert_eq!(makefile_var("VER_OLESELFREGISTER"), Some("1"));
}

#[test]
fn makefile_var_misses_unset_variables() {
    // Not oracle-derived: VER_PRODUCTVERSION is in tools/makedep.c's version_vars, but the
    // comcat Makefile does not set it, so makedep does not forward it. Lookups are case-sensitive.
    assert_eq!(makefile_var("VER_PRODUCTVERSION"), None);
    assert_eq!(makefile_var("module"), None);
    assert_eq!(makefile_var(""), None);
}

#[test]
fn def_line_at_the_ordinal_boundary() {
    // Not oracle-derived: winebuild was not run for a 16-bit ordinal boundary, so this only
    // checks that the number is printed in full, with no PRIVATE and no forward.
    let edge = Export {
        name: "EdgeOrdinal",
        ordinal: 65535,
        arg_bytes: 0,
        forward: None,
        private: false,
    };
    assert_eq!(def_line(&edge, Target::X86_64), "  EdgeOrdinal @65535\n");
    assert_eq!(def_line(&edge, Target::I386), "  EdgeOrdinal@0 @65535\n");
}

#[test]
fn def_file_with_no_exports_keeps_the_headers() {
    // Not oracle-derived: an empty export table. winebuild also prints a warning on stderr for an
    // empty import library, which is not part of the file.
    assert_eq!(
        def_file(MODULE, &[], Target::I386),
        "LIBRARY comcat.dll\n\nEXPORTS\n"
    );
}
