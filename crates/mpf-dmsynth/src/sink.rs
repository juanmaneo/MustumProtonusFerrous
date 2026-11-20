//! The synth sink's conversions (`synthsink.c`): the format it reports, the desired buffer size,
//! and the sample and reference time conversions. The sink reads its format from the synth it is
//! attached to, so the state of that synth is the `Link`.

use crate::format::{pcm16, WaveFormat};
use crate::port::PortParams;
use crate::Error;

/// The synth a sink is attached to: none, one that is not open, or one that is open with these
/// parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link<'a> {
    Detached,
    Closed,
    Open(&'a PortParams),
}

/// The format the sink reports (`synth_sink_get_format`). The default is stereo 22050 Hz, which is
/// also what a synth that refuses `GetFormat` leaves in place.
pub fn sink_format(link: Link) -> WaveFormat {
    match link {
        Link::Open(params) => pcm16(params.audio_channels, params.sample_rate),
        Link::Detached | Link::Closed => pcm16(2, 22050),
    }
}

/// The buffer size the sink asks for (`synth_sink_GetDesiredBufferSize`): one second of 4-byte
/// frames at the synth's rate.
pub fn desired_buffer_size(link: Link) -> Result<u32, Error> {
    match link {
        Link::Detached => Err(Error::NotConfigured),
        Link::Closed => Err(Error::Unexpected),
        Link::Open(params) => {
            let format = pcm16(params.audio_channels, params.sample_rate);
            Ok(format
                .samples_per_sec
                .wrapping_mul(u32::from(format.channels))
                .wrapping_mul(4))
        }
    }
}

/// The reference time of a sample (`synth_sink_SampleToRefTime`). `None` when the format rate is
/// zero, where the C division traps.
pub fn sample_to_ref_time(activate: i64, sample: i64, link: Link) -> Option<i64> {
    let rate = i64::from(sink_format(link).samples_per_sec);
    let ticks = sample.wrapping_mul(10_000).checked_div(rate)?;
    Some(activate.wrapping_add(ticks.wrapping_mul(1000)))
}

/// The sample a reference time falls on (`synth_sink_RefTimeToSample`).
pub fn ref_time_to_sample(activate: i64, reference: i64, link: Link) -> i64 {
    let rate = i64::from(sink_format(link).samples_per_sec);
    (reference.wrapping_sub(activate) / 1000).wrapping_mul(rate) / 10000
}
