//! The instrument region helpers: the key and velocity matching of `play_instrument_no_fallback`,
//! the drum fallback and patch number of a note on, the sample loop settings, and the attenuation
//! of a region.

pub const F_INSTRUMENT_DRUMS: u32 = 0x8000_0000;
pub const WLOOP_TYPE_FORWARD: u32 = 0x0;
pub const WLOOP_TYPE_RELEASE: u32 = 0x1;
pub const FLUID_LOOP_DURING_RELEASE: u32 = 1;
pub const FLUID_LOOP_UNTIL_RELEASE: u32 = 3;
pub const CENTER_PAN_GAIN: f64 = -30.10;

/// An inclusive key or velocity range (`RGNRANGE`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RgnRange {
    pub low: u16,
    pub high: u16,
}

impl RgnRange {
    pub fn contains(self, value: i32) -> bool {
        (i32::from(self.low)..=i32::from(self.high)).contains(&value)
    }
}

/// Whether a region covers a key and a velocity (`play_instrument_no_fallback`).
pub fn region_matches(key_range: RgnRange, vel_range: RgnRange, key: i32, vel: i32) -> bool {
    key_range.contains(key) && vel_range.contains(vel)
}

/// The patch a note on retries with when no region matched: the drum flag, if the patch has it
/// (`play_instrument`).
pub fn drum_fallback(patch: u32, found: bool) -> Option<u32> {
    if !found && (patch & F_INSTRUMENT_DRUMS) != 0 {
        Some(F_INSTRUMENT_DRUMS)
    } else {
        None
    }
}

/// The patch a note on looks up (`synth_preset_noteon`): the preset patch with its bank folded in,
/// and the drum flag on channel 9.
pub fn noteon_patch(patch: i32, bank: i32, chan: i32) -> u32 {
    let bank = bank as u32;
    let mut patch = patch as u32 | ((bank << 8) & 0x007f00) | ((bank << 9) & 0x7f0000);
    if chan == 9 {
        patch |= F_INSTRUMENT_DRUMS;
    }
    patch
}

/// A region's first sample loop (`WLOOP`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveLoop {
    pub loop_type: u32,
    pub start: u32,
    pub length: u32,
}

/// The loop generators a region sets on a voice: the sample mode, if the loop type has one, and
/// the loop start and end offsets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoopSettings {
    pub sample_mode: Option<u32>,
    pub start: u32,
    pub end: u32,
}

/// The loop settings of a region with `count` loops and first loop `first` (`play_region`). `None`
/// when the region has no loops.
pub fn loop_settings(count: u32, first: WaveLoop) -> Option<LoopSettings> {
    if count == 0 {
        return None;
    }
    let sample_mode = match first.loop_type {
        WLOOP_TYPE_FORWARD => Some(FLUID_LOOP_DURING_RELEASE),
        WLOOP_TYPE_RELEASE => Some(FLUID_LOOP_UNTIL_RELEASE),
        _ => None,
    };
    Some(LoopSettings {
        sample_mode,
        start: first.start,
        end: first.start.wrapping_add(first.length),
    })
}

/// The two attenuation increments of a region (`play_region`): the region's attenuation in dB, and
/// the centre pan gain.
pub fn region_attenuation(attenuation: i32) -> [f64; 2] {
    [f64::from(attenuation) / -65536.0, -CENTER_PAN_GAIN]
}
