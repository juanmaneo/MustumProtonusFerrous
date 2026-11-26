//! Port of the portable logic in Wine's `dlls/mswsock/mswsock.c` and `dlls/mswsock/mswsock.spec`.
//!
//! Ported:
//! - The extension-pointer cache behind `AcceptEx`, `GetAcceptExSockaddrs` and `TransmitFile`:
//!   [`Extensions`] with its one-time lookup, the FALSE return of `AcceptEx` and `TransmitFile`
//!   when a pointer is empty, and the [`Guid`] keys [`WSAID_ACCEPTEX`],
//!   [`WSAID_GETACCEPTEXSOCKADDRS`] and [`WSAID_TRANSMITFILE`]
//! - `WSARecvEx`, a FIXME that returns `SOCKET_ERROR`: [`wsa_recv_ex`], [`SOCKET_ERROR`]
//! - The 26 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export, such as
//!   [`get_address_by_name_a`]
//!
//! Skipped:
//! - `EnumProtocolsA` and `EnumProtocolsW`. The spec forwards them to `ws2_32`, so this DLL has
//!   no code for them.
//! - The `WSAIoctl` call and its `SIO_GET_EXTENSION_FUNCTION_POINTER` code. The caller supplies
//!   the lookup as the `ioctl` closure.
//! - The arguments that `AcceptEx`, `GetAcceptExSockaddrs` and `TransmitFile` forward to the
//!   pointer. The caller passes them in the `call` closure, and the port does not read them.
//! - The `FIXME` trace in `WSARecvEx`, and its unused arguments.
//! - The `WINAPI` and `__cdecl` calling conventions.

mod extensions;
mod recv;
mod stub;

pub use extensions::{
    Extensions, Guid, WSAID_ACCEPTEX, WSAID_GETACCEPTEXSOCKADDRS, WSAID_TRANSMITFILE,
};
pub use recv::{wsa_recv_ex, SOCKET_ERROR};
pub use stub::{
    dn_expand, get_address_by_name_a, get_address_by_name_w, get_name_by_type_a,
    get_name_by_type_w, get_service_a, get_service_w, get_type_by_name_a, get_type_by_name_w,
    getnetbyname, inet_network, migrate_winsock_configuration, np_load_name_spaces, nsp_startup,
    rcmd, rexec, rresvport, s_perror, service_main, set_service_a, set_service_w, sethostname,
    start_wsdp_service, stop_wsdp_service, svchost_push_service_globals,
    wine_spec_unimplemented_stub, wsp_startup, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
