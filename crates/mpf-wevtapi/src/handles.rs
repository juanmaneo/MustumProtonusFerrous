//! Handle-returning exports from `dlls/wevtapi/main.c` whose bodies return a fixed value.

use crate::types::{EvtHandle, Outcome, ERROR_CALL_NOT_IMPLEMENTED, NULL_HANDLE};

/// The handle `EvtOpenChannelConfig` returns for every channel: `(EVT_HANDLE)0xdeadbeef`.
pub const CHANNEL_CONFIG_HANDLE: EvtHandle = 0xdead_beef;

/// `EvtOpenSession`: a stub that returns NULL.
pub fn evt_open_session() -> EvtHandle {
    NULL_HANDLE
}

/// `EvtOpenLog`: a stub that returns NULL.
pub fn evt_open_log() -> EvtHandle {
    NULL_HANDLE
}

/// `EvtSubscribe`: a stub that returns NULL.
pub fn evt_subscribe() -> EvtHandle {
    NULL_HANDLE
}

/// `EvtOpenChannelEnum`: a stub that returns NULL.
pub fn evt_open_channel_enum() -> EvtHandle {
    NULL_HANDLE
}

/// `EvtOpenChannelConfig`: returns [`CHANNEL_CONFIG_HANDLE`] for every channel path.
pub fn evt_open_channel_config() -> EvtHandle {
    CHANNEL_CONFIG_HANDLE
}

/// `EvtQuery`: a stub that returns NULL.
pub fn evt_query() -> EvtHandle {
    NULL_HANDLE
}

/// `EvtCreateBookmark`: returns NULL and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn evt_create_bookmark() -> Outcome<EvtHandle> {
    Outcome::failed(NULL_HANDLE, ERROR_CALL_NOT_IMPLEMENTED)
}

/// `EvtCreateRenderContext`: returns NULL and sets `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn evt_create_render_context() -> Outcome<EvtHandle> {
    Outcome::failed(NULL_HANDLE, ERROR_CALL_NOT_IMPLEMENTED)
}
