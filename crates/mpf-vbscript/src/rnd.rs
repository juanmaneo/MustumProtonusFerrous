//! `Global_Randomize` and `Global_Rnd` in `dlls/vbscript/global.c`. The script object keeps one
//! 24-bit generator state, and these functions take that state and return the new one.

const MODULUS: f32 = 16_777_216.0;

/// `get_next_rnd`. C multiplies signed ints here, which overflows. The multiply wraps instead,
/// and the low 24 bits, which are the state, are the same.
fn next_rnd(value: u32) -> u32 {
    value.wrapping_mul(0x43fd_43fd).wrapping_add(0x00c3_9ec3) & 0x00ff_ffff
}

/// The new state after `Randomize seed`. The seed is the argument, or the integrator's seed when
/// the script gives none. The state keeps its low byte.
pub fn randomize(seed: f64, state: u32) -> u32 {
    let mut hi = (seed.to_bits() >> 32) as u32;
    hi ^= hi >> 16;
    ((hi & 0xffff) << 8) | (state & 0xff)
}

/// `Rnd` and `Rnd arg`, as `(new state, value)`. The argument is the VARIANT value converted to
/// `f32`, which is what C's `to_float` produces. A negative argument reseeds from its bits. Zero
/// repeats the last value without advancing. Any other argument, and `None`, advance the state.
/// The value is in `[0, 1)`.
pub fn rnd(state: u32, arg: Option<f32>) -> (u32, f32) {
    let next = match arg {
        None => next_rnd(state),
        Some(f) if f < 0.0 => {
            let bits = f.to_bits();
            next_rnd(bits.wrapping_add(bits >> 24))
        }
        Some(0.0) => state,
        Some(_) => next_rnd(state),
    };
    (next, next as f32 / MODULUS)
}
