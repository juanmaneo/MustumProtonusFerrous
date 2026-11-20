//! The DLS connection block (`CONNECTION` in `dmusicc.h`) and the source, control, destination
//! and transform constants the synth matches on. The `CONN_SRC_CC`, `CONN_SRC_RPN0` and
//! `CONN_TRN_*` bit values are the ones `synth.c` defines locally.

pub const CONN_SRC_NONE: u16 = 0x000;
pub const CONN_SRC_LFO: u16 = 0x001;
pub const CONN_SRC_KEYONVELOCITY: u16 = 0x002;
pub const CONN_SRC_KEYNUMBER: u16 = 0x003;
pub const CONN_SRC_EG1: u16 = 0x004;
pub const CONN_SRC_EG2: u16 = 0x005;
pub const CONN_SRC_PITCHWHEEL: u16 = 0x006;
pub const CONN_SRC_POLYPRESSURE: u16 = 0x007;
pub const CONN_SRC_CHANNELPRESSURE: u16 = 0x008;
pub const CONN_SRC_VIBRATO: u16 = 0x009;
pub const CONN_SRC_CC: u16 = 0x0080;
pub const CONN_SRC_RPN0: u16 = 0x0100;

pub const CONN_DST_NONE: u16 = 0x000;
pub const CONN_DST_GAIN: u16 = 0x001;
pub const CONN_DST_PITCH: u16 = 0x003;
pub const CONN_DST_PAN: u16 = 0x004;
pub const CONN_DST_KEYNUMBER: u16 = 0x005;
pub const CONN_DST_CHORUS: u16 = 0x080;
pub const CONN_DST_REVERB: u16 = 0x081;
pub const CONN_DST_LFO_FREQUENCY: u16 = 0x104;
pub const CONN_DST_LFO_STARTDELAY: u16 = 0x105;
pub const CONN_DST_VIB_FREQUENCY: u16 = 0x114;
pub const CONN_DST_VIB_STARTDELAY: u16 = 0x115;
pub const CONN_DST_EG1_ATTACKTIME: u16 = 0x206;
pub const CONN_DST_EG1_DECAYTIME: u16 = 0x207;
pub const CONN_DST_EG1_RELEASETIME: u16 = 0x209;
pub const CONN_DST_EG1_SUSTAINLEVEL: u16 = 0x20A;
pub const CONN_DST_EG1_DELAYTIME: u16 = 0x20B;
pub const CONN_DST_EG1_HOLDTIME: u16 = 0x20C;
pub const CONN_DST_EG1_SHUTDOWNTIME: u16 = 0x20D;
pub const CONN_DST_EG2_ATTACKTIME: u16 = 0x30A;
pub const CONN_DST_EG2_DECAYTIME: u16 = 0x30B;
pub const CONN_DST_EG2_RELEASETIME: u16 = 0x30D;
pub const CONN_DST_EG2_SUSTAINLEVEL: u16 = 0x30E;
pub const CONN_DST_EG2_DELAYTIME: u16 = 0x30F;
pub const CONN_DST_EG2_HOLDTIME: u16 = 0x310;
pub const CONN_DST_FILTER_CUTOFF: u16 = 0x500;
pub const CONN_DST_FILTER_Q: u16 = 0x501;

pub const CONN_TRN_NONE: u16 = 0;
pub const CONN_TRN_CONCAVE: u16 = 1;
pub const CONN_TRN_CONVEX: u16 = 2;
pub const CONN_TRN_SWITCH: u16 = 3;
pub const CONN_TRN_BIPOLAR: u16 = 1 << 4;
pub const CONN_TRN_INVERT: u16 = 1 << 5;

/// One DLS connection. `scale` is the 16.16 fixed-point value `lScale`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Connection {
    pub source: u16,
    pub control: u16,
    pub destination: u16,
    pub transform: u16,
    pub scale: i32,
}

/// The `CONN_TRANSFORM` macro: packs the source and control transforms and the destination.
pub const fn pack_transform(source: u16, control: u16, destination: u16) -> u16 {
    ((source & 0x3f) << 10) | ((control & 0x3f) << 4) | (destination & 0xf)
}
