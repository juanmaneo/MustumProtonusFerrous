/// `NTSTATUS` from `include/ntstatus.h`.
pub type NtStatus = u32;

/// `STATUS_INVALID_PARAMETER` from `include/ntstatus.h`.
pub const STATUS_INVALID_PARAMETER: NtStatus = 0xc000_000d;

/// `STATUS_INVALID_ADDRESS_WILDCARD` from `include/ntstatus.h`.
pub const STATUS_INVALID_ADDRESS_WILDCARD: NtStatus = 0xc000_0208;

/// `STATUS_NOT_SUPPORTED` from `include/ntstatus.h`.
pub const STATUS_NOT_SUPPORTED: NtStatus = 0xc000_00bb;

/// `AF_INET` from `include/ws2def.h`.
pub const AF_INET: u16 = 2;

/// `AF_INET6` from `include/ws2def.h`. This is the Windows value; Linux uses 10.
pub const AF_INET6: u16 = 23;

/// `FILE_DEVICE_NETWORK` from `include/winioctl.h`.
pub const FILE_DEVICE_NETWORK: u32 = 0x0000_0012;

/// `METHOD_BUFFERED` from `include/winioctl.h`.
pub const METHOD_BUFFERED: u32 = 0;

/// `FILE_ANY_ACCESS` from `include/winioctl.h`.
pub const FILE_ANY_ACCESS: u32 = 0;

/// `CTL_CODE` from `include/winioctl.h`.
pub const fn ctl_code(device_type: u32, function: u32, method: u32, access: u32) -> u32 {
    (device_type << 16) | (access << 14) | (function << 2) | method
}

/// `IOCTL_NSIPROXY_WINE_ENUMERATE_ALL` from `include/wine/nsi.h`.
pub const IOCTL_NSIPROXY_WINE_ENUMERATE_ALL: u32 =
    ctl_code(FILE_DEVICE_NETWORK, 0x400, METHOD_BUFFERED, FILE_ANY_ACCESS);

/// `IOCTL_NSIPROXY_WINE_GET_ALL_PARAMETERS` from `include/wine/nsi.h`.
pub const IOCTL_NSIPROXY_WINE_GET_ALL_PARAMETERS: u32 =
    ctl_code(FILE_DEVICE_NETWORK, 0x401, METHOD_BUFFERED, FILE_ANY_ACCESS);

/// `IOCTL_NSIPROXY_WINE_GET_PARAMETER` from `include/wine/nsi.h`.
pub const IOCTL_NSIPROXY_WINE_GET_PARAMETER: u32 =
    ctl_code(FILE_DEVICE_NETWORK, 0x402, METHOD_BUFFERED, FILE_ANY_ACCESS);

/// `IOCTL_NSIPROXY_WINE_ICMP_ECHO` from `include/wine/nsi.h`.
pub const IOCTL_NSIPROXY_WINE_ICMP_ECHO: u32 =
    ctl_code(FILE_DEVICE_NETWORK, 0x403, METHOD_BUFFERED, FILE_ANY_ACCESS);

/// `IOCTL_NSIPROXY_WINE_CHANGE_NOTIFICATION` from `include/wine/nsi.h`.
pub const IOCTL_NSIPROXY_WINE_CHANGE_NOTIFICATION: u32 =
    ctl_code(FILE_DEVICE_NETWORK, 0x404, METHOD_BUFFERED, FILE_ANY_ACCESS);

/// `sizeof(struct icmp_echo_reply_32)` from `dlls/nsiproxy.sys/nsiproxy_private.h`.
pub const ICMP_ECHO_REPLY_32_SIZE: u32 = 28;

/// `sizeof(struct icmp_echo_reply_64)` from `dlls/nsiproxy.sys/nsiproxy_private.h`.
pub const ICMP_ECHO_REPLY_64_SIZE: u32 = 40;

/// `sizeof(ICMPV6_ECHO_REPLY)` from `include/ipexport.h`.
pub const ICMPV6_ECHO_REPLY_SIZE: u32 = 36;

/// `offsetof(struct nsiproxy_icmp_echo, data)` from `include/wine/nsi.h`.
pub const ICMP_ECHO_DATA_OFFSET: u64 = 84;

/// The handler that `nsi_ioctl` in `dlls/nsiproxy.sys/device.c` selects for a control code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoctlOp {
    EnumerateAll,
    GetAllParameters,
    GetParameter,
    IcmpEcho,
    ChangeNotification,
}

/// The dispatch switch of `nsi_ioctl` in `dlls/nsiproxy.sys/device.c`. The handlers are not
/// ported. An unknown code gets `STATUS_NOT_SUPPORTED`, as in the C default case.
pub fn nsi_ioctl(code: u32) -> Result<IoctlOp, NtStatus> {
    match code {
        IOCTL_NSIPROXY_WINE_ENUMERATE_ALL => Ok(IoctlOp::EnumerateAll),
        IOCTL_NSIPROXY_WINE_GET_ALL_PARAMETERS => Ok(IoctlOp::GetAllParameters),
        IOCTL_NSIPROXY_WINE_GET_PARAMETER => Ok(IoctlOp::GetParameter),
        IOCTL_NSIPROXY_WINE_ICMP_ECHO => Ok(IoctlOp::IcmpEcho),
        IOCTL_NSIPROXY_WINE_CHANGE_NOTIFICATION => Ok(IoctlOp::ChangeNotification),
        _ => Err(STATUS_NOT_SUPPORTED),
    }
}

/// `icmp_echo_reply_struct_len` from `dlls/nsiproxy.sys/device.c`. The 32-bit IPv4 reply is used
/// only when `bits` is 32. An unknown family gives 0.
pub fn icmp_echo_reply_struct_len(family: u16, bits: u32) -> u32 {
    match family {
        AF_INET if bits == 32 => ICMP_ECHO_REPLY_32_SIZE,
        AF_INET => ICMP_ECHO_REPLY_64_SIZE,
        AF_INET6 => ICMPV6_ECHO_REPLY_SIZE,
        _ => 0,
    }
}

/// The request fields of `struct nsiproxy_icmp_echo` that `nsiproxy_icmp_echo_check` reads. The
/// C `SOCKADDR_INET` is a union; the port keeps the IPv4 and IPv6 destinations as separate fields,
/// and only the one that matches `dst_family` is read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IcmpEchoRequest {
    pub dst_family: u16,
    pub dst_ipv4: u32,
    pub dst_ipv6: [u8; 16],
    pub bits: u8,
    pub opt_size: u32,
    pub req_size: u32,
}

/// The parameter checks at the top of `nsiproxy_icmp_echo` in `dlls/nsiproxy.sys/device.c`,
/// before the critical section and `handle_send_echo`, which are not ported. The data end
/// `((opt_size + 3) & ~3) + req_size` is computed in 32 bits, as the C `UINT` arithmetic does,
/// and then offset by [`ICMP_ECHO_DATA_OFFSET`] in 64 bits.
pub fn nsiproxy_icmp_echo_check(
    in_len: u32,
    out_len: u32,
    req: &IcmpEchoRequest,
) -> Result<(), NtStatus> {
    let data_len = (req.opt_size.wrapping_add(3) & !3).wrapping_add(req.req_size);
    let data_end = ICMP_ECHO_DATA_OFFSET + u64::from(data_len);
    let in_len64 = u64::from(in_len);
    if in_len64 < ICMP_ECHO_DATA_OFFSET
        || in_len64 < data_end
        || out_len < icmp_echo_reply_struct_len(req.dst_family, u32::from(req.bits))
    {
        return Err(STATUS_INVALID_PARAMETER);
    }
    match req.dst_family {
        AF_INET if req.dst_ipv4 == 0 => Err(STATUS_INVALID_ADDRESS_WILDCARD),
        AF_INET6 if req.dst_ipv6 == [0; 16] => Err(STATUS_INVALID_ADDRESS_WILDCARD),
        AF_INET | AF_INET6 => Ok(()),
        _ => Err(STATUS_INVALID_PARAMETER),
    }
}
