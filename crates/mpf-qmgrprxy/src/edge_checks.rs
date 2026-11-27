//! Edge-case tests for the ported qmgrprxy logic.
//!
//! These cover the export lookup, the stack size on 64-bit targets, the `DllMain` reasons that the
//! oracle does not print, the registration route without `WINE_REGISTER_DLL`, and the GUID text
//! padding. Not every expectation is oracle-derived; the ones that are not say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winnt.h, include/minwindef.h and include/winerror.h.
    assert_eq!(DLL_PROCESS_ATTACH, 1);
    assert_eq!(TRUE, 1);
    assert_eq!(S_OK, 0);
}

#[test]
fn lookup_matches_whole_names_case_sensitively() {
    // Not oracle-derived: the oracle probes only "DllMain", "dllgetclassobject" and "".
    assert_eq!(export("DllGetClassObject").map(|e| e.params), Some(3));
    assert!(export("dllregisterserver").is_none());
    assert!(export("DllRegisterServer ").is_none());
    assert!(export("GetProxyDllInfo2").is_none());
}

#[test]
fn every_export_resolves_to_its_own_record() {
    // Not oracle-derived: lookup by name returns the same record for every row of the table.
    for entry in EXPORTS {
        assert_eq!(export(entry.name), Some(entry));
    }
}

#[test]
fn args_size_scales_with_pointer_width() {
    // Not oracle-derived: the oracle builds only with -m32 (ptr=4). These are the same formula
    // with an 8-byte pointer, as on x86_64.
    assert_eq!(args_size(0, 8), 0);
    assert_eq!(args_size(2, 8), 16);
    assert_eq!(args_size(3, 8), 24);
}

#[test]
fn dll_main_other_reasons_return_true_without_side_effects() {
    // Not oracle-derived: the oracle probes reasons 0, 1, 2, 3 and 5. Reason 4 and u32::MAX take
    // the branch that is not DLL_PROCESS_ATTACH.
    for reason in [4, u32::MAX] {
        assert_eq!(
            dll_main(reason),
            DllMainOutcome {
                ret: TRUE,
                disable_thread_library_calls: false,
                stores_instance: false,
            }
        );
    }
}

#[test]
fn register_route_without_wine_register_dll_is_ndr_proxy() {
    // Not oracle-derived: the oracle is built with -DWINE_REGISTER_DLL, which Makefile.in sets
    // through dlldata_EXTRADEFS. This is the other branch, and the rpcrt4 call it selects is not
    // ported.
    assert_eq!(register_route(false), RegisterRoute::NdrProxy);
}

#[test]
fn guid_text_pads_every_group() {
    // Not oracle-derived: zero and low-value fields check the zero padding of each group.
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    assert_eq!(guid_string(&zero), "00000000-0000-0000-0000-000000000000");
    let low = Guid {
        data1: 1,
        data2: 0x0abc,
        data3: 0x000f,
        data4: [0, 0, 0, 0, 0, 0, 0, 0xff],
    };
    assert_eq!(guid_string(&low), "00000001-0abc-000f-0000-0000000000ff");
}
