//! Frame arithmetic and the DMO size queries, `GetInputSizeInfo` and `GetOutputSizeInfo`.

use crate::types::WaveFormat;
use crate::{check_stream, hresult, Hresult};

/// Samples in one MPEG-1 Layer III frame.
const SAMPLES_PER_FRAME: u32 = 1152;

/// The size and alignment a stream asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SizeInfo {
    pub size: u32,
    pub alignment: u32,
}

/// The byte size of one frame of input at `format`.
pub fn framesize(format: &WaveFormat) -> u32 {
    SAMPLES_PER_FRAME * u32::from(format.block_align)
}

/// The duration of one frame in 100 ns units, or `None` for a sample rate of zero.
pub fn frametime(format: &WaveFormat) -> Option<i64> {
    if format.samples_per_sec == 0 {
        return None;
    }
    Some(10_000_000 * i64::from(SAMPLES_PER_FRAME) / i64::from(format.samples_per_sec))
}

/// `GetInputSizeInfo`: needs both types set. The input is byte aligned.
pub fn input_size_info(
    stream: u32,
    input: Option<&WaveFormat>,
    output: Option<&WaveFormat>,
) -> Result<SizeInfo, Hresult> {
    check_stream(stream)?;
    match (input, output) {
        (Some(_), Some(_)) => Ok(SizeInfo {
            size: 0,
            alignment: 1,
        }),
        _ => Err(hresult::DMO_E_TYPE_NOT_SET),
    }
}

/// `GetOutputSizeInfo`: needs both types set. The size is two frames of output samples at the
/// output's bit depth, and the output is byte aligned.
pub fn output_size_info(
    stream: u32,
    input: Option<&WaveFormat>,
    output: Option<&WaveFormat>,
) -> Result<SizeInfo, Hresult> {
    check_stream(stream)?;
    match (input, output) {
        (Some(_), Some(output)) => Ok(SizeInfo {
            size: 2 * SAMPLES_PER_FRAME * u32::from(output.bits_per_sample) / 8,
            alignment: 1,
        }),
        _ => Err(hresult::DMO_E_TYPE_NOT_SET),
    }
}
