//! Oracle-derived test vectors for the ported clusapi logic.
//!
//! Rows were captured by compiling the verbatim Wine C body of
//! `dlls/clusapi/clusapi.c` and the verbatim `__wine_spec_unimplemented_stub`
//! body (`dlls/ntdll/exception.c`, lines 504 to 516) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/clusapi/`. The raw
//! output is in `target/canon/clusapi/rows.txt`. The harness keeps Wine's default
//! `FIXME` channel on, so argument expressions are evaluated. The sanitizers
//! flagged no rows, so none are excluded. The 17 rows are 14 export calls, two stub
//! calls, and one summary row for all 108 `stub` names.

use super::*;

#[test]
fn oracle_get_cluster_information_rows() {
    // GetClusterInformation hcluster=0x1 name=buf cch_in=7 info=buf -> ret=0 cch_out=0
    let mut cch = 7;
    assert_eq!(get_cluster_information(&mut cch), 0);
    assert_eq!(cch, 0);

    // GetClusterInformation hcluster=NULL name=NULL cch_in=0xffffffff info=NULL -> ret=0 cch_out=0
    let mut cch = 0xffff_ffff;
    assert_eq!(get_cluster_information(&mut cch), 0);
    assert_eq!(cch, 0);
}

#[test]
fn oracle_get_node_cluster_state_rows() {
    // GetNodeClusterState node=NODE1 state_in=0x13 -> ret=0 state_out=0x0
    let mut state = 0x13;
    assert_eq!(get_node_cluster_state(&mut state), 0);
    assert_eq!(state, 0);

    // GetNodeClusterState node=NULL state_in=0xffffffff -> ret=0 state_out=0x0
    let mut state = 0xffff_ffff;
    assert_eq!(get_node_cluster_state(&mut state), 0);
    assert_eq!(state, 0);
}

#[test]
fn oracle_open_cluster_rows() {
    // OpenCluster name=cluster -> hcluster=0xdeadbeef
    assert_eq!(open_cluster(), 0xdead_beef);
    // OpenCluster name=NULL -> hcluster=0xdeadbeef
    assert_eq!(open_cluster(), 0xdead_beef);
}

#[test]
fn oracle_close_cluster_rows() {
    // CloseCluster hcluster=NULL -> ret=1
    assert!(close_cluster());
    // CloseCluster hcluster=0xdeadbeef -> ret=1
    assert!(close_cluster());
}

#[test]
fn oracle_cluster_open_enum_rows() {
    // ClusterOpenEnum hcluster=0xdeadbeef type=0 -> henum=0xdeadbeef
    assert_eq!(cluster_open_enum(), 0xdead_beef);
    // ClusterOpenEnum hcluster=NULL type=0xffffffff -> henum=0xdeadbeef
    assert_eq!(cluster_open_enum(), 0xdead_beef);
}

#[test]
fn oracle_cluster_close_enum_rows() {
    // ClusterCloseEnum henum=0xdeadbeef -> ret=0
    assert_eq!(cluster_close_enum(), 0);
    // ClusterCloseEnum henum=NULL -> ret=0
    assert_eq!(cluster_close_enum(), 0);
}

#[test]
fn oracle_cluster_enum_rows() {
    // ClusterEnum henum=0xdeadbeef index=0 cch_in=260 -> ret=259 cch_out=260
    // (cch_out is unchanged; the port takes `&u32`, so it cannot be written.)
    assert_eq!(cluster_enum(&260), ERROR_NO_MORE_ITEMS);
    // ClusterEnum henum=NULL index=0xffffffff cch_in=0 -> ret=259 cch_out=0
    assert_eq!(cluster_enum(&0), ERROR_NO_MORE_ITEMS);
}

#[test]
fn oracle_stub_rows() {
    // stub CloseClusterGroup -> raised=1 code=0x80000100 flags=0x1 nparams=2 module=clusapi function=CloseClusterGroup
    assert!(is_spec_stub("CloseClusterGroup"));
    assert_eq!(
        unimplemented_stub_exception(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
        }
    );
    // stub SetClusterServiceAccountPassword -> raised=1 code=0x80000100 flags=0x1 nparams=2 module=clusapi function=SetClusterServiceAccountPassword
    assert!(is_spec_stub("SetClusterServiceAccountPassword"));
    assert_eq!(
        unimplemented_stub_exception(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
        }
    );
}

#[test]
fn oracle_stub_all_rows() {
    // stub_all count=108 mismatches=0 code=0x80000100 flags=0x1 nparams=2
    assert_eq!(SPEC_STUBS.len(), 108);
    for name in SPEC_STUBS {
        assert!(is_spec_stub(name), "{name}");
        assert_eq!(
            unimplemented_stub_exception(),
            StubException {
                code: 0x8000_0100,
                flags: 0x1,
                number_parameters: 2,
            }
        );
    }
}
