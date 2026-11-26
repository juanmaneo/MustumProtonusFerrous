//! BOOL-returning exports from `dlls/wevtapi/main.c` whose bodies return a fixed value.

use crate::types::{Outcome, ERROR_CALL_NOT_IMPLEMENTED};

/// `EvtNextChannelPath`: returns FALSE without touching the last error.
pub fn evt_next_channel_path() -> bool {
    false
}

/// `EvtSetChannelConfigProperty`: accepts every property and returns TRUE.
pub fn evt_set_channel_config_property() -> bool {
    true
}

/// `EvtSaveChannelConfig`: returns TRUE.
pub fn evt_save_channel_config() -> bool {
    true
}

/// `EvtClose`: returns TRUE.
pub fn evt_close() -> bool {
    true
}

/// `EvtNext`: returns FALSE and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn evt_next() -> Outcome<bool> {
    Outcome::failed(false, ERROR_CALL_NOT_IMPLEMENTED)
}

/// `EvtExportLog`: returns FALSE and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn evt_export_log() -> Outcome<bool> {
    Outcome::failed(false, ERROR_CALL_NOT_IMPLEMENTED)
}

/// `EvtRender`: returns FALSE and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn evt_render() -> Outcome<bool> {
    Outcome::failed(false, ERROR_CALL_NOT_IMPLEMENTED)
}
