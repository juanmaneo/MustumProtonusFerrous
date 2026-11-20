//! The message functions of Wine's `main.c`: `ntlm_SpMakeSignature`, `ntlm_SpVerifySignature`,
//! `ntlm_SpSealMessage` and `ntlm_SpUnsealMessage`, with their buffer checks. The buffers are
//! the caller's `SecBufferDesc` entries, and the functions write into them in place.

use crate::session::{Direction, Session, SIGNATURE_LEN};
use crate::{
    Error, NEGOTIATE_ALWAYS_SIGN, NEGOTIATE_EXTENDED_SESSIONSECURITY, NEGOTIATE_KEY_EXCH,
    NEGOTIATE_SEAL, NEGOTIATE_SIGN, SECBUFFER_DATA, SECBUFFER_STREAM, SECBUFFER_TOKEN,
};
use std::mem;

/// A `SecBuffer`: `kind` is the `BufferType`, and `data` is the `pvBuffer` of `cbBuffer` bytes.
pub struct Buffer<'a> {
    pub kind: u32,
    pub data: &'a mut [u8],
}

fn find(buffers: &[Buffer<'_>], kind: u32) -> Option<usize> {
    buffers.iter().position(|buffer| buffer.kind == kind)
}

/// `get_buffer_index` for a required buffer. Fewer than two buffers is an invalid token too.
fn require(buffers: &[Buffer<'_>], kind: u32) -> Result<usize, Error> {
    if buffers.len() < 2 {
        return Err(Error::InvalidToken);
    }
    find(buffers, kind).ok_or(Error::InvalidToken)
}

fn check_len(buffer: &Buffer<'_>) -> Result<(), Error> {
    if buffer.data.len() < SIGNATURE_LEN {
        return Err(Error::BufferTooSmall);
    }
    Ok(())
}

impl Session {
    /// `ntlm_SpMakeSignature`: writes the signature of the message into the TOKEN buffer.
    pub fn make_signature(&mut self, buffers: &mut [Buffer<'_>]) -> Result<(), Error> {
        let token = require(buffers, SECBUFFER_TOKEN)?;
        check_len(&buffers[token])?;
        let sig = self.create_signature(self.flags, Direction::Send, buffers, true)?;
        buffers[token].data[..SIGNATURE_LEN].copy_from_slice(&sig);
        Ok(())
    }

    /// `ntlm_SpVerifySignature`: checks the TOKEN buffer against the message.
    pub fn verify_signature(&mut self, buffers: &[Buffer<'_>]) -> Result<(), Error> {
        let token = require(buffers, SECBUFFER_TOKEN)?;
        check_len(&buffers[token])?;
        self.verify(self.flags, buffers, &buffers[token].data[..SIGNATURE_LEN])
    }

    /// `ntlm_SpSealMessage`: encrypts the first DATA buffer and writes the signature into the
    /// TOKEN buffer. With the extended flags, a failed signature leaves the token as it was.
    pub fn seal_message(&mut self, buffers: &mut [Buffer<'_>]) -> Result<(), Error> {
        let token = require(buffers, SECBUFFER_TOKEN)?;
        let data = require(buffers, SECBUFFER_DATA)?;
        check_len(&buffers[token])?;
        let flags = self.flags;

        if flags & NEGOTIATE_EXTENDED_SESSIONSECURITY != 0 && flags & NEGOTIATE_SEAL != 0 {
            if let Ok(sig) = self.create_signature(flags, Direction::Send, buffers, false) {
                buffers[token].data[..SIGNATURE_LEN].copy_from_slice(&sig);
            }
            self.send_seal.process(buffers[data].data);
            if flags & NEGOTIATE_KEY_EXCH != 0 {
                self.send_seal.process(&mut buffers[token].data[4..12]);
            }
        } else {
            let sig = self.signed(flags | NEGOTIATE_SIGN, Direction::Send, buffers, false);
            buffers[token].data[..SIGNATURE_LEN].copy_from_slice(&sig);
            self.ntlm_rc4.process(buffers[data].data);
            self.ntlm_rc4
                .process(&mut buffers[token].data[4..SIGNATURE_LEN]);
            if flags & NEGOTIATE_ALWAYS_SIGN != 0 || flags == 0 {
                buffers[token].data[4..8].fill(0);
            }
        }
        Ok(())
    }

    /// `ntlm_SpUnsealMessage`. The message comes in either a TOKEN buffer, or a STREAM buffer
    /// that starts with the 16-byte signature. For a STREAM, the rest of it becomes the first DATA
    /// buffer, and the stream keeps only the signature. The DATA buffers are decrypted even when
    /// the signature check fails, as in the C code.
    pub fn unseal_message(&mut self, buffers: &mut [Buffer<'_>]) -> Result<(), Error> {
        let data = require(buffers, SECBUFFER_DATA)?;
        let sig = match (
            find(buffers, SECBUFFER_TOKEN),
            find(buffers, SECBUFFER_STREAM),
        ) {
            (Some(token), None) => token,
            (None, Some(stream)) => stream,
            _ => return Err(Error::InvalidToken),
        };
        check_len(&buffers[sig])?;
        if buffers[sig].kind == SECBUFFER_STREAM {
            let (head, tail) = mem::take(&mut buffers[sig].data).split_at_mut(SIGNATURE_LEN);
            buffers[sig].data = head;
            buffers[data].data = tail;
        }

        let flags = self.flags;
        let rc4 = if flags & NEGOTIATE_EXTENDED_SESSIONSECURITY != 0 && flags & NEGOTIATE_SEAL != 0
        {
            &mut self.recv_seal
        } else {
            &mut self.ntlm_rc4
        };
        for buffer in buffers.iter_mut().filter(|b| b.kind == SECBUFFER_DATA) {
            rc4.process(buffer.data);
        }
        self.verify(
            flags | NEGOTIATE_SIGN,
            buffers,
            &buffers[sig].data[..SIGNATURE_LEN],
        )
    }

    /// `verify_signature`: the computed signature must match the token in bytes 8 to 16.
    fn verify(&mut self, flags: u32, buffers: &[Buffer<'_>], token: &[u8]) -> Result<(), Error> {
        let computed = self.create_signature(flags, Direction::Recv, buffers, true)?;
        if computed[8..] != token[8..SIGNATURE_LEN] {
            return Err(Error::MessageAltered);
        }
        Ok(())
    }
}
