/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = mswsock.dll` in `dlls/mswsock/Makefile.in`.
const MSWSOCK_DLL_NAME: &str = "mswsock.dll";

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

/// `GetAddressByNameA`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn get_address_by_name_a() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "GetAddressByNameA")
}

/// `GetAddressByNameW`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn get_address_by_name_w() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "GetAddressByNameW")
}

/// `GetNameByTypeA`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn get_name_by_type_a() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "GetNameByTypeA")
}

/// `GetNameByTypeW`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn get_name_by_type_w() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "GetNameByTypeW")
}

/// `GetServiceA`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn get_service_a() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "GetServiceA")
}

/// `GetServiceW`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn get_service_w() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "GetServiceW")
}

/// `GetTypeByNameA`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn get_type_by_name_a() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "GetTypeByNameA")
}

/// `GetTypeByNameW`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn get_type_by_name_w() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "GetTypeByNameW")
}

/// `MigrateWinsockConfiguration`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn migrate_winsock_configuration() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "MigrateWinsockConfiguration")
}

/// `NPLoadNameSpaces`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn np_load_name_spaces() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "NPLoadNameSpaces")
}

/// `NSPStartup`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn nsp_startup() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "NSPStartup")
}

/// `ServiceMain`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn service_main() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "ServiceMain")
}

/// `SetServiceA`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn set_service_a() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "SetServiceA")
}

/// `SetServiceW`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn set_service_w() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "SetServiceW")
}

/// `StartWsdpService`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn start_wsdp_service() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "StartWsdpService")
}

/// `StopWsdpService`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn stop_wsdp_service() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "StopWsdpService")
}

/// `SvchostPushServiceGlobals`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn svchost_push_service_globals() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "SvchostPushServiceGlobals")
}

/// `WSPStartup`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn wsp_startup() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "WSPStartup")
}

/// `dn_expand`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn dn_expand() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "dn_expand")
}

/// `getnetbyname`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn getnetbyname() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "getnetbyname")
}

/// `inet_network`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn inet_network() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "inet_network")
}

/// `rcmd`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn rcmd() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "rcmd")
}

/// `rexec`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn rexec() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "rexec")
}

/// `rresvport`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn rresvport() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "rresvport")
}

/// `s_perror`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn s_perror() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "s_perror")
}

/// `sethostname`, exported as `@ stub` in `dlls/mswsock/mswsock.spec`.
pub fn sethostname() -> StubException {
    wine_spec_unimplemented_stub(MSWSOCK_DLL_NAME, "sethostname")
}
