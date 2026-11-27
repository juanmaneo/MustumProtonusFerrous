//! Boundary and branch tests for the ported winmm logic.
//!
//! The oracle rows do not cover every expectation here. Each test says in a comment whether its
//! expectations come from the oracle, from the Wine headers or C source, or from a decision of the
//! port (the ASCII `CP_ACP` shim, and the pure-Rust table storage).

use super::*;

#[test]
fn pinned_header_constants() {
    // Not oracle-derived: values from include/mmsystem.h, include/winnt.h and include/winternl.h.
    assert_eq!(MMSYSERR_HANDLEBUSY, 12);
    assert_eq!(MMSYSERR_LASTERROR, 21);
    assert_eq!(WAVERR_BASE, 32);
    assert_eq!(WAVERR_LASTERROR, 35);
    assert_eq!(MIDIERR_BASE, 64);
    assert_eq!(MIDIERR_BADOPENMODE, 70);
    assert_eq!(MIDIERR_LASTERROR, 71);
    assert_eq!(TIMERR_NOCANDO, 97);
    assert_eq!(JOYERR_PARMS, 165);
    assert_eq!(MMIO_TOUPPER, 0x10);
    assert_eq!(CALLBACK_TYPEMASK, 0x0007_0000);
    assert_eq!(CALLBACK_THREAD, 0x0002_0000);
    assert_eq!(CALLBACK_EVENT, 0x0005_0000);
    assert_eq!(SND_ASYNC, 0x0001);
    assert_eq!(SND_RESOURCE, 0x0004_0004);
    assert_eq!(SND_ALIAS_ID, 0x0011_0000);
    assert_eq!(SND_FILENAME, 0x0002_0000);
    assert_eq!(MMSYSTEM_VERSION, 0x030a);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn error_text_has_rows_only_for_the_stringtable_ids() {
    // Partly oracle-derived: the oracle probes 0, 1, 6, 12, 21, 22, 32 to 36, 64, 69 to 72. The
    // other ids come from the STRINGTABLE copy in resource.inc. Looping over 0 to 72 also calls
    // every arm of `error_text`.
    let with_text: Vec<u32> = (0..=72).filter(|&id| error_text(id).is_some()).collect();
    let expected: Vec<u32> = (0..=11).chain(32..=35).chain(64..=69).collect();
    assert_eq!(with_text, expected);
}

#[test]
fn wide_to_acp_byte_is_ascii_only() {
    // Not oracle-derived: the ASCII CP_ACP shim turns every non-ASCII unit into '?'.
    assert_eq!(wide_to_acp_byte(u16::from(b'A')), b'A');
    assert_eq!(wide_to_acp_byte(0), 0);
    assert_eq!(wide_to_acp_byte(0x7f), 0x7f);
    assert_eq!(wide_to_acp_byte(0xe9), b'?');
    assert_eq!(wide_to_acp_byte(0x100), b'?');
    assert_eq!(wide_to_acp_byte(0x4e2d), b'?');
}

#[test]
fn wide_to_acp_z_stops_at_the_nul_or_the_shorter_buffer() {
    // Not oracle-derived: the stop rules of WideCharToMultiByte with a length of -1.
    let (a, b, c) = (u16::from(b'a'), u16::from(b'b'), u16::from(b'c'));
    let mut dst = [9u8; 4];
    wide_to_acp_z(&[a, 0, b], &mut dst);
    assert_eq!(dst, [b'a', 0, 9, 9]);
    wide_to_acp_z(&[a, b], &mut dst);
    assert_eq!(dst, [b'a', b'b', 9, 9]);
    let mut short = [9u8; 2];
    wide_to_acp_z(&[a, b, c], &mut short);
    assert_eq!(short, [b'a', b'b']);
}

#[test]
fn error_text_w_cuts_at_the_buffer_length() {
    // Not oracle-derived for the cut-off: the oracle rows use 200-unit buffers, plus the 5- and
    // 4-unit prefixes. The expected prefixes are cut from the table text with `len`.
    let text = error_text(MMSYSERR_NOERROR).unwrap();
    for (buflen, keep) in [(39, text.len()), (38, text.len() - 1), (2, 1)] {
        let mut buf = vec![0xffu16; buflen];
        assert_eq!(
            midi_out_get_error_text_w(0, Some(buf.as_mut_slice())),
            MMSYSERR_NOERROR
        );
        assert_eq!(String::from_utf16(&buf[..keep]).unwrap(), &text[..keep]);
        assert_eq!(buf[keep], 0);
    }
}

#[test]
fn error_text_w_checks_null_before_empty() {
    // Partly oracle-derived: the oracle has a NULL row and an empty-buffer row. This pins the
    // order for the wave wrapper too.
    assert_eq!(wave_out_get_error_text_w(0, None), MMSYSERR_INVALPARAM);
    let mut empty: [u16; 0] = [];
    assert_eq!(
        wave_out_get_error_text_w(0, Some(&mut empty[..])),
        MMSYSERR_NOERROR
    );
}

#[test]
fn midi_out_error_text_a_writes_the_narrow_prefix() {
    // Not oracle-derived: the wrapper's own `Vec` allocation, with a 5-byte buffer.
    let mut buf = [0xffu8; 5];
    assert_eq!(
        midi_out_get_error_text_a(64, Some(&mut buf[..])),
        MMSYSERR_NOERROR
    );
    assert_eq!(buf, *b"The \0");
}

#[test]
fn wave_out_error_text_a_fits_the_text() {
    // Not oracle-derived: the wrapper's own `Vec` allocation, with a 26-byte buffer.
    let mut buf = [0xffu8; 26];
    assert_eq!(
        wave_out_get_error_text_a(1, Some(&mut buf[..])),
        MMSYSERR_NOERROR
    );
    assert_eq!(&buf[..25], b"Undefined external error.");
    assert_eq!(buf[25], 0);
}

/// Runs `check_callback` with an `IsWindow` stand-in that answers `window_ok`. Returns the result
/// and how many times the stand-in was asked.
fn callback_with_window(dw: usize, fdw: u32, mixer: bool, window_ok: bool) -> (u32, u32) {
    let mut asked = 0u32;
    let ret = check_callback(dw, fdw, mixer, |_| {
        asked += 1;
        window_ok
    });
    (ret, asked)
}

#[test]
fn check_callback_window_and_null_branches() {
    // Not oracle-derived for the bits outside `CALLBACK_TYPEMASK`. The window cases are oracle rows
    // too.
    assert_eq!(
        callback_with_window(0x1234, CALLBACK_WINDOW | 0xffff, false, true),
        (MMSYSERR_NOERROR, 1)
    );
    assert_eq!(
        callback_with_window(0x1234, CALLBACK_WINDOW | 0xffff, false, false),
        (MMSYSERR_INVALPARAM, 1)
    );
    assert_eq!(
        callback_with_window(0, CALLBACK_WINDOW, false, false),
        (MMSYSERR_NOERROR, 0)
    );
    assert_eq!(
        callback_with_window(0x1234, CALLBACK_NULL | 0xffff, true, false),
        (MMSYSERR_NOERROR, 0)
    );
}

#[test]
fn set_event_keeps_resol_as_uint_and_flags_as_word() {
    // Not oracle-derived: `WINE_TIMERENTRY` holds `wResol` as UINT and `wFlags` as UINT16
    // (dlls/winmm/time.c:31-41), so the high bits of flags are dropped and those of resol are kept.
    let mut table = TimerTable::default();
    assert_eq!(table.set_event(1, 0x1_0007, 0x1_0002, 0), 16);
    assert_eq!(table.timers[0].resol, 0x1_0007);
    assert_eq!(table.timers[0].flags, 2);
}

#[test]
fn set_event_reuses_the_first_freed_slot() {
    // Not oracle-derived: the oracle fills the table but never frees a slot and sets again.
    let mut table = TimerTable::default();
    let ids: Vec<u32> = (0..16).map(|_| table.set_event(1, 0, 0, 0)).collect();
    assert_eq!(table.set_event(1, 0, 0, 0), 0);
    assert_eq!(table.kill_event(ids[5]), 0);
    assert_eq!(table.set_event(1, 0, 0, 0), 277);
}

#[test]
fn export_table_matches_winmm_spec() {
    // Not oracle-derived: the counts and the stub list come from dlls/winmm/winmm.spec.
    assert_eq!(EXPORTS.len(), 189);
    let names: Vec<&str> = EXPORTS
        .iter()
        .filter_map(|(name, _)| match name {
            ExportName::Named(name) => Some(*name),
            ExportName::Ordinal(_) => None,
        })
        .collect();
    assert_eq!(names.len(), 186);
    let mut unique = names.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 186);
    assert_eq!(
        &EXPORTS[..3],
        &[
            (ExportName::Ordinal(2), ExportKind::Stdcall),
            (ExportName::Ordinal(3), ExportKind::Stub),
            (ExportName::Ordinal(4), ExportKind::Stub),
        ]
    );
    let stubs: Vec<ExportName> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|&(name, _)| name)
        .collect();
    assert_eq!(
        stubs,
        [
            ExportName::Ordinal(3),
            ExportName::Ordinal(4),
            ExportName::Named("mmioInstallIOProc16"),
        ]
    );
}

#[test]
fn export_kind_is_case_sensitive_and_ordinal_aware() {
    // Not oracle-derived: lookups against the spec table.
    assert_eq!(
        export_kind(ExportName::Named("PlaySoundA")),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind(ExportName::Ordinal(2)),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind(ExportName::Ordinal(1)), None);
    assert_eq!(export_kind(ExportName::Named("playsounda")), None);
    assert_eq!(
        export_kind(ExportName::Named("mmioInstallIOProc16")),
        Some(ExportKind::Stub)
    );
}

#[test]
fn mmio_string_to_fourcc_w_maps_units_through_the_ascii_shim() {
    // Not oracle-derived: a unit above 0x7f becomes '?' under the ASCII shim, and MMIO_TOUPPER
    // applies to the other units.
    assert_eq!(
        mmio_string_to_fourcc_w(&[0x4e2d, 0x41, 0x62, 0x43], MMIO_TOUPPER),
        mmio_fourcc(b'?', b'A', b'B', b'C')
    );
}

#[test]
fn rtl_upper_char_changes_only_a_to_z() {
    // Not oracle-derived: the port models RtlUpperChar for `a` to `z` only. The bytes on either
    // side of that range are unchanged.
    assert_eq!(rtl_upper_char(b'a'), b'A');
    assert_eq!(rtl_upper_char(b'z'), b'Z');
    for ch in [b'`', b'{', b'@', b'[', 0xe9] {
        assert_eq!(rtl_upper_char(ch), ch);
    }
}

#[test]
fn derived_debug_names_the_variants() {
    // Not oracle-derived: checks the derived Debug output of the public enums and the stub record.
    assert_eq!(format!("{:?}", ExportKind::Stub), "Stub");
    assert_eq!(format!("{:?}", ExportName::Ordinal(2)), "Ordinal(2)");
    assert_eq!(format!("{:?}", StubFunction::Name("x")), "Name(\"x\")");
    assert!(format!("{:?}", stub_ordinal_3()).contains("Ordinal(3)"));
}
