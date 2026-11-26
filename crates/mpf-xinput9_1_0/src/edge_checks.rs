//! Edge-case tests for the ported xinput9_1_0 logic.
//!
//! These cover the check order in `XInputGetDSoundAudioDeviceGuids`, the index boundary, the
//! single load, the pointers resolved one at a time, and the export table. Not every expectation
//! is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use std::cell::Cell;

thread_local! {
    static HITS: Cell<u32> = const { Cell::new(0) };
}

fn hit() {
    HITS.with(|h| h.set(h.get() + 1));
}

fn hits() -> u32 {
    HITS.with(Cell::get)
}

fn caps_ok(_index: u32, _flags: u32, _caps: Option<&mut Capabilities>) -> u32 {
    hit();
    ERROR_SUCCESS
}

fn caps_big(_index: u32, _flags: u32, _caps: Option<&mut Capabilities>) -> u32 {
    hit();
    0xdead_beef
}

fn state_ok(_index: u32, _state: Option<&mut State>) -> u32 {
    hit();
    ERROR_SUCCESS
}

fn set_ok(_index: u32, _vibration: Option<&mut Vibration>) -> u32 {
    hit();
    ERROR_SUCCESS
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h and include/xinput.h. The oracle rows
    // use the same numbers.
    assert_eq!(ERROR_SUCCESS, 0);
    assert_eq!(ERROR_BAD_ARGUMENTS, 160);
    assert_eq!(ERROR_DEVICE_NOT_CONNECTED, 1167);
    assert_eq!(XUSER_MAX_COUNT, 4);
    assert_eq!(GUID_NULL, Guid([0; 16]));
}

#[test]
fn exports_match_the_spec() {
    // Not oracle-derived: the names in dlls/xinput9_1_0/xinput9_1_0.spec, in spec order.
    assert_eq!(
        EXPORTS,
        [
            "XInputGetCapabilities",
            "XInputGetDSoundAudioDeviceGuids",
            "XInputSetState",
            "XInputGetState",
        ]
    );
    for name in EXPORTS {
        assert!(is_export(name), "{name}");
    }
    // Not oracle-derived: names the spec does not export. Lookup is case-sensitive.
    for name in ["xinputgetstate", "XInputGetStateEx", "DllMain", ""] {
        assert!(!is_export(name), "{name}");
    }
}

#[test]
fn index_is_checked_before_pointers_and_before_the_backend() {
    // Not oracle-derived: the check order in main.c. An out-of-range index with no state pointer
    // and NULL GUID pointers must report ERROR_BAD_ARGUMENTS, and the backend must not run.
    HITS.with(|h| h.set(0));
    let mut dev = Xinput9::new(Xinput14Funcs::default);
    assert_eq!(
        dev.get_dsound_audio_device_guids(4, None, None),
        ERROR_BAD_ARGUMENTS
    );
    assert_eq!(hits(), 0);
}

#[test]
fn null_guid_pointers_fail_before_the_backend_even_when_the_index_is_valid() {
    // Not oracle-derived: both GUID pointers NULL, with a valid index and a working backend.
    HITS.with(|h| h.set(0));
    let mut dev = Xinput9::new(|| Xinput14Funcs {
        get_state: Some(state_ok),
        ..Default::default()
    });
    assert_eq!(
        dev.get_dsound_audio_device_guids(0, None, None),
        ERROR_BAD_ARGUMENTS
    );
    assert_eq!(hits(), 0);
}

#[test]
fn index_boundary_is_xuser_max_count() {
    // Not oracle-derived sweep: oracle rows 16, 17 and 21 cover indices 4, u32::MAX and 3 only.
    let mut dev = Xinput9::new(|| Xinput14Funcs {
        get_state: Some(state_ok),
        ..Default::default()
    });
    for index in 0..XUSER_MAX_COUNT {
        let mut render = Guid([1; 16]);
        let mut capture = Guid([1; 16]);
        assert_eq!(
            dev.get_dsound_audio_device_guids(index, Some(&mut render), Some(&mut capture)),
            ERROR_SUCCESS,
            "index {index}"
        );
        assert_eq!((render, capture), (GUID_NULL, GUID_NULL));
    }
    let mut render = Guid([1; 16]);
    let mut capture = Guid([1; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(XUSER_MAX_COUNT, Some(&mut render), Some(&mut capture)),
        ERROR_BAD_ARGUMENTS
    );
    assert_eq!((render, capture), (Guid([1; 16]), Guid([1; 16])));
}

#[test]
fn loader_runs_once_and_before_the_argument_checks() {
    // Not oracle-derived beyond rows 16 and 24: the loader count over a mixed sequence of calls.
    // `Xinput9::new` does not load; the first export call does, even when its arguments fail.
    let loads = Cell::new(0);
    let mut dev = Xinput9::new(|| {
        loads.set(loads.get() + 1);
        Xinput14Funcs {
            get_capabilities: Some(caps_ok),
            get_state: Some(state_ok),
            set_state: Some(set_ok),
        }
    });
    assert_eq!(loads.get(), 0);
    assert_eq!(
        dev.get_dsound_audio_device_guids(4, None, None),
        ERROR_BAD_ARGUMENTS
    );
    assert_eq!(loads.get(), 1);
    assert_eq!(dev.get_capabilities(0, 0, None), ERROR_SUCCESS);
    assert_eq!(dev.get_state(0, None), ERROR_SUCCESS);
    assert_eq!(dev.set_state(0, None), ERROR_SUCCESS);
    let mut render = Guid([0; 16]);
    let mut capture = Guid([0; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(0, Some(&mut render), Some(&mut capture)),
        ERROR_SUCCESS
    );
    assert_eq!(loads.get(), 1);
}

#[test]
fn each_pointer_is_forwarded_on_its_own() {
    // Not oracle-derived: oracle rows 02, 07 and 12 each remove a single pointer or all of them.
    // Here capabilities and vibration resolve while state does not. The dsound call needs
    // `XInputGetState`, so it fails without touching the GUIDs.
    let mut dev = Xinput9::new(|| Xinput14Funcs {
        get_capabilities: Some(caps_ok),
        get_state: None,
        set_state: Some(set_ok),
    });
    assert_eq!(dev.get_capabilities(0, 0, None), ERROR_SUCCESS);
    assert_eq!(dev.get_state(0, None), ERROR_DEVICE_NOT_CONNECTED);
    assert_eq!(dev.set_state(0, None), ERROR_SUCCESS);
    let mut render = Guid([1; 16]);
    let mut capture = Guid([1; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(0, Some(&mut render), Some(&mut capture)),
        ERROR_DEVICE_NOT_CONNECTED
    );
    assert_eq!((render, capture), (Guid([1; 16]), Guid([1; 16])));
}

#[test]
fn backend_return_value_is_passed_through() {
    // Not oracle-derived: an arbitrary backend return value on the capabilities forwarder. Oracle
    // rows 04 and 09 only pass through 1167 and 5.
    let mut dev = Xinput9::new(|| Xinput14Funcs {
        get_capabilities: Some(caps_big),
        ..Default::default()
    });
    assert_eq!(dev.get_capabilities(0, 0, None), 0xdead_beef);
}
