use crate::guid::Guid;
use crate::hresult::{query_hresult, Hresult, E_NOTIMPL};
use crate::interface::{interface_for_iid, Interface};

/// The static objects that `dlls/wpc/wpc.c` hands out through `ppv`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Object {
    ClassFactory,
    ParentalControls,
}

/// `WindowsParentalControls_QueryInterface`: `ppv` becomes the object on success and `None` on
/// failure. The C `IUnknown_AddRef` call on success is skipped because the count it returns is
/// constant and unused.
pub fn query_interface(riid: &Guid, ppv: &mut Option<Object>) -> Hresult {
    let matched = matches!(
        interface_for_iid(riid),
        Some(
            Interface::Unknown
                | Interface::WindowsParentalControlsCore
                | Interface::WindowsParentalControls
        )
    );
    *ppv = matched.then_some(Object::ParentalControls);
    query_hresult(matched)
}

/// `WindowsParentalControls_AddRef`: the count is always 2.
pub fn add_ref() -> u32 {
    2
}

/// `WindowsParentalControls_Release`: the count is always 1.
pub fn release() -> u32 {
    1
}

/// `WindowsParentalControls_GetVisibility`: a FIXME trace, then `E_NOTIMPL`.
pub fn get_visibility() -> Hresult {
    E_NOTIMPL
}

/// `WindowsParentalControls_GetUserSettings`: a FIXME trace, then `E_NOTIMPL`.
pub fn get_user_settings() -> Hresult {
    E_NOTIMPL
}

/// `WindowsParentalControls_GetWebSettings`: a FIXME trace, then `E_NOTIMPL`.
pub fn get_web_settings() -> Hresult {
    E_NOTIMPL
}

/// `WindowsParentalControls_GetWebFilterInfo`: a FIXME trace, then `E_NOTIMPL`.
pub fn get_web_filter_info() -> Hresult {
    E_NOTIMPL
}

/// `WindowsParentalControls_GetGamesSettings`: a FIXME trace, then `E_NOTIMPL`.
pub fn get_games_settings() -> Hresult {
    E_NOTIMPL
}
