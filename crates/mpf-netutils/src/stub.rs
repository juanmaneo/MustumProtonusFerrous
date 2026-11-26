/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = netutils.dll` in `dlls/netutils/Makefile.in`.
const NETUTILS_DLL_NAME: &str = "netutils.dll";

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

/// `NetRemoteComputerSupports`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn net_remote_computer_supports() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetRemoteComputerSupports")
}

/// `NetpIsComputerNameValid`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netp_is_computer_name_valid() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpIsComputerNameValid")
}

/// `NetpIsDomainNameValid`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netp_is_domain_name_valid() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpIsDomainNameValid")
}

/// `NetpIsGroupNameValid`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netp_is_group_name_valid() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpIsGroupNameValid")
}

/// `NetpIsRemote`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netp_is_remote() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpIsRemote")
}

/// `NetpIsRemoteNameValid`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netp_is_remote_name_valid() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpIsRemoteNameValid")
}

/// `NetpIsShareNameValid`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netp_is_share_name_valid() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpIsShareNameValid")
}

/// `NetpIsUncComputerNameValid`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netp_is_unc_computer_name_valid() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpIsUncComputerNameValid")
}

/// `NetpIsUserNameValid`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netp_is_user_name_valid() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpIsUserNameValid")
}

/// `NetpwListCanonicalize`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netpw_list_canonicalize() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpwListCanonicalize")
}

/// `NetpwListTraverse`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netpw_list_traverse() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpwListTraverse")
}

/// `NetpwNameCanonicalize`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netpw_name_canonicalize() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpwNameCanonicalize")
}

/// `NetpwNameCompare`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netpw_name_compare() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpwNameCompare")
}

/// `NetpwNameValidate`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netpw_name_validate() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpwNameValidate")
}

/// `NetpwPathCanonicalize`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netpw_path_canonicalize() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpwPathCanonicalize")
}

/// `NetpwPathCompare`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netpw_path_compare() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpwPathCompare")
}

/// `NetpwPathType`, exported as `@ stub` in `dlls/netutils/netutils.spec`.
pub fn netpw_path_type() -> StubException {
    wine_spec_unimplemented_stub(NETUTILS_DLL_NAME, "NetpwPathType")
}
