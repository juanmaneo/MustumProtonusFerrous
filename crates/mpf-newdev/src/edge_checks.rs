//! Edge-case tests for the ported newdev logic.
//!
//! These cover the buffer boundaries of the ANSI thunk, the multi-string search, the stub record
//! and the export table. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn text(wide: &[u16]) -> String {
    let end = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16(&wide[..end]).expect("ASCII input")
}

#[test]
fn buffer_sizes_match_the_headers() {
    // Not oracle-derived on its own: include/cfgmgr32.h:106 and include/windef.h:108. The oracle
    // rows use the same sizes in their shims.
    assert_eq!(MAX_DEVICE_ID_LEN, 200);
    assert_eq!(MAX_PATH, 260);
}

#[test]
fn ansi_input_that_fills_the_buffer_is_cut_and_keeps_its_nul() {
    // Not oracle-derived, and not bug-compatible. The C body passes an unterminated buffer on
    // overflow, and the wide side reads uninitialised stack. The port keeps the buffers at their
    // full size and NUL-terminated.
    let ret = update_driver_for_plug_and_play_devices_a(
        &[b'A'; 200],
        &[b'B'; 260],
        0,
        None,
        |hw, inf, _, _| {
            assert_eq!(hw.len(), MAX_DEVICE_ID_LEN);
            assert_eq!(inf.len(), MAX_PATH);
            assert_eq!(hw[MAX_DEVICE_ID_LEN - 1], 0);
            assert_eq!(inf[MAX_PATH - 1], 0);
            assert_eq!(text(hw), "A".repeat(199));
            assert_eq!(text(inf), "B".repeat(259));
            true
        },
    );
    assert!(ret);
}

#[test]
fn ansi_input_is_read_up_to_its_first_nul() {
    // Not oracle-derived: a C string ends at its first NUL, so only the bytes before it are widened.
    let ret =
        update_driver_for_plug_and_play_devices_a(b"AB\0CD", b"x\0y", 0, None, |hw, inf, _, _| {
            assert_eq!(text(hw), "AB");
            assert_eq!(text(inf), "x");
            true
        });
    assert!(ret);
}

#[test]
fn non_ascii_bytes_widen_one_to_one() {
    // Not oracle-derived, and not a verified code page. CP_ACP depends on the locale, so the port
    // widens each byte to the code unit of the same value. This test only pins that placeholder.
    let ret = update_driver_for_plug_and_play_devices_a(
        &[0xE9, 0xFF],
        &[0x80],
        0,
        None,
        |hw, inf, _, _| {
            assert_eq!(hw[0], 0x00E9);
            assert_eq!(hw[1], 0x00FF);
            assert_eq!(hw[2], 0);
            assert_eq!(inf[0], 0x0080);
            assert_eq!(inf[1], 0);
            true
        },
    );
    assert!(ret);
}

#[test]
fn flags_and_reboot_reach_the_wide_callee_unchanged() {
    // Not oracle-derived beyond rows 17-19: a flags value with the high bit set, and a `Some`
    // reboot pointer. The thunk must neither change the flags nor write the pointer.
    let mut reboot = false;
    let ret = update_driver_for_plug_and_play_devices_a(
        b"",
        b"",
        0x8000_0001,
        Some(&mut reboot),
        |_, _, flags, reboot| {
            assert_eq!(flags, 0x8000_0001);
            assert!(reboot.is_some());
            false
        },
    );
    assert!(!ret);
    assert!(!reboot);
}

#[test]
fn hardware_id_search_stops_at_the_empty_entry() {
    // Not oracle-derived beyond row 24. The second entry sits behind an empty first one, so the
    // `while (*device_ids)` condition in main.c never reaches it.
    assert!(!hardware_id_matches(&wide("B\0"), &wide("A\0\0B\0\0")));
}

#[test]
fn hardware_id_search_matches_whole_strings_in_any_position() {
    // Not oracle-derived: row 22 covers a prefix mismatch, and row 25 covers the second entry.
    // These cover the last entry and the case where the id is longer than the entry.
    assert!(hardware_id_matches(&wide("C\0"), &wide("A\0B\0C\0\0")));
    assert!(!hardware_id_matches(&wide("AB\0"), &wide("A\0B\0\0")));
    assert!(!hardware_id_matches(&wide("A\0"), &wide("AB\0\0")));
}

#[test]
fn hardware_id_search_treats_the_end_of_a_slice_as_a_terminator() {
    // Not oracle-derived, and not bug-compatible. The C loop would read past the end of an
    // unterminated id or multi-string. The port treats the slice end as the end of the string.
    assert!(hardware_id_matches(&wide("B"), &wide("A\0B")));
    assert!(hardware_id_matches(&wide("A\0"), &wide("A")));
    assert!(!hardware_id_matches(&wide("\0"), &[]));
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the 12 names of newdev.spec. The code, flags and
    // parameter count are the same for any names.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Foo"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "Foo",
        }
    );
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 18 lines of dlls/newdev/newdev.spec, in order. `@ stub` is Stub and
    // `@ stdcall` is Stdcall.
    assert_eq!(
        EXPORTS,
        [
            ("DeviceInternetSettingUiW", ExportKind::Stub),
            ("DiInstallDevice", ExportKind::Stub),
            ("DiInstallDriverA", ExportKind::Stdcall),
            ("DiInstallDriverW", ExportKind::Stdcall),
            ("DiRollbackDriver", ExportKind::Stub),
            ("DiShowUpdateDevice", ExportKind::Stub),
            ("DiUninstallDevice", ExportKind::Stub),
            ("InstallNewDevice", ExportKind::Stdcall),
            ("InstallSelectedDriver", ExportKind::Stdcall),
            ("InstallWindowsUpdateDriver", ExportKind::Stub),
            ("SetInternetPolicies", ExportKind::Stub),
            ("UpdateDriverForPlugAndPlayDevicesA", ExportKind::Stdcall),
            ("UpdateDriverForPlugAndPlayDevicesW", ExportKind::Stdcall),
            ("pDiDeviceInstallActionW", ExportKind::Stub),
            ("pDiDeviceInstallNotificationW", ExportKind::Stub),
            ("pDiDoDeviceInstallAsAdmin", ExportKind::Stub),
            ("pDiDoFinishInstallAsAdmin", ExportKind::Stub),
            ("pDiDoNullDriverInstall", ExportKind::Stub),
        ]
    );
    assert_eq!(export_kind("InstallNewDevice"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("DiInstallDevice"), Some(ExportKind::Stub));
    assert_eq!(export_kind("installnewdevice"), None);
    assert_eq!(export_kind(""), None);
}
