//! RC4 as in Wine's `arc4_init` and `arc4_process` (`main.c`).

pub(crate) struct Rc4 {
    state: [u8; 256],
    x: u8,
    y: u8,
}

impl Rc4 {
    /// `arc4_init`. An empty key is rejected, because the C code reads past the end of the key.
    pub(crate) fn new(key: &[u8]) -> Self {
        assert!(!key.is_empty(), "RC4 key must not be empty");
        let mut state: [u8; 256] = std::array::from_fn(|i| i as u8);
        let mut j = 0u8;
        for i in 0..256 {
            j = j.wrapping_add(state[i]).wrapping_add(key[i % key.len()]);
            state.swap(i, usize::from(j));
        }
        Self { state, x: 0, y: 0 }
    }

    /// `arc4_process`: XORs the keystream into `data` in place.
    pub(crate) fn process(&mut self, data: &mut [u8]) {
        for byte in data {
            self.x = self.x.wrapping_add(1);
            self.y = self.y.wrapping_add(self.state[usize::from(self.x)]);
            self.state.swap(usize::from(self.x), usize::from(self.y));
            let index =
                self.state[usize::from(self.x)].wrapping_add(self.state[usize::from(self.y)]);
            *byte ^= self.state[usize::from(index)];
        }
    }
}
