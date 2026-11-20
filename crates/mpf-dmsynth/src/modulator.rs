//! The DLS connections the synth turns into FluidSynth modulators: `mod_from_connection` maps a
//! source and its transform to a modulator source and flags, and `add_mod_from_connection` builds
//! the whole modulator with its amount.

use crate::dls::*;
use crate::generator::{
    gen_for_destination, shaped_generator, GEN_MODENVSUSTAIN, GEN_MODLFOTOVOL, GEN_VOLENVSUSTAIN,
};

pub const FLUID_MOD_NONE: u32 = 0;
pub const FLUID_MOD_VELOCITY: u32 = 2;
pub const FLUID_MOD_KEY: u32 = 3;
pub const FLUID_MOD_KEYPRESSURE: u32 = 10;
pub const FLUID_MOD_CHANNELPRESSURE: u32 = 13;
pub const FLUID_MOD_PITCHWHEEL: u32 = 14;
pub const FLUID_MOD_PITCHWHEELSENS: u32 = 16;

pub const FLUID_MOD_NEGATIVE: u32 = 1;
pub const FLUID_MOD_BIPOLAR: u32 = 2;
pub const FLUID_MOD_CONCAVE: u32 = 4;
pub const FLUID_MOD_CONVEX: u32 = 8;
pub const FLUID_MOD_SWITCH: u32 = 12;
pub const FLUID_MOD_CC: u32 = 16;
pub const FLUID_MOD_GC: u32 = 0;
pub const FLUID_MOD_LINEAR: u32 = 0;

/// A FluidSynth modulator: two sources with their flags, the destination generator and the amount.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Modulator {
    pub src1: u32,
    pub flags1: u32,
    pub src2: u32,
    pub flags2: u32,
    pub dest: u32,
    pub amount: f64,
}

/// The FluidSynth source and flags for a connection source (or control) and its 6-bit transform
/// (`mod_from_connection`). `None` for the sources the synth does not map.
pub fn mod_source(source: u16, transform: u16) -> Option<(u32, u32)> {
    let mut flags = FLUID_MOD_GC;
    let fluid_source = if (CONN_SRC_CC..=CONN_SRC_CC + 0x7f).contains(&source) {
        flags = FLUID_MOD_CC;
        u32::from(source - CONN_SRC_CC)
    } else {
        match source {
            CONN_SRC_NONE => FLUID_MOD_NONE,
            CONN_SRC_KEYONVELOCITY => FLUID_MOD_VELOCITY,
            CONN_SRC_KEYNUMBER => FLUID_MOD_KEY,
            CONN_SRC_PITCHWHEEL => FLUID_MOD_PITCHWHEEL,
            CONN_SRC_POLYPRESSURE => FLUID_MOD_KEYPRESSURE,
            CONN_SRC_CHANNELPRESSURE => FLUID_MOD_CHANNELPRESSURE,
            CONN_SRC_RPN0 => FLUID_MOD_PITCHWHEELSENS,
            _ => return None,
        }
    };

    if (transform & CONN_TRN_INVERT) != 0 {
        flags |= FLUID_MOD_NEGATIVE;
    }
    if (transform & CONN_TRN_BIPOLAR) != 0 {
        flags |= FLUID_MOD_BIPOLAR;
    }
    flags |= match transform & CONN_TRN_SWITCH {
        CONN_TRN_NONE => FLUID_MOD_LINEAR,
        CONN_TRN_CONCAVE => FLUID_MOD_CONCAVE,
        CONN_TRN_CONVEX => FLUID_MOD_CONVEX,
        _ => FLUID_MOD_SWITCH,
    };
    Some((fluid_source, flags))
}

/// The modulator a connection adds to a voice (`add_mod_from_connection`), or `None` when the
/// connection maps to no modulator.
pub fn modulator(conn: &Connection) -> Option<Modulator> {
    let control_transform = (conn.transform >> 4) & 0x3f;
    let shaped = shaped_generator(conn.source, conn.destination);

    let (src1, flags1, src2, flags2) = if conn.control != CONN_SRC_NONE && shaped.is_some() {
        let (src1, flags1) = mod_source(conn.control, control_transform)?;
        (src1, flags1, FLUID_MOD_NONE, 0)
    } else {
        let source_transform = (conn.transform >> 10) & 0x3f;
        let (src1, flags1) = mod_source(conn.source, source_transform)?;
        let (src2, flags2) = mod_source(conn.control, control_transform)?;
        (src1, flags1, src2, flags2)
    };

    let dest = match shaped {
        Some(generator) => generator,
        None => gen_for_destination(conn.destination)?,
    };

    let scale = conn.scale;
    let amount = if dest == GEN_MODENVSUSTAIN || dest == GEN_VOLENVSUSTAIN {
        f64::from(scale.wrapping_neg()) / 65536.0
    } else if dest == GEN_MODLFOTOVOL && src1 == FLUID_MOD_CHANNELPRESSURE {
        f64::from(scale) / 655360.0
    } else if scale == i32::MIN {
        -32768.0
    } else {
        f64::from(scale) / 65536.0
    };
    Some(Modulator {
        src1,
        flags1,
        src2,
        flags2,
        dest,
        amount,
    })
}
