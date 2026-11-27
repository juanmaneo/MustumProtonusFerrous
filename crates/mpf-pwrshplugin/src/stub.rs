/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = pwrshplugin.dll` in `dlls/pwrshplugin/Makefile.in`.
const PWRSHPLUGIN_DLL_NAME: &str = "pwrshplugin.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `GetCLRVersionForPSVersion`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn get_clr_version_for_ps_version() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "GetCLRVersionForPSVersion")
}

/// `PerformWSManPluginReportCompletion`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn perform_ws_man_plugin_report_completion() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "PerformWSManPluginReportCompletion")
}

/// `WSManPluginCommand`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_command() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginCommand")
}

/// `WSManPluginConnect`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_connect() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginConnect")
}

/// `WSManPluginReceive`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_receive() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginReceive")
}

/// `WSManPluginReleaseCommandContext`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_release_command_context() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginReleaseCommandContext")
}

/// `WSManPluginReleaseShellContext`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_release_shell_context() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginReleaseShellContext")
}

/// `WSManPluginSend`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_send() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginSend")
}

/// `WSManPluginShell`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_shell() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginShell")
}

/// `WSManPluginShutdown`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_shutdown() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginShutdown")
}

/// `WSManPluginSignal`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_signal() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginSignal")
}

/// `WSManPluginStartup`, a `@ stub` export of `dlls/pwrshplugin/pwrshplugin.spec`.
pub fn ws_man_plugin_startup() -> StubException {
    wine_spec_unimplemented_stub(PWRSHPLUGIN_DLL_NAME, "WSManPluginStartup")
}
