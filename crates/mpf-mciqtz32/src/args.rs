//! Argument checks that the `MCIQTZ_mci*` entry points run before their command. Each returns
//! the MCI error of the first check that fails, in the order `mciqtz.c` makes them.

use crate::abi::{
    MCIERR_INTERNAL, MCIERR_INVALID_DEVICE_ID, MCIERR_NULL_PARAMETER_BLOCK, MCI_OPEN_ELEMENT,
    MCI_OPEN_ELEMENT_ID,
};

/// The parameter block is checked first, then the device.
pub fn check_params(has_params: bool, device_exists: bool) -> Result<(), u32> {
    if !has_params {
        return Err(MCIERR_NULL_PARAMETER_BLOCK);
    }
    check_device(device_exists)
}

/// The device id must name an open device.
pub fn check_device(exists: bool) -> Result<(), u32> {
    if exists {
        Ok(())
    } else {
        Err(MCIERR_INVALID_DEVICE_ID)
    }
}

/// `MCI_OPEN` needs `MCI_OPEN_ELEMENT` without `MCI_OPEN_ELEMENT_ID`, and a name whose first
/// character is not NUL. Any other case returns [MCIERR_INTERNAL].
pub fn check_open_element(flags: u32, name: Option<&[u16]>) -> Result<(), u32> {
    let has_name = matches!(name, Some([first, ..]) if *first != 0);
    if flags & MCI_OPEN_ELEMENT != 0 && flags & MCI_OPEN_ELEMENT_ID == 0 && has_name {
        Ok(())
    } else {
        Err(MCIERR_INTERNAL)
    }
}
