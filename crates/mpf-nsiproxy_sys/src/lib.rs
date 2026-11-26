//! Port of the portable logic in Wine's `dlls/nsiproxy.sys/` `ip.c`, `tcp.c`, `icmp_echo.c` and `device.c`.
//!
//! Ported:
//! - `ip.c`: the netmask prefix helpers [`mask_v4_to_prefix`] and [`mask_v6_to_prefix`], and the
//!   hex IPv6 parser [`str_to_in6_addr`]
//! - `tcp.c`: the IPv6 scope lookup [`find_ipv6_addr_scope`] and its table entry [`Ipv6AddrScope`]
//! - `icmp_echo.c`: the RFC 1071 checksum [`chksum`], the pseudo-header part of [`ipv6_chksum`],
//!   and [`errno_to_ip_status`]
//! - `device.c`: [`icmp_echo_reply_struct_len`], the parameter checks of `nsiproxy_icmp_echo`
//!   ([`nsiproxy_icmp_echo_check`], [`IcmpEchoRequest`]), and the IOCTL dispatch of `nsi_ioctl`
//!   ([`nsi_ioctl`], [`IoctlOp`], [`ctl_code`])
//!
//! Skipped:
//! - `nsiproxy.sys` has no `.spec` file, so it has no `@ stub` exports and no
//!   `__wine_spec_unimplemented_stub` record is modelled.
//! - `DriverEntry` and the IRP plumbing: the dispatch table, device creation, notification
//!   threads and critical sections.
//! - The NSI enumeration and parameter routines in `ip.c`, `tcp.c`, `udp.c`, `ndis.c` and `nsi.c`:
//!   they read OS network state and fill caller buffers.
//! - `get_ipv6_addr_scope_table`, which reads `/proc/net/if_inet6` or `getifaddrs`.
//! - The socket side of `icmp_echo.c` (`icmp_send_echo`, `icmp_get_reply`, and the `getsockname`
//!   step of `ipv6_chksum`): [`ipv6_chksum`] takes the source address as an argument.
//! - `ipv4_set_reply_ip_status` and `ipv6_set_reply_ip_status`: they write into caller-owned raw
//!   reply memory.
//! - `ndis.c`'s unix-name and LUID conversions and `udp_num_addrs`: they depend on OS interface
//!   enumeration and NSI.
//! - The `handle_send_echo` tail of `nsiproxy_icmp_echo`: [`nsiproxy_icmp_echo_check`] returns `Ok`
//!   where the C code calls it.

mod device;
mod icmp_echo;
mod ip;
mod tcp;

pub use device::{
    ctl_code, icmp_echo_reply_struct_len, nsi_ioctl, nsiproxy_icmp_echo_check, IcmpEchoRequest,
    IoctlOp, NtStatus, AF_INET, AF_INET6, FILE_ANY_ACCESS, FILE_DEVICE_NETWORK,
    ICMPV6_ECHO_REPLY_SIZE, ICMP_ECHO_DATA_OFFSET, ICMP_ECHO_REPLY_32_SIZE,
    ICMP_ECHO_REPLY_64_SIZE, IOCTL_NSIPROXY_WINE_CHANGE_NOTIFICATION,
    IOCTL_NSIPROXY_WINE_ENUMERATE_ALL, IOCTL_NSIPROXY_WINE_GET_ALL_PARAMETERS,
    IOCTL_NSIPROXY_WINE_GET_PARAMETER, IOCTL_NSIPROXY_WINE_ICMP_ECHO, METHOD_BUFFERED,
    STATUS_INVALID_ADDRESS_WILDCARD, STATUS_INVALID_PARAMETER, STATUS_NOT_SUPPORTED,
};
pub use icmp_echo::{
    chksum, errno_to_ip_status, ipv6_chksum, EHOSTUNREACH, IPPROTO_ICMPV6,
    IP_DEST_HOST_UNREACHABLE, IP_GENERAL_FAILURE, IP_STATUS_BASE,
};
pub use ip::{mask_v4_to_prefix, mask_v6_to_prefix, str_to_in6_addr};
pub use tcp::{find_ipv6_addr_scope, Ipv6AddrScope};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
