//! `struct input_panel` and the `ITextInputPanel` methods from `dlls/tiptsf/tiptsf_main.c`.

use crate::com::{
    is_equal_guid, Guid, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, HRESULT, IID_TEXT_INPUT_PANEL,
    IID_UNKNOWN, S_OK,
};

/// `struct input_panel` without its interface pointer: the object's reference count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextInputPanel {
    /// `LONG ref`. Signed like the C field, so the wrapping arithmetic matches `InterlockedIncrement`.
    pub ref_count: i32,
}

/// `panelinput_CreateInstance`. `malloc_ok` says whether the C `malloc` succeeded.
pub fn panelinput_create_instance(malloc_ok: bool) -> Result<TextInputPanel, HRESULT> {
    if malloc_ok {
        Ok(TextInputPanel { ref_count: 1 })
    } else {
        Err(E_OUTOFMEMORY)
    }
}

impl TextInputPanel {
    /// `input_panel_QueryInterface`. A successful query adds a reference, as the C code's
    /// `IUnknown_AddRef` on the returned pointer does.
    pub fn query_interface(&mut self, riid: &Guid) -> HRESULT {
        if is_equal_guid(riid, &IID_UNKNOWN) || is_equal_guid(riid, &IID_TEXT_INPUT_PANEL) {
            self.add_ref();
            S_OK
        } else {
            E_NOINTERFACE
        }
    }

    /// `input_panel_AddRef`.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `input_panel_Release`. At zero the C code calls `free`; the caller drops the value instead.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        self.ref_count as u32
    }

    /// `input_panel_get_AttachedEditWindow`: returns `E_NOTIMPL`.
    pub fn get_attached_edit_window(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_put_AttachedEditWindow`: returns `E_NOTIMPL`.
    pub fn put_attached_edit_window(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_CurrentInteractionMode`: returns `E_NOTIMPL`.
    pub fn get_current_interaction_mode(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_DefaultInPlaceState`: returns `E_NOTIMPL`.
    pub fn get_default_in_place_state(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_put_DefaultInPlaceState`: returns `E_NOTIMPL`.
    pub fn put_default_in_place_state(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_CurrentInPlaceState`: returns `E_NOTIMPL`.
    pub fn get_current_in_place_state(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_DefaultInputArea`: returns `E_NOTIMPL`.
    pub fn get_default_input_area(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_put_DefaultInputArea`: returns `E_NOTIMPL`.
    pub fn put_default_input_area(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_CurrentInputArea`: returns `E_NOTIMPL`.
    pub fn get_current_input_area(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_CurrentCorrectionMode`: returns `E_NOTIMPL`.
    pub fn get_current_correction_mode(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_PreferredInPlaceDirection`: returns `E_NOTIMPL`.
    pub fn get_preferred_in_place_direction(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_put_PreferredInPlaceDirection`: returns `E_NOTIMPL`.
    pub fn put_preferred_in_place_direction(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_ExpandPostInsertionCorrection`: returns `E_NOTIMPL`.
    pub fn get_expand_post_insertion_correction(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_put_ExpandPostInsertionCorrection`: returns `E_NOTIMPL`.
    pub fn put_expand_post_insertion_correction(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_InPlaceVisibleOnFocus`: returns `E_NOTIMPL`.
    pub fn get_in_place_visible_on_focus(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_put_InPlaceVisibleOnFocus`: returns `E_NOTIMPL`.
    pub fn put_in_place_visible_on_focus(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_InPlaceBoundingRectangle`: returns `E_NOTIMPL`.
    pub fn get_in_place_bounding_rectangle(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_PopUpCorrectionHeight`: returns `E_NOTIMPL`.
    pub fn get_pop_up_correction_height(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_get_PopDownCorrectionHeight`: returns `E_NOTIMPL`.
    pub fn get_pop_down_correction_height(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_CommitPendingInput`: returns `E_NOTIMPL`.
    pub fn commit_pending_input(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_SetInPlaceVisibility`: returns `E_NOTIMPL`.
    pub fn set_in_place_visibility(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_SetInPlacePosition`: returns `E_NOTIMPL`.
    pub fn set_in_place_position(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_SetInPlaceHoverTargetPosition`: returns `E_NOTIMPL`.
    pub fn set_in_place_hover_target_position(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_Advise`: returns `E_NOTIMPL`.
    pub fn advise(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `input_panel_Unadvise`: returns `E_NOTIMPL`.
    pub fn unadvise(&self) -> HRESULT {
        E_NOTIMPL
    }
}
