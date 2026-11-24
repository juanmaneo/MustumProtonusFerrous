//! Edge-case tests for the ported netprofm logic.
//!
//! These cover boundary addresses, every branch of the classifiers, and flag
//! combinations. Not every expectation is oracle-derived; the ones that aren't
//! say so in a comment.

use super::*;

fn v6(bytes: &[(usize, u8)]) -> [u8; 16] {
    let mut a = [0u8; 16];
    for &(i, b) in bytes {
        a[i] = b;
    }
    a
}

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
fn scope_boundaries() {
    // fe80::/10 ends at febf; fec0 (site-local) starts at fec0.
    assert_eq!(
        classify_ipv6(&v6(&[(0, 0xfe), (1, 0xbf)])),
        Ipv6Scope::Local
    );
    assert_eq!(
        classify_ipv6(&v6(&[(0, 0xfe), (1, 0xc0)])),
        Ipv6Scope::Local
    );
    // fe40 is outside both link-local and site-local.
    assert_eq!(
        classify_ipv6(&v6(&[(0, 0xfe), (1, 0x40), (15, 1)])),
        Ipv6Scope::Global
    );
    // Another byte-0 value with a fe80-like second byte is global.
    assert_eq!(
        classify_ipv6(&v6(&[(0, 0xfd), (1, 0x80), (15, 1)])),
        Ipv6Scope::Global
    );
    // ::2 is not loopback.
    assert_eq!(classify_ipv6(&v6(&[(15, 2)])), Ipv6Scope::Global);
    // Byte 14 set with byte 15 == 1 is not loopback (word 7 is 0x0101, not 0x0100).
    assert_eq!(classify_ipv6(&v6(&[(14, 1), (15, 1)])), Ipv6Scope::Global);
    // Multicast with any group bits is still Other.
    assert_eq!(
        classify_ipv6(&v6(&[(0, 0xff), (15, 0x42)])),
        Ipv6Scope::Other
    );
    // Unspecified is Other; a single nonzero byte makes it global.
    assert_eq!(classify_ipv6(&v6(&[])), Ipv6Scope::Other);
    assert_eq!(classify_ipv6(&v6(&[(7, 1)])), Ipv6Scope::Global);
}

#[test]
fn has_ipv6_scope_ignores_non_inet6_and_ors_entries() {
    let link = SockAddr::Inet6(v6(&[(0, 0xfe), (1, 0x80), (15, 1)]));
    let glob = SockAddr::Inet6(v6(&[(0, 0x20), (1, 0x01), (15, 1)]));
    let lo = SockAddr::Inet6(v6(&[(15, 1)]));

    assert_eq!(has_ipv6_scope(&[]), (false, false));
    assert_eq!(
        has_ipv6_scope(&[SockAddr::Inet, SockAddr::Other]),
        (false, false)
    );
    assert_eq!(has_ipv6_scope(&[lo]), (false, false));
    assert_eq!(has_ipv6_scope(&[link, glob]), (true, true));
    assert_eq!(has_ipv6_scope(&[SockAddr::Other, link]), (true, false));
}

#[test]
fn has_ipv4_matches_only_inet() {
    assert!(!has_ipv4(&[]));
    assert!(!has_ipv4(&[SockAddr::Other, SockAddr::Inet6([0; 16])]));
    assert!(has_ipv4(&[SockAddr::Other, SockAddr::Inet]));
}

#[test]
fn derive_gateway_v6_is_ignored() {
    let glob = SockAddr::Inet6(v6(&[(0, 0x20), (1, 0x01), (15, 1)]));
    // Only IPv4 gateways count; an IPv6 gateway sets no flag.
    assert_eq!(
        derive_state(&[], &[glob]),
        state(false, false, false, false)
    );
}

#[test]
fn derive_local_v6_is_connected_not_internet() {
    let link = SockAddr::Inet6(v6(&[(0, 0xfe), (1, 0x80), (15, 1)]));
    assert_eq!(derive_state(&[link], &[]), state(false, true, false, false));
}

#[test]
fn internet_without_connected_still_reports_internet() {
    // A gateway with no unicast address: connectivity reports the internet bit alone.
    let st = derive_state(&[], &[SockAddr::Inet]);
    assert!(st.v4.internet && !st.v4.connected);
    assert!(any_connected_to_internet(&[st]));
    assert!(!any_connected(&[st]));
    assert_eq!(st.connectivity(), NLM_CONNECTIVITY_IPV4_INTERNET);
}

#[test]
fn connectivity_internet_beats_local() {
    // Per family, the internet bit replaces the local-network bit.
    assert_eq!(state(true, true, true, true).connectivity(), 0x0440);
    assert_eq!(
        state(true, false, true, false).connectivity(),
        NLM_CONNECTIVITY_IPV4_INTERNET
    );
    assert_eq!(
        state(false, true, false, true).connectivity(),
        NLM_CONNECTIVITY_IPV6_INTERNET
    );
    assert_eq!(
        state(false, false, false, false).connectivity(),
        NLM_CONNECTIVITY_DISCONNECTED
    );
    assert_eq!(
        state(false, true, false, false).connectivity(),
        NLM_CONNECTIVITY_IPV6_LOCALNETWORK
    );
}

#[test]
fn aggregate_connectivity_empty_and_or() {
    assert_eq!(aggregate_connectivity(&[]), NLM_CONNECTIVITY_DISCONNECTED);
    let nets = [
        state(true, false, false, false),
        state(false, true, false, false),
    ];
    assert_eq!(
        aggregate_connectivity(&nets),
        NLM_CONNECTIVITY_IPV4_LOCALNETWORK | NLM_CONNECTIVITY_IPV6_LOCALNETWORK
    );
}

#[test]
fn any_connected_empty_and_disconnected() {
    assert!(!any_connected(&[]));
    assert!(!any_connected_to_internet(&[]));
    let nets = [state(false, false, false, false)];
    assert!(!any_connected(&nets));
    assert!(!any_connected_to_internet(&nets));
}

#[test]
fn is_connected_flags_per_family() {
    assert!(state(true, false, false, false).is_connected());
    assert!(state(false, true, false, false).is_connected());
    assert!(!state(false, false, true, true).is_connected());
    assert!(state(false, false, true, false).is_connected_to_internet());
    assert!(state(false, false, false, true).is_connected_to_internet());
    assert!(!state(true, true, false, false).is_connected_to_internet());
}

#[test]
fn enum_filter_all_and_single_flags() {
    let connected = state(true, false, false, false);
    let disconnected = state(false, false, false, false);

    assert!(enum_network_matches(NLM_ENUM_NETWORK_ALL, &connected));
    assert!(enum_network_matches(NLM_ENUM_NETWORK_ALL, &disconnected));
    assert!(enum_network_matches(NLM_ENUM_NETWORK_CONNECTED, &connected));
    assert!(!enum_network_matches(
        NLM_ENUM_NETWORK_CONNECTED,
        &disconnected
    ));
    assert!(enum_network_matches(
        NLM_ENUM_NETWORK_DISCONNECTED,
        &disconnected
    ));
    assert!(!enum_network_matches(
        NLM_ENUM_NETWORK_DISCONNECTED,
        &connected
    ));
    // Flags 0 match nothing.
    assert!(!enum_network_matches(0, &connected));
    assert!(!enum_network_matches(0, &disconnected));
}

#[test]
fn map_address_6to4_rejects_nonzero_low_words() {
    // Word 0 nonzero, and word 4 nonzero, each fail the zero-prefix check.
    assert_eq!(
        map_address_6to4(AF_INET6, 0, &v6(&[(0, 1), (10, 0xff), (11, 0xff)])),
        None
    );
    assert_eq!(
        map_address_6to4(AF_INET6, 0, &v6(&[(8, 1), (10, 0xff), (11, 0xff)])),
        None
    );
    // Word 5 zero (no ffff) fails the mapping marker.
    assert_eq!(map_address_6to4(AF_INET6, 0, &[0u8; 16]), None);
}

#[test]
fn map_address_6to4_high_word_wraps() {
    // Word 6 >= 0x8000 makes `Word[6] << 16` overflow a signed int in C (UB,
    // flagged by UBSan, so not an oracle row). The port uses u32 and yields the
    // bits gcc produced on x86-64: 0xc801 << 16 | 0x0101.
    let bytes = v6(&[
        (10, 0xff),
        (11, 0xff),
        (12, 0x01),
        (13, 0xc8),
        (14, 0x01),
        (15, 0x01),
    ]);
    assert_eq!(
        map_address_6to4(AF_INET6, 0, &bytes),
        Some(SockAddrIn {
            family: AF_INET,
            port: 0,
            s_addr: 0xc801_0101,
        })
    );
}

#[test]
fn loopback_adapter_name_stops_at_nul() {
    let lo = [u16::from(b'l'), u16::from(b'o')];
    // Unterminated "lo" still matches.
    assert!(is_loopback_adapter_name(&lo));
    // A NUL ends the name, so trailing junk after it is ignored, as with wcscmp.
    assert!(is_loopback_adapter_name(&[
        u16::from(b'l'),
        u16::from(b'o'),
        0,
        u16::from(b'x')
    ]));
    assert!(!is_loopback_adapter_name(&[u16::from(b'l'), 0]));
    assert!(!is_loopback_adapter_name(&[
        u16::from(b'l'),
        u16::from(b'o'),
        u16::from(b'2'),
        0
    ]));
    assert!(!is_loopback_adapter_name(&[
        u16::from(b'e'),
        u16::from(b't'),
        u16::from(b'h'),
        0
    ]));
    assert!(!is_loopback_adapter_name(&[]));
}

#[test]
fn derived_clone_and_debug_impls() {
    // Calls the derived impls directly; assert_eq! only formats on failure.
    let st = state(true, false, true, false);
    assert_eq!(Clone::clone(&st), st);
    let addr = SockAddr::Inet6([1; 16]);
    assert_eq!(Clone::clone(&addr), addr);
    assert_eq!(Clone::clone(&Ipv6Scope::Local), Ipv6Scope::Local);

    assert!(format!("{:?}", st.v4).contains("connected: true"));
    assert!(format!("{st:?}").contains("NetworkState"));
    assert!(format!("{addr:?}").contains("Inet6"));
    assert!(format!("{:?}", Ipv6Scope::Global).contains("Global"));
    assert!(format!(
        "{:?}",
        SockAddrIn {
            family: AF_INET,
            port: 0,
            s_addr: 0
        }
    )
    .contains("s_addr: 0"));
}
