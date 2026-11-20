//! MD4 (RFC 1320) as in Wine's `MD4Init`, `MD4Update` and `MD4Final`. The NTLM1 session key
//! hashes the password twice with it.

const INIT: [u32; 4] = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];
const CONSTANTS: [u32; 3] = [0, 0x5a82_7999, 0x6ed9_eba1];
const ORDER: [[usize; 16]; 3] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15],
    [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15],
];
const SHIFTS: [[u32; 4]; 3] = [[3, 7, 11, 19], [3, 5, 9, 13], [3, 9, 11, 15]];

/// Hashes `data` in one call.
pub(crate) fn md4(data: &[u8]) -> [u8; 16] {
    let mut padded = data.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&((data.len() as u64) << 3).to_le_bytes());

    let mut state = INIT;
    for block in padded.as_chunks::<64>().0 {
        compress(&mut state, block);
    }
    let mut digest = [0u8; 16];
    for (out, word) in digest.as_chunks_mut::<4>().0.iter_mut().zip(state) {
        out.copy_from_slice(&word.to_le_bytes());
    }
    digest
}

fn compress(state: &mut [u32; 4], block: &[u8]) {
    let mut x = [0u32; 16];
    for (word, bytes) in x.iter_mut().zip(block.as_chunks::<4>().0) {
        *word = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    }

    let [mut a, mut b, mut c, mut d] = *state;
    for (round, (order, shifts)) in ORDER.iter().zip(SHIFTS).enumerate() {
        for (step, &k) in order.iter().enumerate() {
            let f = match round {
                0 => (b & c) | (!b & d),
                1 => (b & c) | (b & d) | (c & d),
                _ => b ^ c ^ d,
            };
            let t = a
                .wrapping_add(f)
                .wrapping_add(x[k])
                .wrapping_add(CONSTANTS[round])
                .rotate_left(shifts[step % 4]);
            (a, b, c, d) = (d, t, b, c);
        }
    }
    for (s, v) in state.iter_mut().zip([a, b, c, d]) {
        *s = s.wrapping_add(v);
    }
}
