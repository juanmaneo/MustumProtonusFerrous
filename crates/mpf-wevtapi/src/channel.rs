//! `EvtGetChannelConfigProperty` from `dlls/wevtapi/main.c`, for `EvtChannelLoggingConfigLogFilePath`.

use std::mem::size_of;

use crate::types::{Outcome, ERROR_CALL_NOT_IMPLEMENTED, ERROR_INSUFFICIENT_BUFFER};

/// `EvtChannelLoggingConfigLogFilePath` from `include/winevt.h`, the tenth `EVT_CHANNEL_CONFIG_PROPERTY_ID`.
pub const LOG_FILE_PATH_PROPERTY: u32 = 9;

/// `EvtVarTypeString` from `include/winevt.h`.
pub const EVT_VAR_TYPE_STRING: u32 = 1;

/// `sizeof(EVT_VARIANT)`: the 8-byte union, then `Count` and `Type`.
pub const EVT_VARIANT_SIZE: u32 = 16;

/// `log_pathW` in `dlls/wevtapi/main.c`, before the terminating NUL.
const LOG_PATH: &str = r"C:\windows\temp\evt.log";

/// The `EVT_VARIANT` that the C code writes into the caller's buffer. Its `StringVal` points just
/// past the variant, at the path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertyVariant {
    pub variant_type: u32,
    /// The UTF-16 code units of the path, including the terminating NUL.
    pub string_val: Vec<u16>,
}

fn log_path_w() -> Vec<u16> {
    LOG_PATH.encode_utf16().chain(std::iter::once(0)).collect()
}

/// `EvtGetChannelConfigProperty`. `used` is the `PropertyValueBufferUsed` out-parameter, which the C
/// code writes before it checks the buffer. `buffer_present` is `PropertyValueBuffer != NULL`. The
/// variant the C code writes into the buffer is returned as data.
pub fn get_channel_config_property(
    property_id: u32,
    buffer_size: u32,
    buffer_present: bool,
    used: &mut u32,
) -> (Outcome<bool>, Option<PropertyVariant>) {
    if property_id != LOG_FILE_PATH_PROPERTY {
        return (Outcome::failed(false, ERROR_CALL_NOT_IMPLEMENTED), None);
    }
    let path = log_path_w();
    let needed = (path.len() * size_of::<u16>()) as u32 + EVT_VARIANT_SIZE;
    *used = needed;
    if buffer_size < needed || !buffer_present {
        return (Outcome::failed(false, ERROR_INSUFFICIENT_BUFFER), None);
    }
    let variant = PropertyVariant {
        variant_type: EVT_VAR_TYPE_STRING,
        string_val: path,
    };
    (Outcome::returned(true), Some(variant))
}
