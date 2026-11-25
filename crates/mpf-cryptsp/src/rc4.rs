//! `SystemFunction032` from Wine's `dlls/cryptsp/arc4.c`: RC4 keystream XOR.

use crate::{NtStatus, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};

struct Arc4 {
    state: [u8; 256],
    x: u8,
    y: u8,
}

impl Arc4 {
    /// `arc4_init`. The key must not be empty.
    fn new(key: &[u8]) -> Self {
        let mut state: [u8; 256] = std::array::from_fn(|i| i as u8);
        let mut key_index = 0;
        let mut state_index = 0u8;
        for i in 0..256 {
            let a = state[i];
            state_index = state_index.wrapping_add(key[key_index]).wrapping_add(a);
            state[i] = state[usize::from(state_index)];
            state[usize::from(state_index)] = a;
            key_index += 1;
            if key_index >= key.len() {
                key_index = 0;
            }
        }
        Self { state, x: 0, y: 0 }
    }

    /// `arc4_ProcessString`: XORs each byte with the next keystream byte.
    fn process(&mut self, data: &mut [u8]) {
        for byte in data {
            self.x = self.x.wrapping_add(1);
            let a = self.state[usize::from(self.x)];
            self.y = self.y.wrapping_add(a);
            let b = self.state[usize::from(self.y)];
            self.state[usize::from(self.x)] = b;
            self.state[usize::from(self.y)] = a;
            *byte ^= self.state[usize::from(a.wrapping_add(b))];
        }
    }
}

/// `SystemFunction032`: XORs `data` in place with the RC4 keystream for `key`.
///
/// An empty key returns `STATUS_UNSUCCESSFUL` and leaves `data` untouched. The C code still
/// reads `key[0]` from a zero-length buffer there, which is an out-of-bounds read.
pub fn system_function_032(data: &mut [u8], key: &[u8]) -> NtStatus {
    if key.is_empty() {
        return STATUS_UNSUCCESSFUL;
    }
    Arc4::new(key).process(data);
    STATUS_SUCCESS
}
