//! Oracle-derived test vectors for the ported newdev logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `InstallNewDevice`,
//! `InstallSelectedDriver`, `DiInstallDriverA`, `DiInstallDriverW`,
//! `UpdateDriverForPlugAndPlayDevicesA` and `hardware_id_matches` (`dlls/newdev/main.c`), and of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`), with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/newdev/`. The harness printed 25 rows.
//! ASan and UBSan flagged none of them, so none was excluded for that reason. One input was never
//! run by design: an ANSI hardware ID of 200 or more bytes, where the C body leaves the wide buffer
//! without a NUL. `edge_checks.rs` covers that case.

use super::*;

/// The text of a wide string up to its first NUL.
fn text(wide: &[u16]) -> String {
    let end = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16(&wide[..end]).expect("ASCII rows")
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// The arguments the thunk passed to the wide function: the hardware ID and INF path as text, the
/// flags, and whether `reboot` was NULL.
type WArgs = (String, String, u32, bool);

/// A stub helper and the export name it must raise for.
type StubRow = (fn() -> StubException, &'static str);

/// Runs the thunk with a recorder in place of `UpdateDriverForPlugAndPlayDevicesW`, which returns
/// `ret_w`. Returns the thunk's return value and the arguments the recorder saw.
fn thunk(
    hardware_id: &[u8],
    inf_path: &[u8],
    flags: u32,
    reboot: Option<&mut bool>,
    ret_w: bool,
) -> (bool, WArgs) {
    let mut seen = None;
    let ret = update_driver_for_plug_and_play_devices_a(
        hardware_id,
        inf_path,
        flags,
        reboot,
        |hw, inf, flags, reboot| {
            seen = Some((text(hw), text(inf), flags, reboot.is_none()));
            ret_w
        },
    );
    (ret, seen.expect("the wide callee runs on every thunk call"))
}

#[test]
fn oracle_stub_rows() {
    // rows 01-12: raise code=0x80000100 flags=0x1 nargs=2 p0=newdev.dll p1=<export>
    let rows: [StubRow; 12] = [
        (device_internet_setting_ui_w, "DeviceInternetSettingUiW"),
        (di_install_device, "DiInstallDevice"),
        (di_rollback_driver, "DiRollbackDriver"),
        (di_show_update_device, "DiShowUpdateDevice"),
        (di_uninstall_device, "DiUninstallDevice"),
        (install_windows_update_driver, "InstallWindowsUpdateDriver"),
        (set_internet_policies, "SetInternetPolicies"),
        (p_di_device_install_action_w, "pDiDeviceInstallActionW"),
        (
            p_di_device_install_notification_w,
            "pDiDeviceInstallNotificationW",
        ),
        (p_di_do_device_install_as_admin, "pDiDoDeviceInstallAsAdmin"),
        (p_di_do_finish_install_as_admin, "pDiDoFinishInstallAsAdmin"),
        (p_di_do_null_driver_install, "pDiDoNullDriverInstall"),
    ];
    for (row, (stub, export)) in rows.into_iter().enumerate() {
        assert_eq!(
            stub(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "newdev.dll",
                function: export,
            },
            "row {:02}",
            row + 1
        );
    }
}

#[test]
fn oracle_return_value_rows() {
    // The C bodies never write their reboot or output arguments, and the port has no such
    // parameters, so these rows check the return value only.
    // row 13 InstallNewDevice(hwnd=NULL, class=NULL, reboot=&7) -> ret=1
    assert!(install_new_device());
    // row 14 InstallSelectedDriver(parent=1, info=2, reserved=x, backup=1, reboot=&7) -> ret=1
    assert!(install_selected_driver());
    // row 15 DiInstallDriverA(parent=NULL, inf="", flags=0xffffffff, reboot=&5) -> ret=1
    assert!(di_install_driver_a());
    // row 16 DiInstallDriverW(all NULL) -> ret=1
    assert!(di_install_driver_w());
}

#[test]
fn oracle_update_driver_a_rows() {
    // row 17 A(hwid=ABC, inf=C:\a.inf, flags=0, reboot=NULL) -> W(same, flags=0, reboot=NULL) ret=1
    assert_eq!(
        thunk(b"ABC", b"C:\\a.inf", 0, None, true),
        (true, ("ABC".into(), "C:\\a.inf".into(), 0, true))
    );

    // row 18 A(hwid=ABC, inf="", flags=0x2, reboot=&5) -> W(flags=0x2, reboot=set) ret=0, and the
    // reboot value stays 5. The C BOOL 5 is modelled as `true`, since `bool` cannot hold 5.
    let mut reboot = true;
    assert_eq!(
        thunk(b"ABC", b"", 2, Some(&mut reboot), false),
        (false, ("ABC".into(), String::new(), 2, false))
    );
    assert!(reboot);

    // row 19 A(hwid="", inf="", flags=0xffffffff, reboot=NULL) -> W(same) ret=1
    assert_eq!(
        thunk(b"", b"", u32::MAX, None, true),
        (true, (String::new(), String::new(), u32::MAX, true))
    );

    // row 20 exact fit: 199 bytes plus NUL fill MAX_DEVICE_ID_LEN, 259 plus NUL fill MAX_PATH
    let hardware_id = [b'A'; 199];
    let inf_path = [b'B'; 259];
    assert_eq!(
        thunk(&hardware_id, &inf_path, 0, None, true),
        (true, ("A".repeat(199), "B".repeat(259), 0, true))
    );
}

#[test]
fn oracle_hardware_id_matches_rows() {
    // row 21 hardware_id_matches(id=ACPI\X, device_ids=PCI\Y|ACPI\X|||) -> 1
    assert!(hardware_id_matches(
        &wide("ACPI\\X\0"),
        &wide("PCI\\Y\0ACPI\\X\0\0")
    ));
    // row 22 hardware_id_matches(id=ACPI, device_ids=ACPI_X|||) -> 0
    assert!(!hardware_id_matches(&wide("ACPI\0"), &wide("ACPI_X\0\0")));
    // row 23 hardware_id_matches(id="", device_ids=|||) -> 0
    assert!(!hardware_id_matches(&wide("\0"), &wide("\0\0")));
    // row 24 hardware_id_matches(id=ACPI\X, device_ids=|ACPI\X|||) -> 0: an empty first entry
    // ends the list
    assert!(!hardware_id_matches(
        &wide("ACPI\\X\0"),
        &wide("\0ACPI\\X\0\0")
    ));
    // row 25 hardware_id_matches(id=B, device_ids=A|B|||) -> 1
    assert!(hardware_id_matches(&wide("B\0"), &wide("A\0B\0\0")));
}
