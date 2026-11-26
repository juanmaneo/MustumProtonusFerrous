/// `EXCEPTION_NONCONTINUABLE` from `winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// `EXCEPTION_WINE_STUB` from `winternl.h`: "stub entry point called".
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// Named exports that `clusapi.spec` declares as `stub`. The seven `stdcall`
/// exports that `clusapi.c` implements are not in this list.
pub const SPEC_STUBS: [&str; 108] = [
    "AddClusterResourceDependency",
    "AddClusterResourceNode",
    "BackupClusterDatabase",
    "CanResourceBeDependent",
    "ChangeClusterResourceGroup",
    "CloseClusterGroup",
    "CloseClusterNetInterface",
    "CloseClusterNetwork",
    "CloseClusterNode",
    "CloseClusterNotifyPort",
    "CloseClusterResource",
    "ClusterControl",
    "ClusterGetEnumCount",
    "ClusterGroupCloseEnum",
    "ClusterGroupControl",
    "ClusterGroupEnum",
    "ClusterGroupGetEnumCount",
    "ClusterGroupOpenEnum",
    "ClusterNetInterfaceControl",
    "ClusterNetworkCloseEnum",
    "ClusterNetworkControl",
    "ClusterNetworkEnum",
    "ClusterNetworkGetEnumCount",
    "ClusterNetworkOpenEnum",
    "ClusterNodeCloseEnum",
    "ClusterNodeControl",
    "ClusterNodeEnum",
    "ClusterNodeGetEnumCount",
    "ClusterNodeOpenEnum",
    "ClusterRegCloseKey",
    "ClusterRegCreateKey",
    "ClusterRegDeleteKey",
    "ClusterRegDeleteValue",
    "ClusterRegEnumKey",
    "ClusterRegEnumValue",
    "ClusterRegGetKeySecurity",
    "ClusterRegOpenKey",
    "ClusterRegQueryInfoKey",
    "ClusterRegQueryValue",
    "ClusterRegSetKeySecurity",
    "ClusterRegSetValue",
    "ClusterResourceCloseEnum",
    "ClusterResourceControl",
    "ClusterResourceEnum",
    "ClusterResourceGetEnumCount",
    "ClusterResourceOpenEnum",
    "ClusterResourceTypeCloseEnum",
    "ClusterResourceTypeControl",
    "ClusterResourceTypeEnum",
    "ClusterResourceTypeGetEnumCount",
    "ClusterResourceTypeOpenEnum",
    "CreateClusterGroup",
    "CreateClusterNotifyPort",
    "CreateClusterResource",
    "CreateClusterResourceType",
    "DeleteClusterGroup",
    "DeleteClusterResource",
    "DeleteClusterResourceType",
    "EvictClusterNode",
    "EvictClusterNodeEx",
    "FailClusterResource",
    "GetClusterFromGroup",
    "GetClusterFromNetInterface",
    "GetClusterFromNetwork",
    "GetClusterFromNode",
    "GetClusterFromResource",
    "GetClusterGroupKey",
    "GetClusterGroupState",
    "GetClusterKey",
    "GetClusterNetInterface",
    "GetClusterNetInterfaceKey",
    "GetClusterNetInterfaceState",
    "GetClusterNetworkId",
    "GetClusterNetworkKey",
    "GetClusterNetworkState",
    "GetClusterNodeId",
    "GetClusterNodeKey",
    "GetClusterNodeState",
    "GetClusterNotify",
    "GetClusterQuorumResource",
    "GetClusterResourceKey",
    "GetClusterResourceNetworkName",
    "GetClusterResourceState",
    "GetClusterResourceTypeKey",
    "MoveClusterGroup",
    "OfflineClusterGroup",
    "OfflineClusterResource",
    "OnlineClusterGroup",
    "OnlineClusterResource",
    "OpenClusterGroup",
    "OpenClusterNetInterface",
    "OpenClusterNetwork",
    "OpenClusterNode",
    "OpenClusterResource",
    "PauseClusterNode",
    "RegisterClusterNotify",
    "RemoveClusterResourceDependency",
    "RemoveClusterResourceNode",
    "RestoreClusterDatabase",
    "ResumeClusterNode",
    "SetClusterGroupName",
    "SetClusterGroupNodeList",
    "SetClusterName",
    "SetClusterNetworkName",
    "SetClusterNetworkPriorityOrder",
    "SetClusterQuorumResource",
    "SetClusterResourceName",
    "SetClusterServiceAccountPassword",
];

/// The `EXCEPTION_RECORD` fields that `__wine_spec_unimplemented_stub` sets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
}

/// What a spec `stub` export does when called. Wine's handler passes the
/// module and function names to `RtlRaiseException` in a loop, so the call
/// never returns to the caller.
pub fn unimplemented_stub_exception() -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
    }
}

/// Whether `name` is one of the named `stub` exports in [`SPEC_STUBS`].
pub fn is_spec_stub(name: &str) -> bool {
    SPEC_STUBS.contains(&name)
}
