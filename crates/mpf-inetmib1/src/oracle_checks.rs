//! Oracle-derived test vectors for the ported inetmib1 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/inetmib1/main.c`, `dlls/snmpapi/main.c` and `include/ipifcons.h` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/inetmib1/`, then running the
//! harness. The harness printed 25 rows, and ASan and UBSan reported nothing, so no vector
//! is excluded. The `row NN` comments match the harness output.

use super::*;

const IP_ADDR: &[u32] = &[1, 3, 6, 1, 2, 1, 4, 20, 1];
const IF_ENTRY: &[u32] = &[1, 3, 6, 1, 2, 1, 2, 2, 1];
const IP_TABLE: &[[u8; 4]] = &[
    [10, 0, 0, 1],
    [10, 0, 0, 1],
    [10, 0, 0, 5],
    [192, 168, 0, 1],
];

fn with_suffix(base: &[u32], suffix: &[u32]) -> Vec<u32> {
    base.iter().chain(suffix).copied().collect()
}

#[test]
fn oracle_oid_n_cmp_rows() {
    // row 01
    assert_eq!(oid_n_cmp(&[1, 3, 6, 1], &[1, 3, 6, 1, 2], 4), 0);
    // row 02
    assert_eq!(oid_n_cmp(&[1, 3, 6, 1], &[1, 3, 6, 1, 2], 5), -1);
    // row 03
    assert_eq!(oid_n_cmp(&[1, 3, 7], &[1, 3, 6, 9], 3), 1);
}

#[test]
fn oracle_oid_to_ip_addr_row() {
    // row 04: the dword is 0x0a01a8c0 as printed by the harness
    let addr = oid_to_ip_addr(&[192, 168, 1, 10]);
    assert_eq!(addr, [192, 168, 1, 10]);
    assert_eq!(u32::from_le_bytes(addr), 0x0a01_a8c0);
}

#[test]
fn oracle_set_oid_rows() {
    // row 05
    assert_eq!(set_oid_with_item(IP_ADDR, 3), with_suffix(IP_ADDR, &[3]));
    // row 06
    assert_eq!(
        set_oid_with_item_and_ip_addr(IP_ADDR, 1, [10, 0, 0, 1]),
        with_suffix(IP_ADDR, &[1, 10, 0, 0, 1])
    );
}

#[test]
fn oracle_get_item_from_oid_rows() {
    // row 07
    assert_eq!(
        get_item_from_oid(&with_suffix(IF_ENTRY, &[5]), IF_ENTRY, SNMP_PDU_GETNEXT),
        Some(6)
    );
    // row 08: the item wraps at u32::MAX
    assert_eq!(
        get_item_from_oid(
            &with_suffix(IF_ENTRY, &[u32::MAX]),
            IF_ENTRY,
            SNMP_PDU_GETNEXT
        ),
        Some(0)
    );
    // row 09
    assert_eq!(
        get_item_from_oid(&with_suffix(IF_ENTRY, &[3]), IF_ENTRY, SNMP_PDU_GET),
        Some(3)
    );
    // row 10
    assert_eq!(get_item_from_oid(IF_ENTRY, IF_ENTRY, SNMP_PDU_GET), None);
}

#[test]
fn oracle_get_item_and_integer_instance_row() {
    // row 11
    assert_eq!(
        get_item_and_integer_instance_from_oid(
            &with_suffix(IF_ENTRY, &[2, 5]),
            IF_ENTRY,
            SNMP_PDU_GETNEXT
        ),
        Some((2, 6))
    );
}

#[test]
fn oracle_table_lookup_rows() {
    let key_of = oid_to_ip_addr;
    let compare = compare_ip_addr_row;
    // row 12
    assert_eq!(
        find_oid_in_table(&[10, 0, 0, 1], IP_TABLE, key_of, compare),
        2
    );
    // row 13
    assert_eq!(
        find_oid_in_table(&[10, 0, 0, 3], IP_TABLE, key_of, compare),
        0
    );
    // row 14
    assert_eq!(
        find_next_oid_in_table(&[10, 0, 0, 0], IP_TABLE, key_of, compare),
        1
    );
    // row 15
    assert_eq!(
        find_next_oid_in_table(&[192, 168, 0, 1], IP_TABLE, key_of, compare),
        5
    );
}

#[test]
fn oracle_get_item_and_instance_from_table_rows() {
    let key_of = oid_to_ip_addr;
    let compare = compare_ip_addr_row;
    let get = |oid: &[u32], pdu: u8| {
        get_item_and_instance_from_table(oid, IP_ADDR, 4, pdu, IP_TABLE, key_of, compare)
    };
    // row 16
    assert_eq!(
        get(&with_suffix(IP_ADDR, &[1, 10, 0, 0, 1]), SNMP_PDU_GET),
        Some((1, 2))
    );
    // row 17
    assert_eq!(
        get(&with_suffix(IP_ADDR, &[2]), SNMP_PDU_GETNEXT),
        Some((2, 1))
    );
    // row 18
    assert_eq!(
        get(&with_suffix(IP_ADDR, &[1, 10, 0, 0, 1]), SNMP_PDU_GETNEXT),
        Some((1, 3))
    );
}

#[test]
fn oracle_compare_udp_row_rows() {
    let a = UdpRow {
        local_addr: [10, 0, 0, 1],
        local_port: 80,
    };
    let b = UdpRow {
        local_addr: [10, 0, 0, 2],
        local_port: 1,
    };
    // row 19: the addresses differ, so the result is 1 and not the signed difference
    assert_eq!(compare_udp_row(&a, &b), 1);
    let c = UdpRow {
        local_addr: [10, 0, 0, 1],
        local_port: 53,
    };
    // row 20: the addresses match and the ports differ, so the result is 1 again
    assert_eq!(compare_udp_row(&c, &a), 1);
}

#[test]
fn oracle_oper_status_rows() {
    // row 21
    assert_eq!(oper_status_to_admin_status(5), 1);
    // row 22
    assert_eq!(oper_status_to_admin_status(4), 3);
    // row 23
    assert_eq!(oper_status_to_admin_status(0), 2);
}

#[test]
fn oracle_supported_query_rows() {
    // row 24
    assert_eq!(
        find_supported_query(&[1, 3, 6, 1, 2, 1, 4], SUPPORTED_IDS),
        Some(4)
    );
    // row 25: the longest match is IpAddr, index 3
    assert_eq!(
        find_longest_supported_query(&[1, 3, 6, 1, 2, 1, 4, 20, 1, 7], SUPPORTED_IDS),
        Some(3)
    );
}
