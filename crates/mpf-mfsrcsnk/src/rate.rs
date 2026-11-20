//! The rate checks of media_source.c: GetSlowestRate, GetFastestRate, IsRateSupported and SetRate.

use crate::{HResult, MF_E_REVERSE_UNSUPPORTED, MF_E_UNSUPPORTED_RATE, S_OK};

/// The largest rate magnitude that the source supports, in either direction.
pub const MAX_RATE: f32 = 1e6;

/// MFRATE_DIRECTION.
pub enum RateDirection {
    Forward,
    Reverse,
}

/// IMFRateSupport::GetSlowestRate.
pub fn slowest_rate() -> f32 {
    0.0
}

/// IMFRateSupport::GetFastestRate.
pub fn fastest_rate(direction: RateDirection) -> f32 {
    match direction {
        RateDirection::Forward => MAX_RATE,
        RateDirection::Reverse => -MAX_RATE,
    }
}

/// IMFRateSupport::IsRateSupported without the thin flag, which the C ignores.
///
/// Returns the HRESULT and the nearest supported rate. The nearest rate is
/// the requested rate even when the request is unsupported, as in the C.
pub fn is_rate_supported(rate: f32) -> (HResult, f32) {
    let hr = if (-MAX_RATE..=MAX_RATE).contains(&rate) {
        S_OK
    } else {
        MF_E_UNSUPPORTED_RATE
    };
    (hr, rate)
}

/// The checks of IMFRateControl::SetRate.
///
/// Returns Ok when the rate is supported and the caller should queue the
/// async change. The C queues it with the thin flag, which is not modelled.
pub fn set_rate(rate: f32) -> Result<(), HResult> {
    if rate < 0.0 {
        return Err(MF_E_REVERSE_UNSUPPORTED);
    }
    let (hr, _) = is_rate_supported(rate);
    if hr == S_OK {
        Ok(())
    } else {
        Err(hr)
    }
}
