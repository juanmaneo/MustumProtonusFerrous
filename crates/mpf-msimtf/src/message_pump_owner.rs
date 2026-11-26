//! The explicit methods of `ActiveIMMMessagePumpOwner` in `dlls/msimtf/activeimmapp.c`. Its
//! `QueryInterface`, `AddRef` and `Release` forward to the `ActiveIMMApp` object, so the port
//! models them with the [`crate::active_imm_app_query_interface`], [`crate::active_imm_app_add_ref`]
//! and [`crate::active_imm_app_release`] functions.

use crate::hresult::{Hresult, E_NOTIMPL};

/// `ActiveIMMMessagePumpOwner_Start`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_message_pump_owner_start() -> Hresult {
    E_NOTIMPL
}

/// `ActiveIMMMessagePumpOwner_End`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_message_pump_owner_end() -> Hresult {
    E_NOTIMPL
}

/// `ActiveIMMMessagePumpOwner_OnTranslateMessage`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_message_pump_owner_on_translate_message() -> Hresult {
    E_NOTIMPL
}

/// `ActiveIMMMessagePumpOwner_Pause`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_message_pump_owner_pause() -> Hresult {
    E_NOTIMPL
}

/// `ActiveIMMMessagePumpOwner_Resume`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_message_pump_owner_resume() -> Hresult {
    E_NOTIMPL
}
