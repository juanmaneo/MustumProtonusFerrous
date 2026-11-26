//! The `ActiveIMMApp` object of `dlls/msimtf/activeimmapp.c`: its reference count, `QueryInterface`,
//! constructor, and the explicit functions that return a fixed value.
//!
//! The explicit functions ignore their arguments, because the C bodies never read them.

use crate::guid::{
    is_equal_guid, Guid, IID_ACTIVE_IMM_APP, IID_ACTIVE_IMM_MESSAGE_PUMP_OWNER, IID_UNKNOWN,
};
use crate::hresult::{
    Hresult, CLASS_E_NOAGGREGATION, E_FAIL, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, S_OK,
};

/// The `ActiveIMMApp` object. `ref_count` is the C struct's `LONG refCount`. The two interface
/// members of the C struct are not modelled, see [`ActiveImmInterface`].
#[derive(Debug, PartialEq, Eq)]
pub struct ActiveImmApp {
    pub ref_count: i32,
}

/// The interface pointer that [`active_imm_app_query_interface`] returns. The C code returns the
/// address of one of the two interface members of the object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveImmInterface {
    /// `IActiveIMMApp`, the `IActiveIMMApp_iface` member.
    ActiveImmApp,
    /// `IActiveIMMMessagePumpOwner`, the `IActiveIMMMessagePumpOwner_iface` member.
    MessagePumpOwner,
}

/// `ActiveIMMApp_Constructor`. A non-NULL `pUnkOuter` gives `CLASS_E_NOAGGREGATION`, and that is
/// checked before the allocation. `alloc_succeeds` is the result of `HeapAlloc`, and a failure gives
/// `E_OUTOFMEMORY`. On success the reference count is 1.
pub fn active_imm_app_constructor(
    has_outer: bool,
    alloc_succeeds: bool,
) -> Result<ActiveImmApp, Hresult> {
    if has_outer {
        return Err(CLASS_E_NOAGGREGATION);
    }
    if !alloc_succeeds {
        return Err(E_OUTOFMEMORY);
    }
    Ok(ActiveImmApp { ref_count: 1 })
}

/// `ActiveIMMApp_QueryInterface`. `IID_IUnknown` and `IID_IActiveIMMApp` select the `IActiveIMMApp`
/// pointer, and `IID_IActiveIMMMessagePumpOwner` selects the pump pointer. A successful query adds a
/// reference, as the C code's `IUnknown_AddRef` does. Any other IID gives `E_NOINTERFACE`, and no
/// reference is added.
pub fn active_imm_app_query_interface(
    this: &mut ActiveImmApp,
    iid: &Guid,
) -> Result<ActiveImmInterface, Hresult> {
    let interface = if is_equal_guid(iid, &IID_UNKNOWN) || is_equal_guid(iid, &IID_ACTIVE_IMM_APP) {
        ActiveImmInterface::ActiveImmApp
    } else if is_equal_guid(iid, &IID_ACTIVE_IMM_MESSAGE_PUMP_OWNER) {
        ActiveImmInterface::MessagePumpOwner
    } else {
        return Err(E_NOINTERFACE);
    };
    active_imm_app_add_ref(this);
    Ok(interface)
}

/// `ActiveIMMApp_AddRef`: `InterlockedIncrement`. The count wraps on overflow, as the Interlocked
/// functions do.
pub fn active_imm_app_add_ref(this: &mut ActiveImmApp) -> u32 {
    this.ref_count = this.ref_count.wrapping_add(1);
    this.ref_count as u32
}

/// `ActiveIMMApp_Release`: `InterlockedDecrement`. At zero the C code calls
/// `ActiveIMMApp_Destructor`, which frees the object. The port frees nothing. The owner drops the
/// value when the returned count is 0.
pub fn active_imm_app_release(this: &mut ActiveImmApp) -> u32 {
    this.ref_count = this.ref_count.wrapping_sub(1);
    this.ref_count as u32
}

/// `ActiveIMMApp_EnumRegisterWordA`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_app_enum_register_word_a() -> Hresult {
    E_NOTIMPL
}

/// `ActiveIMMApp_EnumRegisterWordW`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_app_enum_register_word_w() -> Hresult {
    E_NOTIMPL
}

/// `ActiveIMMApp_Activate`: a stub that returns `S_OK`.
pub fn active_imm_app_activate() -> Hresult {
    S_OK
}

/// `ActiveIMMApp_Deactivate`: a stub that returns `S_OK`.
pub fn active_imm_app_deactivate() -> Hresult {
    S_OK
}

/// `ActiveIMMApp_OnDefWindowProc`: a stub that returns `E_FAIL`.
pub fn active_imm_app_on_def_window_proc() -> Hresult {
    E_FAIL
}

/// `ActiveIMMApp_FilterClientWindows`: a stub that returns `S_OK`.
pub fn active_imm_app_filter_client_windows() -> Hresult {
    S_OK
}

/// `ActiveIMMApp_GetCodePageA`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_app_get_code_page_a() -> Hresult {
    E_NOTIMPL
}

/// `ActiveIMMApp_GetLangId`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_app_get_lang_id() -> Hresult {
    E_NOTIMPL
}

/// `ActiveIMMApp_EnumInputContext`: a stub that returns `E_NOTIMPL`.
pub fn active_imm_app_enum_input_context() -> Hresult {
    E_NOTIMPL
}
