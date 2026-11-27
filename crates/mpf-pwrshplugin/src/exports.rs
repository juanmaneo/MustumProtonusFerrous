use crate::stub::{
    get_clr_version_for_ps_version, perform_ws_man_plugin_report_completion, ws_man_plugin_command,
    ws_man_plugin_connect, ws_man_plugin_receive, ws_man_plugin_release_command_context,
    ws_man_plugin_release_shell_context, ws_man_plugin_send, ws_man_plugin_shell,
    ws_man_plugin_shutdown, ws_man_plugin_signal, ws_man_plugin_startup, StubException,
};

/// The `pwrshplugin.spec` export table, in spec order. Every export is a `@ stub`.
pub const EXPORTS: [&str; 12] = [
    "GetCLRVersionForPSVersion",
    "PerformWSManPluginReportCompletion",
    "WSManPluginCommand",
    "WSManPluginConnect",
    "WSManPluginReceive",
    "WSManPluginReleaseCommandContext",
    "WSManPluginReleaseShellContext",
    "WSManPluginSend",
    "WSManPluginShell",
    "WSManPluginShutdown",
    "WSManPluginSignal",
    "WSManPluginStartup",
];

/// The stub record that `name` raises, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    match name {
        "GetCLRVersionForPSVersion" => Some(get_clr_version_for_ps_version()),
        "PerformWSManPluginReportCompletion" => Some(perform_ws_man_plugin_report_completion()),
        "WSManPluginCommand" => Some(ws_man_plugin_command()),
        "WSManPluginConnect" => Some(ws_man_plugin_connect()),
        "WSManPluginReceive" => Some(ws_man_plugin_receive()),
        "WSManPluginReleaseCommandContext" => Some(ws_man_plugin_release_command_context()),
        "WSManPluginReleaseShellContext" => Some(ws_man_plugin_release_shell_context()),
        "WSManPluginSend" => Some(ws_man_plugin_send()),
        "WSManPluginShell" => Some(ws_man_plugin_shell()),
        "WSManPluginShutdown" => Some(ws_man_plugin_shutdown()),
        "WSManPluginSignal" => Some(ws_man_plugin_signal()),
        "WSManPluginStartup" => Some(ws_man_plugin_startup()),
        _ => None,
    }
}
