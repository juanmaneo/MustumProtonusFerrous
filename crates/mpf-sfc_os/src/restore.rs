//! `SRSetRestorePointA` and `SRSetRestorePointW` from `dlls/sfc_os/sfc_os.c`.

use crate::{Outcome, ERROR_SUCCESS};

/// The `nStatus` field of `STATEMGRSTATUS`, the only field Wine writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateMgrStatus {
    pub n_status: u32,
}

/// `SRSetRestorePointA`: sets `nStatus` to `ERROR_SUCCESS` and returns FALSE.
pub fn sr_set_restore_point_a(status: &mut StateMgrStatus) -> Outcome<bool> {
    set_restore_point_status(status)
}

/// `SRSetRestorePointW`: sets `nStatus` to `ERROR_SUCCESS` and returns FALSE.
pub fn sr_set_restore_point_w(status: &mut StateMgrStatus) -> Outcome<bool> {
    set_restore_point_status(status)
}

fn set_restore_point_status(status: &mut StateMgrStatus) -> Outcome<bool> {
    status.n_status = ERROR_SUCCESS;
    Outcome::without_error(false)
}
