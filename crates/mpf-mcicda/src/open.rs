//! `MCICDA_Open` and `MCICDA_Close`: the use count, and the drive that the device is opened on.

use crate::device::CdRom;
use crate::error::*;
use crate::mci::*;
use crate::Driver;

/// The part of `MCI_OPEN_PARMSW` that `mcicda.c` reads: the element name, as wide characters.
pub struct OpenParms<'a> {
    pub element: &'a [u16],
}

/// `MCICDA_Open`. The device-ID lookup is the caller's, as `Driver` is passed in directly.
pub fn open(
    drv: &mut Driver,
    dev: &mut dyn CdRom,
    flags: u32,
    parms: Option<&OpenParms<'_>>,
) -> u32 {
    let Some(parms) = parms else {
        return MCIERR_NULL_PARAMETER_BLOCK;
    };
    let shareable = (flags & MCI_OPEN_SHAREABLE) != 0;
    if drv.use_count <= 0 {
        drv.use_count = 1;
        drv.shareable = shareable;
    } else if drv.shareable && shareable {
        drv.use_count += 1;
    } else {
        return MCIERR_MUST_USE_SHAREABLE;
    }
    let letter = match select_drive(dev, flags, parms.element) {
        Ok(letter) => letter,
        Err(code) => {
            drv.use_count -= 1;
            return code;
        }
    };
    drv.time_format = MCI_FORMAT_MSF;
    if !dev.open_handle(letter) {
        drv.use_count -= 1;
        return MCIERR_MUST_USE_SHAREABLE;
    }
    0
}

/// The drive letter that `MCICDA_Open` opens. Without an element it is the first CD-ROM drive.
/// With one, only the first character counts, and it must name a CD-ROM drive.
fn select_drive(dev: &dyn CdRom, flags: u32, element: &[u16]) -> Result<u8, u32> {
    if (flags & MCI_OPEN_ELEMENT) == 0 {
        return (b'A'..=b'Z')
            .find(|&letter| dev.is_cd_rom(letter))
            .ok_or(MCIERR_CANNOT_LOAD_DRIVER);
    }
    if (flags & MCI_OPEN_ELEMENT_ID) != 0 {
        return Err(MCIERR_FLAGS_NOT_COMPATIBLE);
    }
    let letter = match u8::try_from(element.first().copied().unwrap_or(0)) {
        Ok(byte) if byte.is_ascii_alphabetic() => byte.to_ascii_uppercase(),
        _ => return Err(MCIERR_INVALID_FILE),
    };
    if !dev.is_cd_rom(letter) {
        return Err(MCIERR_INVALID_FILE);
    }
    Ok(letter)
}

/// `MCICDA_Close`. The handle closes with the last use.
pub fn close(drv: &mut Driver, dev: &mut dyn CdRom) -> u32 {
    if !drv.is_open() {
        return MCIERR_INVALID_DEVICE_ID;
    }
    dev.stop_audio();
    drv.use_count -= 1;
    if drv.use_count == 0 {
        dev.close_handle();
    }
    0
}
