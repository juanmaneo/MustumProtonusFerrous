//! Oracle-derived test vectors for the ported msttsengine logic.
//!
//! The vectors are the 25 rows printed by a harness that compiles the verbatim Wine bodies of
//! `dlls/msttsengine/main.c` (lines 37-123) and `dlls/msttsengine/tts.c` (lines 36-219), and the `libs/winecrt0`
//! defaults of `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`, with
//! `gcc -fsanitize=address,undefined -g`, under `target/canon/msttsengine/`. The shims there replace the Windows
//! headers, record the OS calls, and fail `malloc` on demand. No row was flagged by ASan or UBSan, so no vector
//! is excluded.

use super::*;
use std::cell::Cell;

const E_FAIL: Hresult = 0x8000_4005u32 as i32;

/// A token whose `AddRef` and `Release` update a count the test can read, as the harness's `fake_token` does.
#[derive(Clone, Copy)]
pub(crate) struct FakeToken<'a> {
    pub(crate) refs: &'a Cell<i32>,
}

impl ComPtr for FakeToken<'_> {
    fn add_ref(self) {
        self.refs.set(self.refs.get() + 1);
    }

    fn release(self) {
        self.refs.set(self.refs.get() - 1);
    }
}

/// Runs `dll_main` with `reason`. Returns its result and how many times the thread library call was made.
pub(crate) fn run_dll_main(reason: u32) -> (bool, i32) {
    let calls = Cell::new(0);
    let ret = dll_main(reason, || calls.set(calls.get() + 1));
    (ret, calls.get())
}

/// Row 01: `DllMain(DLL_PROCESS_ATTACH)` disables thread library calls once and returns `TRUE`.
#[test]
fn row_01_dll_main_attach() {
    assert_eq!(run_dll_main(DLL_PROCESS_ATTACH), (true, 1));
}

/// Row 02: `class_factory_QueryInterface(IUnknown)` gives `S_OK` and the factory.
#[test]
fn row_02_class_factory_query_interface_iunknown() {
    assert!(class_factory_query_interface(&IID_UNKNOWN).is_ok());
}

/// Row 03: `class_factory_QueryInterface(ISpTTSEngine)` gives `E_NOINTERFACE` and `*out` NULL.
#[test]
fn row_03_class_factory_query_interface_isptts_engine() {
    assert_eq!(
        class_factory_query_interface(&IID_SP_TTS_ENGINE).err(),
        Some(E_NOINTERFACE)
    );
}

/// Row 04: `class_factory_AddRef` returns 2.
#[test]
fn row_04_class_factory_add_ref() {
    assert_eq!(class_factory_add_ref(), 2);
}

/// Row 05: `class_factory_Release` returns 1.
#[test]
fn row_05_class_factory_release() {
    assert_eq!(class_factory_release(), 1);
}

/// Row 06: `class_factory_LockServer(TRUE)` gives `S_OK`.
#[test]
fn row_06_class_factory_lock_server() {
    assert_eq!(class_factory_lock_server(true), S_OK);
}

/// Row 07: `class_factory_CreateInstance` with a non-NULL outer gives `CLASS_E_NOAGGREGATION` and `*obj` NULL.
#[test]
fn row_07_class_factory_create_instance_outer_set() {
    assert_eq!(
        class_factory_create_instance::<FakeToken>(true, &IID_SP_TTS_ENGINE, true).err(),
        Some(CLASS_E_NOAGGREGATION)
    );
}

/// Row 08: `class_factory_CreateInstance` with a NULL outer gives `S_OK`, `ISpTTSEngine`, and `ref` 1.
#[test]
fn row_08_class_factory_create_instance_ttsengine() {
    let (engine, iface) =
        class_factory_create_instance::<FakeToken>(false, &IID_SP_TTS_ENGINE, true).unwrap();
    assert!(iface == Interface::SpTtsEngine);
    assert_eq!(engine.ref_count, 1);
}

/// Row 09: `DllGetClassObject(CLSID_TTSEngineCom, IClassFactory)` gives `S_OK` and the factory.
#[test]
fn row_09_dll_get_class_object_ttsengine_com() {
    assert!(dll_get_class_object(&CLSID_TTS_ENGINE_COM, &IID_CLASS_FACTORY).is_ok());
}

/// Row 10: `DllGetClassObject` for another CLSID gives `CLASS_E_CLASSNOTAVAILABLE`.
#[test]
fn row_10_dll_get_class_object_other_clsid() {
    assert_eq!(
        dll_get_class_object(&IID_CLASS_FACTORY, &IID_UNKNOWN).err(),
        Some(CLASS_E_CLASSNOTAVAILABLE)
    );
}

/// Row 11: `DllCanUnloadNow` gives `S_FALSE`.
#[test]
fn row_11_dll_can_unload_now() {
    assert_eq!(dll_can_unload_now(), S_FALSE);
}

/// Row 12: `DllRegisterServer` returns the result of `__wine_register_resources`. The harness's stub returns
/// `E_FAIL`.
#[test]
fn row_12_dll_register_server_passthrough() {
    assert_eq!(dll_register_server(|| E_FAIL), E_FAIL);
}

/// Row 13: `DllUnregisterServer` returns the result of `__wine_unregister_resources`. The harness's stub returns
/// `E_FAIL`.
#[test]
fn row_13_dll_unregister_server_passthrough() {
    assert_eq!(dll_unregister_server(|| E_FAIL), E_FAIL);
}

/// Row 14: `ttsengine_create(ISpObjectWithToken)` gives `S_OK`, `ISpObjectWithToken`, and `ref` 1.
#[test]
fn row_14_ttsengine_create_objwithtoken() {
    let (engine, iface) = ttsengine_create::<FakeToken>(&IID_SP_OBJECT_WITH_TOKEN, true).unwrap();
    assert!(iface == Interface::SpObjectWithToken);
    assert_eq!(engine.ref_count, 1);
}

/// Row 15: `ttsengine_create(IClassFactory)` gives `E_NOINTERFACE` and the object is freed.
#[test]
fn row_15_ttsengine_create_iclassfactory() {
    assert_eq!(
        ttsengine_create::<FakeToken>(&IID_CLASS_FACTORY, true).err(),
        Some(E_NOINTERFACE)
    );
}

/// Row 16: `class_factory_CreateInstance` with a failing `malloc` gives `E_OUTOFMEMORY` and `*obj` NULL.
#[test]
fn row_16_class_factory_create_instance_malloc_fails() {
    assert_eq!(
        class_factory_create_instance::<FakeToken>(false, &IID_SP_TTS_ENGINE, false).err(),
        Some(E_OUTOFMEMORY)
    );
}

/// Row 17: `ttsengine_Speak` gives `E_NOTIMPL`.
#[test]
fn row_17_ttsengine_speak() {
    assert_eq!(speak(), E_NOTIMPL);
}

/// Row 18: `ttsengine_GetOutputFormat` gives `E_NOTIMPL`.
#[test]
fn row_18_ttsengine_get_output_format() {
    assert_eq!(get_output_format(), E_NOTIMPL);
}

/// Row 19: `SetObjectToken(NULL)` gives `E_INVALIDARG`.
#[test]
fn row_19_objwithtoken_set_object_token_null() {
    let (mut engine, _) = ttsengine_create::<FakeToken>(&IID_SP_OBJECT_WITH_TOKEN, true).unwrap();
    assert_eq!(engine.set_object_token(None), E_INVALIDARG);
}

/// Row 20: `GetObjectToken(NULL)` gives `E_POINTER`.
#[test]
fn row_20_objwithtoken_get_object_token_null_out() {
    let (engine, _) = ttsengine_create::<FakeToken>(&IID_SP_OBJECT_WITH_TOKEN, true).unwrap();
    assert_eq!(engine.get_object_token(None), E_POINTER);
}

/// Row 21: `GetObjectToken` with no token set gives `S_FALSE` and `*out` NULL.
#[test]
fn row_21_objwithtoken_get_object_token_unset() {
    let (engine, _) = ttsengine_create::<FakeToken>(&IID_SP_OBJECT_WITH_TOKEN, true).unwrap();
    let mut out = None;
    assert_eq!(engine.get_object_token(Some(&mut out)), S_FALSE);
    assert!(out.is_none());
}

/// Rows 22 to 25 share one object, as they do in the harness. Row 22 is `SetObjectToken(A)`, row 23 is
/// `SetObjectToken(B)` after A is set, row 24 is `GetObjectToken` after A is set, and row 25 is the release that
/// takes the engine to zero.
#[test]
fn rows_22_to_25_object_token_lifecycle() {
    let refs_a = Cell::new(0);
    let refs_b = Cell::new(0);
    let token_a = FakeToken { refs: &refs_a };
    let token_b = FakeToken { refs: &refs_b };
    let (mut engine, _) = ttsengine_create::<FakeToken>(&IID_SP_OBJECT_WITH_TOKEN, true).unwrap();

    // Row 22: `SetObjectToken(A)` gives `S_OK`, and A has one reference.
    assert_eq!(engine.set_object_token(Some(token_a)), S_OK);
    assert_eq!(refs_a.get(), 1);

    // Row 23: `SetObjectToken(B)` gives `SPERR_ALREADY_INITIALIZED`, and B is not referenced.
    assert_eq!(
        engine.set_object_token(Some(token_b)),
        SPERR_ALREADY_INITIALIZED
    );
    assert_eq!(refs_b.get(), 0);

    // Row 24: `GetObjectToken` gives `S_OK` and A with a new reference, which the caller releases.
    let mut out = None;
    assert_eq!(engine.get_object_token(Some(&mut out)), S_OK);
    let out = out.unwrap();
    assert!(std::ptr::eq(out.refs, &refs_a));
    assert_eq!(refs_a.get(), 2);
    out.release();

    // Row 25: the release from one to zero returns 0 and releases A.
    assert_eq!(engine.release(), 0);
    assert_eq!(refs_a.get(), 0);
}
