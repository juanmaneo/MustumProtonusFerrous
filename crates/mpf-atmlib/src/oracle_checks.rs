//! Oracle vectors for the atmlib port.
//!
//! Rows come from `target/canon/atmlib/oracle.c`: verbatim Wine bodies of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) and of `ATMProperlyLoaded`
//! and `ATMFinish` (`dlls/atmlib/main.c`), compiled with `gcc -fsanitize=address,undefined -g`.
//! The harness calls every `atmlib.spec` export and prints one row each, 76 in total.
//! ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

/// Module name the harness passes to every stub (`FILE_NAME` in `oracle.c`).
const ORACLE_MODULE: &str = "atmlib.dll";

/// One row per export from `target/canon/atmlib/rows.txt`, in spec order.
const ORACLE_ROWS: &[(&str, ExportKind)] = &[
    ("ATMAddFont", ExportKind::Stub),
    ("ATMAddFontA", ExportKind::Stub),
    ("ATMAddFontEx", ExportKind::Stub),
    ("ATMAddFontExA", ExportKind::Stub),
    ("ATMAddFontExW", ExportKind::Stub),
    ("ATMAddFontW", ExportKind::Stub),
    ("ATMBBoxBaseXYShowText", ExportKind::Stub),
    ("ATMBBoxBaseXYShowTextA", ExportKind::Stub),
    ("ATMBBoxBaseXYShowTextW", ExportKind::Stub),
    ("ATMBeginFontChange", ExportKind::Stub),
    ("ATMClient", ExportKind::Stub),
    ("ATMEndFontChange", ExportKind::Stub),
    ("ATMEnumFonts", ExportKind::Stub),
    ("ATMEnumFontsA", ExportKind::Stub),
    ("ATMEnumFontsW", ExportKind::Stub),
    ("ATMEnumMMFonts", ExportKind::Stub),
    ("ATMEnumMMFontsA", ExportKind::Stub),
    ("ATMEnumMMFontsW", ExportKind::Stub),
    ("ATMFinish", ExportKind::Stdcall),
    ("ATMFontAvailable", ExportKind::Stub),
    ("ATMFontAvailableA", ExportKind::Stub),
    ("ATMFontAvailableW", ExportKind::Stub),
    ("ATMFontSelected", ExportKind::Stub),
    ("ATMFontStatus", ExportKind::Stub),
    ("ATMFontStatusA", ExportKind::Stub),
    ("ATMFontStatusW", ExportKind::Stub),
    ("ATMForceFontChange", ExportKind::Stub),
    ("ATMGetBuildStr", ExportKind::Stub),
    ("ATMGetBuildStrA", ExportKind::Stub),
    ("ATMGetBuildStrW", ExportKind::Stub),
    ("ATMGetFontBBox", ExportKind::Stub),
    ("ATMGetFontInfo", ExportKind::Stub),
    ("ATMGetFontInfoA", ExportKind::Stub),
    ("ATMGetFontInfoW", ExportKind::Stub),
    ("ATMGetFontPaths", ExportKind::Stub),
    ("ATMGetFontPathsA", ExportKind::Stub),
    ("ATMGetFontPathsW", ExportKind::Stub),
    ("ATMGetGlyphList", ExportKind::Stub),
    ("ATMGetGlyphListA", ExportKind::Stub),
    ("ATMGetGlyphListW", ExportKind::Stub),
    ("ATMGetMenuName", ExportKind::Stub),
    ("ATMGetMenuNameA", ExportKind::Stub),
    ("ATMGetMenuNameW", ExportKind::Stub),
    ("ATMGetNtmFields", ExportKind::Stub),
    ("ATMGetNtmFieldsA", ExportKind::Stub),
    ("ATMGetNtmFieldsW", ExportKind::Stub),
    ("ATMGetOutline", ExportKind::Stub),
    ("ATMGetOutlineA", ExportKind::Stub),
    ("ATMGetOutlineW", ExportKind::Stub),
    ("ATMGetPostScriptName", ExportKind::Stub),
    ("ATMGetPostScriptNameA", ExportKind::Stub),
    ("ATMGetPostScriptNameW", ExportKind::Stub),
    ("ATMGetVersion", ExportKind::Stub),
    ("ATMGetVersionEx", ExportKind::Stub),
    ("ATMGetVersionExA", ExportKind::Stub),
    ("ATMGetVersionExW", ExportKind::Stub),
    ("ATMInstallSubstFontA", ExportKind::Stub),
    ("ATMInstallSubstFontW", ExportKind::Stub),
    ("ATMMakePFM", ExportKind::Stub),
    ("ATMMakePFMA", ExportKind::Stub),
    ("ATMMakePFMW", ExportKind::Stub),
    ("ATMMakePSS", ExportKind::Stub),
    ("ATMMakePSSA", ExportKind::Stub),
    ("ATMMakePSSW", ExportKind::Stub),
    ("ATMProperlyLoaded", ExportKind::Stdcall),
    ("ATMRemoveFont", ExportKind::Stub),
    ("ATMRemoveFontA", ExportKind::Stub),
    ("ATMRemoveFontW", ExportKind::Stub),
    ("ATMRemoveSubstFontA", ExportKind::Stub),
    ("ATMRemoveSubstFontW", ExportKind::Stub),
    ("ATMSelectEncoding", ExportKind::Stub),
    ("ATMSelectObject", ExportKind::Stub),
    ("ATMSetFlags", ExportKind::Stub),
    ("ATMXYShowText", ExportKind::Stub),
    ("ATMXYShowTextA", ExportKind::Stub),
    ("ATMXYShowTextW", ExportKind::Stub),
];

#[test]
fn oracle_export_rows() {
    assert_eq!(EXPORTS, ORACLE_ROWS);
    for &(name, kind) in ORACLE_ROWS {
        assert_eq!(export_kind(name), Some(kind));
        if kind == ExportKind::Stub {
            assert_eq!(
                unimplemented_stub(ORACLE_MODULE, name),
                StubException {
                    code: 0x8000_0100,
                    flags: 0x01,
                    number_parameters: 2,
                    module: ORACLE_MODULE,
                    function: name,
                }
            );
        }
    }
}

#[test]
fn oracle_stdcall_rows() {
    assert_eq!(i32::from(atm_properly_loaded()), 0);
    assert_eq!(atm_finish(), 0);
}
