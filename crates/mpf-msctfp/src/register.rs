/// `HRESULT` from `include/winerror.h`.
pub type Hresult = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: Hresult = 0;

/// What one `register_resource` call leaves in `info->result` (`struct reg_info` in
/// `libs/winecrt0/register.c`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallbackOutcome {
    /// The callback returned FALSE without storing a result, as on `if (!str) return FALSE;`.
    Untouched,
    /// The callback stored this `HRESULT` and returned `SUCCEEDED` of it.
    Set(Hresult),
}

/// The loop that `__wine_register_resources` and `__wine_unregister_resources` run through
/// `EnumResourceNamesW`. The two differ only in `do_register`, which the callback reads. The
/// result starts at `S_OK`, and enumeration stops at the first callback that returns FALSE.
pub fn enumerate_registry_resources(outcomes: &[CallbackOutcome]) -> Hresult {
    let mut result = S_OK;
    for outcome in outcomes {
        match *outcome {
            CallbackOutcome::Untouched => break,
            CallbackOutcome::Set(hr) => {
                result = hr;
                // A failed HRESULT makes the callback return FALSE.
                if hr < 0 {
                    break;
                }
            }
        }
    }
    result
}

/// `DllRegisterServer`, which returns `__wine_register_resources()`. msctfp has no `.rc` in the
/// reference, so the image has no REGISTRY resources and the callback never runs.
pub fn dll_register_server() -> Hresult {
    enumerate_registry_resources(&[])
}

/// `DllUnregisterServer`, which returns `__wine_unregister_resources()`. Same resource situation
/// as [`dll_register_server`].
pub fn dll_unregister_server() -> Hresult {
    enumerate_registry_resources(&[])
}
