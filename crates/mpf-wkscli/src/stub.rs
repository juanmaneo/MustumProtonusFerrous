/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = wkscli.dll` in `dlls/wkscli/Makefile.in`.
const WKSCLI_DLL_NAME: &str = "wkscli.dll";

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

/// `NetAddAlternateComputerName`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_add_alternate_computer_name() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetAddAlternateComputerName")
}

/// `NetEnumerateComputerNames`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_enumerate_computer_names() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetEnumerateComputerNames")
}

/// `NetGetJoinableOUs`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_get_joinable_ous() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetGetJoinableOUs")
}

/// `NetJoinDomain`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_join_domain() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetJoinDomain")
}

/// `NetRemoveAlternateComputerName`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_remove_alternate_computer_name() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetRemoveAlternateComputerName")
}

/// `NetRenameMachineInDomain`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_rename_machine_in_domain() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetRenameMachineInDomain")
}

/// `NetSetPrimaryComputerName`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_set_primary_computer_name() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetSetPrimaryComputerName")
}

/// `NetUnjoinDomain`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_unjoin_domain() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetUnjoinDomain")
}

/// `NetValidateName`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_validate_name() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetValidateName")
}

/// `NetWkstaGetInfo`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_wksta_get_info() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetWkstaGetInfo")
}

/// `NetWkstaSetInfo`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_wksta_set_info() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetWkstaSetInfo")
}

/// `NetWkstaStatisticsGet`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_wksta_statistics_get() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetWkstaStatisticsGet")
}

/// `NetWkstaTransportAdd`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_wksta_transport_add() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetWkstaTransportAdd")
}

/// `NetWkstaTransportDel`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_wksta_transport_del() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetWkstaTransportDel")
}

/// `NetWkstaUserEnum`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_wksta_user_enum() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetWkstaUserEnum")
}

/// `NetWkstaUserGetInfo`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_wksta_user_get_info() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetWkstaUserGetInfo")
}

/// `NetWkstaUserSetInfo`, exported as `@ stub` in `dlls/wkscli/wkscli.spec`.
pub fn net_wksta_user_set_info() -> StubException {
    wine_spec_unimplemented_stub(WKSCLI_DLL_NAME, "NetWkstaUserSetInfo")
}
