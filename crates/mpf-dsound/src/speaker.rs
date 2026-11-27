//! The speaker configuration to channel mask mapping from dlls/dsound/primary.c, with the
//! DSSPEAKER and SPEAKER values that it uses.

pub const DSSPEAKER_HEADPHONE: u32 = 1;
pub const DSSPEAKER_MONO: u32 = 2;
pub const DSSPEAKER_QUAD: u32 = 3;
pub const DSSPEAKER_STEREO: u32 = 4;
pub const DSSPEAKER_SURROUND: u32 = 5;
pub const DSSPEAKER_5POINT1_BACK: u32 = 6;
pub const DSSPEAKER_7POINT1_WIDE: u32 = 7;
pub const DSSPEAKER_7POINT1_SURROUND: u32 = 8;

pub const SPEAKER_FRONT_LEFT: u32 = 0x0000_0001;
pub const SPEAKER_FRONT_RIGHT: u32 = 0x0000_0002;
pub const SPEAKER_FRONT_CENTER: u32 = 0x0000_0004;
pub const SPEAKER_LOW_FREQUENCY: u32 = 0x0000_0008;
pub const SPEAKER_BACK_LEFT: u32 = 0x0000_0010;
pub const SPEAKER_BACK_RIGHT: u32 = 0x0000_0020;
pub const SPEAKER_FRONT_LEFT_OF_CENTER: u32 = 0x0000_0040;
pub const SPEAKER_FRONT_RIGHT_OF_CENTER: u32 = 0x0000_0080;
pub const SPEAKER_SIDE_LEFT: u32 = 0x0000_0200;
pub const SPEAKER_SIDE_RIGHT: u32 = 0x0000_0400;

/// KSAUDIO_SPEAKER_5POINT1 from ksmedia.h: the six speakers of 5.1 with no side channels.
pub const KSAUDIO_SPEAKER_5POINT1: u32 = SPEAKER_FRONT_LEFT
    | SPEAKER_FRONT_RIGHT
    | SPEAKER_FRONT_CENTER
    | SPEAKER_LOW_FREQUENCY
    | SPEAKER_BACK_LEFT
    | SPEAKER_BACK_RIGHT;

/// speaker_config_to_channel_mask. Only the low byte of the configuration selects the mask,
/// as DSSPEAKER_CONFIG does. Values the switch does not list, such as DSSPEAKER_SURROUND,
/// give front left and front right.
pub fn speaker_config_to_channel_mask(speaker_config: u32) -> u32 {
    match speaker_config & 0xff {
        DSSPEAKER_MONO => SPEAKER_FRONT_LEFT,
        DSSPEAKER_STEREO | DSSPEAKER_HEADPHONE => SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT,
        DSSPEAKER_QUAD => {
            SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT | SPEAKER_BACK_LEFT | SPEAKER_BACK_RIGHT
        }
        DSSPEAKER_5POINT1_BACK => KSAUDIO_SPEAKER_5POINT1,
        DSSPEAKER_7POINT1_SURROUND => {
            KSAUDIO_SPEAKER_5POINT1 | SPEAKER_SIDE_LEFT | SPEAKER_SIDE_RIGHT
        }
        DSSPEAKER_7POINT1_WIDE => {
            KSAUDIO_SPEAKER_5POINT1 | SPEAKER_FRONT_LEFT_OF_CENTER | SPEAKER_FRONT_RIGHT_OF_CENTER
        }
        _ => SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT,
    }
}
