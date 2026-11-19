use crate::color::{background_color, Ayuv16};
use crate::geometry::{intersect_rect, Rect};
use crate::{E_INVALIDARG, HRESULT};

/// One `DXVA2_VideoSample`, without its surface: the source and destination rectangles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sample {
    pub src: Rect,
    pub dst: Rect,
}

/// One `StretchRect` call of a blt: the index of the sample, its source rectangle, and the
/// destination rectangle after clipping to the target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stretch {
    pub sample: usize,
    pub src: Rect,
    pub dst: Rect,
}

/// The device calls that `VideoProcessBlt` makes, in order: one `ColorFill` of `fill` with
/// `background`, then one `StretchRect` per entry of `stretches`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BltPlan {
    pub background: u32,
    pub fill: Rect,
    pub stretches: Vec<Stretch>,
}

/// The logic of `video_processor_VideoProcessBlt`, without the surface and device calls.
///
/// The background alpha must be `0xffff`, otherwise the call returns `E_INVALIDARG` before
/// it does anything. Each sample's destination is clipped to the target, and samples that do
/// not overlap the target are skipped.
pub fn plan_blt(
    target: &Rect,
    background: &Ayuv16,
    samples: &[Sample],
) -> Result<BltPlan, HRESULT> {
    if background.alpha != 0xffff {
        return Err(E_INVALIDARG);
    }

    let stretches = samples
        .iter()
        .enumerate()
        .filter_map(|(sample, s)| {
            intersect_rect(target, &s.dst).map(|dst| Stretch {
                sample,
                src: s.src,
                dst,
            })
        })
        .collect();

    Ok(BltPlan {
        background: background_color(background),
        fill: *target,
        stretches,
    })
}
