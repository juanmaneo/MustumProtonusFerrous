//! `DllMain` from `dlls/oledb32/main.c` and the `DllCanUnloadNow` default from
//! `libs/winecrt0/dll_canunload.c`.

use crate::class_object::{Hresult, S_FALSE};

/// `DllMain`: `TRUE` for every reason. The attach path stores the instance handle and calls
/// `DisableThreadLibraryCalls`. Neither has a result to model, so they are not ported.
pub fn dll_main(_reason: u32) -> bool {
    true
}

/// `DllCanUnloadNow`: `S_FALSE`, the winecrt0 default that `oledb32.spec` binds to.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}
