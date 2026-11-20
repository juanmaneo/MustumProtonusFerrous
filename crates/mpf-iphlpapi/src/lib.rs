//! Port of the deterministic helpers in Wine's `dlls/iphlpapi/iphlpapi_main.c`.
//!
//! Ported:
//! - [`guid::guid_to_string`] covers `ConvertGuidToStringA` and `ConvertGuidToStringW`.
//! - [`luid::luid_to_name`] covers `ConvertInterfaceLuidToNameW`.
//! - [`luid::name_to_luid`] covers `ConvertInterfaceNameToLuidW`.
//! - [`addr::length_to_ipv4_mask`] covers `ConvertLengthToIpv4Mask`.
//! - [`addr::is_loopback`], [`addr::is_linklocal`] and [`addr::is_unspecified`]
//!   cover `sockaddr_is_loopback`, `sockaddr_is_linklocal` and `is_addr_unspecified`.
//! - [`addr::matches_prefix`] covers `match_ip_address_with_prefix`.
//! - [`addr::map_ipv6_to_ipv4`] covers `map_address_6to4`.
//!
//! Skipped: `ConvertStringToGuidW` (its parser is `RtlGUIDFromString` in ntdll);
//! `ParseNetworkString` (its parsers are `RtlIpv4StringToAddressExW` and
//! `RtlIpv6StringToAddressExW` in ntdll); `CreateSortedAddressPairs` and the
//! other exports that read the adapter, route, neighbour or TCP/UDP tables,
//! or use sockets, the registry, NSI, ICMP or handles; and the FIXME stubs.
//!
//! Deviations:
//! - A and W variants of `ConvertGuidToStringX` write the same characters, so
//!   [`guid::guid_to_string`] returns a `String`.
//! - Failed Wine calls zero or leave their output buffer alone. These functions
//!   return `Err` instead, so the output is not produced.
//! - Names are UTF-16 code units, read up to the first NUL as a C wide string.
//!   White space in `wcstol` is ASCII only (space and `\t` to `\r`).
//! - `wcstol` is modelled with a 64-bit `long` (LP64), the oracle's platform.
//!   Overflow saturates at the 64-bit limits, so `iftype99999999999999999999_1`
//!   is rejected: its type saturates to `LONG_MAX`, whose low 32 bits are all
//!   ones, as Wine on LP64 rejects it. A 32-bit `long` gives different results
//!   for values past 2^31.
//! - [`addr::length_to_ipv4_mask`] stores `INADDR_NONE` in its output on error.
//!   Callers that need that output use [`addr::INADDR_NONE`].
//! - [`addr::map_ipv6_to_ipv4`] returns the bytes Wine produces. Wine builds
//!   the IPv4 value from `Word[6] << 16 | Word[7]`, which puts the address's
//!   last two bytes first. `::ffff:1.2.3.4` therefore maps to `3.4.1.2`. The
//!   inverse helper, `find_src_address`, builds its words in the right order,
//!   so this looks like a Wine bug. The port reproduces it to match the oracle.
//! - Addresses are [`addr::IpAddress`] values with IPv4 or IPv6 bytes. Other
//!   address families, which Wine's `SOCKADDR` predicates accept, are not
//!   modelled. Port numbers and scope ids are not carried.

pub mod addr;
pub mod guid;
pub mod luid;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A failed conversion, with the Win32 code Wine returns for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The output buffer is too small for the result (`ERROR_NOT_ENOUGH_MEMORY`).
    NotEnoughMemory,
    /// The input is out of range or malformed (`ERROR_INVALID_PARAMETER`).
    InvalidParameter,
    /// The interface name does not parse (`ERROR_INVALID_NAME`).
    InvalidName,
    /// The buffer is smaller than a GUID string (`ERROR_INSUFFICIENT_BUFFER`).
    InsufficientBuffer,
}

impl Error {
    /// The Win32 error code.
    pub fn code(self) -> u32 {
        match self {
            Error::NotEnoughMemory => 8,
            Error::InvalidParameter => 87,
            Error::InsufficientBuffer => 122,
            Error::InvalidName => 123,
        }
    }
}
