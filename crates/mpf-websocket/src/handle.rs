/// `E_NOTIMPL` from `include/winerror.h`, as an `HRESULT`.
pub const E_NOTIMPL: i32 = 0x8000_4001_u32 as i32;

/// `WebSocketCreateClientHandle` from `dlls/websocket/websocket.c`. The C body only traces its
/// arguments and returns `E_NOTIMPL`. `properties_set` is whether `properties` is non-NULL and
/// `handle` is the out-parameter, which the C body never writes.
pub fn web_socket_create_client_handle(
    properties_set: bool,
    count: u32,
    handle: Option<&mut usize>,
) -> i32 {
    let _ = (properties_set, count, handle);
    E_NOTIMPL
}

/// `WebSocketAbortHandle` from `dlls/websocket/websocket.c`: traces and returns.
pub fn web_socket_abort_handle(handle: usize) {
    let _ = handle;
}

/// `WebSocketDeleteHandle` from `dlls/websocket/websocket.c`: traces and returns.
pub fn web_socket_delete_handle(handle: usize) {
    let _ = handle;
}
