//! Oracle-derived test vectors for the ported dciman32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `DCIOpenProvider`, `DCICloseProvider` and `DCICreatePrimary` from
//! `dlls/dciman32/dciman_main.c` with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/dciman32/` (harness `oracle.c`). The run printed 23
//! rows; ASan and UBSan reported nothing, so 0 vectors were excluded.
//! The 17 `stub` rows record the fault value from `dciman32.spec` and
//! `winternl.h`, which the harness prints as constants (the stubs are generated
//! by winebuild and cannot be run here).

use super::*;

#[test]
fn oracle_open_provider_rows() {
    // open_provider prev_err=0 -> ret=(nil) last_error=120
    let mut d = Dciman32::default();
    assert_eq!(d.open_provider(), 0);
    assert_eq!(d.last_error(), 120);
    // open_provider prev_err=5 -> ret=(nil) last_error=120
    let mut d = Dciman32 { last_error: 5 };
    assert_eq!(d.open_provider(), 0);
    assert_eq!(d.last_error(), 120);
}

#[test]
fn oracle_close_provider_rows() {
    // close_provider hdc=(nil) -> last_error=120
    let mut d = Dciman32::default();
    d.close_provider(0);
    assert_eq!(d.last_error(), 120);
    // close_provider hdc=0x7 -> last_error=120
    let mut d = Dciman32::default();
    d.close_provider(7);
    assert_eq!(d.last_error(), 120);
}

#[test]
fn oracle_create_primary_rows() {
    // create_primary hdc=(nil) info_null=1 -> ret=-4 last_error=120
    let mut d = Dciman32::default();
    assert_eq!(d.create_primary::<u32>(0, None), -4);
    assert_eq!(d.last_error(), 120);
    // create_primary hdc=0x7 info_null=0 -> ret=-4 last_error=120
    let mut d = Dciman32::default();
    let mut info = 0u32;
    assert_eq!(d.create_primary(7, Some(&mut info)), -4);
    assert_eq!(d.last_error(), 120);
}

#[test]
fn oracle_stub_rows() {
    // stub <name> -> exception=0x80000100 noreturn, one row per name
    let names = [
        "DCIBeginAccess",
        "DCICreateOffscreen",
        "DCICreateOverlay",
        "DCIDestroy",
        "DCIDraw",
        "DCIEndAccess",
        "DCIEnum",
        "DCISetClipList",
        "DCISetDestination",
        "DCISetSrcDestClip",
        "GetDCRegionData",
        "GetWindowRegionData",
        "WinWatchClose",
        "WinWatchDidStatusChange",
        "WinWatchGetClipList",
        "WinWatchNotify",
        "WinWatchOpen",
    ];
    assert_eq!(names, STUB_EXPORTS);
    for name in names {
        let fault = stub_fault(name).expect("stub export");
        assert_eq!(fault.code, 0x8000_0100);
        assert_eq!(fault.module, "dciman32");
        assert_eq!(fault.function, name);
    }
}

#[test]
fn oracle_dci_constants() {
    // Values from include/dciddi.h, included by dciman.h.
    assert_eq!(DCI_OK, 0);
    assert_eq!(DCI_FAIL_GENERIC, -1);
    assert_eq!(DCI_FAIL_UNSUPPORTEDVERSION, -2);
    assert_eq!(DCI_FAIL_INVALIDSURFACE, -3);
    assert_eq!(DCI_FAIL_UNSUPPORTED, -4);
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
}
