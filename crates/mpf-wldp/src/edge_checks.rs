//! Edge-case tests for the ported wldp logic.
//!
//! These cover the constants, the NULL and non-NULL branch of `WldpGetLockdownPolicy`, the
//! out-parameter writes for every initial value, and the export and stub tables. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/wldp.h, include/windef.h,
    // include/winternl.h and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(FACILITY_NT_BIT, 0x1000_0000);
    assert_eq!(FALSE, 0);
    assert_eq!(WLDP_LOCKDOWN_DEFINED_FLAG, 0x8000_0000);
    assert_eq!(WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED, 0);
    assert_eq!(WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL, 1);
    assert_eq!(WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED, 2);
    assert_eq!(WLDP_WINDOWS_LOCKDOWN_MODE_MAX, 3);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(WLDP_DLL_NAME, "wldp.dll");
}

#[test]
fn hresult_from_nt_sets_only_the_facility_bit() {
    // Not oracle-derived, except the zero case, which the oracle's NULL-info rows use.
    assert_eq!(hresult_from_nt(0), 0x1000_0000);
    assert_eq!(hresult_from_nt(0x0000_0005), 0x1000_0005);
    // The facility bit is already set: the value is unchanged.
    assert_eq!(hresult_from_nt(0x1000_0000), 0x1000_0000);
    // 0xc000_0000 | 0x1000_0000 = 0xd000_0000, which is negative as a signed HRESULT.
    assert_eq!(hresult_from_nt(0xc000_0000), -0x3000_0000);
}

#[test]
fn is_dynamic_code_policy_enabled_writes_false_from_any_start() {
    // Not oracle-derived: the oracle starts from one sentinel only. The write is unconditional.
    for start in [0, 1, Bool::MIN, Bool::MAX] {
        let mut enabled = start;
        assert_eq!(is_dynamic_code_policy_enabled(&mut enabled), S_OK);
        assert_eq!(enabled, FALSE);
    }
}

#[test]
fn get_lockdown_policy_writes_state_on_both_branches() {
    // Not oracle-derived for the start values 0 and u32::MAX: the oracle starts from 0xdeadbeef.
    let info = WldpHostInformation;
    for start in [0, u32::MAX] {
        let mut state = start;
        assert_eq!(get_lockdown_policy(Some(&info), &mut state, 0), S_OK);
        assert_eq!(state, WLDP_LOCKDOWN_DEFINED_FLAG);

        let mut state = start;
        assert_eq!(get_lockdown_policy(None, &mut state, 0), hresult_from_nt(0));
        assert_eq!(state, WLDP_LOCKDOWN_DEFINED_FLAG);
    }
}

#[test]
fn get_lockdown_policy_ignores_flags() {
    // Not oracle-derived: the oracle covers flags 0 and 0xffffffff only. Every flag value gives the
    // same return value and state, because the body never reads flags.
    let info = WldpHostInformation;
    for flags in [0, 1, 0x8000_0000, u32::MAX] {
        let mut state = 0;
        assert_eq!(get_lockdown_policy(Some(&info), &mut state, flags), S_OK);
        assert_eq!(state, WLDP_LOCKDOWN_DEFINED_FLAG);
    }
}

#[test]
fn query_windows_lockdown_mode_writes_unlocked_from_any_start() {
    // Not oracle-derived: the oracle starts from 0xdeadbeef only.
    for start in [0, 1, WLDP_WINDOWS_LOCKDOWN_MODE_MAX, u32::MAX] {
        let mut mode = start;
        assert_eq!(query_windows_lockdown_mode(&mut mode), S_OK);
        assert_eq!(mode, WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED);
    }
}

#[test]
fn export_table_has_the_spec_counts() {
    // Not oracle-derived: counts from dlls/wldp/wldp.spec (3 @ stdcall, 26 @ stub).
    assert_eq!(EXPORTS.len(), 29);
    let stdcall = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stdcall)
        .count();
    let stub = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    assert_eq!(stdcall, 3);
    assert_eq!(stub, 26);
}

#[test]
fn export_kind_finds_each_export_and_rejects_others() {
    // Not oracle-derived: the spec kinds of the three explicit exports, and the lookup misses.
    assert_eq!(
        export_kind("WldpQueryWindowsLockdownMode"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind("WldpGetLockdownPolicy"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind("WldpIsDynamicCodePolicyEnabled"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("WldpIsDebugAllowed"), Some(ExportKind::Stub));
    // Names are case-sensitive, and the empty name is not an export.
    assert_eq!(export_kind("wldpgetlockdownpolicy"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_table_matches_the_stub_exports_in_spec_order() {
    // Not oracle-derived: the stub helpers must follow the spec order and name each export.
    let spec_stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let table: Vec<&str> = STUB_EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(table, spec_stubs);

    for (name, helper) in STUB_EXPORTS {
        let stub = helper();
        assert_eq!(stub.function, *name);
        assert_eq!(stub.module, WLDP_DLL_NAME);
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
    }
}

#[test]
fn stub_exception_has_two_parameters_for_any_names() {
    // Not oracle-derived: the fields follow the C body for any module and function strings.
    let stub = wine_spec_unimplemented_stub("other.dll", "Ordinal");
    assert_eq!(stub.code, EXCEPTION_WINE_STUB);
    assert_eq!(stub.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(stub.number_parameters, 2);
    assert_eq!(stub.module, "other.dll");
    assert_eq!(stub.function, "Ordinal");
    assert_eq!(stub.clone(), stub);
}
