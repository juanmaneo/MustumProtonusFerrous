//! Validation performed by `BackgroundCopyJob_SetNotifyFlags`.

use crate::{HResult, JobState, BG_E_INVALID_STATE, E_NOTIMPL, S_OK};

pub const BG_NOTIFY_JOB_TRANSFERRED: u32 = 0x0001;
pub const BG_NOTIFY_JOB_ERROR: u32 = 0x0002;
pub const BG_NOTIFY_DISABLE: u32 = 0x0004;
pub const BG_NOTIFY_JOB_MODIFICATION: u32 = 0x0008;
pub const BG_NOTIFY_FILE_TRANSFERRED: u32 = 0x0010;

const VALID_FLAGS: u32 = BG_NOTIFY_JOB_TRANSFERRED
    | BG_NOTIFY_JOB_ERROR
    | BG_NOTIFY_DISABLE
    | BG_NOTIFY_JOB_MODIFICATION
    | BG_NOTIFY_FILE_TRANSFERRED;

/// Return the result of setting flags, without mutating a COM job object.
pub fn check_notify_flags(state: JobState, flags: u32) -> HResult {
    if crate::is_job_done(state) {
        return BG_E_INVALID_STATE;
    }
    if flags & !VALID_FLAGS != 0 {
        return E_NOTIMPL;
    }
    S_OK
}
