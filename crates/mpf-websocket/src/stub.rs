/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = websocket.dll` in `dlls/websocket/Makefile.in`.
const WEBSOCKET_DLL_NAME: &str = "websocket.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `WebSocketBeginClientHandshake`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_begin_client_handshake() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketBeginClientHandshake")
}

/// `WebSocketBeginServerHandshake`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_begin_server_handshake() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketBeginServerHandshake")
}

/// `WebSocketCompleteAction`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_complete_action() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketCompleteAction")
}

/// `WebSocketCreateServerHandle`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_create_server_handle() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketCreateServerHandle")
}

/// `WebSocketEndClientHandshake`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_end_client_handshake() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketEndClientHandshake")
}

/// `WebSocketEndServerHandshake`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_end_server_handshake() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketEndServerHandshake")
}

/// `WebSocketGetAction`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_get_action() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketGetAction")
}

/// `WebSocketGetGlobalProperty`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_get_global_property() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketGetGlobalProperty")
}

/// `WebSocketReceive`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_receive() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketReceive")
}

/// `WebSocketSend`, exported as `@ stub` in `dlls/websocket/websocket.spec`.
pub fn web_socket_send() -> StubException {
    wine_spec_unimplemented_stub(WEBSOCKET_DLL_NAME, "WebSocketSend")
}
