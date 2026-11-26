//! Oracle-derived test vectors for the ported wldp logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `WldpIsDynamicCodePolicyEnabled`,
//! `WldpGetLockdownPolicy` and `WldpQueryWindowsLockdownMode` (`dlls/wldp/wldp.c`) and of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/wldp/`. The 32 printed rows are the
//! 6 explicit calls (rows 01-06) and the 26 `@ stub` exports (rows 07-32). None of them was flagged
//! by ASan or UBSan, so none is excluded.

use super::*;

#[test]
fn oracle_explicit_rows() {
    // row 01 WldpIsDynamicCodePolicyEnabled in=sentinel1 ret=0x00000000 out=0
    let mut enabled: Bool = 1;
    assert_eq!(is_dynamic_code_policy_enabled(&mut enabled), 0x0000_0000);
    assert_eq!(enabled, 0);

    // row 02 WldpGetLockdownPolicy info=set flags=0 ret=0x00000000 state=0x80000000
    let info = WldpHostInformation;
    let mut state = 0xdead_beef;
    assert_eq!(get_lockdown_policy(Some(&info), &mut state, 0), 0x0000_0000);
    assert_eq!(state, 0x8000_0000);

    // row 03 WldpGetLockdownPolicy info=NULL flags=0 ret=0x10000000 state=0x80000000
    let mut state = 0xdead_beef;
    assert_eq!(get_lockdown_policy(None, &mut state, 0), 0x1000_0000);
    assert_eq!(state, 0x8000_0000);

    // row 04 WldpGetLockdownPolicy info=set flags=0xffffffff ret=0x00000000 state=0x80000000
    let mut state = 0xdead_beef;
    assert_eq!(
        get_lockdown_policy(Some(&info), &mut state, 0xffff_ffff),
        0x0000_0000
    );
    assert_eq!(state, 0x8000_0000);

    // row 05 WldpGetLockdownPolicy info=NULL flags=0xffffffff ret=0x10000000 state=0x80000000
    let mut state = 0xdead_beef;
    assert_eq!(
        get_lockdown_policy(None, &mut state, 0xffff_ffff),
        0x1000_0000
    );
    assert_eq!(state, 0x8000_0000);

    // row 06 WldpQueryWindowsLockdownMode in=sentinel ret=0x00000000 mode=0x00000000
    let mut mode = 0xdead_beef;
    assert_eq!(query_windows_lockdown_mode(&mut mode), 0x0000_0000);
    assert_eq!(mode, 0);
}

/// The 26 `@ stub` rows (oracle rows 07-32): `(function, code, flags, number_parameters, module)`.
const STUB_ROWS: [(&str, u32, u32, u32, &str); 26] = [
    ("WldpIsAppApprovedByPolicy", 0x8000_0100, 0x1, 2, "wldp.dll"),
    (
        "WldpIsProductionConfiguration",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpIsWcosProductionConfiguration",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpQueryPolicySettingEnabled2",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpQueryPolicySettingEnabled",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpQueryPolicySettingEnabledInternal",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpQueryWindowsLockdownRestriction",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpResetProductionConfiguration",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpResetWcosProductionConfiguration",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpSetWindowsLockdownRestriction",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpTraceLoggingWDACBlockDialogShown",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpAddDeveloperCertificateForDynamicCodeTrust",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpCheckDeviceEncryptionNotStarted",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpCheckRetailConfiguration",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpCheckWcosDeviceEncryptionSecure",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    ("WldpDisableDeveloperMode", 0x8000_0100, 0x1, 2, "wldp.dll"),
    ("WldpEnableDeveloperMode", 0x8000_0100, 0x1, 2, "wldp.dll"),
    ("WldpIsAllowedEntryPoint", 0x8000_0100, 0x1, 2, "wldp.dll"),
    ("WldpIsClassInApprovedList", 0x8000_0100, 0x1, 2, "wldp.dll"),
    ("WldpIsDebugAllowed", 0x8000_0100, 0x1, 2, "wldp.dll"),
    (
        "WldpIsProductionConfigurationInProc",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    (
        "WldpQueryDeviceSecurityInformation",
        0x8000_0100,
        0x1,
        2,
        "wldp.dll",
    ),
    ("WldpQueryDynamicCodeTrust", 0x8000_0100, 0x1, 2, "wldp.dll"),
    ("WldpQuerySecurityPolicy", 0x8000_0100, 0x1, 2, "wldp.dll"),
    ("WldpSetDynamicCodeTrust", 0x8000_0100, 0x1, 2, "wldp.dll"),
    ("WldpSetDynamicCodeTrust2", 0x8000_0100, 0x1, 2, "wldp.dll"),
];

#[test]
fn oracle_stub_rows() {
    assert_eq!(STUB_ROWS.len(), 26);
    for (function, code, flags, number_parameters, module) in STUB_ROWS {
        let (_, helper) = STUB_EXPORTS
            .iter()
            .find(|(name, _)| *name == function)
            .unwrap();
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}
