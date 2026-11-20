//! Headless subset of Wine's `dlls/winegstreamer`, the GStreamer media backend.
//!
//! Ported, checked against the verbatim C code by `oracle_checks.rs`:
//! - [`channel_mask`]: `wg_channel_mask_from_gst` and its position table
//!   `wg_channel_position_from_gst` in `wg_format.c`. It maps the channel positions of a
//!   GStreamer audio layout to a WinMM `dwChannelMask`, and fails when the channels are not in
//!   WinMM order.
//!
//! Skipped: the GStreamer calls that read the layout (`GST_AUDIO_INFO_*`), the caps and
//! format conversions, the media pipeline, the Unix library entry points, and the debug logging.
//!
//! The position values are those of `GstAudioChannelPosition`, and the speaker values are those
//! of `mmreg.h`. Both are fixed by their headers, so the port reads no external state.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

const SPEAKER_FRONT_LEFT: u32 = 0x0000_0001;
const SPEAKER_FRONT_RIGHT: u32 = 0x0000_0002;
const SPEAKER_FRONT_CENTER: u32 = 0x0000_0004;
const SPEAKER_LOW_FREQUENCY: u32 = 0x0000_0008;
const SPEAKER_BACK_LEFT: u32 = 0x0000_0010;
const SPEAKER_BACK_RIGHT: u32 = 0x0000_0020;
const SPEAKER_FRONT_LEFT_OF_CENTER: u32 = 0x0000_0040;
const SPEAKER_FRONT_RIGHT_OF_CENTER: u32 = 0x0000_0080;
const SPEAKER_BACK_CENTER: u32 = 0x0000_0100;
const SPEAKER_SIDE_LEFT: u32 = 0x0000_0200;
const SPEAKER_SIDE_RIGHT: u32 = 0x0000_0400;
const SPEAKER_TOP_CENTER: u32 = 0x0000_0800;
const SPEAKER_TOP_FRONT_LEFT: u32 = 0x0000_1000;
const SPEAKER_TOP_FRONT_CENTER: u32 = 0x0000_2000;
const SPEAKER_TOP_FRONT_RIGHT: u32 = 0x0000_4000;
const SPEAKER_TOP_BACK_LEFT: u32 = 0x0000_8000;
const SPEAKER_TOP_BACK_CENTER: u32 = 0x0001_0000;
const SPEAKER_TOP_BACK_RIGHT: u32 = 0x0002_0000;

/// `GST_AUDIO_CHANNEL_POSITION_MONO`, a single mono channel without a direction.
const GST_CHANNEL_POSITION_MONO: i32 = -2;

/// `position_map` of `wg_channel_position_from_gst`, indexed by the non-negative
/// `GstAudioChannelPosition` values. Zero marks a position with no WinMM speaker.
const SPEAKER_MAP: [u32; 21] = [
    SPEAKER_FRONT_LEFT,
    SPEAKER_FRONT_RIGHT,
    SPEAKER_FRONT_CENTER,
    SPEAKER_LOW_FREQUENCY,
    SPEAKER_BACK_LEFT,
    SPEAKER_BACK_RIGHT,
    SPEAKER_FRONT_LEFT_OF_CENTER,
    SPEAKER_FRONT_RIGHT_OF_CENTER,
    SPEAKER_BACK_CENTER,
    0,
    SPEAKER_SIDE_LEFT,
    SPEAKER_SIDE_RIGHT,
    SPEAKER_TOP_FRONT_LEFT,
    SPEAKER_TOP_FRONT_RIGHT,
    SPEAKER_TOP_FRONT_CENTER,
    SPEAKER_TOP_CENTER,
    SPEAKER_TOP_BACK_LEFT,
    SPEAKER_TOP_BACK_RIGHT,
    0,
    0,
    SPEAKER_TOP_BACK_CENTER,
];

/// `wg_channel_position_from_gst`: the speaker of one channel position, or zero when the
/// position has none.
fn speaker_from_position(position: i32) -> u32 {
    if position == GST_CHANNEL_POSITION_MONO {
        return SPEAKER_FRONT_CENTER;
    }
    match usize::try_from(position) {
        Ok(index) if index < SPEAKER_MAP.len() => SPEAKER_MAP[index],
        _ => 0,
    }
}

/// `wg_channel_mask_from_gst`: the `dwChannelMask` of the channel positions, in order.
///
/// Returns zero when a position has no speaker, or when a position is not above every speaker
/// already in the mask, since WinMM orders the channels from the least to the most significant
/// speaker bit.
pub fn channel_mask(positions: &[i32]) -> u32 {
    let mut mask = 0;
    for &position in positions {
        let speaker = speaker_from_position(position);
        if speaker == 0 || mask & !(speaker - 1) != 0 {
            return 0;
        }
        mask |= speaker;
    }
    mask
}
