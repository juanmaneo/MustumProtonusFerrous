//! Oracle-derived test vectors for the ported websocket logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `WebSocketCreateClientHandle`,
//! `WebSocketAbortHandle` and `WebSocketDeleteHandle` (`dlls/websocket/websocket.c`) and
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`, with `RtlRaiseException` replaced
//! by a recorder) with `gcc -fsanitize=address,undefined -g` under `target/canon/websocket/`,
//! and running the harness. All 19 printed rows are used; ASan and UBSan flagged none, so none
//! are excluded.

use super::*;

fn stub_row(got: StubException, function: &'static str) {
    assert_eq!(
        got,
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "websocket.dll",
            function,
        }
    );
}

#[test]
fn oracle_stub_rows() {
    // row 01 WebSocketBeginClientHandshake -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketBeginClientHandshake
    stub_row(
        web_socket_begin_client_handshake(),
        "WebSocketBeginClientHandshake",
    );
    // row 02 WebSocketBeginServerHandshake -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketBeginServerHandshake
    stub_row(
        web_socket_begin_server_handshake(),
        "WebSocketBeginServerHandshake",
    );
    // row 03 WebSocketCompleteAction -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketCompleteAction
    stub_row(web_socket_complete_action(), "WebSocketCompleteAction");
    // row 04 WebSocketCreateServerHandle -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketCreateServerHandle
    stub_row(
        web_socket_create_server_handle(),
        "WebSocketCreateServerHandle",
    );
    // row 05 WebSocketEndClientHandshake -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketEndClientHandshake
    stub_row(
        web_socket_end_client_handshake(),
        "WebSocketEndClientHandshake",
    );
    // row 06 WebSocketEndServerHandshake -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketEndServerHandshake
    stub_row(
        web_socket_end_server_handshake(),
        "WebSocketEndServerHandshake",
    );
    // row 07 WebSocketGetAction -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketGetAction
    stub_row(web_socket_get_action(), "WebSocketGetAction");
    // row 08 WebSocketGetGlobalProperty -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketGetGlobalProperty
    stub_row(
        web_socket_get_global_property(),
        "WebSocketGetGlobalProperty",
    );
    // row 09 WebSocketReceive -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketReceive
    stub_row(web_socket_receive(), "WebSocketReceive");
    // row 10 WebSocketSend -> raise=1 code=0x80000100 flags=0x1 nargs=2 p0=websocket.dll p1=WebSocketSend
    stub_row(web_socket_send(), "WebSocketSend");
}

#[test]
fn oracle_create_client_handle_rows() {
    // The C harness starts with handle = 0x1234; the C body never writes it.
    // row 11 WebSocketCreateClientHandle props=NULL handle=NULL count=0 -> hr=0x80004001 handle=0x1234
    assert_eq!(
        web_socket_create_client_handle(false, 0x0, None),
        0x8000_4001_u32 as i32
    );
    // row 12 WebSocketCreateClientHandle props=set handle=set count=1 -> hr=0x80004001 handle=0x1234
    let mut handle = 0x1234;
    assert_eq!(
        web_socket_create_client_handle(true, 0x1, Some(&mut handle)),
        E_NOTIMPL
    );
    assert_eq!(handle, 0x1234);
    // row 13 WebSocketCreateClientHandle props=NULL handle=set count=7 -> hr=0x80004001 handle=0x1234
    let mut handle = 0x1234;
    assert_eq!(
        web_socket_create_client_handle(false, 0x7, Some(&mut handle)),
        E_NOTIMPL
    );
    assert_eq!(handle, 0x1234);
    // row 14 WebSocketCreateClientHandle props=set handle=NULL count=4294967295 -> hr=0x80004001 handle=0x1234
    assert_eq!(
        web_socket_create_client_handle(true, 0xffffffff, None),
        0x8000_4001_u32 as i32
    );
    // row 15 WebSocketCreateClientHandle props=set handle=set count=0 -> hr=0x80004001 handle=0x1234
    let mut handle = 0x1234;
    assert_eq!(
        web_socket_create_client_handle(true, 0x0, Some(&mut handle)),
        E_NOTIMPL
    );
    assert_eq!(handle, 0x1234);
}

#[test]
fn oracle_abort_delete_handle_rows() {
    // row 16 WebSocketAbortHandle handle=NULL -> returns
    web_socket_abort_handle(0);
    // row 17 WebSocketAbortHandle handle=0x1234 -> returns
    web_socket_abort_handle(0x1234);
    // row 18 WebSocketDeleteHandle handle=NULL -> returns
    web_socket_delete_handle(0);
    // row 19 WebSocketDeleteHandle handle=0x1234 -> returns
    web_socket_delete_handle(0x1234);
}
