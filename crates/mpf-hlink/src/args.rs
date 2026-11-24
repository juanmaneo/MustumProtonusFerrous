use crate::{HResult, E_INVALIDARG};

/// `HlinkUpdateStackItem` needs a frame or a browse context.
pub fn check_update_stack_item(frame: bool, browse_context: bool) -> Result<(), HResult> {
    if frame || browse_context {
        Ok(())
    } else {
        Err(E_INVALIDARG)
    }
}

/// `HlinkClone` needs the link, the interface ID and the object.
pub fn check_clone(hlink: bool, riid: bool, object: bool) -> Result<(), HResult> {
    if hlink && riid && object {
        Ok(())
    } else {
        Err(E_INVALIDARG)
    }
}
