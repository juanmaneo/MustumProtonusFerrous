//! evr subset for MPF: the deterministic logic of Wine's `dlls/evr/main.c`, `mixer.c` and
//! `presenter.c`.
//!
//! The C files are COM implementations whose objects, critical sections, sample queues, clocks
//! and Direct3D devices belong to the runtime. The logic that does not touch them runs here on
//! plain values, and follows the C control flow. Functions that return an `HRESULT` in C return
//! `Result<_, HResult>` here, and `Ok(())` stands for `S_OK`.
//!
//! Ported (names in `dlls/evr/main.c`, `mixer.c` and `presenter.c`; oracle rows in the
//! `oracle_checks` tests):
//! - `MFIsFormatYUV`: [`is_format_yuv`].
//! - `video_mixer_get_interlace_mode_from_video_desc`: [`interlace_mode`].
//! - `video_mixer_append_rt_format`: [`append_rt_format`].
//! - `video_mixer_rgb_to_ycbcr`: [`rgb_to_ycbcr`].
//! - `video_mixer_scale_rect` and `video_mixer_correct_aspect_ratio`: [`scale_rect`] and
//!   [`correct_aspect_ratio`].
//! - The rectangle check of `video_mixer_control_SetStreamOutputRect` and of
//!   `video_presenter_control_SetVideoPosition`: [`check_normalized_rect`] and
//!   [`check_video_position`].
//! - `get_gcd` and `video_presenter_get_native_video_size`: [`gcd`] and [`native_video_size`].
//! - The mode check of `video_presenter_control_SetAspectRatioMode`: [`check_aspect_ratio_mode`].
//! - The input stream table of the mixer, with `AddInputStreams`, `DeleteInputStream`,
//!   `GetStreamIDs`, the zorder and output rectangle controls: [`InputStreams`].
//!
//! Divergences from the C bodies:
//! - `video_mixer_transform_DeleteInputStream` removes the entry at the stream's index in the
//!   sorted table from the insertion-ordered id list as well. The port keeps this as it is in C,
//!   so the list can drop a different stream from the one deleted. This looks like a Wine bug.
//!   See [`InputStreams::delete`].
//! - `video_mixer_transform_GetStreamIDs` also writes `0` to the output id array. The port
//!   returns only the input ids, because the output stream id is always `0`.
//! - A NaN or out-of-range `f32` converted to `i32` saturates in Rust, and NaN gives `0`. In C
//!   the conversion is undefined. The oracle does not test those vectors.
//! - Signed overflow in `correct_aspect_ratio` wraps in Rust. In C it is undefined and the
//!   sanitizer flags it. The oracle excludes those vectors, and the edge checks test them as
//!   wrap cases.
//!
//! Skipped:
//! - `DllMain`, the class factory, `DllGetClassObject`, `MFCreateVideoMixerAndPresenter` and
//!   the COM plumbing of the mixer and presenter. The null-pointer checks are also skipped,
//!   because a reference or a slice cannot be null. The check of `SetVideoPosition` that rejects
//!   two absent rectangles is kept, as `check_video_position` takes `Option`s.
//! - The zorder map of the mixer (`video_mixer_update_zorder_map`) and its `qsort`. The map only
//!   orders the streams for rendering.
//! - The functions that only log `FIXME`, and the attribute, service and alpha-bitmap
//!   interfaces.
//! - The presenter's state, sample queue, scheduling, clock and threads. The window check and
//!   the presenter hooks of `SetVideoPosition` are skipped. `SetAspectRatioMode` checks the mode
//!   but does not store it.
//! - `sample.c` and the filter plumbing of `evr.c`, the mixing preferences, and the frame-time
//!   threshold, which needs `mfplat`.

mod color;
mod format;
mod geometry;
mod mixer;
mod presenter;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use color::{rgb_to_ycbcr, Ayuv16};
pub use format::{append_rt_format, interlace_mode, is_format_yuv, RtFormat, VideoInterlace};
pub use geometry::{
    check_normalized_rect, correct_aspect_ratio, scale_rect, NormalizedRect, Rect, Size,
};
pub use mixer::{InputStreams, MAX_INPUT_STREAMS};
pub use presenter::{
    check_aspect_ratio_mode, check_video_position, gcd, native_video_size, NativeVideoSize,
    AR_MODE_MASK,
};

/// A Win32 `HRESULT`.
pub type HResult = i32;

pub const S_OK: HResult = 0;
pub const E_POINTER: HResult = 0x8000_4003_u32 as HResult;
pub const E_INVALIDARG: HResult = 0x8007_0057_u32 as HResult;
pub const MF_E_BUFFERTOOSMALL: HResult = 0xC00D_36B1_u32 as HResult;
pub const MF_E_INVALIDREQUEST: HResult = 0xC00D_36B2_u32 as HResult;
pub const MF_E_INVALIDSTREAMNUMBER: HResult = 0xC00D_36B3_u32 as HResult;
