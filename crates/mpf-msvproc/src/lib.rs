//! Port of the deterministic logic of Wine's `dlls/msvproc/video_processor.c`: the MF video
//! subtype to pixel format mapping, the display aperture geometry, and the plane pitch and
//! alignment checks of the video processor's frame buffers.
//!
//! FFmpeg's `av_image_fill_arrays()` and the swscale conversions, the `IMediaBuffer` and
//! `IMF2DBuffer2` plumbing, the COM interfaces and the class factory are not modelled. Wine's
//! `msvproc.c` only registers the MFT and dispatches the class object, so it has no logic to port.

pub mod aperture;
pub mod format;
pub mod layout;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `MFOffset`. Wine only ever reads and writes `value`, and `fract` is carried through copies.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Offset {
    pub fract: u16,
    pub value: i16,
}

/// `MFVideoArea`: the two offsets and the `SIZE` of an aperture.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VideoArea {
    pub offset_x: Offset,
    pub offset_y: Offset,
    pub cx: i32,
    pub cy: i32,
}

impl VideoArea {
    /// True when all four fields of the area are zero. `fract` is not consulted, as in Wine.
    pub fn is_empty(&self) -> bool {
        self.offset_x.value == 0 && self.offset_y.value == 0 && self.cx == 0 && self.cy == 0
    }
}

/// The parts of `MFVideoInfo` the aperture logic reads: the frame size and the minimum display
/// aperture.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    pub minimum_display_aperture: VideoArea,
}

/// A Win32 `RECT`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
