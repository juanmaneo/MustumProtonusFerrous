//! Edge checks for the boundaries the oracle rows do not reach: the 32-bit sum wrap, a
//! `max_size` past the slice, offsets past `max_size`, one-byte and odd-length sized strings, NUL
//! placement inside a bounded window, width-coded integers at every width, truncation at each
//! field of both layouts, and the derived trait impls.

use crate::{
    compute_message_size, read_packed_player, read_super_packed_player, width_field, Cursor,
    InvalidPacket, PlayerInfo,
};

fn le_bytes(words: &[u32]) -> Vec<u8> {
    words.iter().flat_map(|word| word.to_le_bytes()).collect()
}

fn super_header(info_mask: u32) -> Vec<u8> {
    le_bytes(&[20, 0x21, 0x4567, info_mask, 13])
}

fn packed_header(sp_length: u32, player_data_length: u32, count: u32) -> Vec<u8> {
    le_bytes(&[
        48,
        0x12,
        0x3456,
        0,
        0,
        sp_length,
        player_data_length,
        count,
        0x77,
        48,
        11,
        0x99,
    ])
}

#[test]
fn e01_message_size_is_the_wrapping_dword_sum() {
    assert_eq!(compute_message_size(&[]), 0);
    assert_eq!(compute_message_size(&[20, 8, 4]), 32);
    assert_eq!(compute_message_size(&[u32::MAX, 2]), 1);
}

#[test]
fn e02_max_size_past_the_slice_is_rejected() {
    let data = [0u8; 8];
    let mut offset = 0;
    assert_eq!(
        read_packed_player(&data, &mut offset, 9),
        Err(InvalidPacket)
    );
    assert_eq!(
        read_super_packed_player(&data, &mut offset, 9),
        Err(InvalidPacket)
    );
    assert_eq!(offset, 0);
}

#[test]
fn e03_offset_past_max_size_is_rejected_and_left_unchanged() {
    let data = [0u8; 64];
    let mut offset = 30;
    assert_eq!(
        read_super_packed_player(&data, &mut offset, 20),
        Err(InvalidPacket)
    );
    assert_eq!(offset, 30);
}

#[test]
fn e04_read_bytes_is_bounded_by_max_size_without_overflow() {
    let data = [1u8, 2, 3, 4];
    let mut cursor = Cursor::new(&data, 1, 3).unwrap();
    assert_eq!(cursor.read_bytes(usize::MAX), Err(InvalidPacket));
    assert_eq!(cursor.read_bytes(3), Err(InvalidPacket));
    assert_eq!(cursor.read_bytes(2), Ok(&data[1..3]));
    assert_eq!(cursor.read_bytes(0), Ok(&data[3..3]));
    assert_eq!(cursor.position, 3);
}

#[test]
fn e05_sized_strings_shorter_than_two_bytes_are_rejected() {
    let data = [0u8; 4];
    let mut cursor = Cursor::new(&data, 0, 4).unwrap();
    assert_eq!(cursor.read_sized_string(1), Err(InvalidPacket));
    assert_eq!(cursor.read_sized_string(0), Err(InvalidPacket));
    assert_eq!(cursor.position, 0);
}

#[test]
fn e06_odd_sized_string_takes_its_last_whole_word_as_the_terminator() {
    let data = [0x00, 0x00, 0xFF, 0x42, 0x00, 0x00];
    let mut cursor = Cursor::new(&data, 0, 6).unwrap();
    assert_eq!(cursor.read_sized_string(3), Ok(vec![]));
    assert_eq!(cursor.position, 3);

    let mut cursor = Cursor::new(&data, 3, 6).unwrap();
    assert_eq!(cursor.read_sized_string(3), Err(InvalidPacket));
}

#[test]
fn e07_a_sized_name_stops_at_its_first_nul() {
    let data = [0x42, 0x00, 0x00, 0x00, 0x43, 0x00, 0x00, 0x00];
    let mut cursor = Cursor::new(&data, 0, 8).unwrap();
    assert_eq!(cursor.read_sized_string(8), Ok(vec![0x42]));
    assert_eq!(cursor.position, 8);
}

#[test]
fn e08_a_nul_terminated_string_must_end_inside_the_window() {
    let data = [0x41, 0x00, 0x42];
    let mut cursor = Cursor::new(&data, 0, 3).unwrap();
    assert_eq!(cursor.read_nul_terminated_string(), Err(InvalidPacket));
    assert_eq!(cursor.position, 0);

    let data = [0x41, 0x00, 0x00, 0x00, 0x42];
    let mut cursor = Cursor::new(&data, 0, 5).unwrap();
    assert_eq!(cursor.read_nul_terminated_string(), Ok(vec![0x41]));
    assert_eq!(cursor.position, 4);
}

#[test]
fn e09_width_coded_integers_read_zero_to_four_bytes() {
    let data = [0x05, 0x06, 0x00, 0x07, 0x00, 0x00, 0x00];
    let mut cursor = Cursor::new(&data, 0, 7).unwrap();
    assert_eq!(cursor.read_width_integer(0), Ok(0));
    assert_eq!(cursor.read_width_integer(1), Ok(5));
    assert_eq!(cursor.read_width_integer(2), Ok(6));
    assert_eq!(cursor.read_width_integer(3), Ok(7));
    assert_eq!(cursor.position, 7);
}

#[test]
fn e10_width_coded_integers_fail_when_their_field_is_truncated() {
    let data = [0x01];
    let mut cursor = Cursor::new(&data, 0, 1).unwrap();
    assert_eq!(cursor.read_width_integer(2), Err(InvalidPacket));
    assert_eq!(cursor.position, 0);

    let empty: [u8; 0] = [];
    let mut cursor = Cursor::new(&empty, 0, 0).unwrap();
    assert_eq!(cursor.read_width_integer(1), Err(InvalidPacket));
    assert_eq!(cursor.read_width_integer(3), Err(InvalidPacket));
}

#[test]
fn e11_width_field_extracts_a_two_bit_field() {
    assert_eq!(width_field(0b11 << 6, 6), 3);
    assert_eq!(width_field(0b10 << 4, 4), 2);
    assert_eq!(width_field(0x300, 8), 3);
    assert_eq!(width_field(u32::MAX, 2), 3);
}

#[test]
fn e12_a_huge_id_count_is_rejected_without_allocating() {
    let data = [0u8; 16];
    let mut cursor = Cursor::new(&data, 0, 16).unwrap();
    assert_eq!(cursor.read_ids(u32::MAX), Err(InvalidPacket));
    assert_eq!(cursor.read_ids(0), Ok(vec![]));
    assert_eq!(cursor.position, 0);
}

#[test]
fn e13_truncated_fields_are_rejected_and_leave_the_offset_alone() {
    let cases: Vec<(bool, Vec<u8>)> = vec![
        (true, super_header(1 << 2)),
        (true, [super_header(1 << 2), vec![0x03, 0xAA]].concat()),
        (true, super_header(1 << 6)),
        (
            true,
            [super_header(1 << 6), vec![0x02, 0x01, 0, 0, 0]].concat(),
        ),
        (true, super_header(1 << 9)),
        (
            true,
            [super_header(1 << 9), vec![0x02, 0x01, 0, 0, 0]].concat(),
        ),
        (true, [super_header(0x001), vec![0x41, 0x00]].concat()),
        (true, [super_header(0x002), vec![0x41, 0x00]].concat()),
        (true, super_header(0x100)),
        (true, [super_header(2 << 4), vec![0x05]].concat()),
        (true, [super_header(1 << 4), vec![0x03, 0xAA]].concat()),
        (
            false,
            [
                le_bytes(&[48, 0x12, 0x3456, 0, 4, 0, 0, 0, 0x77, 48, 11, 0x99]),
                vec![0x5a, 0, 0],
            ]
            .concat(),
        ),
        (false, [packed_header(3, 0, 0), vec![0x01, 0x02]].concat()),
        (false, [packed_header(0, 2, 0), vec![0x01]].concat()),
        (
            false,
            [packed_header(0, 0, 2), vec![0x01, 0, 0, 0]].concat(),
        ),
    ];
    for (is_super, data) in &cases {
        let mut offset = 0;
        let result = if *is_super {
            read_super_packed_player(data, &mut offset, data.len())
        } else {
            read_packed_player(data, &mut offset, data.len())
        };
        assert_eq!(result, Err(InvalidPacket), "{data:?}");
        assert_eq!(offset, 0, "{data:?}");
    }
}

#[test]
fn e14_a_successful_read_from_a_nonzero_offset_leaves_the_tail_unread() {
    let mut data = vec![0xAA, 0xBB];
    data.extend(super_header(0));
    data.push(0xCC);
    let mut offset = 2;
    let info = read_super_packed_player(&data, &mut offset, 22).unwrap();
    assert_eq!(offset, 22);
    assert_eq!(info.flags, 0x21);
    assert_eq!(info.id, 0x4567);
    assert_eq!(info.version_or_system_player_id, 13);
    assert_eq!(info.short_name, None);
}

#[test]
fn e15_derived_impls_behave_as_value_types() {
    let info = PlayerInfo::default();
    assert_eq!(info.clone(), info);
    assert_eq!(info.flags, 0);
    assert!(info.short_name.is_none());
    assert!(format!("{info:?}").contains("PlayerInfo"));

    let error = InvalidPacket;
    assert_eq!(error.clone(), InvalidPacket);
    assert_eq!(format!("{error:?}"), "InvalidPacket");
}
