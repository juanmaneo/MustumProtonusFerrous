//! Port of the portable logic in Wine's `dlls/netprofm/list.c`.
//!
//! Ported:
//! - Connectivity bits (`network_GetConnectivity`, `connection_GetConnectivity`)
//!   and their OR-aggregate (`list_manager_GetConnectivity`):
//!   [`NetworkState::connectivity`], [`aggregate_connectivity`]
//! - `IsConnected` / `IsConnectedToInternet` (`network_get_*`, `list_manager_*`):
//!   [`NetworkState::is_connected`], [`NetworkState::is_connected_to_internet`],
//!   [`any_connected`], [`any_connected_to_internet`]
//! - `match_enum_network_flags`: [`enum_network_matches`]
//! - `has_ipv4_address`, `has_ipv4_gateway_address`, `has_ipv6_address`,
//!   and the flag derivation in `init_networks`: [`has_ipv4`], [`has_ipv6_scope`], [`derive_state`]
//! - `IN6_IS_ADDR_*` and the scope test: [`classify_ipv6`]
//! - `map_address_6to4`: [`map_address_6to4`]
//! - Loopback adapter filter (`wcscmp(FriendlyName, L"lo")`): [`is_loopback_adapter_name`]
//!
//! Skipped:
//! - `GetAdaptersAddresses`, `GetBestInterface`, `ConvertInterface*`: OS networking.
//! - `networks_enum_Next/Skip/Reset`, `connections_enum_*`: walk Wine `struct list`
//!   nodes and AddRef COM objects.
//! - `connection_point_*` sinks: COM AddRef/Release and allocation.
//! - `cost_manager_GetDataPlanStatus`: calls `GetBestInterface`.
//! - All COM vtable entries, `list_manager_create`, and the `FIXME` stubs returning `E_NOTIMPL`.

mod adapter;
mod ipv6;

pub use adapter::{derive_state, has_ipv4, has_ipv6_scope, is_loopback_adapter_name, SockAddr};
pub use ipv6::{classify_ipv6, map_address_6to4, Ipv6Scope, SockAddrIn};

/// `NLM_CONNECTIVITY` bits from `netlistmgr.idl`.
pub const NLM_CONNECTIVITY_DISCONNECTED: u32 = 0x0000;
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: u32 = 0x0020;
pub const NLM_CONNECTIVITY_IPV4_INTERNET: u32 = 0x0040;
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: u32 = 0x0200;
pub const NLM_CONNECTIVITY_IPV6_INTERNET: u32 = 0x0400;

/// `NLM_ENUM_NETWORK` flags from `netlistmgr.idl`.
pub const NLM_ENUM_NETWORK_CONNECTED: u32 = 0x01;
pub const NLM_ENUM_NETWORK_DISCONNECTED: u32 = 0x02;
pub const NLM_ENUM_NETWORK_ALL: u32 = 0x03;

pub const AF_INET: u16 = 2;
pub const AF_INET6: u16 = 23;

/// Reachability of one address family, as cached in `struct network`
/// (`connected_v4` / `connected_to_internet_v4`, and the v6 pair).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reach {
    pub connected: bool,
    pub internet: bool,
}

/// The four cached flags of one network. `struct connection` holds the same
/// flags, so a single value serves both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkState {
    pub v4: Reach,
    pub v6: Reach,
}

impl NetworkState {
    /// `network_GetConnectivity`. Per family, internet takes precedence over local network.
    pub fn connectivity(&self) -> u32 {
        let mut bits = NLM_CONNECTIVITY_DISCONNECTED;
        if self.v4.internet {
            bits |= NLM_CONNECTIVITY_IPV4_INTERNET;
        } else if self.v4.connected {
            bits |= NLM_CONNECTIVITY_IPV4_LOCALNETWORK;
        }
        if self.v6.internet {
            bits |= NLM_CONNECTIVITY_IPV6_INTERNET;
        } else if self.v6.connected {
            bits |= NLM_CONNECTIVITY_IPV6_LOCALNETWORK;
        }
        bits
    }

    /// `network_get_IsConnected`.
    pub fn is_connected(&self) -> bool {
        self.v4.connected || self.v6.connected
    }

    /// `network_get_IsConnectedToInternet`.
    pub fn is_connected_to_internet(&self) -> bool {
        self.v4.internet || self.v6.internet
    }
}

/// `list_manager_GetConnectivity`: OR of every network's connectivity bits.
pub fn aggregate_connectivity(networks: &[NetworkState]) -> u32 {
    networks
        .iter()
        .fold(NLM_CONNECTIVITY_DISCONNECTED, |bits, n| {
            bits | n.connectivity()
        })
}

/// `list_manager_IsConnected`.
pub fn any_connected(networks: &[NetworkState]) -> bool {
    networks.iter().any(NetworkState::is_connected)
}

/// `list_manager_IsConnectedToInternet`.
pub fn any_connected_to_internet(networks: &[NetworkState]) -> bool {
    networks.iter().any(NetworkState::is_connected_to_internet)
}

/// `match_enum_network_flags`: whether `network` passes the `NLM_ENUM_NETWORK` filter.
pub fn enum_network_matches(flags: u32, network: &NetworkState) -> bool {
    if flags == NLM_ENUM_NETWORK_ALL {
        return true;
    }
    if network.is_connected() {
        flags & NLM_ENUM_NETWORK_CONNECTED != 0
    } else {
        flags & NLM_ENUM_NETWORK_DISCONNECTED != 0
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
