//! Oracle-derived test vectors for the ported winmm logic.
//!
//! The vectors come from the harness in `target/canon/winmm/`, which compiles the verbatim Wine C
//! bodies with `gcc -fsanitize=address,undefined -g` and prints 144 vectors: 141 data rows, the
//! timer wrap row (a 4095-pair loop that [`oracle_timer_wrap_row`] replays), and the two header
//! values `JOY_NUM` 16 and `VERSION` 778. ASan and UBSan reported nothing, so no vector is excluded.
//! The `CP_ACP` shim is ASCII, so the A-path rows use ASCII text only.

use super::*;
use crate::error::get_error_text_a;

/// One call on the timer table, as the harness records it. `trigger` and `stored_flags` are what
/// the table holds after a successful set.
enum Op {
    Set {
        delay: u32,
        resol: u32,
        flags: u32,
        now: u32,
        id: u32,
        trigger: u32,
        stored_flags: u16,
    },
    Kill {
        id: u32,
        ret: u32,
    },
}

const MIDI_W_ROWS: &[(u32, u32, bool, u32, usize, &str)] = &[
    (0, 0, false, 0, 0, ""),
    (0, 1, false, 9, 1, ""),
    (0, 200, false, 0, 39, "The specified command was carried out."),
    (1, 200, false, 0, 26, "Undefined external error."),
    (6, 200, false, 0, 45, "There is no driver installed on your system!"),
    (12, 200, false, 9, 1, ""),
    (21, 200, false, 9, 1, ""),
    (22, 200, false, 9, 0, ""),
    (32, 200, false, 9, 0, ""),
    (64, 200, false, 0, 102, "The MIDI header was not prepared. Use the Prepare function to prepare the header, and then try again."),
    (69, 200, false, 0, 123, "The current MIDI setup is damaged. Copy the original MIDIMAP.CFG file to the Windows SYSTEM directory, and then try again."),
    (70, 200, false, 9, 1, ""),
    (71, 200, false, 9, 1, ""),
    (72, 200, false, 9, 0, ""),
    (4294967295, 200, false, 9, 0, ""),
    (64, 5, false, 0, 5, "The "),
    (64, 1, false, 9, 1, ""),
    (64, 0, false, 0, 0, ""),
    (0, 200, true, 11, 0, ""),
    (64, 200, true, 11, 0, ""),
];

const WAVE_W_ROWS: &[(u32, u32, bool, u32, usize, &str)] = &[
    (32, 200, false, 0, 129, "The specified format is not supported or cannot be translated. Use the Capabilities function to determine the supported formats."),
    (33, 200, false, 0, 127, "Cannot perform this operation while media data is still playing. Reset the device, or wait until the data is finished playing."),
    (34, 4, false, 0, 4, "The"),
    (35, 200, false, 0, 96, "Cannot open the device without using the WAVE_ALLOWSYNC flag. Use the flag, and then try again."),
    (36, 200, false, 9, 0, ""),
    (64, 200, false, 9, 0, ""),
    (1, 200, false, 0, 26, "Undefined external error."),
    (12, 200, false, 9, 1, ""),
];

const MIDI_A_ROWS: &[(u32, u32, bool, bool, u32, usize, &str)] = &[
    (0, 0, false, false, 0, 0, ""),
    (0, 0, false, true, 0, 0, ""),
    (0, 200, false, false, 0, 39, "The specified command was carried out."),
    (0, 200, false, true, 7, 0, ""),
    (64, 200, false, false, 0, 102, "The MIDI header was not prepared. Use the Prepare function to prepare the header, and then try again."),
    (64, 200, true, false, 11, 0, ""),
    (64, 5, false, false, 0, 5, "The "),
    (0, 1, false, false, 9, 0, ""),
    (12, 200, false, false, 9, 0, ""),
    (71, 200, false, false, 9, 0, ""),
];

const WAVE_A_ROWS: &[(u32, u32, bool, bool, u32, usize, &str)] = &[
    (35, 200, false, false, 0, 96, "Cannot open the device without using the WAVE_ALLOWSYNC flag. Use the flag, and then try again."),
    (33, 200, false, true, 7, 0, ""),
    (64, 200, false, false, 9, 0, ""),
    (34, 4, false, false, 0, 4, "The"),
    (1, 200, false, false, 0, 26, "Undefined external error."),
];

const CALLBACK_ROWS: &[(usize, u32, bool, bool, u32, u32)] = &[
    (0x0, 0x0, false, false, 0, 0),
    (0x0, 0x0, true, false, 0, 0),
    (0x1234, 0x0, false, false, 0, 0),
    (0x1234, 0x10000, false, true, 0, 1),
    (0x1234, 0x10000, false, false, 11, 1),
    (0x0, 0x10000, false, false, 0, 0),
    (0x1234, 0x30000, false, false, 0, 0),
    (0x1234, 0x30000, true, false, 10, 0),
    (0x1234, 0x20000, false, false, 0, 0),
    (0x1234, 0x20000, true, false, 8, 0),
    (0x1234, 0x50000, false, false, 0, 0),
    (0x1234, 0x50000, true, false, 8, 0),
    (0x1234, 0x40000, false, false, 0, 0),
    (0x1234, 0x70000, true, false, 0, 0),
    (0x1234, 0x10003, false, true, 0, 1),
];

const DRIVER_ROWS: &[(u32, isize)] = &[
    (0, 0),
    (1, 1),
    (2, 1),
    (3, 0),
    (4, 0),
    (5, 1),
    (6, 1),
    (7, 0),
    (8, 0),
    (9, 1),
    (10, 1),
    (11, 0),
    (12, 0),
    (4096, 0),
];

const TIMER_FULL_OPS: &[Op] = &[
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 16,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 33,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 50,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 67,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 84,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 101,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 118,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 135,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 152,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 169,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 186,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 203,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 220,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 237,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 254,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 271,
        trigger: 1,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 0,
        id: 0,
        trigger: 0,
        stored_flags: 0,
    },
];

const TIMER_BASIC_OPS: &[Op] = &[
    Op::Set {
        delay: 0,
        resol: 0,
        flags: 0,
        now: 100,
        id: 0,
        trigger: 0,
        stored_flags: 0,
    },
    Op::Set {
        delay: 1,
        resol: 0,
        flags: 0,
        now: 100,
        id: 16,
        trigger: 101,
        stored_flags: 0,
    },
    Op::Set {
        delay: 65535,
        resol: 5,
        flags: 1,
        now: 200,
        id: 33,
        trigger: 65735,
        stored_flags: 1,
    },
    Op::Set {
        delay: 65536,
        resol: 0,
        flags: 0,
        now: 0,
        id: 0,
        trigger: 0,
        stored_flags: 0,
    },
    Op::Kill { id: 16, ret: 0 },
    Op::Kill { id: 16, ret: 97 },
    Op::Kill { id: 33, ret: 0 },
    Op::Set {
        delay: 16,
        resol: 0,
        flags: 0,
        now: 4294967280,
        id: 48,
        trigger: 0,
        stored_flags: 0,
    },
    Op::Kill { id: 65584, ret: 97 },
    Op::Kill { id: 48, ret: 0 },
    Op::Kill { id: 0, ret: 0 },
    Op::Set {
        delay: 2,
        resol: 0,
        flags: 65537,
        now: 0,
        id: 64,
        trigger: 2,
        stored_flags: 1,
    },
    Op::Kill { id: 64, ret: 0 },
];

const SND_ROWS: &[(u32, bool, u32)] = &[
    (0, false, 0),
    (1, false, 1),
    (8192, false, 0),
    (131072, false, 131072),
    (262148, false, 262148),
    (1114112, false, 1114112),
    (4294967295, false, 1507359),
    (64, false, 0),
    (16, false, 16),
    (16, true, 16),
    (2147483648, true, 0),
    (131080, true, 131080),
];

const JOY_ROWS: &[(u32, u32)] = &[(0, 0), (1, 165), (16, 165), (2147483648, 165)];

const NOT_SUPPORTED_ROWS: &[(&str, u32)] = &[
    ("midiOutCachePatches", 8),
    ("midiOutCacheDrumPatches", 8),
    ("waveOutGetPitch", 8),
    ("waveOutSetPitch", 8),
    ("waveOutGetPlaybackRate", 8),
    ("waveOutSetPlaybackRate", 8),
    ("mixerMessage", 8),
];

const FOURCC_A_ROWS: &[(&[u8], u32, u32)] = &[
    (b"\x72\x69\x66\x66", 0, 0x66666972),
    (b"\x72\x69\x66\x66", 16, 0x46464952),
    (b"\x61\x62", 0, 0x20206261),
    (b"", 0, 0x20202020),
    (b"\x61\x62\x63\x64\x65\x66", 16, 0x44434241),
    (b"\x61\xe9", 16, 0x2020e941),
    (b"\x78\x00\x79\x7a", 0, 0x20202078),
    (b"\xe9\x74\x80", 0, 0x208074e9),
];

const FOURCC_W_ROWS: &[(&[u16], u32, u32)] = &[
    (&[0x0072, 0x0069, 0x0066, 0x0066], 0, 0x66666972),
    (&[0x0072, 0x0069, 0x0066, 0x0066], 16, 0x46464952),
    (&[0x0061, 0x0062, 0x0000, 0x0000], 0, 0x20206261),
    (&[0x0061, 0x0062, 0x0063, 0x0064], 16, 0x44434241),
    (&[0x0061, 0x0000, 0x0062, 0x0063], 0, 0x20202061),
];

const STUB_ROWS: &[StubException] = &[
    StubException {
        code: 0x80000100,
        flags: 1,
        number_parameters: 2,
        module: "winmm.dll",
        function: StubFunction::Name("mmioInstallIOProc16"),
    },
    StubException {
        code: 0x80000100,
        flags: 1,
        number_parameters: 2,
        module: "winmm.dll",
        function: StubFunction::Ordinal(3),
    },
    StubException {
        code: 0x80000100,
        flags: 1,
        number_parameters: 2,
        module: "winmm.dll",
        function: StubFunction::Ordinal(4),
    },
];

fn replay(ops: &[Op]) {
    let mut table = TimerTable::default();
    for op in ops {
        match *op {
            Op::Set {
                delay,
                resol,
                flags,
                now,
                id,
                trigger,
                stored_flags,
            } => {
                assert_eq!(table.set_event(delay, resol, flags, now), id);
                if id != 0 {
                    let timer = table
                        .timers
                        .iter()
                        .find(|timer| u32::from(timer.id) == id)
                        .unwrap();
                    assert_eq!(timer.trigger_time, trigger);
                    assert_eq!(timer.flags, stored_flags);
                }
            }
            Op::Kill { id, ret } => assert_eq!(table.kill_event(id), ret),
        }
    }
}

#[test]
fn oracle_timer_full_rows() {
    replay(TIMER_FULL_OPS);
}

#[test]
fn oracle_timer_basic_rows() {
    replay(TIMER_BASIC_OPS);
}

/// After 4095 set and kill pairs the counter is at 4095. The next counter value, 4096, gives an id
/// of 65536, which truncates to 0, so the C body takes the next value and returns 16.
#[test]
fn oracle_timer_wrap_row() {
    let mut table = TimerTable::default();
    for k in 1..=4095 {
        let id = table.set_event(1, 0, 0, 0);
        assert_eq!(id, 16 * k);
        assert_eq!(table.kill_event(id), 0);
    }
    assert_eq!(table.set_event(1, 0, 0, 0), 16);
}

/// Runs `call` on a buffer of `len` units that starts as `sentinel`, or on NULL when `null` is set.
/// Returns the return value, the number of units before the first sentinel (the NUL counts), and
/// the buffer.
fn run_buffer<T: Copy + PartialEq>(
    sentinel: T,
    len: u32,
    null: bool,
    call: impl FnOnce(Option<&mut [T]>) -> u32,
) -> (u32, usize, Vec<T>) {
    let mut buf = vec![sentinel; len as usize];
    let ret = call((!null).then_some(buf.as_mut_slice()));
    let written = buf.iter().take_while(|&&unit| unit != sentinel).count();
    (ret, written, buf)
}

/// The text up to the first NUL unit. Both wide and narrow buffers are read as UTF-16 units.
fn text_before_nul<T: Copy + Into<u16>>(buf: &[T]) -> String {
    let units: Vec<u16> = buf
        .iter()
        .map(|&unit| unit.into())
        .take_while(|&unit| unit != 0)
        .collect();
    String::from_utf16(&units).unwrap()
}

fn check_error_text_w(
    rows: &[(u32, u32, bool, u32, usize, &str)],
    get: fn(u32, Option<&mut [u16]>) -> u32,
) {
    for &(error, buflen, null, ret, written, text) in rows {
        let (got_ret, got_written, buf) =
            run_buffer::<u16>(0x7f, buflen, null, |dst| get(error, dst));
        assert_eq!(
            (got_ret, got_written),
            (ret, written),
            "error {error} buflen {buflen}"
        );
        assert_eq!(text_before_nul(&buf[..written]), text);
    }
}

fn check_error_text_a(rows: &[(u32, u32, bool, bool, u32, usize, &str)], base: u32, last: u32) {
    for &(error, buflen, null, alloc_fails, ret, written, text) in rows {
        let (got_ret, got_written, buf) = run_buffer::<u8>(0x7f, buflen, null, |dst| {
            get_error_text_a(error, dst, base, last, |size| {
                (!alloc_fails).then(|| vec![0; size])
            })
        });
        assert_eq!(
            (got_ret, got_written),
            (ret, written),
            "error {error} buflen {buflen}"
        );
        assert_eq!(text_before_nul(&buf[..written]), text);
    }
}

#[test]
fn oracle_midi_out_get_error_text_w_rows() {
    check_error_text_w(MIDI_W_ROWS, midi_out_get_error_text_w);
}

#[test]
fn oracle_wave_out_get_error_text_w_rows() {
    check_error_text_w(WAVE_W_ROWS, wave_out_get_error_text_w);
}

#[test]
fn oracle_midi_out_get_error_text_a_rows() {
    check_error_text_a(MIDI_A_ROWS, MIDIERR_BASE, MIDIERR_LASTERROR);
}

#[test]
fn oracle_wave_out_get_error_text_a_rows() {
    check_error_text_a(WAVE_A_ROWS, WAVERR_BASE, WAVERR_LASTERROR);
}

#[test]
fn oracle_check_callback_rows() {
    for &(dw, fdw, mixer, is_window_result, ret, is_window_calls) in CALLBACK_ROWS {
        let mut seen = 0u32;
        let got = check_callback(dw, fdw, mixer, |_| {
            seen += 1;
            is_window_result
        });
        assert_eq!(
            (got, seen),
            (ret, is_window_calls),
            "dw {dw:#x} fdw {fdw:#x} mixer {mixer}"
        );
    }
}

#[test]
fn oracle_def_driver_proc_rows() {
    for &(msg, ret) in DRIVER_ROWS {
        assert_eq!(def_driver_proc(msg), ret, "msg {msg}");
    }
}

/// The unicode column is not an input to the port: both `sndPlaySound` variants pass the same
/// flags to `MULTIMEDIA_PlaySound`, so only the flags decide the result.
#[test]
fn oracle_snd_play_sound_rows() {
    for &(flags, _unicode, ret) in SND_ROWS {
        assert_eq!(snd_play_sound_flags(flags), ret, "flags {flags:#x}");
    }
}

#[test]
fn oracle_joy_config_changed_rows() {
    for &(flags, ret) in JOY_ROWS {
        assert_eq!(joy_config_changed(flags), ret, "flags {flags:#x}");
    }
}

#[test]
fn oracle_not_supported_rows() {
    type NoArgFn = fn() -> u32;
    let fns: [(&str, NoArgFn); 7] = [
        ("midiOutCachePatches", midi_out_cache_patches),
        ("midiOutCacheDrumPatches", midi_out_cache_drum_patches),
        ("waveOutGetPitch", wave_out_get_pitch),
        ("waveOutSetPitch", wave_out_set_pitch),
        ("waveOutGetPlaybackRate", wave_out_get_playback_rate),
        ("waveOutSetPlaybackRate", wave_out_set_playback_rate),
        ("mixerMessage", mixer_message),
    ];
    for (&(name, f), &(row_name, ret)) in fns.iter().zip(NOT_SUPPORTED_ROWS) {
        assert_eq!(name, row_name);
        assert_eq!(f(), ret, "{name}");
    }
}

#[test]
fn oracle_mmio_string_to_fourcc_a_rows() {
    for &(sz, flags, ret) in FOURCC_A_ROWS {
        assert_eq!(
            mmio_string_to_fourcc_a(sz, flags),
            ret,
            "sz {sz:?} flags {flags:#x}"
        );
    }
}

#[test]
fn oracle_mmio_string_to_fourcc_w_rows() {
    for &(sz, flags, ret) in FOURCC_W_ROWS {
        let sz: &[u16; 4] = sz.try_into().unwrap();
        assert_eq!(
            mmio_string_to_fourcc_w(sz, flags),
            ret,
            "sz {sz:?} flags {flags:#x}"
        );
    }
}

#[test]
fn oracle_stub_exception_rows() {
    let got = [mmio_install_io_proc16(), stub_ordinal_3(), stub_ordinal_4()];
    assert_eq!(&got[..], STUB_ROWS);
}

#[test]
fn oracle_header_values() {
    assert_eq!(joy_get_num_devs(), 16);
    assert_eq!(mmsystem_get_version(), 778);
}
