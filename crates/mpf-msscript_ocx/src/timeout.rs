//! The `Timeout` property of `ScriptControl`.

use crate::HResult;

/// `CTL_E_INVALIDPROPERTYVALUE`, which is `STD_CTL_SCODE(380)`.
pub const CTL_E_INVALIDPROPERTYVALUE: HResult = 0x800A_017C;

/// `ScriptControl_put_Timeout`. `-1` (no timeout) and any value from 0 up are stored. The reference
/// logs a FIXME for values other than `-1` but still stores them. Values below `-1` are rejected and
/// leave the stored value unchanged.
pub fn set_timeout(stored: &mut i32, timeout: i32) -> Result<(), HResult> {
    if timeout < -1 {
        return Err(CTL_E_INVALIDPROPERTYVALUE);
    }
    *stored = timeout;
    Ok(())
}
