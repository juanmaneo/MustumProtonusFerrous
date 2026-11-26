/// `ERROR_SUCCESS` from `winerror.h`.
pub const ERROR_SUCCESS: u32 = 0;

/// `ERROR_NO_MORE_ITEMS` from `winerror.h`.
pub const ERROR_NO_MORE_ITEMS: u32 = 259;

/// `(HCLUSTER)0xdeadbeef` and `(HCLUSENUM)0xdeadbeef`: the fixed handle that
/// `OpenCluster` and `ClusterOpenEnum` return.
pub const STUB_HANDLE: usize = 0xdead_beef;

/// `GetNodeClusterState` states, from the `PARAMS` comment in `clusapi.c`.
pub const CLUSTER_STATE_NOT_INSTALLED: u32 = 0x00;
pub const CLUSTER_STATE_NOT_CONFIGURED: u32 = 0x01;
pub const CLUSTER_STATE_NOT_RUNNING: u32 = 0x03;
pub const CLUSTER_STATE_RUNNING: u32 = 0x13;

/// `GetClusterInformation`: stores 0 in `*lpcchClusterName` and returns
/// `ERROR_SUCCESS`. The name buffer and `CLUSTERVERSIONINFO` are not touched.
pub fn get_cluster_information(lpcch_cluster_name: &mut u32) -> u32 {
    *lpcch_cluster_name = 0;
    ERROR_SUCCESS
}

/// `GetNodeClusterState`: always stores `CLUSTER_STATE_NOT_INSTALLED` in
/// `*pdwClusterState` and returns `ERROR_SUCCESS`. The node name is not read.
pub fn get_node_cluster_state(pdw_cluster_state: &mut u32) -> u32 {
    *pdw_cluster_state = CLUSTER_STATE_NOT_INSTALLED;
    ERROR_SUCCESS
}

/// `OpenCluster`: returns [`STUB_HANDLE`] whatever the name is.
pub fn open_cluster() -> usize {
    STUB_HANDLE
}

/// `CloseCluster`: returns `TRUE`.
pub fn close_cluster() -> bool {
    true
}

/// `ClusterOpenEnum`: returns [`STUB_HANDLE`] whatever the cluster and type are.
pub fn cluster_open_enum() -> usize {
    STUB_HANDLE
}

/// `ClusterCloseEnum`: returns `ERROR_SUCCESS`.
pub fn cluster_close_enum() -> u32 {
    ERROR_SUCCESS
}

/// `ClusterEnum`: returns `ERROR_NO_MORE_ITEMS` and writes nothing. The C body
/// reads `*lpcchName` in its `FIXME` trace, so the port takes it by reference.
pub fn cluster_enum(_lpcch_name: &u32) -> u32 {
    ERROR_NO_MORE_ITEMS
}
