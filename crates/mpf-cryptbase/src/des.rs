//! The DES block functions from Wine's `dlls/cryptbase/des.c`: `DEShash` and `DESunhash`, with
//! their permutation tables and bit-level key shifts. The key is 7 bytes with no parity
//! adjustment, as in the C code. Each step follows the C loops, so the results match bit for bit.

const INITIAL_PERMUTE_MAP: [u8; 64] = [
    57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61, 53, 45, 37, 29, 21, 13, 5, 63,
    55, 47, 39, 31, 23, 15, 7, 56, 48, 40, 32, 24, 16, 8, 0, 58, 50, 42, 34, 26, 18, 10, 2, 60, 52,
    44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
];

const KEY_PERMUTE_MAP: [u8; 56] = [
    49, 42, 35, 28, 21, 14, 7, 0, 50, 43, 36, 29, 22, 15, 8, 1, 51, 44, 37, 30, 23, 16, 9, 2, 52,
    45, 38, 31, 55, 48, 41, 34, 27, 20, 13, 6, 54, 47, 40, 33, 26, 19, 12, 5, 53, 46, 39, 32, 25,
    18, 11, 4, 24, 17, 10, 3,
];

const KEY_ROTATION: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

const KEY_COMPRESSION: [u8; 48] = [
    13, 16, 10, 23, 0, 4, 2, 27, 14, 5, 20, 9, 22, 18, 11, 3, 25, 7, 15, 6, 26, 19, 12, 1, 40, 51,
    30, 36, 46, 54, 29, 39, 50, 44, 32, 47, 43, 48, 38, 55, 33, 52, 45, 41, 49, 35, 28, 31,
];

const DATA_EXPANSION: [u8; 48] = [
    31, 0, 1, 2, 3, 4, 3, 4, 5, 6, 7, 8, 7, 8, 9, 10, 11, 12, 11, 12, 13, 14, 15, 16, 15, 16, 17,
    18, 19, 20, 19, 20, 21, 22, 23, 24, 23, 24, 25, 26, 27, 28, 27, 28, 29, 30, 31, 0,
];

const S_BOX: [[u8; 64]; 8] = [
    [
        14, 0, 4, 15, 13, 7, 1, 4, 2, 14, 15, 2, 11, 13, 8, 1, 3, 10, 10, 6, 6, 12, 12, 11, 5, 9,
        9, 5, 0, 3, 7, 8, 4, 15, 1, 12, 14, 8, 8, 2, 13, 4, 6, 9, 2, 1, 11, 7, 15, 5, 12, 11, 9, 3,
        7, 14, 3, 10, 10, 0, 5, 6, 0, 13,
    ],
    [
        15, 3, 1, 13, 8, 4, 14, 7, 6, 15, 11, 2, 3, 8, 4, 14, 9, 12, 7, 0, 2, 1, 13, 10, 12, 6, 0,
        9, 5, 11, 10, 5, 0, 13, 14, 8, 7, 10, 11, 1, 10, 3, 4, 15, 13, 4, 1, 2, 5, 11, 8, 6, 12, 7,
        6, 12, 9, 0, 3, 5, 2, 14, 15, 9,
    ],
    [
        10, 13, 0, 7, 9, 0, 14, 9, 6, 3, 3, 4, 15, 6, 5, 10, 1, 2, 13, 8, 12, 5, 7, 14, 11, 12, 4,
        11, 2, 15, 8, 1, 13, 1, 6, 10, 4, 13, 9, 0, 8, 6, 15, 9, 3, 8, 0, 7, 11, 4, 1, 15, 2, 14,
        12, 3, 5, 11, 10, 5, 14, 2, 7, 12,
    ],
    [
        7, 13, 13, 8, 14, 11, 3, 5, 0, 6, 6, 15, 9, 0, 10, 3, 1, 4, 2, 7, 8, 2, 5, 12, 11, 1, 12,
        10, 4, 14, 15, 9, 10, 3, 6, 15, 9, 0, 0, 6, 12, 10, 11, 1, 7, 13, 13, 8, 15, 9, 1, 4, 3, 5,
        14, 11, 5, 12, 2, 7, 8, 2, 4, 14,
    ],
    [
        2, 14, 12, 11, 4, 2, 1, 12, 7, 4, 10, 7, 11, 13, 6, 1, 8, 5, 5, 0, 3, 15, 15, 10, 13, 3, 0,
        9, 14, 8, 9, 6, 4, 11, 2, 8, 1, 12, 11, 7, 10, 1, 13, 14, 7, 2, 8, 13, 15, 6, 9, 15, 12, 0,
        5, 9, 6, 10, 3, 4, 0, 5, 14, 3,
    ],
    [
        12, 10, 1, 15, 10, 4, 15, 2, 9, 7, 2, 12, 6, 9, 8, 5, 0, 6, 13, 1, 3, 13, 4, 14, 14, 0, 7,
        11, 5, 3, 11, 8, 9, 4, 14, 3, 15, 2, 5, 12, 2, 9, 8, 5, 12, 15, 3, 10, 7, 11, 0, 14, 4, 1,
        10, 7, 1, 6, 13, 0, 11, 8, 6, 13,
    ],
    [
        4, 13, 11, 0, 2, 11, 14, 7, 15, 4, 0, 9, 8, 1, 13, 10, 3, 14, 12, 3, 9, 5, 7, 12, 5, 2, 10,
        15, 6, 8, 1, 6, 1, 6, 4, 11, 11, 13, 13, 8, 12, 1, 3, 4, 7, 10, 14, 7, 10, 9, 15, 5, 6, 0,
        8, 15, 0, 14, 5, 2, 9, 3, 2, 12,
    ],
    [
        13, 1, 2, 15, 8, 13, 4, 8, 6, 10, 15, 3, 11, 7, 1, 4, 10, 12, 9, 5, 3, 6, 14, 11, 5, 0, 0,
        14, 12, 9, 7, 2, 7, 2, 11, 1, 4, 14, 1, 7, 9, 4, 12, 10, 14, 8, 2, 13, 0, 15, 6, 12, 10, 9,
        13, 0, 15, 3, 3, 5, 5, 6, 8, 11,
    ],
];

const P_BOX: [u8; 32] = [
    15, 6, 19, 20, 28, 11, 27, 16, 0, 14, 22, 25, 4, 17, 30, 9, 1, 7, 23, 13, 31, 26, 2, 8, 18, 12,
    29, 5, 21, 10, 3, 24,
];

const FINAL_PERMUTE_MAP: [u8; 64] = [
    7, 39, 15, 47, 23, 55, 31, 63, 6, 38, 14, 46, 22, 54, 30, 62, 5, 37, 13, 45, 21, 53, 29, 61, 4,
    36, 12, 44, 20, 52, 28, 60, 3, 35, 11, 43, 19, 51, 27, 59, 2, 34, 10, 42, 18, 50, 26, 58, 1,
    33, 9, 41, 17, 49, 25, 57, 0, 32, 8, 40, 16, 48, 24, 56,
];

fn get_bit(bytes: &[u8], idx: usize) -> bool {
    (bytes[idx / 8] >> (7 - idx % 8)) & 1 != 0
}

fn set_bit(bytes: &mut [u8], idx: usize) {
    bytes[idx / 8] |= 0x01 << (7 - idx % 8);
}

fn clr_bit(bytes: &mut [u8], idx: usize) {
    bytes[idx / 8] &= !(0x01 << (7 - idx % 8));
}

/// Bit `i` of the result is bit `map[i]` of `src`, MSB first. `map` has `N * 8` entries.
fn permute<const N: usize>(src: &[u8], map: &[u8]) -> [u8; N] {
    let mut dst = [0u8; N];
    for (i, &from) in map.iter().enumerate() {
        if get_bit(src, usize::from(from)) {
            set_bit(&mut dst, i);
        }
    }
    dst
}

fn key_shift_left(key: &mut [u8; 7], numbits: usize) {
    let mut keep = key[0];
    for _ in 0..numbits {
        for j in 0..7 {
            if j > 0 && key[j] & 0x80 != 0 {
                key[j - 1] |= 0x01;
            }
            key[j] <<= 1;
        }
        if get_bit(key, 27) {
            clr_bit(key, 27);
            set_bit(key, 55);
        }
        if keep & 0x80 != 0 {
            set_bit(key, 27);
        }
        keep <<= 1;
    }
}

fn key_shift_right(key: &mut [u8; 7], numbits: usize) {
    let mut keep = key[6];
    for _ in 0..numbits {
        for j in (0..7).rev() {
            if j != 6 && key[j] & 0x01 != 0 {
                key[j + 1] |= 0x80;
            }
            key[j] >>= 1;
        }
        if get_bit(key, 28) {
            clr_bit(key, 28);
            set_bit(key, 0);
        }
        if keep & 0x01 != 0 {
            set_bit(key, 28);
        }
        keep >>= 1;
    }
}

fn sbox(src: &[u8; 6]) -> [u8; 4] {
    let mut dst = [0u8; 4];
    for (i, table) in S_BOX.iter().enumerate() {
        let mut snum = 0usize;
        for bit in 0..6 {
            snum = (snum << 1) | usize::from(get_bit(src, i * 6 + bit));
        }
        let value = table[snum];
        if i % 2 == 0 {
            dst[i / 2] |= value << 4;
        } else {
            dst[i / 2] |= value;
        }
    }
    dst
}

/// One Feistel round on the 8-byte state: the new left half is the old right half, and the new
/// right half is the old left half XOR the mixed right half.
fn feistel(state: &mut [u8; 8], sub_key: &[u8; 6]) {
    let right = [state[4], state[5], state[6], state[7]];
    let mut expanded: [u8; 6] = permute(&right, &DATA_EXPANSION);
    for (byte, key) in expanded.iter_mut().zip(sub_key) {
        *byte ^= key;
    }
    let mixed: [u8; 4] = permute(&sbox(&expanded), &P_BOX);
    let mut next_right = [0u8; 4];
    for ((next, left), mix) in next_right.iter_mut().zip(&state[..4]).zip(mixed) {
        *next = left ^ mix;
    }
    state.copy_within(4..8, 0);
    state[4..8].copy_from_slice(&next_right);
}

/// `DEShash`: encrypts one block. The subkeys run in the same order as the C code.
pub(crate) fn hash(key: &[u8; 7], src: &[u8; 8]) -> [u8; 8] {
    let mut k: [u8; 7] = permute(key, &KEY_PERMUTE_MAP);
    let mut state: [u8; 8] = permute(src, &INITIAL_PERMUTE_MAP);
    for &rotation in &KEY_ROTATION {
        key_shift_left(&mut k, usize::from(rotation));
        let sub_key: [u8; 6] = permute(&k, &KEY_COMPRESSION);
        feistel(&mut state, &sub_key);
    }
    permute(&state, &FINAL_PERMUTE_MAP)
}

/// `DESunhash`: decrypts one block. The key shifts right after each round, in reverse order.
pub(crate) fn unhash(key: &[u8; 7], src: &[u8; 8]) -> [u8; 8] {
    let mut k: [u8; 7] = permute(key, &KEY_PERMUTE_MAP);
    let mut state: [u8; 8] = permute(src, &INITIAL_PERMUTE_MAP);
    for round in 0..16 {
        let sub_key: [u8; 6] = permute(&k, &KEY_COMPRESSION);
        feistel(&mut state, &sub_key);
        key_shift_right(&mut k, usize::from(KEY_ROTATION[15 - round]));
    }
    permute(&state, &FINAL_PERMUTE_MAP)
}
