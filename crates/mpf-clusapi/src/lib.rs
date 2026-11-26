//! Port of the portable logic in Wine's `dlls/clusapi/clusapi.c`, plus the
//! `stub` exports declared in `dlls/clusapi/clusapi.spec`.
//!
//! Ported:
//! - `GetClusterInformation`, `GetNodeClusterState`, `OpenCluster`, `CloseCluster`,
//!   `ClusterOpenEnum`, `ClusterCloseEnum` and `ClusterEnum`: each logs `FIXME` and
//!   returns a fixed value, writing only the out-parameter it always writes:
//!   [`get_cluster_information`], [`get_node_cluster_state`], [`open_cluster`],
//!   [`close_cluster`], [`cluster_open_enum`], [`cluster_close_enum`], [`cluster_enum`]
//! - The fixed handle returned by `OpenCluster` and `ClusterOpenEnum`: [`STUB_HANDLE`]
//! - `ERROR_SUCCESS`, `ERROR_NO_MORE_ITEMS` and the `GetNodeClusterState` states:
//!   [`ERROR_SUCCESS`], [`ERROR_NO_MORE_ITEMS`], [`CLUSTER_STATE_NOT_INSTALLED`] and the
//!   other `CLUSTER_STATE_*` constants
//! - The 108 `stub` exports in `clusapi.spec`. Their `__wine_spec_unimplemented_stub`
//!   behaviour (`ntdll/exception.c`) raises `EXCEPTION_WINE_STUB` and never returns:
//!   [`unimplemented_stub_exception`], [`is_spec_stub`], [`SPEC_STUBS`]
//!
//! Skipped:
//! - Arguments that the C bodies never read or write (`hCluster`, `lpszClusterName`,
//!   `lpClusterInfo`, `lpszNodeName`, `dwType`, `hEnum`, `dwIndex`, `lpdwType`, `lpszName`).
//! - `FIXME` logging, `WINE_DEFAULT_DEBUG_CHANNEL`, `debugstr_w`, and the `stdcall`
//!   wrappers.
//! - NULL out-pointers. `GetClusterInformation` and `GetNodeClusterState` write through
//!   `lpcchClusterName` and `pdwClusterState` unconditionally. `ClusterEnum` reads
//!   `lpcchName` in its `FIXME` trace, which Wine enables by default
//!   (`libs/winecrt0/debug.c`). Each therefore crashes on NULL. The reference parameters
//!   make that case unrepresentable.
//! - The `stub` exports have no body in `clusapi.c`, so there is no C logic to port. Only
//!   the shared stub path is modelled. Cluster-service behaviour (RPC, registry, cluster
//!   state) is not in Wine's `clusapi.c` and is not ported.

mod cluster;
mod stub;

pub use cluster::{
    close_cluster, cluster_close_enum, cluster_enum, cluster_open_enum, get_cluster_information,
    get_node_cluster_state, open_cluster, CLUSTER_STATE_NOT_CONFIGURED,
    CLUSTER_STATE_NOT_INSTALLED, CLUSTER_STATE_NOT_RUNNING, CLUSTER_STATE_RUNNING,
    ERROR_NO_MORE_ITEMS, ERROR_SUCCESS, STUB_HANDLE,
};
pub use stub::{
    is_spec_stub, unimplemented_stub_exception, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB, SPEC_STUBS,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
