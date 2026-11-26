//! Oracle-derived test vectors for the ported msimtf logic.
//!
//! The vectors are the 25 rows printed by a harness under `target/canon/msimtf/harness.c`. It compiles
//! the verbatim Wine bodies of `dlls/msimtf/activeimmapp.c` and `dlls/msimtf/main.c`, the
//! `DllCanUnloadNow` default in `libs/winecrt0/dll_canunload.c`, and `__wine_spec_unimplemented_stub`
//! from `dlls/ntdll/exception.c`, with `gcc -fsanitize=address,undefined -g`. The shims there replace
//! the Windows headers. No row was flagged by ASan or UBSan, so no vector is excluded.

use super::*;

/// Rows 01 to 09: the `ActiveIMMApp` explicit functions.
const APP_ROWS: [(&str, u32); 9] = [
    ("ActiveIMMApp_EnumRegisterWordA", 0x8000_4001),
    ("ActiveIMMApp_EnumRegisterWordW", 0x8000_4001),
    ("ActiveIMMApp_Activate", 0x0000_0000),
    ("ActiveIMMApp_Deactivate", 0x0000_0000),
    ("ActiveIMMApp_OnDefWindowProc", 0x8000_4005),
    ("ActiveIMMApp_FilterClientWindows", 0x0000_0000),
    ("ActiveIMMApp_GetCodePageA", 0x8000_4001),
    ("ActiveIMMApp_GetLangId", 0x8000_4001),
    ("ActiveIMMApp_EnumInputContext", 0x8000_4001),
];

/// Rows 10 to 14: the `ActiveIMMMessagePumpOwner` explicit methods.
const PUMP_ROWS: [(&str, u32); 5] = [
    ("ActiveIMMMessagePumpOwner_Start", 0x8000_4001),
    ("ActiveIMMMessagePumpOwner_End", 0x8000_4001),
    ("ActiveIMMMessagePumpOwner_OnTranslateMessage", 0x8000_4001),
    ("ActiveIMMMessagePumpOwner_Pause", 0x8000_4001),
    ("ActiveIMMMessagePumpOwner_Resume", 0x8000_4001),
];

/// Compares each actual `HRESULT` with the value the harness printed for the same row.
fn assert_hr_rows(rows: &[(&str, u32)], actual: &[Hresult]) {
    assert_eq!(rows.len(), actual.len());
    for ((name, expected), actual) in rows.iter().zip(actual) {
        assert_eq!(*actual as u32, *expected, "{name}");
    }
}

/// The `HRESULT` of a `Result` in the C style: `Ok` is `S_OK`.
fn hresult_of<T>(result: Result<T, Hresult>) -> Hresult {
    match result {
        Ok(_) => S_OK,
        Err(hr) => hr,
    }
}

#[test]
fn rows_01_to_09_active_imm_app_explicit_functions() {
    assert_hr_rows(
        &APP_ROWS,
        &[
            active_imm_app_enum_register_word_a(),
            active_imm_app_enum_register_word_w(),
            active_imm_app_activate(),
            active_imm_app_deactivate(),
            active_imm_app_on_def_window_proc(),
            active_imm_app_filter_client_windows(),
            active_imm_app_get_code_page_a(),
            active_imm_app_get_lang_id(),
            active_imm_app_enum_input_context(),
        ],
    );
}

#[test]
fn rows_10_to_14_active_imm_message_pump_owner_methods() {
    assert_hr_rows(
        &PUMP_ROWS,
        &[
            active_imm_message_pump_owner_start(),
            active_imm_message_pump_owner_end(),
            active_imm_message_pump_owner_on_translate_message(),
            active_imm_message_pump_owner_pause(),
            active_imm_message_pump_owner_resume(),
        ],
    );
}

#[test]
fn rows_15_and_16_dll_get_class_object() {
    assert_eq!(
        hresult_of(dll_get_class_object(&CLSID_CACTIVE_IMM, &IID_CLASS_FACTORY)) as u32,
        0x0000_0000
    );
    assert_eq!(
        hresult_of(dll_get_class_object(&IID_UNKNOWN, &IID_CLASS_FACTORY)) as u32,
        0x8004_0111
    );
}

#[test]
fn rows_17_to_19_class_factory_methods() {
    assert_eq!(class_factory_add_ref(), 2);
    assert_eq!(class_factory_release(), 1);
    assert_eq!(class_factory_lock_server(true) as u32, 0x0000_0000);
}

#[test]
fn row_20_dll_can_unload_now() {
    assert_eq!(dll_can_unload_now() as u32, 0x0000_0001);
}

#[test]
fn rows_21_and_22_stub_exceptions() {
    assert_eq!(
        msimtf_is_guid_map_enable(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "msimtf.dll",
            function: "MsimtfIsGuidMapEnable",
        }
    );
    assert_eq!(
        msimtf_is_window_filtered(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "msimtf.dll",
            function: "MsimtfIsWindowFiltered",
        }
    );
}

#[test]
fn rows_23_to_25_constructor_and_create_instance() {
    assert_eq!(
        active_imm_app_constructor(true, true).unwrap_err() as u32,
        0x8004_0110
    );
    assert_eq!(
        active_imm_app_constructor(false, true),
        Ok(ActiveImmApp { ref_count: 1 })
    );
    let (obj, interface) = class_factory_create_instance(&IID_ACTIVE_IMM_APP, false, true).unwrap();
    assert_eq!(obj.ref_count, 1);
    assert_eq!(interface, ActiveImmInterface::ActiveImmApp);
}
