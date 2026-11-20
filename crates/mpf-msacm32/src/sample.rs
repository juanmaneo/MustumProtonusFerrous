//! Sample readers, writers and mixers (`C816`, `C168`, `C248`, `C2416`, `R16`,
//! `R24`, `W16`, `W24`, `M8`, `M16` and `M24`).
//!
//! A sample is held left-justified in an `i32`: an 8-bit sample occupies the top
//! byte, a 16-bit sample the top two and a 24-bit sample the whole word. Each
//! conversion of Wine's is then a shift of the same value, and the shifts that
//! Wine does on negative `int`s are defined.

/// The depth of one sample of a PCM stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Depth {
    Bits8 = 0,
    Bits16 = 1,
    Bits24 = 2,
}

impl Depth {
    /// The number of bytes in one sample.
    pub const fn bytes(self) -> usize {
        self as usize + 1
    }
}

/// `C816`, `R16`, `R24`: reads the sample at the start of `src`, left-justified.
/// An 8-bit sample is unsigned and is centred on zero.
pub fn read(depth: Depth, src: &[u8]) -> i32 {
    match depth {
        Depth::Bits8 => (i32::from(src[0]) - 128) << 24,
        Depth::Bits16 => i32::from(i16::from_le_bytes([src[0], src[1]])) << 16,
        Depth::Bits24 => i32::from_le_bytes([0, src[0], src[1], src[2]]),
    }
}

/// `C168`, `C248`, `W16`, `W24`: writes the top bits of the left-justified sample
/// `c` at the start of `dst`. An 8-bit sample is written unsigned.
pub fn write(depth: Depth, c: i32, dst: &mut [u8]) {
    match depth {
        Depth::Bits8 => dst[0] = ((c >> 24) as u8) ^ 0x80,
        Depth::Bits16 => dst[..2].copy_from_slice(&((c >> 16) as i16).to_le_bytes()),
        Depth::Bits24 => dst[..3].copy_from_slice(&((c as u32) >> 8).to_le_bytes()[..3]),
    }
}

/// `M8`, `M16`, `M24`: the saturated sum of a left and a right sample, read at `depth`.
pub fn mix(depth: Depth, l: i32, r: i32) -> i32 {
    match depth {
        Depth::Bits8 => ((l >> 24) + (r >> 24)).clamp(-128, 127) << 24,
        Depth::Bits16 => ((l >> 16) + (r >> 16)).clamp(-32768, 32767) << 16,
        Depth::Bits24 => l.wrapping_add(r).clamp(-0x7fff_ff00, 0x7fff_ff00),
    }
}

/// The deeper of two depths, the depth that a stereo mix is done at.
pub fn wider(a: Depth, b: Depth) -> Depth {
    if a.bytes() >= b.bytes() {
        a
    } else {
        b
    }
}
