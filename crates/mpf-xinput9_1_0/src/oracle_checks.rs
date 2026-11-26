//! Oracle-derived test vectors for the ported xinput9_1_0 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `XInputGetCapabilities`,
//! `XInputGetDSoundAudioDeviceGuids`, `XInputGetState` and `XInputSetState` (`dlls/xinput9_1_0/main.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/xinput9_1_0/`. The harness fakes
//! xinput1_4 and prints 25 rows. ASan and UBSan reported nothing, so no vector is excluded.
//! Each row's comment gives the C harness row. `lib` is the number of times the loader ran.

use super::*;
use std::cell::Cell;

/// What the fake xinput1_4 backend recorded: the return value it gives, and the last call.
#[derive(Clone, Copy, Default)]
struct Backend {
    ret: u32,
    calls: u32,
    index: u32,
    flags: u32,
    last_null: bool,
}

thread_local! {
    static BACKEND: Cell<Backend> = Cell::new(Backend::default());
}

fn reset(ret: u32) {
    BACKEND.with(|b| {
        b.set(Backend {
            ret,
            ..Backend::default()
        })
    });
}

fn backend() -> Backend {
    BACKEND.with(Cell::get)
}

fn record(index: u32, flags: u32, last_null: bool) -> u32 {
    BACKEND.with(|b| {
        let mut s = b.get();
        s.calls += 1;
        s.index = index;
        s.flags = flags;
        s.last_null = last_null;
        b.set(s);
        s.ret
    })
}

/// `fake_caps` in the C harness: writes `Type` = 0x11 when the pointer is not NULL.
fn fake_caps(index: u32, flags: u32, caps: Option<&mut Capabilities>) -> u32 {
    let last_null = caps.is_none();
    if let Some(caps) = caps {
        caps.0[0] = 0x11;
    }
    record(index, flags, last_null)
}

/// `fake_state` in the C harness: writes `dwPacketNumber` = 42 when the pointer is not NULL.
fn fake_state(index: u32, state: Option<&mut State>) -> u32 {
    let last_null = state.is_none();
    if let Some(state) = state {
        state.0[..4].copy_from_slice(&42u32.to_le_bytes());
    }
    record(index, 0, last_null)
}

/// `fake_set` in the C harness: writes `wLeftMotorSpeed` = 7 when the pointer is not NULL.
fn fake_set(index: u32, vibration: Option<&mut Vibration>) -> u32 {
    let last_null = vibration.is_none();
    if let Some(vibration) = vibration {
        vibration.0[..2].copy_from_slice(&7u16.to_le_bytes());
    }
    record(index, 0, last_null)
}

fn funcs(caps: bool, state: bool, set: bool) -> Xinput14Funcs {
    Xinput14Funcs {
        get_capabilities: caps.then_some(fake_caps as GetCapabilitiesFn),
        get_state: state.then_some(fake_state as GetStateFn),
        set_state: set.then_some(fake_set as SetStateFn),
    }
}

/// A process whose loader counts its runs in `loads` and resolves the pointers `caps`, `state`
/// and `set`. `false` leaves a pointer NULL.
fn device<'a>(
    loads: &'a Cell<u32>,
    caps: bool,
    state: bool,
    set: bool,
) -> Xinput9<impl FnOnce() -> Xinput14Funcs + 'a> {
    Xinput9::new(move || {
        loads.set(loads.get() + 1);
        funcs(caps, state, set)
    })
}

#[test]
fn oracle_xinput9_rows() {
    // row 01 caps lib=fail -> ret=1167 lib=1 backend=0 out0=ee
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, false, false);
    reset(0);
    let mut caps = Capabilities([0xee; 20]);
    assert_eq!(dev.get_capabilities(0, 0, Some(&mut caps)), 1167);
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!(caps.0[0], 0xee);

    // row 02 caps no-fn -> ret=1167 lib=1 backend=0 out0=ee
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, true);
    reset(0);
    let mut caps = Capabilities([0xee; 20]);
    assert_eq!(dev.get_capabilities(0, 0, Some(&mut caps)), 1167);
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!(caps.0[0], 0xee);

    // row 03 caps ok idx0 flags0 -> ret=0 lib=1 backend=1 idx=0 flags=0 null=0 out0=11
    let loads = Cell::new(0);
    let mut dev = device(&loads, true, false, false);
    reset(0);
    let mut caps = Capabilities([0xee; 20]);
    assert_eq!(dev.get_capabilities(0, 0, Some(&mut caps)), 0);
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.flags, b.last_null),
        (1, 1, 0, 0, false)
    );
    assert_eq!(caps.0[0], 0x11);

    // row 04 caps backend-err idx3 flags1 -> ret=1167 lib=1 backend=1 idx=3 flags=1 null=0 out0=11
    let loads = Cell::new(0);
    let mut dev = device(&loads, true, false, false);
    reset(ERROR_DEVICE_NOT_CONNECTED);
    let mut caps = Capabilities([0xee; 20]);
    assert_eq!(dev.get_capabilities(3, 1, Some(&mut caps)), 1167);
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.flags, b.last_null),
        (1, 1, 3, 1, false)
    );
    assert_eq!(caps.0[0], 0x11);

    // row 05 caps idx4 null-out -> ret=0 lib=1 backend=1 idx=4 flags=2 null=1
    let loads = Cell::new(0);
    let mut dev = device(&loads, true, false, false);
    reset(0);
    assert_eq!(dev.get_capabilities(4, 2, None), 0);
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.flags, b.last_null),
        (1, 1, 4, 2, true)
    );

    // row 06 state lib=fail -> ret=1167 lib=1 backend=0 pkt=ee
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, false, false);
    reset(0);
    let mut state = State([0xee; 16]);
    assert_eq!(dev.get_state(0, Some(&mut state)), 1167);
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!(state.0, [0xee; 16]);

    // row 07 state no-fn -> ret=1167 lib=1 backend=0 pkt=ee
    // The C row removes the state pointer and the others too, so the Rust inputs match row 06.
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, false, false);
    reset(0);
    let mut state = State([0xee; 16]);
    assert_eq!(dev.get_state(0, Some(&mut state)), 1167);
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!(state.0, [0xee; 16]);

    // row 08 state ok idx2 -> ret=0 lib=1 backend=1 idx=2 null=0 pkt=42
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(0);
    let mut state = State([0xee; 16]);
    assert_eq!(dev.get_state(2, Some(&mut state)), 0);
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.last_null),
        (1, 1, 2, false)
    );
    assert_eq!(state.0[0], 42);

    // row 09 state backend-ret5 idx9 -> ret=5 lib=1 backend=1 idx=9 null=0 pkt=42
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(5);
    let mut state = State([0xee; 16]);
    assert_eq!(dev.get_state(9, Some(&mut state)), 5);
    let b = backend();
    assert_eq!((loads.get(), b.calls, b.index), (1, 1, 9));
    assert_eq!(state.0[0], 42);

    // row 10 state null-out -> ret=0 lib=1 backend=1 idx=1 null=1
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(0);
    assert_eq!(dev.get_state(1, None), 0);
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.last_null),
        (1, 1, 1, true)
    );

    // row 11 set lib=fail -> ret=1167 lib=1 backend=0 left=ee
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, false, false);
    reset(0);
    let mut vib = Vibration([0xee; 4]);
    assert_eq!(dev.set_state(0, Some(&mut vib)), 1167);
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!(vib.0, [0xee; 4]);

    // row 12 set no-fn -> ret=1167 lib=1 backend=0 left=ee
    // As in row 07, the C row has no pointers at all, so the Rust inputs match row 11.
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, false, false);
    reset(0);
    let mut vib = Vibration([0xee; 4]);
    assert_eq!(dev.set_state(0, Some(&mut vib)), 1167);
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!(vib.0, [0xee; 4]);

    // row 13 set ok idx1 -> ret=0 lib=1 backend=1 idx=1 null=0 left=7
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, false, true);
    reset(0);
    let mut vib = Vibration([0xee; 4]);
    assert_eq!(dev.set_state(1, Some(&mut vib)), 0);
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.last_null),
        (1, 1, 1, false)
    );
    assert_eq!(vib.0[0], 7);

    // row 14 set backend-err idx-1 -> ret=1167 lib=1 backend=1 idx=ffffffff null=0 left=7
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, false, true);
    reset(ERROR_DEVICE_NOT_CONNECTED);
    let mut vib = Vibration([0xee; 4]);
    assert_eq!(dev.set_state(u32::MAX, Some(&mut vib)), 1167);
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.last_null),
        (1, 1, u32::MAX, false)
    );
    assert_eq!(vib.0[0], 7);

    // row 15 set null-out -> ret=0 lib=1 backend=1 idx=0 null=1
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, false, true);
    reset(0);
    assert_eq!(dev.set_state(0, None), 0);
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.last_null),
        (1, 1, 0, true)
    );

    // row 16 dsound idx4 -> ret=160 lib=1 backend=0 render=aa capture=bb
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(0);
    let mut render = Guid([0xaa; 16]);
    let mut capture = Guid([0xbb; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(4, Some(&mut render), Some(&mut capture)),
        160
    );
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!((render, capture), (Guid([0xaa; 16]), Guid([0xbb; 16])));

    // row 17 dsound idx-1 -> ret=160 lib=1 backend=0 render=aa capture=bb
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(0);
    let mut render = Guid([0xaa; 16]);
    let mut capture = Guid([0xbb; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(u32::MAX, Some(&mut render), Some(&mut capture)),
        160
    );
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!((render, capture), (Guid([0xaa; 16]), Guid([0xbb; 16])));

    // row 18 dsound render-null -> ret=160 lib=1 backend=0 render=- capture=bb
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(0);
    let mut capture = Guid([0xbb; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(0, None, Some(&mut capture)),
        160
    );
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!(capture, Guid([0xbb; 16]));

    // row 19 dsound capture-null -> ret=160 lib=1 backend=0 render=aa capture=-
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(0);
    let mut render = Guid([0xaa; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(0, Some(&mut render), None),
        160
    );
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!(render, Guid([0xaa; 16]));

    // row 20 dsound lib=fail -> ret=1167 lib=1 backend=0 render=aa capture=bb
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, false, false);
    reset(0);
    let mut render = Guid([0xaa; 16]);
    let mut capture = Guid([0xbb; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(0, Some(&mut render), Some(&mut capture)),
        1167
    );
    assert_eq!((loads.get(), backend().calls), (1, 0));
    assert_eq!((render, capture), (Guid([0xaa; 16]), Guid([0xbb; 16])));

    // row 21 dsound ok idx3 -> ret=0 lib=1 backend=1 idx=3 null=0 render=00 capture=00
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(0);
    let mut render = Guid([0xaa; 16]);
    let mut capture = Guid([0xbb; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(3, Some(&mut render), Some(&mut capture)),
        0
    );
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.last_null),
        (1, 1, 3, false)
    );
    assert_eq!((render, capture), (GUID_NULL, GUID_NULL));

    // row 22 dsound state-err 1167 -> ret=1167 lib=1 backend=1 idx=0 null=0 render=aa capture=bb
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(ERROR_DEVICE_NOT_CONNECTED);
    let mut render = Guid([0xaa; 16]);
    let mut capture = Guid([0xbb; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(0, Some(&mut render), Some(&mut capture)),
        1167
    );
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.last_null),
        (1, 1, 0, false)
    );
    assert_eq!((render, capture), (Guid([0xaa; 16]), Guid([0xbb; 16])));

    // row 23 dsound state-err 5 -> ret=5 lib=1 backend=1 idx=1 null=0 render=aa capture=bb
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(5);
    let mut render = Guid([0xaa; 16]);
    let mut capture = Guid([0xbb; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(1, Some(&mut render), Some(&mut capture)),
        5
    );
    let b = backend();
    assert_eq!((loads.get(), b.calls, b.index), (1, 1, 1));
    assert_eq!((render, capture), (Guid([0xaa; 16]), Guid([0xbb; 16])));

    // row 24 state second-call -> ret=0 lib=1 backend=2 idx=1 null=0 pkt=42
    let loads = Cell::new(0);
    let mut dev = device(&loads, false, true, false);
    reset(0);
    let mut state = State([0xee; 16]);
    assert_eq!(dev.get_state(0, Some(&mut state)), 0);
    assert_eq!(dev.get_state(1, Some(&mut state)), 0);
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.last_null),
        (1, 2, 1, false)
    );
    assert_eq!(state.0[0], 42);

    // row 25 dsound ok idx2 all-fns -> ret=0 lib=1 backend=1 idx=2 null=0 render=00 capture=00
    let loads = Cell::new(0);
    let mut dev = device(&loads, true, true, true);
    reset(0);
    let mut render = Guid([0xaa; 16]);
    let mut capture = Guid([0xbb; 16]);
    assert_eq!(
        dev.get_dsound_audio_device_guids(2, Some(&mut render), Some(&mut capture)),
        0
    );
    let b = backend();
    assert_eq!(
        (loads.get(), b.calls, b.index, b.last_null),
        (1, 1, 2, false)
    );
    assert_eq!((render, capture), (GUID_NULL, GUID_NULL));
}
