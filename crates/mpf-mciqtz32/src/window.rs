//! The video window of `MCIQTZ_mciOpen`: its size, style and the reply for it.

use crate::abi::{
    failed, HResult, E_NOINTERFACE, MCIERR_INTERNAL, MCI_DGV_OPEN_WS, WS_CHILD, WS_CLIPCHILDREN,
    WS_MAXIMIZEBOX, WS_OVERLAPPEDWINDOW, WS_POPUP,
};

/// The default window style: overlapped, clip children, and no maximize box.
pub const DEFAULT_STYLE: u32 = (WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN) & !WS_MAXIMIZEBOX;

/// The result of `IBasicVideo::GetVideoSize`. [VideoSize::Audio] is an audio-only file, which
/// needs no window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoSize {
    Audio,
    Failed,
    Size { width: i32, height: i32 },
}

/// Reads the `GetVideoSize` result. `E_NOINTERFACE` means the file has no video.
pub fn video_size(hr: HResult, width: i32, height: i32) -> VideoSize {
    if hr == E_NOINTERFACE {
        VideoSize::Audio
    } else if failed(hr) {
        VideoSize::Failed
    } else {
        VideoSize::Size { width, height }
    }
}

/// The style for `CreateWindowW`. The requested style is used only with `MCI_DGV_OPEN_WS`.
pub const fn style(flags: u32, requested: u32) -> u32 {
    if flags & MCI_DGV_OPEN_WS != 0 {
        requested
    } else {
        DEFAULT_STYLE
    }
}

/// What `create_window` does next. `Create` gives the arguments for `CreateWindowW`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowPlan {
    Audio,
    Failed,
    Create { style: u32, width: i32, height: i32 },
}

/// Plans the window. For video, the size is grown to the outer frame by `frame` and is at
/// least `min_width` wide. Wine always sizes the frame as an overlapped window.
pub fn plan_window(
    video: VideoSize,
    flags: u32,
    requested: u32,
    min_width: i32,
    frame: fn(i32, i32) -> (i32, i32),
) -> WindowPlan {
    match video {
        VideoSize::Audio => WindowPlan::Audio,
        VideoSize::Failed => WindowPlan::Failed,
        VideoSize::Size { width, height } => {
            let (frame_width, frame_height) = frame(width, height);
            WindowPlan::Create {
                style: style(flags, requested),
                width: frame_width.max(min_width),
                height: frame_height,
            }
        }
    }
}

/// The reply of `MCI_OPEN` for the window step. `created` is whether `CreateWindowW` succeeded.
pub fn window_status(plan: WindowPlan, created: bool) -> u32 {
    match plan {
        WindowPlan::Audio => 0,
        WindowPlan::Failed => MCIERR_INTERNAL,
        WindowPlan::Create { .. } => {
            if created {
                0
            } else {
                MCIERR_INTERNAL
            }
        }
    }
}

/// The size of the video window after it is shown. A popup or child window takes the video
/// size. Other windows take their client area.
pub fn placed_size(style: u32, video: (i32, i32), client: (i32, i32)) -> (i32, i32) {
    if style & (WS_POPUP | WS_CHILD) != 0 {
        video
    } else {
        client
    }
}
