//! Edge-case tests for the ported mtxdm logic.
//!
//! These cover the out-pointer forwarding, the HRESULT pass-through and the export table. Not
//! every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: CLSCTX_ALL from include/combaseapi.h, and the GUIDs from the uuid
    // attributes in include/comsvcs.idl. The oracle harness defines the same GUIDs.
    assert_eq!(CLSCTX_ALL, 0x17);
    assert_eq!(
        CLSID_DISPENSER_MANAGER,
        Guid {
            data1: 0xecabb0c0,
            data2: 0x7f19,
            data3: 0x11d2,
            data4: [0x97, 0x8e, 0x00, 0x00, 0xf8, 0x75, 0x7e, 0x2a],
        }
    );
    assert_eq!(
        IID_IDISPENSER_MANAGER,
        Guid {
            data1: 0x5cb31e10,
            data2: 0x2b5f,
            data3: 0x11cf,
            data4: [0xbe, 0x10, 0x00, 0xaa, 0x00, 0xa2, 0xfa, 0x25],
        }
    );
}

#[test]
fn null_out_pointer_reaches_the_call_as_none() {
    // Not oracle-derived: the oracle's NULL rows cover the nine HRESULTs in its table. This test
    // checks that a NULL out-pointer reaches the call as `None` and that a value outside that
    // table is returned unchanged.
    let no_slot: Option<&mut usize> = None;
    let mut seen_none = false;
    let got = get_dispenser_manager(no_slot, |_, _, _, ppv| {
        seen_none = ppv.is_none();
        -1
    });
    assert!(seen_none);
    assert_eq!(got, -1);
}

#[test]
fn every_hresult_passes_through_unchanged() {
    // Not oracle-derived: the harness returned only the nine HRESULTs in its table. The forwarder
    // must not map any value, so the extremes are checked too.
    for ret in [0, 1, -1, i32::MIN, i32::MAX, 0x8000_4005_u32 as i32] {
        let mut slot = 0usize;
        assert_eq!(
            get_dispenser_manager(Some(&mut slot), |_, _, _, _| ret),
            ret
        );
    }
}

#[test]
fn out_pointer_is_the_callers_slot() {
    // Not oracle-derived: the harness recorder does not write through `ppv`. This shows the
    // forwarder hands over the caller's own slot, not a copy.
    let mut slot = 0usize;
    let got = get_dispenser_manager(Some(&mut slot), |_, _, _, ppv| {
        *ppv.unwrap() = 0x1234;
        0
    });
    assert_eq!(got, 0);
    assert_eq!(slot, 0x1234);
}

#[test]
fn export_table_matches_spec() {
    // Not oracle-derived: the one line of dlls/mtxdm/mtxdm.spec, `@ cdecl GetDispenserManager(ptr)`.
    assert_eq!(EXPORTS, [("GetDispenserManager", ExportKind::Cdecl)]);
    assert_eq!(export_kind("GetDispenserManager"), Some(ExportKind::Cdecl));
    assert_eq!(export_kind("getdispensermanager"), None);
    assert_eq!(export_kind("GetDispenserManager2"), None);
    assert_eq!(export_kind(""), None);
}
