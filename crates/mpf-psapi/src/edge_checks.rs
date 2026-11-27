//! Edge-case tests for the ported `psapi.spec` table.
//!
//! These cover the export lookup, the link targets, the stdcall byte count around the empty and
//! mixed signatures, and the `Makefile.in` constants. Not every expectation is oracle-derived;
//! the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from dlls/psapi/Makefile.in. Row 01 of the oracle confirms
    // MODULE_NAME.
    assert_eq!(MODULE_NAME, "psapi.dll");
    assert_eq!(IMPORT_LIB, "psapi");
    assert_eq!(PRODUCT_VERSION, [5, 1, 2600, 3264]);
    assert_eq!(FILE_DESCRIPTION, "Wine Process Status Helper");
}

#[test]
fn table_has_sequential_ordinals_and_unique_names() {
    // Ordinals are oracle-derived: rows 02 to 28 print @1 to @27 in spec order. Uniqueness is
    // not oracle-derived; it is a check on the transcribed table.
    for (i, export) in EXPORTS.iter().enumerate() {
        assert_eq!(usize::from(export.ordinal), i + 1);
        assert_eq!(EXPORTS.iter().filter(|e| e.name == export.name).count(), 1);
    }
}

#[test]
fn every_link_target_is_the_k32_symbol_of_the_same_name() {
    // Not a rule the oracle states directly: psapi.spec writes `K32` before every export name,
    // and the `=K32...` text of each oracle row confirms it.
    for export in &EXPORTS {
        assert_eq!(export.link_name, format!("K32{}", export.name));
    }
}

#[test]
fn export_lookup_is_exact_and_case_sensitive() {
    // Not oracle-derived: lookup behaviour of the Rust table.
    let last = export("QueryWorkingSetEx").unwrap();
    assert_eq!(last.ordinal, 27);
    assert_eq!(last.link_name, "K32QueryWorkingSetEx");
    assert_eq!(export("EmptyWorkingSet").map(|e| e.ordinal), Some(1));
    assert_eq!(export("emptyworkingset"), None);
    assert_eq!(export("K32EmptyWorkingSet"), None);
    assert_eq!(export(""), None);
}

#[test]
fn stdcall_bytes_count_four_per_argument() {
    // Not oracle-derived for these synthetic signatures: winebuild's get_args_size adds
    // get_ptr_size(), which is 4 on i386, for each long or ptr argument. The `@N` values of the
    // real exports are oracle-derived in oracle_checks.rs.
    let export_with = |args: &'static [ArgKind]| Export {
        name: "Probe",
        ordinal: 1,
        args,
        link_name: "K32Probe",
    };
    assert_eq!(export_with(&[]).stdcall_bytes(), 0);
    assert_eq!(export_with(&[ArgKind::Ptr]).stdcall_bytes(), 4);
    assert_eq!(
        export_with(&[ArgKind::Long, ArgKind::Ptr, ArgKind::Long]).stdcall_bytes(),
        12
    );
    let widest = export("EnumProcessModulesEx").unwrap();
    assert_eq!(widest.stdcall_bytes(), 20);
}

#[test]
fn def_body_is_header_plus_one_line_per_export() {
    // Not oracle-derived: a structural check. The header and line text are oracle-derived.
    let body = def_body();
    assert!(body.starts_with("LIBRARY psapi.dll\n\nEXPORTS\n"));
    assert_eq!(body.lines().count(), 3 + EXPORTS.len());
    assert!(body.ends_with('\n'));
}
