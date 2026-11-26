//! Oracle-derived test vectors for the ported tiptsf logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/tiptsf/tiptsf_main.c`
//! (lines 35 to 417) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`, lines 504 to
//! 516) with `gcc -fsanitize=address,undefined -g` under `target/canon/tiptsf/`, and running the
//! harness. The 52 printed rows all ran clean: no sanitizer report and no leak, so no vector is
//! excluded.
//!
//! The C out-pointers are not modelled. Where a row prints `out=...`, the Rust assertion checks
//! only the return value and the reference count, which are the parts the port models.

use super::*;

/// A panel as `panelinput_CreateInstance` leaves it: reference count 1.
fn panel() -> TextInputPanel {
    TextInputPanel { ref_count: 1 }
}

#[test]
fn oracle_create_instance_rows() {
    // row 01 CreateInstance malloc=ok -> ret=0x00000000 ref=1 ppv_set=1
    assert_eq!(
        panelinput_create_instance(true),
        Ok(TextInputPanel { ref_count: 1 })
    );
    // row 02 CreateInstance malloc=fail -> ret=0x8007000e ppv_set=0
    assert_eq!(panelinput_create_instance(false), Err(E_OUTOFMEMORY));
}

#[test]
fn oracle_panel_query_interface_rows() {
    // row 03 QueryInterface IID_IUnknown -> ret=0x00000000 out=panel ref=2
    let mut p = panel();
    assert_eq!(p.query_interface(&IID_UNKNOWN), S_OK);
    assert_eq!(p.ref_count, 2);

    // row 04 QueryInterface IID_ITextInputPanel -> ret=0x00000000 out=panel ref=2
    let mut p = panel();
    assert_eq!(p.query_interface(&IID_TEXT_INPUT_PANEL), S_OK);
    assert_eq!(p.ref_count, 2);

    // row 05 QueryInterface IID_IClassFactory -> ret=0x80004002 out=NULL ref=1
    let mut p = panel();
    assert_eq!(p.query_interface(&IID_CLASS_FACTORY), E_NOINTERFACE);
    assert_eq!(p.ref_count, 1);

    // row 06 QueryInterface CLSID_TextInputPanel -> ret=0x80004002 out=NULL ref=1
    let mut p = panel();
    assert_eq!(p.query_interface(&CLSID_TEXT_INPUT_PANEL), E_NOINTERFACE);
    assert_eq!(p.ref_count, 1);
}

#[test]
fn oracle_panel_reference_count_rows() {
    // row 07 AddRef from=1 -> ret=2
    assert_eq!(panel().add_ref(), 2);

    // row 08 Release from=2 -> ret=1
    let mut p = TextInputPanel { ref_count: 2 };
    assert_eq!(p.release(), 1);

    // row 09 Release from=1 -> ret=0 freed
    assert_eq!(panel().release(), 0);
}

#[test]
fn oracle_panel_not_implemented_rows() {
    // row 10 get_AttachedEditWindow -> ret=0x80004001
    assert_eq!(panel().get_attached_edit_window(), E_NOTIMPL);
    // row 11 put_AttachedEditWindow -> ret=0x80004001
    assert_eq!(panel().put_attached_edit_window(), E_NOTIMPL);
    // row 12 get_CurrentInteractionMode -> ret=0x80004001
    assert_eq!(panel().get_current_interaction_mode(), E_NOTIMPL);
    // row 13 get_DefaultInPlaceState -> ret=0x80004001
    assert_eq!(panel().get_default_in_place_state(), E_NOTIMPL);
    // row 14 put_DefaultInPlaceState -> ret=0x80004001
    assert_eq!(panel().put_default_in_place_state(), E_NOTIMPL);
    // row 15 get_CurrentInPlaceState -> ret=0x80004001
    assert_eq!(panel().get_current_in_place_state(), E_NOTIMPL);
    // row 16 get_DefaultInputArea -> ret=0x80004001
    assert_eq!(panel().get_default_input_area(), E_NOTIMPL);
    // row 17 put_DefaultInputArea -> ret=0x80004001
    assert_eq!(panel().put_default_input_area(), E_NOTIMPL);
    // row 18 get_CurrentInputArea -> ret=0x80004001
    assert_eq!(panel().get_current_input_area(), E_NOTIMPL);
    // row 19 get_CurrentCorrectionMode -> ret=0x80004001
    assert_eq!(panel().get_current_correction_mode(), E_NOTIMPL);
    // row 20 get_PreferredInPlaceDirection -> ret=0x80004001
    assert_eq!(panel().get_preferred_in_place_direction(), E_NOTIMPL);
    // row 21 put_PreferredInPlaceDirection -> ret=0x80004001
    assert_eq!(panel().put_preferred_in_place_direction(), E_NOTIMPL);
    // row 22 get_ExpandPostInsertionCorrection -> ret=0x80004001
    assert_eq!(panel().get_expand_post_insertion_correction(), E_NOTIMPL);
    // row 23 put_ExpandPostInsertionCorrection -> ret=0x80004001
    assert_eq!(panel().put_expand_post_insertion_correction(), E_NOTIMPL);
    // row 24 get_InPlaceVisibleOnFocus -> ret=0x80004001
    assert_eq!(panel().get_in_place_visible_on_focus(), E_NOTIMPL);
    // row 25 put_InPlaceVisibleOnFocus -> ret=0x80004001
    assert_eq!(panel().put_in_place_visible_on_focus(), E_NOTIMPL);
    // row 26 get_InPlaceBoundingRectangle -> ret=0x80004001
    assert_eq!(panel().get_in_place_bounding_rectangle(), E_NOTIMPL);
    // row 27 get_PopUpCorrectionHeight -> ret=0x80004001
    assert_eq!(panel().get_pop_up_correction_height(), E_NOTIMPL);
    // row 28 get_PopDownCorrectionHeight -> ret=0x80004001
    assert_eq!(panel().get_pop_down_correction_height(), E_NOTIMPL);
    // row 29 CommitPendingInput -> ret=0x80004001
    assert_eq!(panel().commit_pending_input(), E_NOTIMPL);
    // row 30 SetInPlaceVisibility -> ret=0x80004001
    assert_eq!(panel().set_in_place_visibility(), E_NOTIMPL);
    // row 31 SetInPlacePosition -> ret=0x80004001
    assert_eq!(panel().set_in_place_position(), E_NOTIMPL);
    // row 32 SetInPlaceHoverTargetPosition -> ret=0x80004001
    assert_eq!(panel().set_in_place_hover_target_position(), E_NOTIMPL);
    // row 33 Advise -> ret=0x80004001
    assert_eq!(panel().advise(), E_NOTIMPL);
    // row 34 Unadvise -> ret=0x80004001
    assert_eq!(panel().unadvise(), E_NOTIMPL);
}

#[test]
fn oracle_class_factory_rows() {
    // row 35 ClassFactory_QueryInterface IID_IUnknown -> ret=0x00000000 out_is_factory=1
    assert_eq!(class_factory_query_interface(&IID_UNKNOWN), S_OK);
    // row 36 ClassFactory_QueryInterface IID_IClassFactory -> ret=0x00000000 out_is_factory=1
    assert_eq!(class_factory_query_interface(&IID_CLASS_FACTORY), S_OK);
    // row 37 ClassFactory_QueryInterface IID_ITextInputPanel -> ret=0x80004002 out=NULL
    assert_eq!(
        class_factory_query_interface(&IID_TEXT_INPUT_PANEL),
        E_NOINTERFACE
    );
    // row 38 ClassFactory_AddRef -> ret=2
    assert_eq!(class_factory_add_ref(), 2);
    // row 39 ClassFactory_Release -> ret=1
    assert_eq!(class_factory_release(), 1);
    // row 40 ClassFactory_LockServer TRUE -> ret=0x00000000
    assert_eq!(class_factory_lock_server(true), S_OK);
    // row 41 ClassFactory_LockServer FALSE -> ret=0x00000000
    assert_eq!(class_factory_lock_server(false), S_OK);
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 42 DllGetClassObject CLSID_TextInputPanel IID_IClassFactory -> ret=0x00000000 out_is_factory=1
    assert_eq!(
        dll_get_class_object(&CLSID_TEXT_INPUT_PANEL, &IID_CLASS_FACTORY),
        S_OK
    );
    // row 43 DllGetClassObject CLSID_TextInputPanel IID_IUnknown -> ret=0x00000000 out_is_factory=1
    assert_eq!(
        dll_get_class_object(&CLSID_TEXT_INPUT_PANEL, &IID_UNKNOWN),
        S_OK
    );
    // row 44 DllGetClassObject CLSID_TextInputPanel IID_ITextInputPanel -> ret=0x80004002 out=NULL
    assert_eq!(
        dll_get_class_object(&CLSID_TEXT_INPUT_PANEL, &IID_TEXT_INPUT_PANEL),
        E_NOINTERFACE
    );
    // row 45 DllGetClassObject other_clsid -> ret=0x80040111 out=untouched
    assert_eq!(
        dll_get_class_object(&IID_CLASS_FACTORY, &IID_CLASS_FACTORY),
        CLASS_E_CLASSNOTAVAILABLE
    );
}

#[test]
fn oracle_stub_rows() {
    // row 46 AdviseHook -> code=0x80000100 flags=0x1 nparams=2 p0=tiptsf.dll p1=AdviseHook raises=1
    assert_eq!(
        advise_hook(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "tiptsf.dll",
            function: "AdviseHook",
        }
    );
    // row 47 EndCaretTracking -> code=0x80000100 flags=0x1 nparams=2 p0=tiptsf.dll p1=EndCaretTracking raises=1
    assert_eq!(
        end_caret_tracking(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "tiptsf.dll",
            function: "EndCaretTracking",
        }
    );
    // row 48 ProcessCaretEvents -> code=0x80000100 flags=0x1 nparams=2 p0=tiptsf.dll p1=ProcessCaretEvents raises=1
    assert_eq!(
        process_caret_events(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "tiptsf.dll",
            function: "ProcessCaretEvents",
        }
    );
    // row 49 ProcessCiceroCaretEvent -> code=0x80000100 flags=0x1 nparams=2 p0=tiptsf.dll p1=ProcessCiceroCaretEvent raises=1
    assert_eq!(
        process_cicero_caret_event(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "tiptsf.dll",
            function: "ProcessCiceroCaretEvent",
        }
    );
    // row 50 RestrictedModeMsgWndProc -> code=0x80000100 flags=0x1 nparams=2 p0=tiptsf.dll p1=RestrictedModeMsgWndProc raises=1
    assert_eq!(
        restricted_mode_msg_wnd_proc(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "tiptsf.dll",
            function: "RestrictedModeMsgWndProc",
        }
    );
    // row 51 StartCaretTracking -> code=0x80000100 flags=0x1 nparams=2 p0=tiptsf.dll p1=StartCaretTracking raises=1
    assert_eq!(
        start_caret_tracking(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "tiptsf.dll",
            function: "StartCaretTracking",
        }
    );
    // row 52 UnadviseHook -> code=0x80000100 flags=0x1 nparams=2 p0=tiptsf.dll p1=UnadviseHook raises=1
    assert_eq!(
        unadvise_hook(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "tiptsf.dll",
            function: "UnadviseHook",
        }
    );
}
