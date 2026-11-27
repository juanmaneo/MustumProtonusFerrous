//! Edge-case tests for the ported mapistub logic.
//!
//! These cover the stub record's argument handling, the export table's shape, and its lookups.
//! Not every expectation is oracle-derived; the ones that are not say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_arguments_pass_through_unchanged() {
    // Not oracle-derived: __wine_spec_unimplemented_stub copies both arguments into the record
    // without reading them, so any name or ordinal comes back unchanged.
    let named = wine_spec_unimplemented_stub("other.dll", StubFunction::Name("Foo@0"));
    assert_eq!(named.module, "other.dll");
    assert_eq!(named.function, StubFunction::Name("Foo@0"));
    assert_eq!(named.code, EXCEPTION_WINE_STUB);
    assert_eq!(named.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(named.number_parameters, 2);

    let ordinal = wine_spec_unimplemented_stub("", StubFunction::Ordinal(u16::MAX));
    assert_eq!(ordinal.module, "");
    assert_eq!(ordinal.function, StubFunction::Ordinal(u16::MAX));
    assert_eq!(ordinal.code, EXCEPTION_WINE_STUB);
}

#[test]
fn export_table_has_the_spec_counts() {
    // Not oracle-derived: dlls/mapistub/mapistub.spec has 191 entries: 100 `stub`, 90 `stdcall`
    // forwards to a `module.function` target, and one `stdcall -private` with no target.
    let kinds: Vec<ExportKind> = EXPORTS.iter().map(|&(_, _, kind)| kind).collect();
    assert_eq!(kinds.len(), 191);
    assert_eq!(
        kinds
            .iter()
            .filter(|kind| **kind == ExportKind::Stub)
            .count(),
        100
    );
    assert_eq!(
        kinds
            .iter()
            .filter(|kind| matches!(kind, ExportKind::Forward(_)))
            .count(),
        90
    );
    assert_eq!(
        kinds
            .iter()
            .filter(|kind| **kind == ExportKind::Unbound)
            .count(),
        1
    );
}

#[test]
fn export_table_ordinals_strictly_increase() {
    // Not oracle-derived: dlls/mapistub/mapistub.spec lists its ordinals in ascending order.
    assert!(EXPORTS.windows(2).all(|pair| pair[0].0 < pair[1].0));
}

#[test]
fn export_table_ends_match_the_spec() {
    // Not oracle-derived: the first and last lines of dlls/mapistub/mapistub.spec.
    assert_eq!(EXPORTS[0], (8, None, ExportKind::Stub));
    assert_eq!(
        EXPORTS[EXPORTS.len() - 1],
        (
            256,
            Some("MAPISendMailW"),
            ExportKind::Forward("mapi32.MAPISendMailW")
        )
    );
}

#[test]
fn forward_targets_are_module_dot_function() {
    // Not oracle-derived: winebuild marks an entry as a forward when its target contains a dot
    // (tools/winebuild/parser.c, parse_spec_extern), so each target has a module and a function.
    for (_, _, kind) in EXPORTS.iter() {
        if let ExportKind::Forward(target) = kind {
            let (module, function) = target.split_once('.').expect("target has a dot");
            assert!(!module.is_empty() && !function.is_empty(), "{target}");
        }
    }
}

#[test]
fn export_kind_resolves_spec_names_exactly() {
    // Not oracle-derived: the names are the spec tokens, so an `@N` suffix is part of the name.
    assert_eq!(export_kind("LAUNCHWIZARD"), Some(ExportKind::Stub));
    assert_eq!(export_kind("LaunchWizard@20"), Some(ExportKind::Stub));
    assert_eq!(export_kind("LaunchWizard"), None);
    assert_eq!(
        export_kind("MAPILogonEx"),
        Some(ExportKind::Forward("mapi32.MAPILogonEx"))
    );
    assert_eq!(
        export_kind("MAPILogonEx@20"),
        Some(ExportKind::Forward("mapi32.MAPILogonEx"))
    );
    assert_eq!(
        export_kind("FtAddFt@16"),
        Some(ExportKind::Forward("mapi32.MAPI32_FtAddFt"))
    );
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Unbound));
    assert_eq!(export_kind("launchwizard"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn every_named_entry_resolves_to_its_own_kind() {
    // Not oracle-derived: spec names are unique, so each named entry resolves to itself.
    for (_, name, kind) in EXPORTS.iter() {
        if let Some(name) = name {
            assert_eq!(
                EXPORTS.iter().filter(|(_, n, _)| n == &Some(*name)).count(),
                1,
                "{name}"
            );
            assert_eq!(export_kind(name), Some(*kind), "{name}");
        }
    }
}
