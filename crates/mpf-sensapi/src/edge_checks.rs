//! Edge-case tests for the ported sensapi logic.
//!
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/sensapi.h.
    assert_eq!(NETWORK_ALIVE_LAN, 1);
    assert_eq!(NETWORK_ALIVE_WAN, 2);
    assert_eq!(NETWORK_ALIVE_AOL, 4);
}

#[test]
fn network_alive_assigns_rather_than_ors() {
    // Not oracle-derived: the C body assigns NETWORK_ALIVE_LAN, so a value that already holds the
    // WAN and AOL bits ends up as exactly LAN. Oracle rows 15 to 17 cover the same write.
    let mut flags = NETWORK_ALIVE_WAN | NETWORK_ALIVE_AOL;
    assert!(is_network_alive(Some(&mut flags)));
    assert_eq!(flags, NETWORK_ALIVE_LAN);
}

#[test]
fn export_table_is_in_spec_order() {
    // Not oracle-derived: names and order from dlls/sensapi/sensapi.spec.
    assert_eq!(
        EXPORTS,
        [
            "IsDestinationReachableA",
            "IsDestinationReachableW",
            "IsNetworkAlive",
        ]
    );
}

#[test]
fn is_export_matches_only_exact_spec_names() {
    // Not oracle-derived: name lookup is case-sensitive and exact. The macro name
    // IsDestinationReachable is defined in include/sensapi.h, not exported by the spec.
    for name in EXPORTS {
        assert!(is_export(name), "{name}");
    }
    for name in [
        "",
        "IsDestinationReachable",
        "isnetworkalive",
        "IsNetworkAlive ",
        "IsNetworkAliveW",
    ] {
        assert!(!is_export(name), "{name:?}");
    }
}
