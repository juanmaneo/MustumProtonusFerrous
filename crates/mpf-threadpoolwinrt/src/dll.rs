//! Port of `DllGetClassObject` and `DllGetActivationFactory` in Wine's `dlls/threadpoolwinrt/main.c`,
//! and of the `DllCanUnloadNow` default in `libs/winecrt0/dll_canunload.c`.

use crate::factory::ThreadPoolFactory;
use crate::hresult::{CLASS_E_CLASSNOTAVAILABLE, HRESULT, S_FALSE, S_OK};
use crate::query::Iid;

/// `RuntimeClass_Windows_System_Threading_ThreadPool`.
pub const THREAD_POOL_CLASS: &str = "Windows.System.Threading.ThreadPool";

/// `DllGetClassObject`: no class objects are provided, so the result is always `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object() -> HRESULT {
    CLASS_E_CLASSNOTAVAILABLE
}

/// `DllCanUnloadNow`. `main.c` does not define it; the DLL links the winecrt0 default, which returns `S_FALSE`.
pub fn dll_can_unload_now() -> HRESULT {
    S_FALSE
}

/// `DllGetActivationFactory`. On a match, the static factory is queried for `IActivationFactory`,
/// which adds a reference.
pub fn dll_get_activation_factory(
    factory: &mut ThreadPoolFactory,
    class_id: Option<&[u16]>,
) -> HRESULT {
    if !class_id_is_thread_pool(class_id) {
        return CLASS_E_CLASSNOTAVAILABLE;
    }
    // The static factory accepts IActivationFactory, so this query cannot fail.
    let _ = factory.query_interface(Iid::ActivationFactory);
    S_OK
}

/// `WindowsGetStringRawBuffer` followed by `wcscmp`. The class id ends at its first NUL, and a NULL
/// `HSTRING` reads as the empty string.
fn class_id_is_thread_pool(class_id: Option<&[u16]>) -> bool {
    let raw = class_id.unwrap_or(&[]);
    let end = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
    THREAD_POOL_CLASS
        .encode_utf16()
        .eq(raw[..end].iter().copied())
}
