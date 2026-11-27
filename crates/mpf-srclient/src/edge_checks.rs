//! Edge-case tests for the ported srclient export table and stub model.
//!
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h. The harness takes the
    // same values from its own copies of those headers.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of srclient.spec. The code and
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
fn stub_helpers_cover_every_stub_export_in_spec_order() {
    // Not oracle-derived: checks the helper list against the EXPORTS table. Every helper must name
    // an export that the spec marks as a stub, and every stub export must have a helper.
    let helpers = [
        sysprep_cleanup(),
        sysprep_generalize(),
        disable_sr(),
        disable_sr_internal(),
        enable_sr(),
        enable_sr_ex(),
        enable_sr_internal(),
        sr_new_system_id(),
        sr_remove_restore_point(),
        sr_set_restore_point_internal(),
        set_sr_state_after_setup(),
    ];
    assert!(helpers.iter().all(|helper| helper.module == "srclient.dll"));

    let functions: Vec<&str> = helpers.iter().map(|helper| helper.function).collect();
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, _, kind)| *kind == ExportKind::Stub)
        .map(|&(_, name, _)| name)
        .collect();
    assert_eq!(functions, stubs);
}

#[test]
fn export_kind_matches_names_exactly() {
    // Not oracle-derived: lookups of spec names, plus names that differ only in case, that are a
    // prefix of an export, or that are empty.
    assert_eq!(export_kind("SysprepCleanup"), Some(ExportKind::Stub));
    assert_eq!(export_kind("SetSRStateAfterSetup"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("SRSetRestorePointA"),
        Some(ExportKind::Forward("sfc_os.SRSetRestorePointA"))
    );
    assert_eq!(export_kind("sysprepcleanup"), None);
    assert_eq!(export_kind("SRSetRestorePoint"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn ordinals_are_contiguous_from_one() {
    // Not oracle-derived: a property of the rows 01-13 table. The lowest explicit ordinal is 1, so
    // winebuild's base is 1, and the 13 entries fill 1 to 13 with no gaps or repeats.
    let ordinals: Vec<u32> = EXPORTS.iter().map(|&(ordinal, _, _)| ordinal).collect();
    assert_eq!(ordinals, (1..=13).collect::<Vec<u32>>());
}

#[test]
fn forward_targets_name_the_same_export_in_sfc_os() {
    // Not oracle-derived: each forward target is `sfc_os.` followed by the export's own name.
    for (_, name, kind) in EXPORTS {
        if let ExportKind::Forward(target) = kind {
            assert_eq!(target, format!("sfc_os.{}", name));
        }
    }
}
