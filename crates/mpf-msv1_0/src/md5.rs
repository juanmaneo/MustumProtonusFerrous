//! MD5 (RFC 1321) and HMAC-MD5 (RFC 2104), as in Wine's `MD5Init`, `MD5Update`, `MD5Final` and
//! `hmac_md5_*`. The NTLM2 subkeys and signatures use them.

const INIT: [u32; 4] = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];
const SHIFTS: [[u32; 4]; 4] = [
    [7, 12, 17, 22],
    [5, 9, 14, 20],
    [4, 11, 16, 23],
    [6, 10, 15, 21],
];
const K: [u32; 64] = [
    0xd76a_a478,
    0xe8c7_b756,
    0x2420_70db,
    0xc1bd_ceee,
    0xf57c_0faf,
    0x4787_c62a,
    0xa830_4613,
    0xfd46_9501,
    0x6980_98d8,
    0x8b44_f7af,
    0xffff_5bb1,
    0x895c_d7be,
    0x6b90_1122,
    0xfd98_7193,
    0xa679_438e,
    0x49b4_0821,
    0xf61e_2562,
    0xc040_b340,
    0x265e_5a51,
    0xe9b6_c7aa,
    0xd62f_105d,
    0x0244_1453,
    0xd8a1_e681,
    0xe7d3_fbc8,
    0x21e1_cde6,
    0xc337_07d6,
    0xf4d5_0d87,
    0x455a_14ed,
    0xa9e3_e905,
    0xfcef_a3f8,
    0x676f_02d9,
    0x8d2a_4c8a,
    0xfffa_3942,
    0x8771_f681,
    0x6d9d_6122,
    0xfde5_380c,
    0xa4be_ea44,
    0x4bde_cfa9,
    0xf6bb_4b60,
    0xbebf_bc70,
    0x289b_7ec6,
    0xeaa1_27fa,
    0xd4ef_3085,
    0x0488_1d05,
    0xd9d4_d039,
    0xe6db_99e5,
    0x1fa2_7cf8,
    0xc4ac_5665,
    0xf429_2244,
    0x432a_ff97,
    0xab94_23a7,
    0xfc93_a039,
    0x655b_59c3,
    0x8f0c_cc92,
    0xffef_f47d,
    0x8584_5dd1,
    0x6fa8_7e4f,
    0xfe2c_e6e0,
    0xa301_4314,
    0x4e08_11a1,
    0xf753_7e82,
    0xbd3a_f235,
    0x2ad7_d2bb,
    0xeb86_d391,
];

/// Hashes `data` in one call.
pub(crate) fn md5(data: &[u8]) -> [u8; 16] {
    let mut hash = Md5::default();
    hash.update(data);
    hash.finalize()
}

/// Streaming MD5 state, the `MD5_CTX` of the C code.
pub(crate) struct Md5 {
    state: [u32; 4],
    buffer: [u8; 64],
    buffered: usize,
    length: u64,
}

impl Default for Md5 {
    fn default() -> Self {
        Self {
            state: INIT,
            buffer: [0; 64],
            buffered: 0,
            length: 0,
        }
    }
}

impl Md5 {
    pub(crate) fn update(&mut self, mut data: &[u8]) {
        self.length = self.length.wrapping_add(data.len() as u64);
        if self.buffered > 0 {
            let take = (64 - self.buffered).min(data.len());
            self.buffer[self.buffered..self.buffered + take].copy_from_slice(&data[..take]);
            self.buffered += take;
            data = &data[take..];
            if self.buffered < 64 {
                return;
            }
            compress(&mut self.state, &self.buffer);
            self.buffered = 0;
        }
        let whole = data.len() - data.len() % 64;
        let (blocks, rest) = data.split_at(whole);
        for block in blocks.as_chunks::<64>().0 {
            compress(&mut self.state, block);
        }
        self.buffer[..rest.len()].copy_from_slice(rest);
        self.buffered = rest.len();
    }

    pub(crate) fn finalize(mut self) -> [u8; 16] {
        let bits = self.length.wrapping_mul(8);
        let padding = if self.buffered < 56 {
            56 - self.buffered
        } else {
            120 - self.buffered
        };
        let mut tail = [0u8; 72];
        tail[0] = 0x80;
        tail[padding..padding + 8].copy_from_slice(&bits.to_le_bytes());
        self.update(&tail[..padding + 8]);

        let mut digest = [0u8; 16];
        for (out, word) in digest.as_chunks_mut::<4>().0.iter_mut().zip(self.state) {
            out.copy_from_slice(&word.to_le_bytes());
        }
        digest
    }
}

fn compress(state: &mut [u32; 4], block: &[u8]) {
    let mut x = [0u32; 16];
    for (word, bytes) in x.iter_mut().zip(block.as_chunks::<4>().0) {
        *word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    }

    let [mut a, mut b, mut c, mut d] = *state;
    for (i, &k) in K.iter().enumerate() {
        let (f, g) = match i / 16 {
            0 => ((b & c) | (!b & d), i),
            1 => ((b & d) | (c & !d), (5 * i + 1) % 16),
            2 => (b ^ c ^ d, (3 * i + 5) % 16),
            _ => (c ^ (b | !d), (7 * i) % 16),
        };
        let next = b.wrapping_add(
            a.wrapping_add(f)
                .wrapping_add(k)
                .wrapping_add(x[g])
                .rotate_left(SHIFTS[i / 16][i % 4]),
        );
        (a, b, c, d) = (d, next, b, c);
    }
    for (s, v) in state.iter_mut().zip([a, b, c, d]) {
        *s = s.wrapping_add(v);
    }
}

/// HMAC-MD5 state: the inner hash and the outer key pad, which is applied at the end.
pub(crate) struct HmacMd5 {
    inner: Md5,
    outer_pad: [u8; 64],
}

impl HmacMd5 {
    pub(crate) fn new(key: &[u8]) -> Self {
        let hashed;
        let key = if key.len() > 64 {
            hashed = md5(key);
            &hashed[..]
        } else {
            key
        };
        let mut inner_pad = [0x36u8; 64];
        let mut outer_pad = [0x5cu8; 64];
        for (i, &k) in key.iter().enumerate() {
            inner_pad[i] ^= k;
            outer_pad[i] ^= k;
        }
        let mut inner = Md5::default();
        inner.update(&inner_pad);
        Self { inner, outer_pad }
    }

    pub(crate) fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub(crate) fn finalize(self) -> [u8; 16] {
        let inner = self.inner.finalize();
        let mut outer = Md5::default();
        outer.update(&self.outer_pad);
        outer.update(&inner);
        outer.finalize()
    }
}
