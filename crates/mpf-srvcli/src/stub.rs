/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = srvcli.dll` in `dlls/srvcli/Makefile.in`.
const SRVCLI_DLL_NAME: &str = "srvcli.dll";

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

/// `I_NetDfsGetVersion`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn i_net_dfs_get_version() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "I_NetDfsGetVersion")
}

/// `I_NetServerSetServiceBits`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn i_net_server_set_service_bits() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "I_NetServerSetServiceBits")
}

/// `I_NetServerSetServiceBitsEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn i_net_server_set_service_bits_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "I_NetServerSetServiceBitsEx")
}

/// `LocalAliasGet`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_alias_get() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalAliasGet")
}

/// `LocalFileClose`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_file_close() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalFileClose")
}

/// `LocalFileEnum`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_file_enum() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalFileEnum")
}

/// `LocalFileEnumEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_file_enum_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalFileEnumEx")
}

/// `LocalFileGetInfo`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_file_get_info() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalFileGetInfo")
}

/// `LocalFileGetInfoEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_file_get_info_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalFileGetInfoEx")
}

/// `LocalServerCertificateMappingAdd`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_server_certificate_mapping_add() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalServerCertificateMappingAdd")
}

/// `LocalServerCertificateMappingEnum`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_server_certificate_mapping_enum() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalServerCertificateMappingEnum")
}

/// `LocalServerCertificateMappingGet`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_server_certificate_mapping_get() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalServerCertificateMappingGet")
}

/// `LocalServerCertificateMappingRemove`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_server_certificate_mapping_remove() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalServerCertificateMappingRemove")
}

/// `LocalSessionDel`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_session_del() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalSessionDel")
}

/// `LocalSessionEnum`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_session_enum() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalSessionEnum")
}

/// `LocalSessionEnumEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_session_enum_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalSessionEnumEx")
}

/// `LocalSessionGetInfo`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_session_get_info() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalSessionGetInfo")
}

/// `LocalSessionGetInfoEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_session_get_info_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalSessionGetInfoEx")
}

/// `LocalShareAdd`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_share_add() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalShareAdd")
}

/// `LocalShareDelEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_share_del_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalShareDelEx")
}

/// `LocalShareEnum`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_share_enum() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalShareEnum")
}

/// `LocalShareEnumEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_share_enum_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalShareEnumEx")
}

/// `LocalShareGetInfo`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_share_get_info() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalShareGetInfo")
}

/// `LocalShareGetInfoEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_share_get_info_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalShareGetInfoEx")
}

/// `LocalShareSetInfo`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn local_share_set_info() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "LocalShareSetInfo")
}

/// `NetConnectionEnum`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_connection_enum() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetConnectionEnum")
}

/// `NetFileClose`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_file_close() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetFileClose")
}

/// `NetFileGetInfo`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_file_get_info() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetFileGetInfo")
}

/// `NetServerAliasAdd`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_alias_add() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerAliasAdd")
}

/// `NetServerAliasDel`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_alias_del() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerAliasDel")
}

/// `NetServerAliasEnum`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_alias_enum() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerAliasEnum")
}

/// `NetServerComputerNameAdd`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_computer_name_add() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerComputerNameAdd")
}

/// `NetServerComputerNameDel`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_computer_name_del() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerComputerNameDel")
}

/// `NetServerSetInfo`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_set_info() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerSetInfo")
}

/// `NetServerStatisticsGet`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_statistics_get() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerStatisticsGet")
}

/// `NetServerTransportAdd`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_transport_add() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerTransportAdd")
}

/// `NetServerTransportAddEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_transport_add_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerTransportAddEx")
}

/// `NetServerTransportDel`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_transport_del() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerTransportDel")
}

/// `NetServerTransportEnum`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_server_transport_enum() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetServerTransportEnum")
}

/// `NetSessionDel`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_session_del() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetSessionDel")
}

/// `NetSessionGetInfo`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_session_get_info() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetSessionGetInfo")
}

/// `NetShareCheck`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_share_check() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetShareCheck")
}

/// `NetShareDelEx`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_share_del_ex() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetShareDelEx")
}

/// `NetShareDelSticky`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_share_del_sticky() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetShareDelSticky")
}

/// `NetShareEnumSticky`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_share_enum_sticky() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetShareEnumSticky")
}

/// `NetShareSetInfo`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn net_share_set_info() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetShareSetInfo")
}

/// `NetpsNameCanonicalize`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn netps_name_canonicalize() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetpsNameCanonicalize")
}

/// `NetpsNameCompare`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn netps_name_compare() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetpsNameCompare")
}

/// `NetpsNameValidate`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn netps_name_validate() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetpsNameValidate")
}

/// `NetpsPathCanonicalize`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn netps_path_canonicalize() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetpsPathCanonicalize")
}

/// `NetpsPathCompare`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn netps_path_compare() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetpsPathCompare")
}

/// `NetpsPathType`, exported as `@ stub` in `dlls/srvcli/srvcli.spec`.
pub fn netps_path_type() -> StubException {
    wine_spec_unimplemented_stub(SRVCLI_DLL_NAME, "NetpsPathType")
}
