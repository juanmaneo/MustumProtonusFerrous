//! Boundary and branch checks for the scarddlg stubs and export table.
//!
//! Not every expectation is oracle-derived. The comment on each test says which kind it is.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: checked against include/winternl.h and include/winnt.h by grep. The
    // oracle rows print the same values as code=0x80000100 and flags=0x1.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn record_fields_do_not_depend_on_the_names() {
    // Not oracle-derived: the oracle only passed the names listed above. This checks that code,
    // flags and number_parameters stay fixed for any module and function, including an empty pair.
    for (module, function) in [
        ("", ""),
        ("a", "b"),
        ("scarddlg.dll", "SCardUIDlgSelectCardW"),
    ] {
        let record = wine_spec_unimplemented_stub(module, function);
        assert_eq!(record.code, EXCEPTION_WINE_STUB);
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(record.number_parameters, 2);
        assert_eq!(record.module, module);
        assert_eq!(record.function, function);
    }
}

#[test]
fn each_helper_raises_its_own_export_name() {
    // Not oracle-derived: checks the five helpers against the order of EXPORTS. The oracle rows
    // 01-05 cover the same five pairs.
    type Helper = fn() -> StubException;
    let helpers: [(&str, Helper); 5] = [
        ("GetOpenCardNameA", get_open_card_name_a),
        ("GetOpenCardNameW", get_open_card_name_w),
        ("SCardDlgExtendedError", scard_dlg_extended_error),
        ("SCardUIDlgSelectCardA", scard_ui_dlg_select_card_a),
        ("SCardUIDlgSelectCardW", scard_ui_dlg_select_card_w),
    ];
    for (i, (name, helper)) in helpers.into_iter().enumerate() {
        assert_eq!(EXPORTS[i].1, name);
        assert_eq!(helper(), wine_spec_unimplemented_stub("scarddlg.dll", name));
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the five lines of dlls/scarddlg/scarddlg.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            (1, "GetOpenCardNameA"),
            (2, "GetOpenCardNameW"),
            (3, "SCardDlgExtendedError"),
            (4, "SCardUIDlgSelectCardA"),
            (5, "SCardUIDlgSelectCardW"),
        ]
    );
}

#[test]
fn export_ordinal_is_case_sensitive_and_total_over_the_table() {
    // Not oracle-derived: the boundary and miss cases of the lookup.
    for (ordinal, name) in EXPORTS {
        assert_eq!(export_ordinal(name), Some(ordinal));
    }
    assert_eq!(export_ordinal("getopencardnamea"), None);
    assert_eq!(export_ordinal("SCardDlgExtendedError "), None);
    assert_eq!(export_ordinal(""), None);
}
