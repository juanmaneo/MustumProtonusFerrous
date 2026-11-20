//! Port of the deterministic helpers in Wine's `dlls/netio.sys/netio.c`.
//!
//! Ported:
//! - [`sock_error_to_ntstatus`] (`sock_error_to_ntstatus`).
//! - [`pending::select_slot`] (the slot choice in `allocate_pending_io`).
//! - [`pending::find_wait`] (`find_pending_io`).
//!
//! Skipped, because they need sockets, IRPs, thread pool waits, overlapped I/O,
//! the registry or the process: the `wsk_*` entry points and their argument
//! checks, the callbacks, `WskRegister` and `WskCaptureProviderNPI`, and the
//! `FIXME` stubs. The argument checks sit inside those entry points, next to
//! the socket and IRP calls, so they cannot be run verbatim as an oracle.
//!
//! Deviations:
//! - NTSTATUS values are `u32` bit patterns. Wine's `NTSTATUS` is a signed `LONG`.
//! - The `FIXME` trace for an unmapped error is not emitted.
//! - `find_wait` takes a wait that is not NULL. Wine would also match a NULL
//!   wait against an empty slot, but its callers never pass NULL.

pub mod pending;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

const STATUS_SUCCESS: u32 = 0x0000_0000;
const STATUS_BUFFER_OVERFLOW: u32 = 0x8000_0005;
const STATUS_UNSUCCESSFUL: u32 = 0xC000_0001;
const STATUS_NOT_IMPLEMENTED: u32 = 0xC000_0002;
const STATUS_PENDING: u32 = 0x0000_0103;
const STATUS_INVALID_HANDLE: u32 = 0xC000_0008;
const STATUS_INVALID_PARAMETER: u32 = 0xC000_000D;
const STATUS_NO_MEMORY: u32 = 0xC000_0017;
const STATUS_ACCESS_DENIED: u32 = 0xC000_0022;
const STATUS_OBJECT_TYPE_MISMATCH: u32 = 0xC000_0024;
const STATUS_IO_TIMEOUT: u32 = 0xC000_00B5;
const STATUS_PIPE_DISCONNECTED: u32 = 0xC000_00B0;
const STATUS_NOT_SUPPORTED: u32 = 0xC000_00BB;
const STATUS_NETWORK_BUSY: u32 = 0xC000_00BF;
const STATUS_CANT_WAIT: u32 = 0xC000_00D8;
const STATUS_TOO_MANY_OPENED_FILES: u32 = 0xC000_011F;
const STATUS_ADDRESS_ALREADY_ASSOCIATED: u32 = 0xC000_0238;
const STATUS_CONNECTION_REFUSED: u32 = 0xC000_0236;
const STATUS_CONNECTION_DISCONNECTED: u32 = 0xC000_020C;
const STATUS_CONNECTION_RESET: u32 = 0xC000_020D;
const STATUS_CONNECTION_ABORTED: u32 = 0xC000_0241;
const STATUS_NETWORK_UNREACHABLE: u32 = 0xC000_023C;
const STATUS_NOT_FOUND: u32 = 0xC000_0225;

const WSABASEERR: u32 = 10000;

const WSAEBADF: u32 = WSABASEERR + 9;
const WSAEACCES: u32 = WSABASEERR + 13;
const WSAEFAULT: u32 = WSABASEERR + 14;
const WSAEINVAL: u32 = WSABASEERR + 22;
const WSAEMFILE: u32 = WSABASEERR + 24;
const WSAEWOULDBLOCK: u32 = WSABASEERR + 35;
const WSAEINPROGRESS: u32 = WSABASEERR + 36;
const WSAEALREADY: u32 = WSABASEERR + 37;
const WSAENOTSOCK: u32 = WSABASEERR + 38;
const WSAEDESTADDRREQ: u32 = WSABASEERR + 39;
const WSAEMSGSIZE: u32 = WSABASEERR + 40;
const WSAEPROTOTYPE: u32 = WSABASEERR + 41;
const WSAENOPROTOOPT: u32 = WSABASEERR + 42;
const WSAEPROTONOSUPPORT: u32 = WSABASEERR + 43;
const WSAESOCKTNOSUPPORT: u32 = WSABASEERR + 44;
const WSAEOPNOTSUPP: u32 = WSABASEERR + 45;
const WSAEPFNOSUPPORT: u32 = WSABASEERR + 46;
const WSAEAFNOSUPPORT: u32 = WSABASEERR + 47;
const WSAEADDRINUSE: u32 = WSABASEERR + 48;
const WSAEADDRNOTAVAIL: u32 = WSABASEERR + 49;
const WSAENETDOWN: u32 = WSABASEERR + 50;
const WSAENETUNREACH: u32 = WSABASEERR + 51;
const WSAECONNABORTED: u32 = WSABASEERR + 53;
const WSAECONNRESET: u32 = WSABASEERR + 54;
const WSAENOTCONN: u32 = WSABASEERR + 57;
const WSAESHUTDOWN: u32 = WSABASEERR + 58;
const WSAETIMEDOUT: u32 = WSABASEERR + 60;
const WSAECONNREFUSED: u32 = WSABASEERR + 61;
const WSAHOST_NOT_FOUND: u32 = WSABASEERR + 1001;

/// Maps a Winsock error to the NTSTATUS that Wine's `sock_error_to_ntstatus`
/// returns. An error that Wine does not map gives `STATUS_UNSUCCESSFUL`.
pub fn sock_error_to_ntstatus(err: u32) -> u32 {
    match err {
        0 => STATUS_SUCCESS,
        WSAEBADF => STATUS_INVALID_HANDLE,
        WSAEACCES => STATUS_ACCESS_DENIED,
        WSAEFAULT => STATUS_NO_MEMORY,
        WSAEINVAL => STATUS_INVALID_PARAMETER,
        WSAEMFILE => STATUS_TOO_MANY_OPENED_FILES,
        WSAEWOULDBLOCK => STATUS_CANT_WAIT,
        WSAEINPROGRESS => STATUS_PENDING,
        WSAEALREADY => STATUS_NETWORK_BUSY,
        WSAENOTSOCK => STATUS_OBJECT_TYPE_MISMATCH,
        WSAEDESTADDRREQ => STATUS_INVALID_PARAMETER,
        WSAEMSGSIZE => STATUS_BUFFER_OVERFLOW,
        WSAEPROTONOSUPPORT | WSAESOCKTNOSUPPORT | WSAEPFNOSUPPORT | WSAEAFNOSUPPORT
        | WSAEPROTOTYPE => STATUS_NOT_SUPPORTED,
        WSAENOPROTOOPT => STATUS_INVALID_PARAMETER,
        WSAEOPNOTSUPP => STATUS_NOT_IMPLEMENTED,
        WSAEADDRINUSE => STATUS_ADDRESS_ALREADY_ASSOCIATED,
        WSAEADDRNOTAVAIL => STATUS_INVALID_PARAMETER,
        WSAECONNREFUSED => STATUS_CONNECTION_REFUSED,
        WSAESHUTDOWN => STATUS_PIPE_DISCONNECTED,
        WSAENOTCONN => STATUS_CONNECTION_DISCONNECTED,
        WSAETIMEDOUT => STATUS_IO_TIMEOUT,
        WSAENETUNREACH => STATUS_NETWORK_UNREACHABLE,
        WSAENETDOWN => STATUS_NETWORK_BUSY,
        WSAECONNRESET => STATUS_CONNECTION_RESET,
        WSAECONNABORTED => STATUS_CONNECTION_ABORTED,
        WSAHOST_NOT_FOUND => STATUS_NOT_FOUND,
        _ => STATUS_UNSUCCESSFUL,
    }
}
