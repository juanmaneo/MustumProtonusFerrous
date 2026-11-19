//! `MCICDA_Info`: the product name and the disc identity, copied into a caller's wide buffer.

use crate::device::{toc_or_error, CdRom};
use crate::error::*;
use crate::mci::*;
use crate::Driver;

/// `MCICDA_Info`. The buffer holds `dwRetSize` wide characters, and `lstrcpynW` leaves room for
/// the terminating NUL.
pub fn info(drv: &Driver, dev: &mut dyn CdRom, flags: u32, buffer: Option<&mut [u16]>) -> u32 {
    let Some(buffer) = buffer else {
        return MCIERR_NULL_PARAMETER_BLOCK;
    };
    if !drv.is_open() {
        return MCIERR_INVALID_DEVICE_ID;
    }
    let text = match info_text(dev, flags) {
        Ok(text) => text,
        Err(code) => return code,
    };
    if buffer.is_empty() {
        return MCIERR_PARAM_OVERFLOW;
    }
    let copied = text.len().min(buffer.len() - 1);
    for (slot, byte) in buffer.iter_mut().zip(text.bytes().take(copied)) {
        *slot = u16::from(byte);
    }
    buffer[copied] = 0;
    0
}

fn info_text(dev: &mut dyn CdRom, flags: u32) -> Result<String, u32> {
    if (flags & MCI_INFO_PRODUCT) != 0 {
        Ok("Wine's audio CD".to_string())
    } else if (flags & MCI_INFO_MEDIA_UPC) != 0 {
        Err(MCIERR_NO_IDENTITY)
    } else if (flags & MCI_INFO_MEDIA_IDENTITY) != 0 {
        Ok(toc_or_error(dev)?.serial().to_string())
    } else {
        Err(MCIERR_MISSING_PARAMETER)
    }
}
