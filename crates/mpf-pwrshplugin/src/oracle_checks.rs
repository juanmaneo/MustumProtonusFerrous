//! Oracle-derived test vectors for the ported pwrshplugin stubs.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/pwrshplugin/`, and running the harness. The harness passes `pwrshplugin.dll` and
//! each export name of `pwrshplugin.spec`, and records the raised record. The 12 printed rows are
//! one per `@ stub` export. ASan and UBSan flagged no row, so none is excluded.

use super::*;

const PWRSHPLUGIN_DLL: &str = "pwrshplugin.dll";

/// The printed rows, in spec order: the export name the harness passed, and the record it raised.
const ORACLE_ROWS: [(&str, StubException); 12] = [
    (
        "GetCLRVersionForPSVersion",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "GetCLRVersionForPSVersion",
        },
    ),
    (
        "PerformWSManPluginReportCompletion",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "PerformWSManPluginReportCompletion",
        },
    ),
    (
        "WSManPluginCommand",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginCommand",
        },
    ),
    (
        "WSManPluginConnect",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginConnect",
        },
    ),
    (
        "WSManPluginReceive",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginReceive",
        },
    ),
    (
        "WSManPluginReleaseCommandContext",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginReleaseCommandContext",
        },
    ),
    (
        "WSManPluginReleaseShellContext",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginReleaseShellContext",
        },
    ),
    (
        "WSManPluginSend",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginSend",
        },
    ),
    (
        "WSManPluginShell",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginShell",
        },
    ),
    (
        "WSManPluginShutdown",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginShutdown",
        },
    ),
    (
        "WSManPluginSignal",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginSignal",
        },
    ),
    (
        "WSManPluginStartup",
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: PWRSHPLUGIN_DLL,
            function: "WSManPluginStartup",
        },
    ),
];

#[test]
fn oracle_export_rows() {
    // rows 01-12: export_stub(name) raises the record the harness printed for that name.
    for (name, expected) in ORACLE_ROWS {
        assert_eq!(export_stub(name), Some(expected), "{name}");
    }
}

#[test]
fn oracle_helper_rows() {
    // rows 01-12, one helper per export, in spec order.
    assert_eq!(get_clr_version_for_ps_version(), ORACLE_ROWS[0].1);
    assert_eq!(perform_ws_man_plugin_report_completion(), ORACLE_ROWS[1].1);
    assert_eq!(ws_man_plugin_command(), ORACLE_ROWS[2].1);
    assert_eq!(ws_man_plugin_connect(), ORACLE_ROWS[3].1);
    assert_eq!(ws_man_plugin_receive(), ORACLE_ROWS[4].1);
    assert_eq!(ws_man_plugin_release_command_context(), ORACLE_ROWS[5].1);
    assert_eq!(ws_man_plugin_release_shell_context(), ORACLE_ROWS[6].1);
    assert_eq!(ws_man_plugin_send(), ORACLE_ROWS[7].1);
    assert_eq!(ws_man_plugin_shell(), ORACLE_ROWS[8].1);
    assert_eq!(ws_man_plugin_shutdown(), ORACLE_ROWS[9].1);
    assert_eq!(ws_man_plugin_signal(), ORACLE_ROWS[10].1);
    assert_eq!(ws_man_plugin_startup(), ORACLE_ROWS[11].1);
}
