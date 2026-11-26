/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = netcfgx.dll` in `dlls/netcfgx/Makefile.in`.
const NETCFGX_DLL_NAME: &str = "netcfgx.dll";

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

/// `HrDiAddComponentToINetCfg`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn hr_di_add_component_to_inet_cfg() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "HrDiAddComponentToINetCfg")
}

/// `LanaCfgFromCommandArgs`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn lana_cfg_from_command_args() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "LanaCfgFromCommandArgs")
}

/// `ModemClassCoInstaller`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn modem_class_co_installer() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "ModemClassCoInstaller")
}

/// `NetCfgDiagFromCommandArgs`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn net_cfg_diag_from_command_args() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "NetCfgDiagFromCommandArgs")
}

/// `NetCfgDiagRepairRegistryBindings`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn net_cfg_diag_repair_registry_bindings() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "NetCfgDiagRepairRegistryBindings")
}

/// `NetClassInstaller`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn net_class_installer() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "NetClassInstaller")
}

/// `NetPropPageProvider`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn net_prop_page_provider() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "NetPropPageProvider")
}

/// `RasAddBindings`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn ras_add_bindings() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "RasAddBindings")
}

/// `RasCountBindings`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn ras_count_bindings() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "RasCountBindings")
}

/// `RasRemoveBindings`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn ras_remove_bindings() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "RasRemoveBindings")
}

/// `SvchostChangeSvchostGroup`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn svchost_change_svchost_group() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "SvchostChangeSvchostGroup")
}

/// `UpdateLanaConfigUsingAnswerfile`, `@ stub` in `dlls/netcfgx/netcfgx.spec`.
pub fn update_lana_config_using_answerfile() -> StubException {
    wine_spec_unimplemented_stub(NETCFGX_DLL_NAME, "UpdateLanaConfigUsingAnswerfile")
}
