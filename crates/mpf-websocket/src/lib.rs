//! Port of the portable logic in Wine's `dlls/websocket/websocket.c` and `dlls/websocket/websocket.spec`.
//!
//! Ported:
//! - `WebSocketCreateClientHandle`, `WebSocketAbortHandle` and `WebSocketDeleteHandle`, the three
//!   explicit C functions: [`web_socket_create_client_handle`] (fixed `E_NOTIMPL`),
//!   [`web_socket_abort_handle`] and [`web_socket_delete_handle`] (no-ops)
//! - `websocket.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The ten `@ stub` exports (`WebSocketBeginClientHandshake`, `WebSocketBeginServerHandshake`,
//!   `WebSocketCompleteAction`, `WebSocketCreateServerHandle`, `WebSocketEndClientHandshake`,
//!   `WebSocketEndServerHandshake`, `WebSocketGetAction`, `WebSocketGetGlobalProperty`,
//!   `WebSocketReceive`, `WebSocketSend`) and the `__wine_spec_unimplemented_stub` record they
//!   raise: [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `FIXME` traces of the arguments.
//! - The `WEB_SOCKET_PROPERTY` contents: the C body never reads `properties` or `count`, so the
//!   port takes only whether `properties` and `handle` are NULL.
//! - The `__stdcall` calling convention and the OS exception dispatch: the stub record is returned
//!   instead of being raised in a loop.

mod exports;
mod handle;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use handle::{
    web_socket_abort_handle, web_socket_create_client_handle, web_socket_delete_handle, E_NOTIMPL,
};
pub use stub::{
    web_socket_begin_client_handshake, web_socket_begin_server_handshake,
    web_socket_complete_action, web_socket_create_server_handle, web_socket_end_client_handshake,
    web_socket_end_server_handshake, web_socket_get_action, web_socket_get_global_property,
    web_socket_receive, web_socket_send, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
