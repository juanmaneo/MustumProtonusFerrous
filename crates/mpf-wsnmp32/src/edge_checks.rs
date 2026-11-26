//! Edge-case tests for the ported wsnmp32 logic.
//!
//! These cover the out-parameter writes of `SnmpStartup`, the handle returned by `SnmpOpen`, and
//! the stub and export tables. Not every expectation is oracle-derived; the ones that aren't say
//! so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winsnmp.h, include/winternl.h and include/winnt.h.
    assert_eq!(SNMPAPI_SUCCESS, 1);
    assert_eq!(SNMPAPI_FAILURE, 0);
    assert_eq!(SNMPAPI_V2_SUPPORT, 2);
    assert_eq!(SNMPAPI_UNTRANSLATED_V1, 1);
    assert_eq!(SNMPAPI_ON, 1);
    assert_eq!(NULL_SESSION, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(WSNMP32_DLL_NAME, "wsnmp32.dll");
}

#[test]
fn startup_overwrites_existing_values() {
    // Not oracle-derived: the oracle only started from the UNSET sentinel. The C body assigns
    // each non-NULL out-parameter unconditionally, so earlier values are replaced.
    let (mut major, mut minor, mut level, mut tmode, mut rmode) = (7, 8, 9, 10, 11);
    let ret = snmp_startup(
        Some(&mut major),
        Some(&mut minor),
        Some(&mut level),
        Some(&mut tmode),
        Some(&mut rmode),
    );
    assert_eq!(ret, SNMPAPI_SUCCESS);
    assert_eq!((major, minor, level, tmode, rmode), (2, 0, 2, 1, 1));
}

#[test]
fn startup_each_none_out_parameter_is_left_alone() {
    // Not oracle-derived beyond rows 13 to 17: a `None` is a NULL pointer, which the C body
    // skips, and the one `Some` in each call is the only out-parameter written.
    let mut level = 0;
    assert_eq!(
        snmp_startup(None, None, Some(&mut level), None, None),
        SNMPAPI_SUCCESS
    );
    assert_eq!(level, SNMPAPI_V2_SUPPORT);

    let mut tmode = 0;
    assert_eq!(
        snmp_startup(None, None, None, Some(&mut tmode), None),
        SNMPAPI_SUCCESS
    );
    assert_eq!(tmode, SNMPAPI_UNTRANSLATED_V1);
}

#[test]
fn open_ignores_window_and_message() {
    // Not oracle-derived beyond rows 02 and 03: any window and message give the same NULL
    // session, because the C body only logs them.
    for (hwnd, msg) in [(0, 0), (usize::MAX, u32::MAX), (1, 0x8000_0000)] {
        assert_eq!(snmp_open(hwnd, msg), NULL_SESSION);
    }
}

#[test]
fn setters_ignore_their_argument() {
    // Not oracle-derived beyond rows 04 to 11: the setters return success for any value.
    for mode in [0, 3, 0x8000_0000, u32::MAX] {
        assert_eq!(snmp_set_retransmit_mode(mode), SNMPAPI_SUCCESS);
        assert_eq!(snmp_set_translate_mode(mode), SNMPAPI_SUCCESS);
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of wsnmp32.spec. The code and
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
fn stub_table_matches_the_stub_entries_of_the_export_table() {
    // Not oracle-derived: the helper table and the export table must list the same stub names
    // in the same order, and each helper must raise for its own name.
    assert_eq!(STUB_EXPORTS.len(), 43);
    let stub_names: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let helper_names: Vec<&str> = STUB_EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(helper_names, stub_names);

    for (name, helper) in STUB_EXPORTS {
        assert_eq!(helper().function, *name);
        assert_eq!(helper().module, WSNMP32_DLL_NAME);
    }
}

#[test]
fn named_helpers_match_their_export_names() {
    // Not oracle-derived: spot checks that the snake_case helpers call the right export names,
    // including the decorated `_Name@4` exports.
    assert_eq!(snmp_get_timeout().function, "SnmpGetTimeout");
    assert_eq!(
        snmp_convey_agent_address().function,
        "_SnmpConveyAgentAddress@4"
    );
    assert_eq!(snmp_set_agent_address().function, "_SnmpSetAgentAddress@4");
    assert_eq!(snmp_get_last_error().function, "SnmpGetLastError");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 48 lines of dlls/wsnmp32/wsnmp32.spec, in order. The first and last
    // entries and the five stdcall exports are spot-checked.
    assert_eq!(EXPORTS.len(), 48);
    assert_eq!(EXPORTS[0], ("SnmpGetTranslateMode", ExportKind::Stub));
    assert_eq!(EXPORTS[1], ("SnmpSetTranslateMode", ExportKind::Stdcall));
    assert_eq!(EXPORTS[47], ("SnmpGetLastError", ExportKind::Stub));
    let stdcall: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stdcall)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(
        stdcall,
        [
            "SnmpSetTranslateMode",
            "SnmpSetRetransmitMode",
            "SnmpStartup",
            "SnmpCleanup",
            "SnmpOpen",
        ]
    );
    assert_eq!(export_kind("SnmpStartup"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("SnmpClose"), Some(ExportKind::Stub));
    assert_eq!(export_kind("snmpstartup"), None);
    assert_eq!(export_kind(""), None);
}
