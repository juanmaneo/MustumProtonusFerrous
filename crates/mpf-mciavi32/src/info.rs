//! `MCIAVI_mciInfo`.

use std::borrow::Cow;

use crate::abi::*;
use crate::Device;

const PRODUCT: &str = "Wine's AVI player";
const VERSION: &str = "1.1";

/// `buffer` is `None` when `lpstrReturn` is NULL. Its length is `dwRetSize`. The return value is
/// the MCI return code.
pub fn mci_info(device: Option<&Device>, flags: u32, buffer: Option<&mut [u16]>) -> u32 {
    let Some(buffer) = buffer else {
        return MCIERR_NULL_PARAMETER_BLOCK;
    };
    let Some(device) = device else {
        return MCIERR_INVALID_DEVICE_ID;
    };
    if flags & MCI_TEST != 0 {
        return 0;
    }
    let text: Cow<[u16]> = if flags & MCI_INFO_PRODUCT != 0 {
        Cow::Owned(PRODUCT.encode_utf16().collect())
    } else if flags & MCI_INFO_VERSION != 0 {
        Cow::Owned(VERSION.encode_utf16().collect())
    } else if flags & MCI_INFO_FILE != 0 {
        Cow::Borrowed(&device.file_name)
    } else {
        return MCIERR_UNRECOGNIZED_COMMAND;
    };
    copy_text(buffer, &text);
    0
}

/// `lstrcpynW` into a buffer of `buffer.len()` units: copies up to the first NUL of `text`, keeps
/// room for a terminating NUL, and writes that NUL when the buffer has room for it.
fn copy_text(buffer: &mut [u16], text: &[u16]) {
    let end = text
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(text.len());
    let len = end.min(buffer.len().saturating_sub(1));
    buffer[..len].copy_from_slice(&text[..len]);
    if let Some(nul) = buffer.get_mut(len) {
        *nul = 0;
    }
}
