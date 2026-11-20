//! Position slots of the LZX format: the extra bits and base offset of each slot, from lzx.c.

/// Extra offset bits of each of the 51 position slots.
#[rustfmt::skip]
pub(crate) const EXTRA: [u32; 51] = [
     0,  0,  0,  0,  1,  1,  2,  2,  3,  3,  4,  4,  5,  5,  6,  6,
     7,  7,  8,  8,  9,  9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14,
    15, 15, 16, 16, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
    17, 17, 17,
];

/// Base offset of each position slot. The decoder subtracts 2 from it before adding the extra bits.
#[rustfmt::skip]
pub(crate) const BASE: [u32; 51] = [
          0,       1,       2,      3,      4,      6,      8,     12,     16,     24,     32,       48,      64,      96,     128,     192,
        256,     384,     512,    768,   1024,   1536,   2048,   3072,   4096,   6144,   8192,    12288,   16384,   24576,   32768,   49152,
      65536,   98304,  131072, 196608, 262144, 393216, 524288, 655360, 786432, 917504, 1048576, 1179648, 1310720, 1441792, 1572864, 1703936,
    1835008, 1966080, 2097152,
];

/// The number of position slots a window of `window` bytes needs: the slots whose base is below it.
pub(crate) fn posn_slots(window: u32) -> usize {
    BASE.iter().take_while(|&&base| base < window).count()
}
