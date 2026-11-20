//! The DLS connections the synth turns into FluidSynth generators: `gen_from_connection`,
//! `set_gen_from_connection` and the channel attenuation of `update_channel_volume`.

use crate::dls::*;

pub const GEN_PITCH: u32 = 59;
pub const GEN_PAN: u32 = 17;
pub const GEN_ATTENUATION: u32 = 48;
pub const GEN_FILTERFC: u32 = 8;
pub const GEN_FILTERQ: u32 = 9;
pub const GEN_CHORUSSEND: u32 = 15;
pub const GEN_REVERBSEND: u32 = 16;
pub const GEN_MODLFOTOPITCH: u32 = 5;
pub const GEN_VIBLFOTOPITCH: u32 = 6;
pub const GEN_MODENVTOPITCH: u32 = 7;
pub const GEN_MODLFOTOFILTERFC: u32 = 10;
pub const GEN_MODENVTOFILTERFC: u32 = 11;
pub const GEN_MODLFOTOVOL: u32 = 13;
pub const GEN_MODLFODELAY: u32 = 21;
pub const GEN_MODLFOFREQ: u32 = 22;
pub const GEN_VIBLFODELAY: u32 = 23;
pub const GEN_VIBLFOFREQ: u32 = 24;
pub const GEN_MODENVDELAY: u32 = 25;
pub const GEN_MODENVATTACK: u32 = 26;
pub const GEN_MODENVHOLD: u32 = 27;
pub const GEN_MODENVDECAY: u32 = 28;
pub const GEN_MODENVSUSTAIN: u32 = 29;
pub const GEN_MODENVRELEASE: u32 = 30;
pub const GEN_VOLENVDELAY: u32 = 33;
pub const GEN_VOLENVATTACK: u32 = 34;
pub const GEN_VOLENVHOLD: u32 = 35;
pub const GEN_VOLENVDECAY: u32 = 36;
pub const GEN_VOLENVSUSTAIN: u32 = 37;
pub const GEN_VOLENVRELEASE: u32 = 38;

/// A generator and the value a connection sets it to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GenSetting {
    pub generator: u32,
    pub value: f64,
}

/// The generator a connection destination maps to (`gen_from_connection`). `None` for the
/// destinations the synth does not map, including `CONN_DST_EG1_SHUTDOWNTIME`.
pub fn gen_for_destination(destination: u16) -> Option<u32> {
    match destination {
        CONN_DST_FILTER_CUTOFF => Some(GEN_FILTERFC),
        CONN_DST_FILTER_Q => Some(GEN_FILTERQ),
        CONN_DST_CHORUS => Some(GEN_CHORUSSEND),
        CONN_DST_REVERB => Some(GEN_REVERBSEND),
        CONN_DST_PAN => Some(GEN_PAN),
        CONN_DST_LFO_STARTDELAY => Some(GEN_MODLFODELAY),
        CONN_DST_LFO_FREQUENCY => Some(GEN_MODLFOFREQ),
        CONN_DST_VIB_STARTDELAY => Some(GEN_VIBLFODELAY),
        CONN_DST_VIB_FREQUENCY => Some(GEN_VIBLFOFREQ),
        CONN_DST_EG2_DELAYTIME => Some(GEN_MODENVDELAY),
        CONN_DST_EG2_ATTACKTIME => Some(GEN_MODENVATTACK),
        CONN_DST_EG2_HOLDTIME => Some(GEN_MODENVHOLD),
        CONN_DST_EG2_DECAYTIME => Some(GEN_MODENVDECAY),
        CONN_DST_EG2_SUSTAINLEVEL => Some(GEN_MODENVSUSTAIN),
        CONN_DST_EG2_RELEASETIME => Some(GEN_MODENVRELEASE),
        CONN_DST_EG1_DELAYTIME => Some(GEN_VOLENVDELAY),
        CONN_DST_EG1_ATTACKTIME => Some(GEN_VOLENVATTACK),
        CONN_DST_EG1_HOLDTIME => Some(GEN_VOLENVHOLD),
        CONN_DST_EG1_DECAYTIME => Some(GEN_VOLENVDECAY),
        CONN_DST_EG1_SUSTAINLEVEL => Some(GEN_VOLENVSUSTAIN),
        CONN_DST_EG1_RELEASETIME => Some(GEN_VOLENVRELEASE),
        CONN_DST_GAIN => Some(GEN_ATTENUATION),
        CONN_DST_PITCH => Some(GEN_PITCH),
        _ => None,
    }
}

/// The generator of a modulated destination: an LFO, EG2 or vibrato source feeding pitch, filter
/// cutoff or volume. Shared by the generator and the modulator conversions.
pub fn shaped_generator(source: u16, destination: u16) -> Option<u32> {
    match (source, destination) {
        (CONN_SRC_LFO, CONN_DST_PITCH) => Some(GEN_MODLFOTOPITCH),
        (CONN_SRC_LFO, CONN_DST_FILTER_CUTOFF) => Some(GEN_MODLFOTOFILTERFC),
        (CONN_SRC_LFO, CONN_DST_GAIN) => Some(GEN_MODLFOTOVOL),
        (CONN_SRC_EG2, CONN_DST_PITCH) => Some(GEN_MODENVTOPITCH),
        (CONN_SRC_EG2, CONN_DST_FILTER_CUTOFF) => Some(GEN_MODENVTOFILTERFC),
        (CONN_SRC_VIBRATO, CONN_DST_PITCH) => Some(GEN_VIBLFOTOPITCH),
        _ => None,
    }
}

/// The generator and value a connection sets on a voice (`set_gen_from_connection`), or `None`
/// when the connection is not one the synth applies that way.
pub fn gen_setting(conn: &Connection) -> Option<GenSetting> {
    if conn.control != CONN_SRC_NONE {
        return None;
    }
    if conn.transform != CONN_TRN_NONE {
        if conn.transform != pack_transform(CONN_TRN_BIPOLAR, CONN_TRN_NONE, CONN_TRN_NONE) {
            return None;
        }
        if conn.source != CONN_SRC_LFO && conn.source != CONN_SRC_VIBRATO {
            return None;
        }
    }

    let generator = match conn.source {
        CONN_SRC_NONE => gen_for_destination(conn.destination)?,
        CONN_SRC_LFO | CONN_SRC_EG2 | CONN_SRC_VIBRATO => {
            shaped_generator(conn.source, conn.destination)?
        }
        _ => return None,
    };

    let value = if generator == GEN_MODENVSUSTAIN || generator == GEN_VOLENVSUSTAIN {
        1000.0 - f64::from(conn.scale) / 65536.0
    } else if conn.scale == i32::MIN {
        -32768.0
    } else {
        f64::from(conn.scale) / 65536.0
    };
    Some(GenSetting { generator, value })
}

/// The attenuation, in dB, that the two channel volumes give (`update_channel_volume`). The sum
/// wraps where the C `LONG` addition would overflow, which the oracle leaves out.
pub fn channel_attenuation(volume0: i32, volume1: i32) -> f64 {
    f64::from(volume0.wrapping_add(volume1)) * -0.1
}
