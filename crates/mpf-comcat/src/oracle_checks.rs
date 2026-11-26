//! Oracle-derived test vectors for the ported comcat logic.
//!
//! comcat has no C bodies, so the oracle is Wine's own `tools/winebuild`. It was built from the
//! reference sources with `gcc -fsanitize=address,undefined -g` under `target/canon/comcat/tools/winebuild/`,
//! and run in `--def` mode by `target/canon/comcat/run_oracle.sh`. The inputs are
//! `dlls/comcat/comcat.spec` (4 rows per target) and a two-line synthetic spec, `Plain(ptr ptr)` and
//! `FwdPublic(ptr) ole32.FwdPublic`, which reaches the branches comcat.spec does not: no `-private`,
//! and a forward without `-private`. ASan and UBSan reported nothing, and stderr was empty for every
//! run. The 16 rows are 12 per-line rows and 4 whole-file rows. No vector was excluded.

use super::*;

const PLAIN: Export = Export {
    name: "Plain",
    ordinal: 1,
    arg_bytes: 8,
    forward: None,
    private: false,
};

const FWD_PUBLIC: Export = Export {
    name: "FwdPublic",
    ordinal: 2,
    arg_bytes: 4,
    forward: Some("ole32.FwdPublic"),
    private: false,
};

/// The `.def` body winebuild writes after its header comment: `lines` joined with newlines.
fn body(lines: &[&str]) -> String {
    lines.iter().map(|line| format!("{line}\n")).collect()
}

#[test]
fn oracle_def_lines() {
    // row 01 comcat.spec x86_64 DllCanUnloadNow
    assert_eq!(
        def_line(&EXPORTS[0], Target::X86_64),
        "  DllCanUnloadNow @1 PRIVATE\n"
    );
    // row 02 comcat.spec x86_64 DllGetClassObject
    assert_eq!(
        def_line(&EXPORTS[1], Target::X86_64),
        "  DllGetClassObject=ole32.DllGetClassObject @2 PRIVATE\n"
    );
    // row 03 comcat.spec x86_64 DllRegisterServer
    assert_eq!(
        def_line(&EXPORTS[2], Target::X86_64),
        "  DllRegisterServer @3 PRIVATE\n"
    );
    // row 04 comcat.spec x86_64 DllUnregisterServer
    assert_eq!(
        def_line(&EXPORTS[3], Target::X86_64),
        "  DllUnregisterServer @4 PRIVATE\n"
    );
    // row 05 comcat.spec i386 DllCanUnloadNow
    assert_eq!(
        def_line(&EXPORTS[0], Target::I386),
        "  DllCanUnloadNow@0 @1 PRIVATE\n"
    );
    // row 06 comcat.spec i386 DllGetClassObject
    assert_eq!(
        def_line(&EXPORTS[1], Target::I386),
        "  DllGetClassObject@12=ole32.DllGetClassObject @2 PRIVATE\n"
    );
    // row 07 comcat.spec i386 DllRegisterServer
    assert_eq!(
        def_line(&EXPORTS[2], Target::I386),
        "  DllRegisterServer@0 @3 PRIVATE\n"
    );
    // row 08 comcat.spec i386 DllUnregisterServer
    assert_eq!(
        def_line(&EXPORTS[3], Target::I386),
        "  DllUnregisterServer@0 @4 PRIVATE\n"
    );
    // row 09 synthetic x86_64 Plain
    assert_eq!(def_line(&PLAIN, Target::X86_64), "  Plain @1\n");
    // row 10 synthetic i386 Plain
    assert_eq!(def_line(&PLAIN, Target::I386), "  Plain@8 @1\n");
    // row 11 synthetic x86_64 FwdPublic
    assert_eq!(
        def_line(&FWD_PUBLIC, Target::X86_64),
        "  FwdPublic=ole32.FwdPublic @2\n"
    );
    // row 12 synthetic i386 FwdPublic
    assert_eq!(
        def_line(&FWD_PUBLIC, Target::I386),
        "  FwdPublic@4=ole32.FwdPublic @2\n"
    );
}

#[test]
fn oracle_def_files() {
    // row 13 comcat.spec x86_64 whole file
    assert_eq!(
        def_file(MODULE, &EXPORTS, Target::X86_64),
        body(&[
            "LIBRARY comcat.dll",
            "",
            "EXPORTS",
            "  DllCanUnloadNow @1 PRIVATE",
            "  DllGetClassObject=ole32.DllGetClassObject @2 PRIVATE",
            "  DllRegisterServer @3 PRIVATE",
            "  DllUnregisterServer @4 PRIVATE",
        ])
    );
    // row 14 comcat.spec i386 whole file
    assert_eq!(
        def_file(MODULE, &EXPORTS, Target::I386),
        body(&[
            "LIBRARY comcat.dll",
            "",
            "EXPORTS",
            "  DllCanUnloadNow@0 @1 PRIVATE",
            "  DllGetClassObject@12=ole32.DllGetClassObject @2 PRIVATE",
            "  DllRegisterServer@0 @3 PRIVATE",
            "  DllUnregisterServer@0 @4 PRIVATE",
        ])
    );
    // row 15 synthetic x86_64 whole file
    assert_eq!(
        def_file(MODULE, &[PLAIN, FWD_PUBLIC], Target::X86_64),
        body(&[
            "LIBRARY comcat.dll",
            "",
            "EXPORTS",
            "  Plain @1",
            "  FwdPublic=ole32.FwdPublic @2",
        ])
    );
    // row 16 synthetic i386 whole file
    assert_eq!(
        def_file(MODULE, &[PLAIN, FWD_PUBLIC], Target::I386),
        body(&[
            "LIBRARY comcat.dll",
            "",
            "EXPORTS",
            "  Plain@8 @1",
            "  FwdPublic@4=ole32.FwdPublic @2",
        ])
    );
}
