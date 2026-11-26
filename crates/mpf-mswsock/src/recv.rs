/// `SOCKET_ERROR` from `include/winsock2.h`.
pub const SOCKET_ERROR: i32 = -1;

/// `WSARecvEx`. The C body is a FIXME that ignores its arguments and returns `SOCKET_ERROR`.
pub fn wsa_recv_ex() -> i32 {
    SOCKET_ERROR
}
