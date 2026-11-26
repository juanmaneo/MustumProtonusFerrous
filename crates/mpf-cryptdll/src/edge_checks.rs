//! Edge-case tests for the ported cryptdll export table and stub model.
//!
//! The DLL has no C logic, so these cover the table's shape and lookups, the stub record's
//! fields, and the link between each stub helper and its spec name. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of cryptdll.spec. The code and
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
fn stub_helpers_match_the_spec_stub_names_in_order() {
    // Not oracle-derived: each helper must raise the record for the `@ stub` lines of
    // cryptdll.spec, in spec order, with the module from Makefile.in.
    let helpers = [
        cd_build_integrity_vect(),
        cd_build_vect(),
        cd_find_common_csystem(),
        cd_find_common_csystem_with_key(),
        cd_generate_random_bits(),
        cd_locate_csystem(),
        cd_locate_check_sum(),
        cd_locate_rng(),
        cd_register_csystem(),
        cd_register_check_sum(),
        cd_register_rng(),
    ];
    let stub_names: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let functions: Vec<&str> = helpers.iter().map(|s| s.function).collect();
    assert_eq!(functions, stub_names);
    for helper in helpers {
        assert_eq!(helper.module, "cryptdll.dll");
        assert_eq!(helper.code, EXCEPTION_WINE_STUB);
        assert_eq!(helper.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(helper.number_parameters, 2);
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 14 export lines of dlls/cryptdll/cryptdll.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("CDBuildIntegrityVect", ExportKind::Stub),
            ("CDBuildVect", ExportKind::Stub),
            ("CDFindCommonCSystem", ExportKind::Stub),
            ("CDFindCommonCSystemWithKey", ExportKind::Stub),
            ("CDGenerateRandomBits", ExportKind::Stub),
            ("CDLocateCSystem", ExportKind::Stub),
            ("CDLocateCheckSum", ExportKind::Stub),
            ("CDLocateRng", ExportKind::Stub),
            ("CDRegisterCSystem", ExportKind::Stub),
            ("CDRegisterCheckSum", ExportKind::Stub),
            ("CDRegisterRng", ExportKind::Stub),
            ("MD5Final", ExportKind::Forward("advapi32.MD5Final")),
            ("MD5Init", ExportKind::Forward("advapi32.MD5Init")),
            ("MD5Update", ExportKind::Forward("advapi32.MD5Update")),
        ]
    );
}

#[test]
fn export_kind_matches_names_exactly() {
    // Not oracle-derived: lookups are exact and case-sensitive, as the spec spells the names.
    assert_eq!(export_kind("CDBuildVect"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("MD5Init"),
        Some(ExportKind::Forward("advapi32.MD5Init"))
    );
    assert_eq!(export_kind("cdbuildvect"), None);
    assert_eq!(export_kind("MD5Init "), None);
    assert_eq!(export_kind("CDBuildVect2"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn forward_targets_name_the_same_export_in_advapi32() {
    // Not oracle-derived: each forward in the spec is `advapi32.<same name>`.
    let mut forwards = 0;
    for (name, kind) in EXPORTS {
        if let ExportKind::Forward(target) = kind {
            assert_eq!(target, format!("advapi32.{name}"));
            forwards += 1;
        }
    }
    assert_eq!(forwards, 3);
}
