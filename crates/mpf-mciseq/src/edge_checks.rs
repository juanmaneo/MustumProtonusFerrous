//! Edge checks for the branches the oracle rows do not reach: failed reads, the SMPTE frame rules,
//! the play ranges, and the header and track errors. The expectations were derived from the C in
//! `mcimidi.c`. Where the input fits in argv, they were also checked against the harness binary in
//! `target/canon/mciseq`.

use super::*;

const MTRK: &[u8] = b"MTrk\x00\x00\x00\x04\x00\xFF\x2F\x00";

fn mthd(length: u32, format: u16, ntracks: u16, division: u16) -> Vec<u8> {
    let mut data = b"MThd".to_vec();
    data.extend_from_slice(&length.to_be_bytes());
    for word in [format, ntracks, division] {
        data.extend_from_slice(&word.to_be_bytes());
    }
    data.extend_from_slice(MTRK);
    data
}

fn read_err(data: &[u8], offset: u32) -> Option<u32> {
    read_mthd(&mut MmioFile::new(data), offset).err()
}

fn scan(data: &[u8]) -> (Result<(), u32>, Track, usize) {
    let mut file = MmioFile::new(data);
    let mut track = Track::default();
    let result = read_mtrk(&mut file, &mut track);
    (result, track, file.position())
}

fn next_event(
    data: &[u8],
    index: u32,
    last: u32,
    pulse: u32,
    last_command: u8,
) -> (Result<(), u32>, Track) {
    let mut track = Track {
        index,
        last,
        event_pulse: pulse,
        last_command,
        event_data: 0xFFFF_FFFF,
        event_length: 0xFFFF,
        ..Default::default()
    };
    let result = read_next_event(&mut MmioFile::new(data), &mut track);
    (result, track)
}

fn range(
    has_file: bool,
    flags: u32,
    tf: u32,
    parms: Option<(u32, u32)>,
    position: u32,
) -> Result<(u32, u32), u32> {
    play_range(
        has_file,
        flags,
        tf,
        parms.map(|(from, to)| PlayParms { from, to }),
        position,
    )
}

#[test]
fn seek_moves_within_the_image_and_rejects_out_of_range_targets() {
    let data = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let mut file = MmioFile::new(&data);
    assert_eq!(file.seek(5, Whence::Set), 5);
    assert_eq!(file.seek(-2, Whence::Cur), 3);
    assert_eq!(file.seek(-1, Whence::Set), -1);
    assert_eq!(file.position(), 3);
    assert_eq!(file.seek(-6, Whence::Cur), -1);
    assert_eq!(file.seek(i32::MAX, Whence::Set), i32::MAX);
    assert_eq!(file.seek(1, Whence::Cur), -1);
    assert_eq!(file.position(), i32::MAX as usize);
}

#[test]
fn read_stops_at_the_end_of_the_image() {
    let data = [0xAA, 0xBB, 0xCC];
    let mut file = MmioFile::new(&data);
    let mut buf = [0u8; 2];
    assert_eq!(file.read(&mut buf), 2);
    assert_eq!(file.read(&mut buf), 1);
    assert_eq!(buf[0], 0xCC);
    assert_eq!(file.read(&mut buf), 0);
    assert_eq!(file.position(), 3);
    file.seek(10, Whence::Set);
    assert_eq!(file.read(&mut buf), 0);
    assert_eq!(file.position(), 10);
}

#[test]
fn skip_advances_and_stops_at_the_end() {
    let data = [1u8, 2, 3];
    let mut file = MmioFile::new(&data);
    file.skip(2);
    assert_eq!(file.position(), 2);
    file.skip(5);
    assert_eq!(file.position(), 3);
    file.seek(10, Whence::Set);
    file.skip(1);
    assert_eq!(file.position(), 10);
}

#[test]
fn scalar_reads_fail_on_short_input() {
    assert_eq!(MmioFile::new(&[]).read_byte(), None);
    assert_eq!(MmioFile::new(&[]).read_word(), None);
    let mut file = MmioFile::new(&[1]);
    assert_eq!(file.read_word(), None);
    assert_eq!(file.position(), 1);
    assert_eq!(MmioFile::new(&[1, 2]).read_word(), Some(0x0102));
    assert_eq!(MmioFile::new(&[]).read_long(), None);
    assert_eq!(MmioFile::new(&[1, 2, 3]).read_long(), None);
    assert_eq!(MmioFile::new(&[1, 2, 3, 4]).read_long(), Some(0x0102_0304));
}

#[test]
fn var_len_reads_seven_bit_groups_and_fails_on_truncation() {
    let cases: [(&[u8], (u16, u32)); 6] = [
        (&[0x7F], (1, 0x7F)),
        (&[0xFF, 0x7F], (2, 0x3FFF)),
        (&[0x81, 0x00], (2, 128)),
        (&[0x81], (0, 0)),
        (&[], (0, 0)),
        (&[0xFF, 0xFF, 0xFF, 0xFF, 0x7F], (5, 0xFFFF_FFFF)),
    ];
    for (data, expected) in cases {
        assert_eq!(MmioFile::new(data).read_var_len(), expected, "{data:02x?}");
    }
}

#[test]
fn var_len_length_wraps_at_sixteen_bits() {
    let mut data = vec![0x80; 65536];
    data.push(0);
    assert_eq!(MmioFile::new(&data).read_var_len(), (1, 0));
}

#[test]
fn mmr2mci_maps_each_known_code() {
    assert_eq!(mmr2mci(4), 337);
    assert_eq!(mmr2mci(7), 264);
    assert_eq!(mmr2mci(2), 338);
    assert_eq!(mmr2mci(69), 339);
    assert_eq!(mmr2mci(0), 0);
    assert_eq!(mmr2mci(277), 277);
}

#[test]
fn seek_target_checks_the_flags_and_sets_the_position() {
    assert_eq!(seek_target(0, 0, 0), Err(273));
    assert_eq!(seek_target(1, 0, 0), Err(273));
    assert_eq!(seek_target(256 | 8, 0, 0), Err(284));
    assert_eq!(seek_target(768, 0, 0), Err(284));
    assert_eq!(seek_target(8, 1000, 4), Ok(835_380_000));
    assert_eq!(seek_target(256, 0, 0), Ok(0));
    assert_eq!(seek_target(512, 0, 0), Ok(u32::MAX));
    assert_eq!(seek_target(257, 0, 0), Ok(0));
}

#[test]
fn play_range_checks_the_file_and_the_bounds() {
    let (to, from) = (MCI_TO, MCI_FROM);
    assert_eq!(range(false, 0, 0, None, 7), Err(275));
    assert_eq!(range(true, 0, 0, None, 7), Ok((7, u32::MAX)));
    assert_eq!(range(true, to, 0, None, 7), Ok((7, u32::MAX)));
    assert_eq!(range(true, 0, 0, Some((10, 20)), 7), Ok((7, u32::MAX)));
    assert_eq!(range(true, to, 0, Some((0, 50)), 100), Err(282));
    assert_eq!(range(true, to, 0, Some((0, 50)), 0), Ok((0, 50)));
    assert_eq!(range(true, from, 0, Some((50, 0)), 0), Ok((50, u32::MAX)));
    assert_eq!(range(true, from | to, 0, Some((50, 100)), 0), Ok((50, 100)));
    assert_eq!(range(true, from | to, 0, Some((5, 5)), 0), Ok((5, 5)));
    assert_eq!(range(true, from | to, 0, Some((60, 50)), 0), Err(282));
}

#[test]
fn pulse_to_ms_handles_ppq_and_smpte_divisions() {
    assert_eq!(pulse_to_ms(0, 500_000, 1000), 0);
    assert_eq!(pulse_to_ms(96, 500_000, 192), 1000);
    assert_eq!(pulse_to_ms(0x8000, 1000, 32768), 1);
    assert_eq!(pulse_to_ms(0xE804, 0, 1000), 10416);
    assert_eq!(pulse_to_ms(0x8001, 0, 1000), 7812);
    assert_eq!(pulse_to_ms(0xE200, 0, 1000), 0);
}

#[test]
fn time_format_to_ms_unpacks_each_format() {
    assert_eq!(time_format_to_ms(0, 12345), 12345);
    assert_eq!(time_format_to_ms(4, 0x0102_0304), 14_582_041);
    assert_eq!(time_format_to_ms(5, 0x0102_0304), 14_582_040);
    assert_eq!(time_format_to_ms(6, 0x0102_0304), 14_582_033);
    assert_eq!(time_format_to_ms(4, 0xFF00_0000), 10_625);
    assert_eq!(time_format_to_ms(7, 0x0102_0304), 0);
    assert_eq!(time_format_to_ms(99, 0x0102_0304), 0);
}

#[test]
fn ms_to_time_format_packs_each_format() {
    assert_eq!(ms_to_time_format(0, 12345), 12345);
    assert_eq!(ms_to_time_format(4, 3_723_456), 1_375_928_833);
    assert_eq!(ms_to_time_format(5, 1040), 436_273_152);
    assert_eq!(ms_to_time_format(6, 1000), 503_382_016);
    assert_eq!(ms_to_time_format(4, 1000), 402_718_720);
    assert_eq!(ms_to_time_format(5, 61_000), 419_496_192);
    assert_eq!(ms_to_time_format(5, 3_600_000), 1);
    assert_eq!(ms_to_time_format(4, 921_600_000), 256);
    assert_eq!(ms_to_time_format(7, 1000), 0);
    assert_eq!(ms_to_time_format(99, 1000), 0);
}

fn header_with_lengths(format: u16, lengths: &[u32]) -> Header {
    Header {
        format,
        division: 96,
        time_format: MCI_FORMAT_MILLISECONDS,
        tempo: 500_000,
        tracks: lengths
            .iter()
            .map(|&length| Track {
                length,
                ..Default::default()
            })
            .collect(),
    }
}

#[test]
fn mthd_length_sums_format_2_and_takes_the_longest_track_otherwise() {
    assert_eq!(mthd_length_ms(&header_with_lengths(2, &[96, 192])), 1500);
    assert_eq!(mthd_length_ms(&header_with_lengths(1, &[96, 192])), 1000);
    assert_eq!(mthd_length_ms(&header_with_lengths(2, &[u32::MAX, 2])), 5);
    assert_eq!(mthd_length_ms(&header_with_lengths(1, &[])), 0);
}

fn track_at(status: bool, event_pulse: u32) -> Track {
    Track {
        status,
        event_pulse,
        ..Default::default()
    }
}

#[test]
fn find_next_event_picks_the_earliest_playing_track() {
    assert_eq!(
        find_next_event(&[track_at(true, 10), track_at(true, 5), track_at(false, 1)]),
        (Some(1), 5)
    );
    assert_eq!(
        find_next_event(&[track_at(true, 5), track_at(true, 5)]),
        (Some(0), 5)
    );
    assert_eq!(find_next_event(&[]), (None, u32::MAX));
    assert_eq!(find_next_event(&[track_at(false, 3)]), (None, u32::MAX));
    assert_eq!(
        find_next_event(&[track_at(true, u32::MAX)]),
        (None, u32::MAX)
    );
}

#[test]
fn read_mthd_rejects_a_bad_chunk() {
    let good = mthd(6, 1, 1, 96);
    let mut bad_fourcc = good.clone();
    bad_fourcc[0] = b'X';
    let mut bad_track = mthd(6, 0, 1, 96);
    bad_track[14] = b'X';
    assert_eq!(read_err(&good, 0x8000_0000), Some(296));
    assert_eq!(read_err(&[], 0), Some(296));
    assert_eq!(read_err(&bad_fourcc, 0), Some(296));
    assert_eq!(read_err(&mthd(5, 1, 1, 96), 0), Some(296));
    assert_eq!(read_err(b"MThd\x00", 0), Some(296));
    assert_eq!(read_err(b"MThd\x00\x00", 0), Some(296));
    assert_eq!(read_err(b"MThd\x00\x00\x00\x06", 0), Some(296));
    assert_eq!(read_err(b"MThd\x00\x00\x00\x06\x00\x01", 0), Some(296));
    assert_eq!(
        read_err(b"MThd\x00\x00\x00\x06\x00\x01\x00\x01", 0),
        Some(296)
    );
    assert_eq!(read_err(&mthd(6, 1, 1, 0), 0), Some(296));
    assert_eq!(read_err(&mthd(6, 1, 1, 0xE250), 0), Some(296));
    assert_eq!(read_err(&mthd(6, 3, 1, 96), 0), Some(296));
    assert_eq!(read_err(&bad_track, 0), Some(296));
}

#[test]
fn read_mthd_accepts_a_smpte_division_of_0x8000() {
    let data = mthd(6, 1, 1, 0x8000);
    let header = read_mthd(&mut MmioFile::new(&data), 0).unwrap();
    assert_eq!(header.division, 0x8000);
    assert_eq!(header.time_format, MCI_FORMAT_MILLISECONDS);
    assert_eq!(header.tracks[0].first, 22);
}

#[test]
fn read_mthd_reads_from_the_current_position_when_the_seek_target_is_minus_one() {
    let data = mthd(6, 1, 1, 96);
    let header = read_mthd(&mut MmioFile::new(&data), u32::MAX).unwrap();
    assert_eq!(header.tracks[0].first, 22);
}

#[test]
fn read_mthd_caps_the_track_count_at_0x80() {
    let mut data = b"MThd\x00\x00\x00\x06\x00\x01\x00\x81\x00\x60".to_vec();
    for _ in 0..128 {
        data.extend_from_slice(MTRK);
    }
    let header = read_mthd(&mut MmioFile::new(&data), 0).unwrap();
    assert_eq!(header.tracks.len(), 128);
    assert_eq!(header.tracks[127].first, 22 + 12 * 127);
}

#[test]
fn read_mthd_accepts_a_format_1_file_without_tracks() {
    let data = mthd(6, 1, 0, 96);
    let header = read_mthd(&mut MmioFile::new(&data), 0).unwrap();
    assert!(header.tracks.is_empty());
}

#[test]
fn read_mtrk_rejects_a_short_or_missing_header() {
    assert_eq!(scan(b"MT").0, Err(296));
    assert_eq!(scan(b"MTrk").0, Err(296));
}

#[test]
fn read_mtrk_keeps_running_status_across_events() {
    let (result, track, pos) =
        scan(b"MTrk\x00\x00\x00\x0b\x00\x90\x3c\x40\x05\x3c\x40\x00\xFF\x2F\x00");
    assert_eq!(result, Ok(()));
    assert_eq!((track.length, track.last_command, pos), (5, 0x90, 19));
}

#[test]
fn read_mtrk_skips_text_meta_events() {
    let (result, track, pos) =
        scan(b"MTrk\x00\x00\x00\x0a\x00\xFF\x03\x02\x41\x42\x00\xFF\x2F\x00");
    assert_eq!(result, Ok(()));
    assert_eq!((track.length, pos), (0, 18));
}

#[test]
fn read_mtrk_stops_at_a_truncated_event_and_seeks_to_the_end() {
    let (result, track, pos) = scan(b"MTrk\x00\x00\x00\x10\x00\x90\x3c\x40");
    assert_eq!(result, Ok(()));
    assert_eq!((track.length, track.last_command, pos), (0, 0x90, 24));
}

#[test]
fn read_mtrk_stops_at_an_event_past_the_track_end() {
    let (result, track, pos) = scan(b"MTrk\x00\x00\x00\x03\x00\x90\x3c\x40");
    assert_eq!(result, Ok(()));
    assert_eq!((track.last_command, pos), (0x90, 11));
}

#[test]
fn read_next_event_rejects_a_bad_index_and_short_events() {
    assert_eq!(
        next_event(&[0x00, 0x90, 0x3c, 0x40], 0x8000_0000, u32::MAX, 0, 0).0,
        Err(296)
    );
    assert_eq!(next_event(&[0x00], 0, u32::MAX, 0, 0).0, Err(296));
    assert_eq!(next_event(&[0x00, 0x90], 0, u32::MAX, 0, 0).0, Err(296));
}

#[test]
fn read_next_event_reads_an_event_at_the_maximum_index() {
    let (result, track) = next_event(&[0x00, 0x90, 0x3c, 0x40], u32::MAX, u32::MAX, 0, 0);
    assert_eq!(result, Ok(()));
    assert_eq!((track.event_data, track.event_length), (0x0040_3c90, 4));
}

#[test]
fn read_next_event_decodes_each_event_form() {
    let cases: [(&[u8], u8, u32, u16); 7] = [
        (&[0x00, 0xF0, 0x02, 0xAA, 0xBB], 0, 0xF0, 5),
        (&[0x81, 0x00, 0xF7, 0x00], 0, 0xF7, 4),
        (
            &[0x00, 0xFF, 0x51, 0x03, 0x07, 0xA1, 0x20],
            0,
            0x0004_51FF,
            7,
        ),
        (&[0x00, 0xC0, 0x05], 0, 0x05C0, 3),
        (&[0x00, 0x05], 0xD0, 0x05D0, 2),
        (&[0x00, 0x42], 0x70, 0x4270, 2),
        (&[0x00, 0x90, 0x3C, 0x40], 0, 0x0040_3C90, 4),
    ];
    for (data, last_command, data_word, length) in cases {
        let (result, track) = next_event(data, u32::MAX, u32::MAX, 0, last_command);
        assert_eq!(result, Ok(()), "{data:02x?}");
        assert_eq!(
            (track.event_data, track.event_length),
            (data_word, length),
            "{data:02x?}"
        );
    }
}

#[test]
fn read_next_event_adds_the_delta_to_the_pulse_with_wrap() {
    let (_, track) = next_event(&[0x81, 0x00, 0xF7, 0x00], u32::MAX, u32::MAX, 0, 0);
    assert_eq!(track.event_pulse, 128);
    let (_, track) = next_event(&[0x02, 0x90, 0x3C, 0x40], u32::MAX, u32::MAX, u32::MAX, 0);
    assert_eq!(track.event_pulse, 1);
}

#[test]
fn read_next_event_limits_a_long_meta_event_to_sixteen_bits() {
    let mut data = vec![0x80; 65534];
    data.extend_from_slice(&[0x00, 0xFF, 0x2F, 0x00]);
    let (result, track) = next_event(&data, 0, 70_000, 0, 70);
    assert_eq!(result, Ok(()));
    assert_eq!((track.event_data, track.event_length), (0xFFFF_2FFF, 2));
}
