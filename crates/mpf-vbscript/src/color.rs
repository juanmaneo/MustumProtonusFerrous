//! `Global_RGB` in `dlls/vbscript/global.c`.

use crate::VbsError;

/// One channel. A value above 255 clamps to 255, and a negative value is `IllegalFuncCall`.
fn channel(c: i32) -> Result<u32, VbsError> {
    if c < 0 {
        return Err(VbsError::IllegalFuncCall);
    }
    Ok(c.min(255) as u32)
}

/// `RGB`: the `COLORREF` `0x00BBGGRR`.
pub fn rgb(red: i32, green: i32, blue: i32) -> Result<u32, VbsError> {
    Ok(channel(red)? | (channel(green)? << 8) | (channel(blue)? << 16))
}
