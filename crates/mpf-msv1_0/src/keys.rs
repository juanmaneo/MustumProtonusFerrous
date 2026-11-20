//! The NTLM key derivation of Wine's `main.c`: the NTLM1 session key (`create_ntlm1_session_key`)
//! and the NTLM2 subkeys (`create_ntlm2_subkeys`).

use crate::md4::md4;
use crate::md5::Md5;

/// The `MODE_*` values of `unixlib.h`. Only `Client` selects a different subkey layout.
pub enum Mode {
    Client,
    Server,
    Both,
}

/// The four NTLM2 session keys: `send_*` signs and seals outgoing messages, `recv_*` checks
/// incoming ones.
pub struct Subkeys {
    pub send_sign: [u8; 16],
    pub send_seal: [u8; 16],
    pub recv_sign: [u8; 16],
    pub recv_seal: [u8; 16],
}

const CLIENT_TO_SERVER_SIGN: &[u8] =
    b"session key to client-to-server signing key magic constant\0";
const CLIENT_TO_SERVER_SEAL: &[u8] =
    b"session key to client-to-server sealing key magic constant\0";
const SERVER_TO_CLIENT_SIGN: &[u8] =
    b"session key to server-to-client signing key magic constant\0";
const SERVER_TO_CLIENT_SEAL: &[u8] =
    b"session key to server-to-client sealing key magic constant\0";

/// `create_ntlm1_session_key`: `MD4(MD4(secret))`.
pub fn ntlm1_session_key(secret: &[u8]) -> [u8; 16] {
    md4(&md4(secret))
}

/// `create_ntlm2_subkeys`. The client sends with the client-to-server keys. Any other mode
/// sends with the server-to-client keys, as the C code does.
pub fn ntlm2_subkeys(mode: Mode, session_key: &[u8; 16]) -> Subkeys {
    let client_to_server = (
        subkey(session_key, CLIENT_TO_SERVER_SIGN),
        subkey(session_key, CLIENT_TO_SERVER_SEAL),
    );
    let server_to_client = (
        subkey(session_key, SERVER_TO_CLIENT_SIGN),
        subkey(session_key, SERVER_TO_CLIENT_SEAL),
    );
    let ((send_sign, send_seal), (recv_sign, recv_seal)) = if matches!(mode, Mode::Client) {
        (client_to_server, server_to_client)
    } else {
        (server_to_client, client_to_server)
    };
    Subkeys {
        send_sign,
        send_seal,
        recv_sign,
        recv_seal,
    }
}

/// `calc_ntlm2_subkey`: MD5 over the session key and the magic string, which ends in a NUL.
fn subkey(session_key: &[u8; 16], magic: &[u8]) -> [u8; 16] {
    let mut md5 = Md5::default();
    md5.update(session_key);
    md5.update(magic);
    md5.finalize()
}
