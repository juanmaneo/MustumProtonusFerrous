//! Edge-case tests for the ported shdocvw logic.
//!
//! These cover the constants, the class routing near-misses, the out-pointer check order, the
//! ordinals the forwards ask for, the argument order of the forwards, and the export and stub
//! tables. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use std::cell::Cell;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h,
    // and the ordinals passed to GetProcAddress in dlls/shdocvw/shdocvw_main.c.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_FAIL, 0x8000_4005);
    assert_eq!(E_INVALIDARG, 0x8007_0057);
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(SHLWAPI_STOP_WATCH_MODE, 241);
    assert_eq!(SHLWAPI_STOP_WATCH_FLUSH, 242);
    assert_eq!(SHLWAPI_STOP_WATCH_A, 243);
    assert_eq!(SHLWAPI_STOP_WATCH_W, 244);
    assert_eq!(SHELL32_SHELL_DDE_INIT, 188);
}

#[test]
fn class_guids_match_their_headers() {
    // Not oracle-derived: the first field of each GUID from the header that defines it.
    assert_eq!(CLSID_WEB_BROWSER.data1, 0x8856_f961);
    assert_eq!(CLSID_WEB_BROWSER_V1.data1, 0xeab2_2ac3);
    assert_eq!(CLSID_INTERNET_SHORTCUT.data1, 0xfbf2_3b40);
    assert_eq!(CLSID_CURL_HISTORY.data1, 0x3c37_4a40);
    assert_eq!(CLSID_TASKBAR_LIST.data1, 0x56fd_f344);
    assert_eq!(IID_IUNKNOWN.data1, 0);
    assert_eq!(IID_IPROPERTY_BAG.data1, 0x5527_2a00);
}

#[test]
fn can_unload_now_is_false_for_any_nonzero_count() {
    // Not oracle-derived: negative and extreme counts take the non-zero branch.
    assert_eq!(dll_can_unload_now(-1), S_FALSE);
    assert_eq!(dll_can_unload_now(i32::MIN), S_FALSE);
    assert_eq!(dll_can_unload_now(i32::MAX), S_FALSE);
}

#[test]
fn class_route_needs_an_exact_match() {
    // Not oracle-derived: flipping one bit of a browser CLSID, in the last byte or in the first
    // field, must send it to the shell instance lookup.
    let browser = [
        CLSID_WEB_BROWSER,
        CLSID_WEB_BROWSER_V1,
        CLSID_INTERNET_SHORTCUT,
        CLSID_CURL_HISTORY,
        CLSID_TASKBAR_LIST,
    ];
    for clsid in browser {
        let mut last_byte = clsid;
        last_byte.data4[7] ^= 1;
        let mut first_field = clsid;
        first_field.data1 ^= 1;
        assert_eq!(
            dll_get_class_object_route(&last_byte),
            ClassRoute::ShellInstance
        );
        assert_eq!(
            dll_get_class_object_route(&first_field),
            ClassRoute::ShellInstance
        );
    }
    assert_eq!(
        dll_get_class_object_route(&IID_IUNKNOWN),
        ClassRoute::ShellInstance
    );
}

#[test]
fn property_bag_checks_the_out_pointer_before_the_iid() {
    // Not oracle-derived: E25 covers only IUnknown with a NULL out-pointer. The C body checks
    // `ppv` first, so every IID gets E_INVALIDARG when the out-pointer is missing.
    let other = Guid {
        data1: 0x1234_5678,
        data2: 0x1234,
        data3: 0x1234,
        data4: [1, 2, 3, 4, 5, 6, 7, 8],
    };
    assert_eq!(
        property_bag_query_interface(&IID_IPROPERTY_BAG, false),
        E_INVALIDARG
    );
    assert_eq!(property_bag_query_interface(&other, false), E_INVALIDARG);
}

#[test]
fn forwards_ask_for_the_shlwapi_and_shell32_ordinals() {
    // Not oracle-derived: the ordinals are the literal arguments of the `fetch_shlwapi_ordinal`
    // and `GetProcAddress` calls in dlls/shdocvw/shdocvw_main.c, which oracle.c compiles verbatim.
    // Each resolver runs with a target and without one, so both arms run.
    let asked = Cell::new(0);
    for present in [true, false] {
        stop_watch_mode_forward(|ord: u16| {
            asked.set(ord);
            if present {
                Some(|| {})
            } else {
                None
            }
        });
        assert_eq!(asked.get(), 241);

        stop_watch_flush_forward(|ord: u16| {
            asked.set(ord);
            if present {
                Some(|| {})
            } else {
                None
            }
        });
        assert_eq!(asked.get(), 242);

        stop_watch_a_forward(1, "s", 2, 3, 4, |ord: u16| {
            asked.set(ord);
            if present {
                Some(|c: u32, _: &str, u: u32, m: u32, t: u32| c + u + m + t)
            } else {
                None
            }
        });
        assert_eq!(asked.get(), 243);

        let wide: [u16; 2] = [u16::from(b's'), 0];
        stop_watch_w_forward(1, &wide[..], 2, 3, 4, |ord: u16| {
            asked.set(ord);
            if present {
                Some(|c: u32, _: &[u16], u: u32, m: u32, t: u32| c + u + m + t)
            } else {
                None
            }
        });
        assert_eq!(asked.get(), 244);

        shell_dde_init(true, true, |ord: u16| {
            asked.set(ord);
            if present {
                Some(|start: bool| start)
            } else {
                None
            }
        });
        assert_eq!(asked.get(), 188);
        asked.set(0);
    }
}

#[test]
fn shell_dde_init_skips_the_lookup_without_shell32() {
    // Not oracle-derived: the port's own branch. With shell32 unavailable the C body returns
    // FALSE before it looks up ordinal 188, which oracle row E34 shows as FALSE. The resolver
    // runs only when shell32 is available.
    let looked_up = Cell::new(false);
    for (shell32, present) in [(true, true), (true, false), (false, true)] {
        looked_up.set(false);
        let ret = shell_dde_init(true, shell32, |_: u16| {
            looked_up.set(true);
            if present {
                Some(|start: bool| start)
            } else {
                None
            }
        });
        assert_eq!(looked_up.get(), shell32);
        assert_eq!(ret, shell32 && present);
    }
}

#[test]
fn stop_watch_a_and_w_pass_the_arguments_in_order() {
    // Not oracle-derived: the weights differ per argument, so a swapped argument changes the
    // result. The C body passes the arguments in declaration order.
    for (present, expected) in [(true, 1234), (false, 120)] {
        let ret = stop_watch_a_forward(1, "s", 2, 3, 4, |_: u16| {
            if present {
                Some(|c: u32, _: &str, u: u32, m: u32, t: u32| c * 1000 + u * 100 + m * 10 + t)
            } else {
                None
            }
        });
        assert_eq!(ret, expected);

        let wide: [u16; 2] = [u16::from(b's'), 0];
        let ret = stop_watch_w_forward(1, &wide[..], 2, 3, 4, |_: u16| {
            if present {
                Some(|c: u32, _: &[u16], u: u32, m: u32, t: u32| c * 1000 + u * 100 + m * 10 + t)
            } else {
                None
            }
        });
        assert_eq!(ret, expected);
    }
}

#[test]
fn stub_passes_its_names_through_unchanged() {
    // Not oracle-derived: the names are arbitrary, so this checks that the record copies them and
    // the constants stay fixed. The export rows in oracle_checks.rs cover the real names.
    assert_eq!(
        wine_spec_unimplemented_stub("example.dll", "Example"),
        StubException {
            code: 0x8000_0100,
            flags: 1,
            number_parameters: 2,
            module: "example.dll",
            function: "Example",
        }
    );
}

#[test]
fn export_table_counts_match_the_spec() {
    // Not oracle-derived: the counts of the 128 lines of dlls/shdocvw/shdocvw.spec, by kind.
    let count = |kind| EXPORTS.iter().filter(|(_, k)| *k == kind).count();
    assert_eq!(EXPORTS.len(), 128);
    assert_eq!(count(ExportKind::Stub), 101);
    assert_eq!(count(ExportKind::Ported), 21);
    assert_eq!(count(ExportKind::NotPorted), 4);
    assert_eq!(count(ExportKind::Forwarded), 2);
}

#[test]
fn export_kind_lookups_are_exact() {
    // Not oracle-derived: one name of each kind, and names that are absent or differ in case.
    assert_eq!(export_kind("WinList_Init"), Some(ExportKind::Ported));
    assert_eq!(export_kind("DllGetVersion"), Some(ExportKind::NotPorted));
    assert_eq!(export_kind("OpenURL"), Some(ExportKind::Forwarded));
    assert_eq!(export_kind("WhichPlatform"), Some(ExportKind::Forwarded));
    assert_eq!(export_kind("161"), Some(ExportKind::Stub));
    assert_eq!(export_kind("NoSuchExport"), None);
    assert_eq!(export_kind("winlist_init"), None);
}

#[test]
fn only_stub_exports_have_a_stub_record() {
    // Not oracle-derived: every stub export has a record, and no other export does.
    for (name, kind) in EXPORTS {
        assert_eq!(
            stub_for_export(name).is_some(),
            kind == ExportKind::Stub,
            "export {name}"
        );
    }
    assert_eq!(stub_for_export("NoSuchExport"), None);
}
