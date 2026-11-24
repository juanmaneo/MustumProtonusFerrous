//! Adapter checks from `dlls/netprofm/list.c`: the `has_*_address` helpers and
//! the flag derivation in `init_networks`. The caller flattens each
//! `IP_ADAPTER_*` linked list into a slice.

use crate::ipv6::{classify_ipv6, Ipv6Scope};
use crate::{NetworkState, Reach};

/// One address from an adapter's unicast or gateway list, reduced to the
/// fields the checks read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SockAddr {
    /// `sa_family == AF_INET`.
    Inet,
    /// `sa_family == AF_INET6`, with the 16 address bytes.
    Inet6([u8; 16]),
    /// Any other family.
    Other,
}

/// `has_ipv4_address` and `has_ipv4_gateway_address`: any `AF_INET` entry.
pub fn has_ipv4(addrs: &[SockAddr]) -> bool {
    addrs.contains(&SockAddr::Inet)
}

/// `has_ipv6_address`: returns `(has_local, has_global)` over the `AF_INET6` entries.
pub fn has_ipv6_scope(addrs: &[SockAddr]) -> (bool, bool) {
    let mut has_local = false;
    let mut has_global = false;
    for addr in addrs {
        if let SockAddr::Inet6(bytes) = addr {
            match classify_ipv6(bytes) {
                Ipv6Scope::Local => has_local = true,
                Ipv6Scope::Global => has_global = true,
                Ipv6Scope::Other => {}
            }
        }
    }
    (has_local, has_global)
}

/// The flag derivation in `init_networks` for one adapter. An IPv4 gateway
/// alone sets the v4 internet flag, and an IPv6 global unicast sets the v6
/// internet flag.
pub fn derive_state(unicast: &[SockAddr], gateway: &[SockAddr]) -> NetworkState {
    let (has_local, has_global) = has_ipv6_scope(unicast);
    NetworkState {
        v4: Reach {
            connected: has_ipv4(unicast),
            internet: has_ipv4(gateway),
        },
        v6: Reach {
            connected: has_local || has_global,
            internet: has_global,
        },
    }
}

/// `wcscmp(aa->FriendlyName, L"lo")` in `init_networks`. The name is NUL-terminated,
/// so only the part before the first NUL is compared.
pub fn is_loopback_adapter_name(name: &[u16]) -> bool {
    let end = name.iter().position(|&c| c == 0).unwrap_or(name.len());
    name[..end] == [u16::from(b'l'), u16::from(b'o')]
}
