//! Boundary and branch tests for the ported inetmib1 logic.
//!
//! Only the 25 printed rows in `oracle_checks.rs` come from the C harness. Every expectation
//! here is worked out by hand from the C source, and each one is marked `non-oracle`.

use super::*;

const IP_ADDR: &[u32] = &[1, 3, 6, 1, 2, 1, 4, 20, 1];
const IP_TABLE: &[[u8; 4]] = &[
    [10, 0, 0, 1],
    [10, 0, 0, 1],
    [10, 0, 0, 5],
    [192, 168, 0, 1],
];
const UDP_TABLE: &[UdpRow] = &[
    UdpRow {
        local_addr: [10, 0, 0, 1],
        local_port: 53,
    },
    UdpRow {
        local_addr: [10, 0, 0, 1],
        local_port: 80,
    },
    UdpRow {
        local_addr: [10, 0, 0, 5],
        local_port: 53,
    },
];

fn with_suffix(base: &[u32], suffix: &[u32]) -> Vec<u32> {
    base.iter().chain(suffix).copied().collect()
}

fn ip_item(oid: &[u32], pdu: u8) -> Option<(u32, u32)> {
    get_item_and_instance_from_table(
        oid,
        IP_ADDR,
        4,
        pdu,
        IP_TABLE,
        oid_to_ip_addr,
        compare_ip_addr_row,
    )
}

#[test]
fn edge_oid_n_cmp_arc_order() {
    // non-oracle: the first differing arc decides, in both directions
    assert_eq!(oid_n_cmp(&[1, 3, 5], &[1, 3, 6], 3), -1);
    assert_eq!(oid_n_cmp(&[1, 3, 6], &[1, 3, 5], 3), 1);
    // non-oracle: arcs past `count` are not compared
    assert_eq!(oid_n_cmp(&[1, 3, 9], &[1, 3, 6], 2), 0);
}

#[test]
fn edge_oid_n_cmp_count_bounds() {
    // non-oracle: a count of zero always matches
    assert_eq!(oid_n_cmp(&[1], &[2], 0), 0);
    // non-oracle: both OIDs are shorter than count and equal, so the result is 0
    assert_eq!(oid_n_cmp(&[1, 3], &[1, 3], 5), 0);
    // non-oracle: when the common prefix matches but count is not reached, the shorter OID sorts first
    assert_eq!(oid_n_cmp(&[1, 3], &[1, 3, 6], 5), -1);
    assert_eq!(oid_n_cmp(&[1, 3, 6], &[1, 3], 5), 1);
}

#[test]
fn edge_oid_to_ip_addr_truncates_arcs() {
    // non-oracle: each arc is truncated to a byte, like the (BYTE) casts; extra arcs are ignored
    assert_eq!(
        oid_to_ip_addr(&[0x1c0, 0x1a8, 0x101, 0x10a, 99]),
        [0xc0, 0xa8, 0x01, 0x0a]
    );
}

#[test]
#[should_panic(expected = "at least 4 arcs")]
fn edge_oid_to_ip_addr_short_oid_panics() {
    // non-oracle: the C code asserts idLength >= 4
    oid_to_ip_addr(&[1, 2, 3]);
}

#[test]
fn edge_set_oid_builders() {
    // non-oracle
    assert_eq!(set_oid_with_item(&[1, 3], 7), vec![1, 3, 7]);
    assert_eq!(
        set_oid_with_item_and_ip_addr(&[1, 3], 2, [255, 255, 255, 0]),
        vec![1, 3, 2, 255, 255, 255, 0]
    );
    assert_eq!(
        set_oid_with_item_and_integer(&[1, 3], 2, 9),
        vec![1, 3, 2, 9]
    );
}

#[test]
fn edge_get_item_from_oid_getnext() {
    let base: &[u32] = &[1, 3, 6];
    // non-oracle: an OID before the base gives item 1
    assert_eq!(get_item_from_oid(&[1, 3], base, SNMP_PDU_GETNEXT), Some(1));
    // non-oracle: the base itself gives item 1
    assert_eq!(get_item_from_oid(base, base, SNMP_PDU_GETNEXT), Some(1));
    // non-oracle: a plain item successor
    assert_eq!(
        get_item_from_oid(&[1, 3, 6, 8], base, SNMP_PDU_GETNEXT),
        Some(9)
    );
    // non-oracle: an OID after the base has no successor
    assert_eq!(
        get_item_from_oid(&[1, 3, 7, 1], base, SNMP_PDU_GETNEXT),
        None
    );
}

#[test]
fn edge_get_item_from_oid_get_and_set() {
    let base: &[u32] = &[1, 3, 6];
    // non-oracle: an OID outside the base, or shorter than it
    assert_eq!(get_item_from_oid(&[1, 3, 7, 1], base, SNMP_PDU_GET), None);
    assert_eq!(get_item_from_oid(&[1, 3], base, SNMP_PDU_GET), None);
    // non-oracle: item 0 is not a valid GET
    assert_eq!(get_item_from_oid(&[1, 3, 6, 0], base, SNMP_PDU_GET), None);
    // non-oracle: GET and SET share the non-GETNEXT path
    assert_eq!(
        get_item_from_oid(&[1, 3, 6, 3], base, SNMP_PDU_GET),
        Some(3)
    );
    assert_eq!(
        get_item_from_oid(&[1, 3, 6, 3], base, SNMP_PDU_SET),
        Some(3)
    );
}

#[test]
fn edge_get_item_and_integer_instance_getnext() {
    let base: &[u32] = &[1, 3, 6];
    // non-oracle: an OID before the base gives (1, 1)
    assert_eq!(
        get_item_and_integer_instance_from_oid(&[1, 3], base, SNMP_PDU_GETNEXT),
        Some((1, 1))
    );
    // non-oracle: the base itself has no item or instance, so (1, 1)
    assert_eq!(
        get_item_and_integer_instance_from_oid(base, base, SNMP_PDU_GETNEXT),
        Some((1, 1))
    );
    // non-oracle: an item with no instance gives instance 1
    assert_eq!(
        get_item_and_integer_instance_from_oid(&[1, 3, 6, 4], base, SNMP_PDU_GETNEXT),
        Some((4, 1))
    );
    // non-oracle: the instance wraps at u32::MAX
    assert_eq!(
        get_item_and_integer_instance_from_oid(&[1, 3, 6, 4, u32::MAX], base, SNMP_PDU_GETNEXT),
        Some((4, 0))
    );
    // non-oracle: an OID outside the base has no successor
    assert_eq!(
        get_item_and_integer_instance_from_oid(&[1, 3, 7, 4, 5], base, SNMP_PDU_GETNEXT),
        None
    );
}

#[test]
fn edge_get_item_and_integer_instance_get_and_set() {
    let base: &[u32] = &[1, 3, 6];
    // non-oracle: GET needs an item and an instance, so the base and the item alone fail
    assert_eq!(
        get_item_and_integer_instance_from_oid(base, base, SNMP_PDU_GET),
        None
    );
    assert_eq!(
        get_item_and_integer_instance_from_oid(&[1, 3, 6, 4], base, SNMP_PDU_GET),
        None
    );
    // non-oracle: GET has no zero check on item or instance, unlike getItemFromOid
    assert_eq!(
        get_item_and_integer_instance_from_oid(&[1, 3, 6, 0, 0], base, SNMP_PDU_GET),
        Some((0, 0))
    );
    // non-oracle: SET takes the GET path
    assert_eq!(
        get_item_and_integer_instance_from_oid(&[1, 3, 6, 4, 5], base, SNMP_PDU_SET),
        Some((4, 5))
    );
    // non-oracle: an OID outside the base
    assert_eq!(
        get_item_and_integer_instance_from_oid(&[1, 3, 7, 4, 5], base, SNMP_PDU_GET),
        None
    );
}

#[test]
fn edge_find_value_in_table_probes() {
    // non-oracle: an empty table never matches
    let empty: &[[u8; 4]] = &[];
    assert_eq!(
        find_value_in_table(&[10, 0, 0, 1], empty, compare_ip_addr_row),
        0
    );
    // non-oracle: below and above every entry, so the probe walks off one end
    assert_eq!(
        find_value_in_table(&[9, 0, 0, 0], IP_TABLE, compare_ip_addr_row),
        0
    );
    assert_eq!(
        find_value_in_table(&[200, 0, 0, 0], IP_TABLE, compare_ip_addr_row),
        0
    );
    // non-oracle: the probe reaches a middle entry, and the last entry
    assert_eq!(
        find_value_in_table(&[10, 0, 0, 5], IP_TABLE, compare_ip_addr_row),
        3
    );
    assert_eq!(
        find_value_in_table(&[192, 168, 0, 1], IP_TABLE, compare_ip_addr_row),
        4
    );
    // non-oracle: a duplicate resolves to the entry the bsearch probe hits first
    assert_eq!(
        find_value_in_table(&[10, 0, 0, 1], IP_TABLE, compare_ip_addr_row),
        2
    );
}

#[test]
fn edge_find_oid_in_table_lookup() {
    // non-oracle: the last row is found through the key conversion
    assert_eq!(
        find_oid_in_table(
            &[192, 168, 0, 1],
            IP_TABLE,
            oid_to_ip_addr,
            compare_ip_addr_row
        ),
        4
    );
    // non-oracle: an address that is not in the table
    assert_eq!(
        find_oid_in_table(
            &[10, 0, 0, 9],
            IP_TABLE,
            oid_to_ip_addr,
            compare_ip_addr_row
        ),
        0
    );
}

#[test]
fn edge_find_next_oid_in_table_successors() {
    // non-oracle: a key that is not found and sorts after the first entry has no successor
    assert_eq!(
        find_next_oid_in_table(
            &[10, 0, 0, 3],
            IP_TABLE,
            oid_to_ip_addr,
            compare_ip_addr_row
        ),
        0
    );
    assert_eq!(
        find_next_oid_in_table(
            &[192, 168, 0, 2],
            IP_TABLE,
            oid_to_ip_addr,
            compare_ip_addr_row
        ),
        0
    );
    // non-oracle: the first of two duplicates gives the entry after the match
    assert_eq!(
        find_next_oid_in_table(
            &[10, 0, 0, 1],
            IP_TABLE,
            oid_to_ip_addr,
            compare_ip_addr_row
        ),
        3
    );
    // non-oracle: the skip loop passes every duplicate of the match
    let run: &[[u8; 4]] = &[
        [10, 0, 0, 1],
        [10, 0, 0, 1],
        [10, 0, 0, 1],
        [10, 0, 0, 1],
        [10, 0, 0, 2],
    ];
    assert_eq!(
        find_next_oid_in_table(&[10, 0, 0, 1], run, oid_to_ip_addr, compare_ip_addr_row),
        5
    );
    // non-oracle: an empty table has no successor (the C code reads past numEntries here)
    let empty: &[[u8; 4]] = &[];
    assert_eq!(
        find_next_oid_in_table(&[10, 0, 0, 1], empty, oid_to_ip_addr, compare_ip_addr_row),
        0
    );
}

#[test]
fn edge_get_item_and_instance_empty_table() {
    // non-oracle: an empty table is NOSUCHNAME for GET and GETNEXT alike
    let empty: &[[u8; 4]] = &[];
    let oid = with_suffix(IP_ADDR, &[1, 10, 0, 0, 1]);
    for pdu in [SNMP_PDU_GET, SNMP_PDU_GETNEXT] {
        assert_eq!(
            get_item_and_instance_from_table(
                &oid,
                IP_ADDR,
                4,
                pdu,
                empty,
                oid_to_ip_addr,
                compare_ip_addr_row,
            ),
            None
        );
    }
}

#[test]
fn edge_get_item_and_instance_getnext_item_branch() {
    // non-oracle: before the base, or equal to it, gives (1, 1)
    assert_eq!(ip_item(&[1, 3, 6], SNMP_PDU_GETNEXT), Some((1, 1)));
    assert_eq!(ip_item(IP_ADDR, SNMP_PDU_GETNEXT), Some((1, 1)));
    // non-oracle: item 0 is replaced by 1 in the table-or-item case
    assert_eq!(
        ip_item(&with_suffix(IP_ADDR, &[0]), SNMP_PDU_GETNEXT),
        Some((1, 1))
    );
    // non-oracle: an OID past the base that is not the full row length is out of range
    assert_eq!(
        ip_item(
            &with_suffix(IP_ADDR, &[1, 10, 0, 0, 1, 7]),
            SNMP_PDU_GETNEXT
        ),
        None
    );
    // non-oracle: an OID that departs from the base has no successor
    assert_eq!(
        ip_item(
            &[1, 3, 6, 1, 2, 1, 4, 21, 1, 1, 10, 0, 0, 1],
            SNMP_PDU_GETNEXT
        ),
        None
    );
}

#[test]
fn edge_get_item_and_instance_getnext_instance_branch() {
    // non-oracle: item 0 gives (1, 1) without searching the table
    assert_eq!(
        ip_item(&with_suffix(IP_ADDR, &[0, 10, 0, 0, 1]), SNMP_PDU_GETNEXT),
        Some((1, 1))
    );
    // non-oracle: an instance below the first row gives instance 1
    assert_eq!(
        ip_item(&with_suffix(IP_ADDR, &[1, 10, 0, 0, 0]), SNMP_PDU_GETNEXT),
        Some((1, 1))
    );
    // non-oracle: an instance that is not in the table and sorts after the first row
    assert_eq!(
        ip_item(&with_suffix(IP_ADDR, &[1, 10, 0, 0, 3]), SNMP_PDU_GETNEXT),
        None
    );
    // non-oracle: the successor of the last row is past the end of the table
    assert_eq!(
        ip_item(
            &with_suffix(IP_ADDR, &[1, 192, 168, 0, 1]),
            SNMP_PDU_GETNEXT
        ),
        None
    );
}

#[test]
fn edge_get_item_and_instance_get_branches() {
    // non-oracle: a missing item or instance, or a different base, is NOSUCHNAME
    assert_eq!(ip_item(&with_suffix(IP_ADDR, &[1]), SNMP_PDU_GET), None);
    assert_eq!(
        ip_item(&with_suffix(IP_ADDR, &[0, 10, 0, 0, 1]), SNMP_PDU_GET),
        None
    );
    assert_eq!(
        ip_item(&with_suffix(IP_ADDR, &[1, 10, 0, 0, 3]), SNMP_PDU_GET),
        None
    );
    assert_eq!(
        ip_item(&[1, 3, 6, 1, 2, 1, 4, 21, 1, 1, 10, 0, 0, 1], SNMP_PDU_GET),
        None
    );
    // non-oracle: the duplicate 10.0.0.1 row is found at index 2
    assert_eq!(
        ip_item(&with_suffix(IP_ADDR, &[1, 10, 0, 0, 1]), SNMP_PDU_SET),
        Some((1, 2))
    );
}

#[test]
fn edge_dword_cmp_orders() {
    // non-oracle
    assert_eq!(dword_cmp(1, 2), -1);
    assert_eq!(dword_cmp(2, 1), 1);
    assert_eq!(dword_cmp(7, 7), 0);
}

#[test]
fn edge_compare_ip_addr_row_is_big_endian() {
    // non-oracle: the octets compare as a big-endian number, so the first octet dominates
    assert_eq!(compare_ip_addr_row(&[1, 0, 0, 0], &[0, 255, 255, 255]), 1);
    assert_eq!(compare_ip_addr_row(&[9, 255, 255, 255], &[10, 0, 0, 0]), -1);
    assert_eq!(compare_ip_addr_row(&[10, 0, 0, 1], &[10, 0, 0, 1]), 0);
}

#[test]
fn edge_compare_udp_row_is_bug_compatible() {
    let base = UdpRow {
        local_addr: [10, 0, 0, 1],
        local_port: 53,
    };
    // non-oracle: equal rows compare as 0
    assert_eq!(compare_udp_row(&base, &base), 0);
    // non-oracle: a higher address gives 1, not a negative value
    let higher = UdpRow {
        local_addr: [10, 0, 0, 2],
        local_port: 53,
    };
    assert_eq!(compare_udp_row(&higher, &base), 1);
    // non-oracle: a higher port also gives 1, not a positive difference
    let port_80 = UdpRow {
        local_addr: [10, 0, 0, 1],
        local_port: 80,
    };
    assert_eq!(compare_udp_row(&port_80, &base), 1);
    // non-oracle: the bug also breaks the lookup. The first UDP row is a bsearch miss,
    // because every unequal probe returns 1 and the search moves right.
    assert_eq!(find_value_in_table(&base, UDP_TABLE, compare_udp_row), 0);
    let second = UDP_TABLE[1];
    assert_eq!(find_value_in_table(&second, UDP_TABLE, compare_udp_row), 2);
}

#[test]
fn edge_oid_to_udp_row_fields() {
    // non-oracle: the port is arc 5 truncated to 16 bits, and extra arcs are ignored
    assert_eq!(
        oid_to_udp_row(&[10, 0, 0, 1, 0x1_0035, 9]),
        UdpRow {
            local_addr: [10, 0, 0, 1],
            local_port: 53,
        }
    );
}

#[test]
#[should_panic(expected = "at least 5 arcs")]
fn edge_oid_to_udp_row_short_oid_panics() {
    // non-oracle: the C code asserts idLength >= 5
    oid_to_udp_row(&[10, 0, 0, 1]);
}

#[test]
fn edge_oper_status_other_values_are_down() {
    // non-oracle: the states other than OPERATIONAL, CONNECTING and CONNECTED map to DOWN
    for oper in [
        MIB_IF_OPER_STATUS_NON_OPERATIONAL,
        MIB_IF_OPER_STATUS_UNREACHABLE,
        MIB_IF_OPER_STATUS_DISCONNECTED,
        u32::MAX,
    ] {
        assert_eq!(oper_status_to_admin_status(oper), MIB_IF_ADMIN_STATUS_DOWN);
    }
    // non-oracle: CONNECTING maps to TESTING, as CONNECTED does (row 22)
    assert_eq!(
        oper_status_to_admin_status(MIB_IF_OPER_STATUS_CONNECTING),
        MIB_IF_ADMIN_STATUS_TESTING
    );
}

#[test]
fn edge_find_supported_query_probes() {
    // non-oracle: an empty query is NOSUCHNAME
    assert_eq!(find_supported_query(&[], SUPPORTED_IDS), None);
    // non-oracle: the first probe (IpRoute) is too high, so the search moves left (cmp < 0)
    assert_eq!(
        find_supported_query(&[1, 3, 6, 1, 2, 1, 2, 2, 1], SUPPORTED_IDS),
        Some(1)
    );
    // non-oracle: the first probe is too low, so the search moves right (cmp > 0)
    assert_eq!(
        find_supported_query(&[1, 3, 6, 1, 2, 1, 7], SUPPORTED_IDS),
        Some(8)
    );
    // non-oracle: no name starts with this query, so the search runs out of range
    assert_eq!(
        find_supported_query(&[1, 3, 6, 1, 2, 1, 3], SUPPORTED_IDS),
        None
    );
    // non-oracle: an empty table
    assert_eq!(find_supported_query(&[1, 3, 6, 1, 2, 1, 4], &[]), None);
}

#[test]
fn edge_find_longest_supported_query_bounds() {
    // non-oracle: the exact 9-arc name is found at its own length
    assert_eq!(
        find_longest_supported_query(&[1, 3, 6, 1, 2, 1, 7, 5, 1], SUPPORTED_IDS),
        Some(9)
    );
    // non-oracle: a 10-arc query with no 10-arc match falls back to the 9-arc prefix
    assert_eq!(
        find_longest_supported_query(&[1, 3, 6, 1, 2, 1, 7, 5, 1, 4], SUPPORTED_IDS),
        Some(9)
    );
    // non-oracle: the shortest name is 7 arcs, so a 6-arc OID has no candidate length
    assert_eq!(
        find_longest_supported_query(&[1, 3, 6, 1, 2, 1], SUPPORTED_IDS),
        None
    );
    // non-oracle: no name matches at any length
    assert_eq!(
        find_longest_supported_query(&[1, 3, 6, 1, 2, 1, 9, 9, 9, 9], SUPPORTED_IDS),
        None
    );
    // non-oracle: an empty table has no minimum name length, so nothing matches
    assert_eq!(
        find_longest_supported_query(&[1, 3, 6, 1, 2, 1, 4], &[]),
        None
    );
}

#[test]
fn edge_in_mib2_prefix_test() {
    // non-oracle: the prefix itself and anything below it are in MIB-2
    assert!(in_mib2(MIB2));
    assert!(in_mib2(&[1, 3, 6, 1, 2, 1, 4]));
    // non-oracle: shorter than the prefix, or different on either side of it
    assert!(!in_mib2(&[1, 3, 6, 1, 2]));
    assert!(!in_mib2(&[1, 3, 6, 1, 1, 9]));
    assert!(!in_mib2(&[1, 3, 6, 1, 3, 1]));
}
