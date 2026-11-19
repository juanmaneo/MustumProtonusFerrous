//! Headless port of Wine's `dlls/mciqtz32`, the DirectShow MCI driver (`mciqtz.c`).
//!
//! The C reference is a `DriverProc` driver. Its commands run on a DirectShow filter graph, a
//! notify thread and a window, and it reads registry values and resources. The deterministic
//! parts run here as pure functions over values. The integrator supplies each COM, window,
//! handle and thread result as a value, and gets back the MCI reply that `mciqtz.c` returns.
//!
//! Ported (names in `dlls/mciqtz32/mciqtz.c`):
//! - The `MCIQTZ_DriverProc` dispatch: [driver::route], [driver::Route], [driver::Command].
//! - `MCIQTZ_drvOpen`, `MCIQTZ_drvClose` and `MCIQTZ_drvConfigure` replies: [driver::drv_open_reply],
//!   [driver::drv_close_reply], [driver::drv_configure_reply].
//! - The class registration check in `register_class`: [driver::class_ok].
//! - The argument checks of the `MCIQTZ_mci*` entry points: [args::check_params],
//!   [args::check_device], [args::check_open_element].
//! - `MCIQTZ_mciSet` time format: [time::set_time_format], [time::TimeFormat].
//! - `MCIQTZ_mciStatus`: [status::status_item], [status::length], [status::position],
//!   [status::mode], [time::time_format_value].
//! - `MCIQTZ_mciPlay` and `MCIQTZ_mciSeek`: [seek::play_range], [seek::seek_target],
//!   [seek::play_reply].
//! - `MCIQTZ_mciGetDevCaps`: [caps::devcap].
//! - `MCIQTZ_mciSetAudio`: [audio::setaudio], [audio::volume_for].
//! - `MCIQTZ_mciPut`: [put::plan_put].
//! - `MCIQTZ_mciOpen` and `create_window`: [window::video_size], [window::plan_window],
//!   [window::window_status], [window::placed_size].
//! - The COM `FAILED` to MCI error mapping used throughout: [abi::com_status].
//!
//! Skipped: `DllMain`; the filter-graph calls of `MCIQTZ_mciOpen`, `MCIQTZ_mciClose`,
//! `MCIQTZ_mciStop`, `MCIQTZ_mciPause` and `MCIQTZ_mciResume`; `MCIQTZ_notifyThread` and
//! `MCIQTZ_mciNotify`; the bodies of `MCIQTZ_mciWhere`, `MCIQTZ_mciUpdate` and `MCIQTZ_mciWindow`;
//! the `CreateWindowW`, `ShowWindow` and window-property calls; the `HeapAlloc` and
//! `CreateEventW` of `MCIQTZ_drvOpen`; the registry and resource calls; and all `TRACE`, `FIXME`,
//! `WARN` and `ERR` logging. `MCI_STATUS_NUMBER_OF_TRACKS`, `MCI_STATUS_MEDIA_PRESENT`,
//! `MCI_STATUS_READY` and `MCI_STATUS_CURRENT_TRACK` are not ported as replies. [status::status_item]
//! returns their [abi::MCIERR_UNRECOGNIZED_COMMAND] result, as Wine does.
//!
//! Deviations from Wine, where Wine has undefined behaviour:
//! - [put::plan_put] wraps the width and height of the rectangle. Wine's `LONG` subtraction
//!   overflows, which C leaves undefined.
//! - [audio::volume_for] casts with `as i32`, which saturates. Wine uses a C cast. The results
//!   agree for every value up to [audio::MAX_VOLUME].
//!
//! Open order for the integrator (`MCIQTZ_mciOpen`, in C order):
//! 1. [args::check_params] on the parameter block and the device.
//! 2. `CoInitializeEx`, then `CoCreateInstance(FilterGraph)`, then the six `QueryInterface`
//!    calls for IMediaControl, IMediaSeeking, IMediaEvent, IVideoWindow, IBasicVideo and
//!    IBasicAudio. A failed step maps through [abi::com_status] and releases what it holds.
//! 3. [args::check_open_element] on the flags and the element name.
//! 4. `RenderFile` on the element name, mapped through [abi::com_status].
//! 5. `GetVideoSize` into [window::video_size], then [window::plan_window] with the
//!    `AdjustWindowRect` frame and `GetSystemMetrics(SM_CXMIN)`. For a `Create` plan, call
//!    `CreateWindowW` and pass its result to [window::window_status].
//! 6. On success return 0. Otherwise return the error from the failed step.

pub mod abi;
pub mod args;
pub mod audio;
pub mod caps;
pub mod driver;
pub mod put;
pub mod seek;
pub mod status;
pub mod time;
pub mod window;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

use crate::abi::*;
use crate::driver::{Command, Route, SESSION_DEVICE};

/// Routes a message to `MCIQTZ_DriverProc`. Driver-level messages come first, then the session
/// instance, then the device commands.
pub fn route(msg: u32, dev_id: u32) -> Route {
    match msg {
        DRV_LOAD | DRV_FREE | DRV_ENABLE | DRV_DISABLE | DRV_QUERYCONFIGURE => Route::Reply(1),
        DRV_OPEN => Route::DrvOpen,
        DRV_CLOSE => Route::DrvClose,
        DRV_CONFIGURE => Route::DrvConfigure,
        DRV_INSTALL | DRV_REMOVE => Route::Reply(DRVCNF_RESTART),
        _ if dev_id == SESSION_DEVICE => Route::Reply(1),
        MCI_OPEN_DRIVER => Route::Command(Command::Open),
        MCI_CLOSE_DRIVER => Route::Command(Command::Close),
        MCI_PLAY => Route::Command(Command::Play),
        MCI_SEEK => Route::Command(Command::Seek),
        MCI_STOP => Route::Command(Command::Stop),
        MCI_PAUSE => Route::Command(Command::Pause),
        MCI_RESUME => Route::Command(Command::Resume),
        MCI_GETDEVCAPS => Route::Command(Command::GetDevCaps),
        MCI_SET => Route::Command(Command::Set),
        MCI_STATUS => Route::Command(Command::Status),
        MCI_WHERE => Route::Command(Command::Where),
        MCI_SETAUDIO => Route::Command(Command::SetAudio),
        MCI_UPDATE => Route::Command(Command::Update),
        MCI_WINDOW => Route::Command(Command::Window),
        MCI_PUT => Route::Command(Command::Put),
        MCI_RECORD | MCI_INFO | MCI_LOAD | MCI_SAVE | MCI_FREEZE | MCI_REALIZE | MCI_UNFREEZE
        | MCI_STEP | MCI_COPY | MCI_CUT | MCI_DELETE | MCI_PASTE | MCI_CUE | MCI_CAPTURE
        | MCI_MONITOR | MCI_RESERVE | MCI_SIGNAL | MCI_SETVIDEO | MCI_QUALITY | MCI_LIST
        | MCI_UNDO | MCI_CONFIGURE | MCI_RESTORE | MCI_SPIN | MCI_ESCAPE | MCI_OPEN | MCI_CLOSE => {
            Route::Unrecognized
        }
        _ => Route::Default,
    }
}
