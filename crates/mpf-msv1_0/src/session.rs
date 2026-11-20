//! The NTLMSSP session state of Wine's `main.c` and `create_signature`, which builds the 16-byte
//! signature of a message. The entry points that check buffers are in `message`.

use crate::crc32::crc32;
use crate::keys::{ntlm2_subkeys, Mode, Subkeys};
use crate::md5::HmacMd5;
use crate::message::Buffer;
use crate::rc4::Rc4;
use crate::{
    Error, NEGOTIATE_ALWAYS_SIGN, NEGOTIATE_EXTENDED_SESSIONSECURITY, NEGOTIATE_KEY_EXCH,
    NEGOTIATE_SIGN, SECBUFFER_DATA,
};

pub(crate) const SIGNATURE_LEN: usize = 16;
const SIGN_VERSION: u32 = 1;

/// `sign_direction`: which sequence counter and keys a signature uses.
pub(crate) enum Direction {
    Send,
    Recv,
}

/// The `user_ctx` of the C code: the negotiated flags, the NTLM1 stream, and the NTLM2 subkeys
/// with their sequence numbers and seal streams.
pub struct Session {
    pub(crate) flags: u32,
    pub(crate) ntlm_seq: u32,
    pub(crate) ntlm_rc4: Rc4,
    pub(crate) keys: Subkeys,
    pub(crate) send_seq: u32,
    pub(crate) recv_seq: u32,
    pub(crate) send_seal: Rc4,
    pub(crate) recv_seal: Rc4,
}

impl Session {
    /// `init_session`: the NTLM1 stream and both seal streams are keyed at once, and the
    /// sequence counters start at zero.
    pub fn new(mode: Mode, flags: u32, session_key: &[u8; 16]) -> Self {
        let keys = ntlm2_subkeys(mode, session_key);
        Self {
            flags,
            ntlm_seq: 0,
            ntlm_rc4: Rc4::new(session_key),
            send_seq: 0,
            recv_seq: 0,
            send_seal: Rc4::new(&keys.send_seal),
            recv_seal: Rc4::new(&keys.recv_seal),
            keys,
        }
    }

    /// `create_signature`. Fails with `UnsupportedFunction` when the flags neither sign nor
    /// request the dummy signature.
    pub(crate) fn create_signature(
        &mut self,
        flags: u32,
        dir: Direction,
        buffers: &[Buffer<'_>],
        encrypt: bool,
    ) -> Result<[u8; SIGNATURE_LEN], Error> {
        if flags & NEGOTIATE_SIGN != 0 {
            return Ok(self.signed(flags, dir, buffers, encrypt));
        }
        if flags & NEGOTIATE_ALWAYS_SIGN != 0 || flags == 0 {
            let mut sig = [0u8; SIGNATURE_LEN];
            sig[0] = 1;
            return Ok(sig);
        }
        Err(Error::UnsupportedFunction)
    }

    /// The signature of a message that is signed, with the extended format when the flags ask
    /// for it.
    pub(crate) fn signed(
        &mut self,
        flags: u32,
        dir: Direction,
        buffers: &[Buffer<'_>],
        encrypt: bool,
    ) -> [u8; SIGNATURE_LEN] {
        if flags & NEGOTIATE_EXTENDED_SESSIONSECURITY != 0 {
            self.extended_signature(flags, dir, buffers, encrypt)
        } else {
            self.ntlm1_signature(buffers, encrypt)
        }
    }

    fn extended_signature(
        &mut self,
        flags: u32,
        dir: Direction,
        buffers: &[Buffer<'_>],
        encrypt: bool,
    ) -> [u8; SIGNATURE_LEN] {
        let (seq, sign_key, seal) = match dir {
            Direction::Send => (
                next(&mut self.send_seq),
                &self.keys.send_sign,
                &mut self.send_seal,
            ),
            Direction::Recv => (
                next(&mut self.recv_seq),
                &self.keys.recv_sign,
                &mut self.recv_seal,
            ),
        };
        let seq_bytes = seq.to_le_bytes();
        let mut hmac = HmacMd5::new(sign_key);
        hmac.update(&seq_bytes);
        for buffer in buffers.iter().filter(|b| b.kind & SECBUFFER_DATA != 0) {
            hmac.update(buffer.data);
        }
        let mut digest = hmac.finalize();
        if encrypt && flags & NEGOTIATE_KEY_EXCH != 0 {
            seal.process(&mut digest[..8]);
        }

        let mut sig = [0u8; SIGNATURE_LEN];
        sig[..4].copy_from_slice(&SIGN_VERSION.to_le_bytes());
        sig[4..12].copy_from_slice(&digest[..8]);
        sig[12..].copy_from_slice(&seq_bytes);
        sig
    }

    fn ntlm1_signature(&mut self, buffers: &[Buffer<'_>], encrypt: bool) -> [u8; SIGNATURE_LEN] {
        let crc = buffers
            .iter()
            .filter(|b| b.kind & SECBUFFER_DATA != 0)
            .fold(0, |crc, b| crc32(crc, b.data));
        let seq = next(&mut self.ntlm_seq);

        let mut sig = [0u8; SIGNATURE_LEN];
        sig[..4].copy_from_slice(&SIGN_VERSION.to_le_bytes());
        sig[8..12].copy_from_slice(&crc.to_le_bytes());
        sig[12..].copy_from_slice(&seq.to_le_bytes());
        if encrypt {
            self.ntlm_rc4.process(&mut sig[4..]);
        }
        sig
    }
}

/// Returns the counter's value and then advances it, wrapping like the C `unsigned int`.
fn next(counter: &mut u32) -> u32 {
    let value = *counter;
    *counter = counter.wrapping_add(1);
    value
}
