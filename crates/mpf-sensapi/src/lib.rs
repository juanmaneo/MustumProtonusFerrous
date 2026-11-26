//! Port of the portable logic in Wine's `dlls/sensapi/sensapi.c` and `dlls/sensapi/sensapi.spec`.
//!
//! Ported:
//! - `IsDestinationReachableA` and `IsDestinationReachableW`, which always report the destination
//!   as reachable: [`is_destination_reachable_a`], [`is_destination_reachable_w`]
//! - `IsNetworkAlive`, which always reports a LAN connection and stores [`NETWORK_ALIVE_LAN`]
//!   through the optional flags pointer: [`is_network_alive`]
//! - The `NETWORK_ALIVE_*` values from `include/sensapi.h`: [`NETWORK_ALIVE_LAN`],
//!   [`NETWORK_ALIVE_WAN`], [`NETWORK_ALIVE_AOL`]
//! - `sensapi.spec` export table: [`EXPORTS`], [`is_export`]
//!
//! Skipped:
//! - The `FIXME` and `TRACE` output and `debugstr_w`.
//! - The destination string and the `QOCINFO` pointer of the two `IsDestinationReachable`
//!   functions. The C body never reads them, so the port takes no arguments.
//! - The `WINAPI` calling convention.
//! - `@ stub` exports: `sensapi.spec` has none, so no `__wine_spec_unimplemented_stub` model is
//!   needed.

mod destination;
mod exports;
mod network;

pub use destination::{is_destination_reachable_a, is_destination_reachable_w};
pub use exports::{is_export, EXPORTS};
pub use network::{is_network_alive, NETWORK_ALIVE_AOL, NETWORK_ALIVE_LAN, NETWORK_ALIVE_WAN};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
