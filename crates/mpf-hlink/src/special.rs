use crate::{HResult, E_INVALIDARG, E_NOTIMPL};

const HLSR_HOME: u32 = 0;
const HLSR_SEARCHPAGE: u32 = 1;
const HLSR_HISTORYFOLDER: u32 = 2;

/// Returns the registry value name for a special reference. The history folder is
/// `E_NOTIMPL` in Wine, and any other reference is `E_INVALIDARG`.
pub fn special_value_name(reference: u32) -> Result<&'static str, HResult> {
    match reference {
        HLSR_HOME => Ok("Start Page"),
        HLSR_SEARCHPAGE => Ok("Search Page"),
        HLSR_HISTORYFOLDER => Err(E_NOTIMPL),
        _ => Err(E_INVALIDARG),
    }
}
