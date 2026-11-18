use super::*;
use crate::process_list::*;

fn put(buf: &mut [u8], at: usize, bytes: &[u8]) {
    buf[at..at + bytes.len()].copy_from_slice(bytes);
}

/// An entry header with the given fields. Length and MaximumLength are both name_len.
fn header(next: u32, name_len: u16, name_ptr: u64, pid: u64, session: u32) -> Vec<u8> {
    let mut buf = vec![0u8; HEADER_LEN];
    put(&mut buf, NEXT_ENTRY_OFFSET, &next.to_le_bytes());
    put(&mut buf, PROCESS_NAME_LENGTH, &name_len.to_le_bytes());
    put(&mut buf, PROCESS_NAME_LENGTH + 2, &name_len.to_le_bytes());
    put(&mut buf, PROCESS_NAME_BUFFER, &name_ptr.to_le_bytes());
    put(&mut buf, UNIQUE_PROCESS_ID, &pid.to_le_bytes());
    put(&mut buf, SESSION_ID, &session.to_le_bytes());
    buf
}

fn utf16(text: &str) -> Vec<u8> {
    text.encode_utf16().flat_map(u16::to_le_bytes).collect()
}

#[test]
fn empty_buffer_is_truncated() {
    assert_eq!(parse_process_list(&[], 0), Err(ParseError::Truncated));
}

#[test]
fn header_shorter_than_its_fields_is_truncated() {
    assert_eq!(
        parse_process_list(&[0u8; HEADER_LEN - 1], 0),
        Err(ParseError::Truncated)
    );
}

#[test]
fn next_offset_past_the_end_is_truncated() {
    assert_eq!(
        parse_process_list(&header(200, 0, 0, 4, 0), 0),
        Err(ParseError::Truncated)
    );
    assert_eq!(
        parse_process_list(&header(u32::MAX, 0, 0, 4, 0), 0),
        Err(ParseError::Truncated)
    );
}

#[test]
fn name_below_the_base_is_out_of_range() {
    let mut buf = header(0, 4, 0x0F, 4, 0);
    buf.extend_from_slice(&utf16("ab"));
    assert_eq!(
        parse_process_list(&buf, 0x1000),
        Err(ParseError::NameOutOfRange)
    );
}

#[test]
fn name_past_the_end_is_out_of_range() {
    let mut buf = header(0, 4, 106, 4, 0);
    buf.extend_from_slice(&utf16("a"));
    assert_eq!(parse_process_list(&buf, 0), Err(ParseError::NameOutOfRange));
}

#[test]
fn name_pointer_at_the_top_of_the_range_saturates() {
    let buf = header(0, 4, u64::MAX, 4, 0);
    assert_eq!(parse_process_list(&buf, 0), Err(ParseError::NameOutOfRange));
}

#[test]
fn odd_name_length_is_rejected() {
    let mut buf = header(0, 3, 104, 4, 0);
    buf.extend_from_slice(b"A\x00B");
    assert_eq!(parse_process_list(&buf, 0), Err(ParseError::OddNameLength));
}

#[test]
fn zero_length_name_ignores_the_pointer() {
    let buf = header(0, 0, u64::MAX, 4, 0);
    assert_eq!(
        parse_process_list(&buf, 0x1000),
        Ok(vec![ProcessEntry {
            session_id: 0,
            process_id: 4,
            name: Vec::new(),
        }])
    );
}

#[test]
fn name_pointer_is_relative_to_the_address() {
    let mut buf = header(0, 4, 0x1000 + 104, 4, 0);
    buf.extend_from_slice(&utf16("ab"));
    assert_eq!(
        parse_process_list(&buf, 0x1000),
        Ok(vec![ProcessEntry {
            session_id: 0,
            process_id: 4,
            name: "ab".encode_utf16().collect(),
        }])
    );
}

#[test]
fn process_id_keeps_the_low_32_bits() {
    let buf = header(0, 0, 0, 0x0100_0000_01F4, 7);
    assert_eq!(
        parse_process_list(&buf, 0),
        Ok(vec![ProcessEntry {
            session_id: 7,
            process_id: 500,
            name: Vec::new(),
        }])
    );
}

#[test]
fn walk_follows_next_offsets_across_gap_bytes() {
    let mut buf = header(128, 8, 256, 4, 0);
    buf.extend_from_slice(&[0xCC; 24]);
    buf.extend_from_slice(&header(0, 10, 264, 8, 1));
    buf.resize(256, 0);
    buf.extend_from_slice(&utf16("smss"));
    buf.extend_from_slice(&utf16("csrss"));
    assert_eq!(
        parse_process_list(&buf, 0),
        Ok(vec![
            ProcessEntry {
                session_id: 0,
                process_id: 4,
                name: "smss".encode_utf16().collect(),
            },
            ProcessEntry {
                session_id: 1,
                process_id: 8,
                name: "csrss".encode_utf16().collect(),
            },
        ])
    );
}

#[test]
fn size_of_no_processes_is_zero() {
    assert_eq!(process_info_size(&[]), 0);
}

#[test]
fn size_counts_the_terminator_and_the_sid_space() {
    let entries = [
        ProcessEntry {
            session_id: 0,
            process_id: 0,
            name: Vec::new(),
        },
        ProcessEntry {
            session_id: 0,
            process_id: 0,
            name: vec![0x41, 0x42],
        },
    ];
    assert_eq!(process_info_size(&entries), 94 + 98);
}

#[test]
fn enumerate_w_checks_run_in_wine_order() {
    assert_eq!(
        check_enumerate_processes_w(1, 1),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        check_enumerate_processes_w(0, 0),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        check_enumerate_processes_w(0, 2),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(check_enumerate_processes_w(0, 1), Ok(()));
}

#[test]
fn enumerate_ex_checks_pointers_before_level() {
    assert_eq!(
        check_enumerate_processes_ex_w(false, false, 1),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        check_enumerate_processes_ex_w(false, true, 0),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        check_enumerate_processes_ex_w(true, false, 0),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        check_enumerate_processes_ex_w(true, true, 1),
        Err(ERROR_CALL_NOT_IMPLEMENTED)
    );
    assert_eq!(check_enumerate_processes_ex_w(true, true, 0), Ok(()));
}

#[test]
fn query_checks_reject_either_missing_pointer() {
    assert_eq!(
        check_query_session_information(false, false),
        Err(ERROR_INVALID_USER_BUFFER)
    );
    assert_eq!(
        check_query_session_information(false, true),
        Err(ERROR_INVALID_USER_BUFFER)
    );
    assert_eq!(
        check_query_session_information(true, false),
        Err(ERROR_INVALID_USER_BUFFER)
    );
    assert_eq!(check_query_session_information(true, true), Ok(()));
}

#[test]
fn session_info_routes_cover_every_arm() {
    assert_eq!(session_info_route_a(8), SessionInfoRouteA::Passthrough);
    assert_eq!(session_info_route_a(16), SessionInfoRouteA::Passthrough);
    assert_eq!(session_info_route_a(24), SessionInfoRouteA::SessionInfo);
    assert_eq!(session_info_route_a(5), SessionInfoRouteA::Convert);
    assert_eq!(session_info_route_a(-1), SessionInfoRouteA::Convert);
}
