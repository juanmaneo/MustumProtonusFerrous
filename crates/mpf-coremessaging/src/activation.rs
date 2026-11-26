//! `DllGetActivationFactory` class-name dispatch from Wine's `dlls/coremessaging/main.c`.

use crate::{Hresult, CLASS_E_CLASSNOTAVAILABLE};

const DISPATCHER_QUEUE_CONTROLLER: &str = "Windows.System.DispatcherQueueController";

/// The runtime classes this DLL can activate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivationClass {
    DispatcherQueueController,
}

/// Looks up a runtime class name. Like `wcscmp`, the name ends at the first NUL,
/// so anything after an embedded NUL is ignored. Unknown names give
/// `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_activation_factory(name: &[u16]) -> Result<ActivationClass, Hresult> {
    let end = name.iter().position(|&c| c == 0).unwrap_or(name.len());
    if name[..end]
        .iter()
        .copied()
        .eq(DISPATCHER_QUEUE_CONTROLLER.encode_utf16())
    {
        Ok(ActivationClass::DispatcherQueueController)
    } else {
        Err(CLASS_E_CLASSNOTAVAILABLE)
    }
}
