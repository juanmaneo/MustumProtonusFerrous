//! Oracle-derived test vectors for the ported netprofm logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/netprofm/list.c` and `include/ws2ipdef.h` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/netprofm/` and
//! running the harness. The 25 printed rows include one row
//! (`flagged_1.200.1.1`, `map_address_6to4` with `Word[6] = 0xc801`) that
//! UBSan flags as a left shift of a signed int; it is excluded here, so 24 rows remain.

use super::*;

/// Builds the 16 address bytes from `IN6_ADDR.u.Word` values (little-endian host).
fn words(w: [u16; 8]) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    for (i, x) in w.iter().enumerate() {
        bytes[2 * i..2 * i + 2].copy_from_slice(&x.to_le_bytes());
    }
    bytes
}

/// Builds a `NetworkState` from the four cached flags the oracle harness sets.
fn state(cv4: bool, cv6: bool, iv4: bool, iv6: bool) -> NetworkState {
    NetworkState {
        v4: Reach {
            connected: cv4,
            internet: iv4,
        },
        v6: Reach {
            connected: cv6,
            internet: iv6,
        },
    }
}

#[test]
fn oracle_map_address_6to4_rows() {
    // ipv4mapped_1.2.3.4_port8080
    let w = [0, 0, 0, 0, 0, 0xffff, 0x0201, 0x0403];
    assert_eq!(
        map_address_6to4(AF_INET6, 8080, &words(w)),
        Some(SockAddrIn {
            family: 2,
            port: 8080,
            s_addr: 0x0201_0403
        })
    );
    // ipv4mapped_127.0.0.1
    let w = [0, 0, 0, 0, 0, 0xffff, 0x007f, 0x0100];
    assert_eq!(
        map_address_6to4(AF_INET6, 0, &words(w)),
        Some(SockAddrIn {
            family: 2,
            port: 0,
            s_addr: 0x007f_0100
        })
    );
    // wrong_family
    assert_eq!(map_address_6to4(AF_INET, 0, &words(w)), None);
    // word5_not_ffff
    let w = [0, 0, 0, 0, 0, 0xfffe, 0x007f, 0x0100];
    assert_eq!(map_address_6to4(AF_INET6, 0, &words(w)), None);
}

#[test]
fn oracle_has_ipv6_scope_rows() {
    let addr = |b: &[(usize, u8)]| {
        let mut a = [0u8; 16];
        for &(i, v) in b {
            a[i] = v;
        }
        SockAddr::Inet6(a)
    };
    // scope linklocal_fe80::1 local=1 global=0
    assert_eq!(
        has_ipv6_scope(&[addr(&[(0, 0xfe), (1, 0x80), (15, 1)])]),
        (true, false)
    );
    // scope sitelocal_fec0::1 local=1 global=0
    assert_eq!(
        has_ipv6_scope(&[addr(&[(0, 0xfe), (1, 0xc0), (15, 1)])]),
        (true, false)
    );
    // scope global_2001:db8::1 local=0 global=1
    assert_eq!(
        has_ipv6_scope(&[addr(&[(0, 0x20), (1, 0x01), (2, 0x0d), (3, 0xb8), (15, 1)])]),
        (false, true)
    );
    // scope loopback_::1 local=0 global=0
    assert_eq!(has_ipv6_scope(&[addr(&[(15, 1)])]), (false, false));
    // scope unspecified_:: local=0 global=0
    assert_eq!(has_ipv6_scope(&[addr(&[])]), (false, false));
    // scope multicast_ff02::1 local=0 global=0
    assert_eq!(
        has_ipv6_scope(&[addr(&[(0, 0xff), (1, 0x02), (15, 1)])]),
        (false, false)
    );
}

#[test]
fn oracle_derive_state_rows() {
    let v4 = SockAddr::Inet;
    let global = SockAddr::Inet6([0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    let linklocal = SockAddr::Inet6([0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

    // derive ipv4_unicast_with_gateway cv4=1 cv6=0 iv4=1 iv6=0
    assert_eq!(derive_state(&[v4], &[v4]), state(true, false, true, false));
    // derive ipv6_global_unicast_no_gateway cv4=0 cv6=1 iv4=0 iv6=1
    assert_eq!(
        derive_state(&[global], &[]),
        state(false, true, false, true)
    );
    // derive ipv6_linklocal_unicast_only cv4=0 cv6=1 iv4=0 iv6=0
    assert_eq!(
        derive_state(&[linklocal], &[]),
        state(false, true, false, false)
    );
    // derive ipv4_gateway_without_unicast cv4=0 cv6=0 iv4=1 iv6=0
    assert_eq!(derive_state(&[], &[v4]), state(false, false, true, false));
    // derive no_addresses cv4=0 cv6=0 iv4=0 iv6=0
    assert_eq!(derive_state(&[], &[]), state(false, false, false, false));
}

#[test]
fn oracle_connectivity_rows() {
    // netconn internet_v4_only bits=0x0040
    assert_eq!(state(false, false, true, false).connectivity(), 0x0040);
    // netconn local_v4_only bits=0x0020
    assert_eq!(state(true, false, false, false).connectivity(), 0x0020);
    // netconn local_v6_only_internet_v6 bits=0x0400
    assert_eq!(state(false, true, false, true).connectivity(), 0x0400);
    // netconn all_four bits=0x0440
    assert_eq!(state(true, true, true, true).connectivity(), 0x0440);
}

#[test]
fn oracle_list_rows() {
    // listconn local_v4_plus_internet_v6 bits=0x0420
    let nets = [
        state(true, false, false, false),
        state(false, false, false, true),
    ];
    assert_eq!(aggregate_connectivity(&nets), 0x0420);

    // listconnected none_plus_local_v6 value=1
    let nets = [
        state(false, false, false, false),
        state(false, true, false, false),
    ];
    assert!(any_connected(&nets));
}

#[test]
fn oracle_enum_rows() {
    // enum all_matches_disconnected match=1
    assert!(enum_network_matches(
        NLM_ENUM_NETWORK_ALL,
        &state(false, false, false, false)
    ));
    // enum connected_filter_disconnected match=0
    assert!(!enum_network_matches(
        NLM_ENUM_NETWORK_CONNECTED,
        &state(false, false, false, false)
    ));
}

#[test]
fn oracle_loopback_adapter_row() {
    // lo_adapter 1
    assert!(is_loopback_adapter_name(&[
        u16::from(b'l'),
        u16::from(b'o'),
        0
    ]));
}
