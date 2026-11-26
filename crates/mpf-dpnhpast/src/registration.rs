//! `DllRegisterServer` and `DllUnregisterServer`, which wrap
//! `__wine_register_resources` and `__wine_unregister_resources` in `libs/winecrt0/register.c`.

use crate::hresult::{succeeded, Hresult, E_OUTOFMEMORY, S_OK};

/// The outcome of one `WINE_REGISTRY` resource inside the `EnumResourceNamesW` callback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceStep {
    /// `LoadResource` returned NULL. The callback returns FALSE and the result is unchanged.
    NotLoaded,
    /// Creating the ATL registrar failed with this HRESULT. Enumeration stops with it.
    RegistrarFailed(Hresult),
    /// The UTF-16 buffer from `HeapAlloc` was NULL. Enumeration stops with `E_OUTOFMEMORY`.
    OutOfMemory,
    /// `IRegistrar_StringRegister` or `IRegistrar_StringUnregister` returned this HRESULT.
    /// Enumeration continues only on success.
    Registered(Hresult),
}

/// `DllRegisterServer`: the result of `__wine_register_resources` for resources
/// in enumeration order. With no resources the result is `S_OK`.
pub fn dll_register_server(resources: &[ResourceStep]) -> Hresult {
    registration_result(resources)
}

/// `DllUnregisterServer`: the same loop as registration. Only the registrar call differs,
/// and that choice is already encoded in each [`ResourceStep::Registered`] value.
pub fn dll_unregister_server(resources: &[ResourceStep]) -> Hresult {
    registration_result(resources)
}

fn registration_result(resources: &[ResourceStep]) -> Hresult {
    let mut result = S_OK;
    for step in resources {
        let (hr, keep_going) = match *step {
            ResourceStep::NotLoaded => break,
            ResourceStep::RegistrarFailed(hr) => (hr, false),
            ResourceStep::OutOfMemory => (E_OUTOFMEMORY, false),
            ResourceStep::Registered(hr) => (hr, succeeded(hr)),
        };
        result = hr;
        if !keep_going {
            break;
        }
    }
    result
}
