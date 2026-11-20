//! The `IDirectMusicSynth8::Open` parameter negotiation (`synth_Open`): the `DMUS_PORTPARAMS`
//! values the synth stores after clamping the requested ones.

use crate::Error;

/// `sizeof(DMUS_PORTPARAMS)`, the structure with `dwFeatures`.
pub const PORT_PARAMS_SIZE: u32 = 36;
/// `sizeof(DMUS_PORTPARAMS7)`, the smallest structure `synth_Open` accepts.
pub const PORT_PARAMS7_SIZE: u32 = 32;

pub const VALID_VOICES: u32 = 0x01;
pub const VALID_CHANNEL_GROUPS: u32 = 0x02;
pub const VALID_AUDIO_CHANNELS: u32 = 0x04;
pub const VALID_SAMPLE_RATE: u32 = 0x08;
pub const VALID_EFFECTS: u32 = 0x20;
pub const VALID_SHARE: u32 = 0x40;
pub const VALID_FEATURES: u32 = 0x80;

pub const EFFECT_REVERB: u32 = 0x1;
pub const EFFECT_CHORUS: u32 = 0x2;
pub const EFFECT_DELAY: u32 = 0x4;
const EFFECT_MASK: u32 = EFFECT_REVERB | EFFECT_CHORUS | EFFECT_DELAY;

pub const FEATURE_AUDIOPATH: u32 = 0x1;
pub const FEATURE_STREAMING: u32 = 0x2;
const FEATURE_MASK: u32 = FEATURE_AUDIOPATH | FEATURE_STREAMING;

const MIN_SAMPLE_RATE: u32 = 11025;
const MAX_SAMPLE_RATE: u32 = 96000;

/// The `DMUS_PORTPARAMS` fields the synth reads and stores.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PortParams {
    pub size: u32,
    pub valid_params: u32,
    pub voices: u32,
    pub channel_groups: u32,
    pub audio_channels: u32,
    pub sample_rate: u32,
    pub effect_flags: u32,
    pub share: i32,
    pub features: u32,
}

/// The values `synth_Open` stores for every field the caller does not ask for.
pub const SYNTH_DEFAULTS: PortParams = PortParams {
    size: PORT_PARAMS_SIZE,
    valid_params: VALID_VOICES
        | VALID_CHANNEL_GROUPS
        | VALID_AUDIO_CHANNELS
        | VALID_SAMPLE_RATE
        | VALID_EFFECTS
        | VALID_SHARE
        | VALID_FEATURES,
    voices: 32,
    channel_groups: 2,
    audio_channels: 2,
    sample_rate: 22050,
    effect_flags: EFFECT_REVERB,
    share: 0,
    features: 0,
};

/// The limits the requested voice and channel group counts are clamped to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Caps {
    pub max_voices: u32,
    pub max_channel_groups: u32,
}

/// The synth's capabilities, as `synth_create` sets them.
pub const SYNTH_CAPS: Caps = Caps {
    max_voices: 1000,
    max_channel_groups: 1000,
};

/// A successful negotiation: the stored parameters, and whether a requested value was changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Opened {
    pub params: PortParams,
    pub modified: bool,
}

impl Opened {
    /// `S_FALSE` when a requested value was changed, `S_OK` otherwise.
    pub fn hresult(self) -> u32 {
        u32::from(self.modified)
    }
}

fn requests(requested: &PortParams, bit: u32) -> bool {
    (requested.valid_params & bit) != 0
}

/// Negotiates the parameters of `IDirectMusicSynth8::Open`. `current` is the state of an open
/// synth and `requested` the caller's structure, `None` for the defaults.
pub fn negotiate(
    current: Option<&PortParams>,
    requested: Option<&PortParams>,
    caps: Caps,
) -> Result<Opened, Error> {
    if current.is_some() {
        return Err(Error::AlreadyOpen);
    }
    let mut params = SYNTH_DEFAULTS;
    let mut modified = false;

    if let Some(req) = requested {
        if req.size < PORT_PARAMS7_SIZE {
            return Err(Error::InvalidArg);
        }
        if requests(req, VALID_VOICES) {
            params.voices = req.voices.max(1).min(caps.max_voices);
            modified |= params.voices != req.voices;
        }
        if requests(req, VALID_CHANNEL_GROUPS) {
            params.channel_groups = req.channel_groups.max(1).min(caps.max_channel_groups);
            modified |= params.channel_groups != req.channel_groups;
        }
        if requests(req, VALID_AUDIO_CHANNELS) {
            // FluidSynth only works with stereo.
            params.audio_channels = 2;
            modified |= params.audio_channels != req.audio_channels;
        }
        if requests(req, VALID_SAMPLE_RATE) {
            params.sample_rate = req.sample_rate.clamp(MIN_SAMPLE_RATE, MAX_SAMPLE_RATE);
            modified |= params.sample_rate != req.sample_rate;
        }
        if requests(req, VALID_EFFECTS) {
            params.effect_flags = req.effect_flags & EFFECT_MASK;
            modified |= params.effect_flags != req.effect_flags;
        }
        if requests(req, VALID_SHARE) {
            modified |= req.share != 0;
        }
        if req.size < PORT_PARAMS_SIZE {
            params.valid_params &= !VALID_FEATURES;
        } else if requests(req, VALID_FEATURES) {
            params.features = req.features & FEATURE_MASK;
            modified |= params.features != req.features;
        }
    }

    Ok(Opened { params, modified })
}
