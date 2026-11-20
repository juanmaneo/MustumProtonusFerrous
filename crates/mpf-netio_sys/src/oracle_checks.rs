use super::{pending, sock_error_to_ntstatus};

#[test]
fn maps_success() {
    assert_eq!(sock_error_to_ntstatus(0), 0x0000_0000);
}

#[test]
fn maps_bad_descriptor() {
    assert_eq!(sock_error_to_ntstatus(10009), 0xC000_0008);
}

#[test]
fn maps_access_denied() {
    assert_eq!(sock_error_to_ntstatus(10013), 0xC000_0022);
}

#[test]
fn maps_fault_to_no_memory() {
    assert_eq!(sock_error_to_ntstatus(10014), 0xC000_0017);
}

#[test]
fn maps_invalid_parameter_errors() {
    for error in [10022, 10039, 10042, 10049] {
        assert_eq!(sock_error_to_ntstatus(error), 0xC000_000D);
    }
}

#[test]
fn maps_too_many_open_files() {
    assert_eq!(sock_error_to_ntstatus(10024), 0xC000_011F);
}

#[test]
fn maps_would_block() {
    assert_eq!(sock_error_to_ntstatus(10035), 0xC000_00D8);
}

#[test]
fn maps_in_progress() {
    assert_eq!(sock_error_to_ntstatus(10036), 0x0000_0103);
}

#[test]
fn maps_already_in_progress_and_network_down() {
    for error in [10037, 10050] {
        assert_eq!(sock_error_to_ntstatus(error), 0xC000_00BF);
    }
}

#[test]
fn maps_not_a_socket() {
    assert_eq!(sock_error_to_ntstatus(10038), 0xC000_0024);
}

#[test]
fn maps_message_size() {
    assert_eq!(sock_error_to_ntstatus(10040), 0x8000_0005);
}

#[test]
fn maps_unsupported_protocol_errors() {
    for error in [10041, 10043, 10044, 10046, 10047] {
        assert_eq!(sock_error_to_ntstatus(error), 0xC000_00BB);
    }
}

#[test]
fn maps_operation_not_supported() {
    assert_eq!(sock_error_to_ntstatus(10045), 0xC000_0002);
}

#[test]
fn maps_address_in_use() {
    assert_eq!(sock_error_to_ntstatus(10048), 0xC000_0238);
}

#[test]
fn maps_connection_refused() {
    assert_eq!(sock_error_to_ntstatus(10061), 0xC000_0236);
}

#[test]
fn maps_connection_and_shutdown_errors() {
    let cases = [
        (10058, 0xC000_00B0),
        (10057, 0xC000_020C),
        (10060, 0xC000_00B5),
        (10054, 0xC000_020D),
        (10053, 0xC000_0241),
    ];
    for (error, status) in cases {
        assert_eq!(sock_error_to_ntstatus(error), status);
    }
}

#[test]
fn maps_network_unreachable() {
    assert_eq!(sock_error_to_ntstatus(10051), 0xC000_023C);
}

#[test]
fn maps_host_not_found() {
    assert_eq!(sock_error_to_ntstatus(11001), 0xC000_0225);
}

#[test]
fn maps_unmapped_errors_to_unsuccessful() {
    for error in [1, 10004, 10005, 65_535] {
        assert_eq!(sock_error_to_ntstatus(error), 0xC000_0001);
    }
}

fn slots(spec: &str) -> Vec<pending::Slot<char>> {
    spec.split_whitespace()
        .map(|pair| {
            let bytes = pair.as_bytes();
            pending::Slot {
                in_use: bytes[0] == b'1',
                callback: (bytes[1] != b'-').then_some(char::from(bytes[1])),
            }
        })
        .collect()
}

#[test]
fn alloc_oracle_prefers_a_free_matching_callback() {
    assert_eq!(
        pending::select_slot(&slots("0B 0A 0- 0- 0- 0- 0- 0- 0- 0-"), &'A'),
        Some(1)
    );
    assert_eq!(
        pending::select_slot(&slots("0A 0- 0- 0- 0- 0- 0- 0- 0- 0-"), &'A'),
        Some(0)
    );
    assert_eq!(
        pending::select_slot(&slots("0C 0C 0C 0C 0C 0C 0C 0C 0C 0C"), &'A'),
        Some(0)
    );
}

#[test]
fn alloc_oracle_uses_first_free_or_reports_full() {
    assert_eq!(
        pending::select_slot(&slots("0- 0- 0- 0- 0- 0- 0- 0- 0- 0-"), &'A'),
        Some(0)
    );
    assert_eq!(
        pending::select_slot(&slots("1- 0- 0- 0- 0- 0- 0- 0- 0- 0-"), &'A'),
        Some(1)
    );
    assert_eq!(
        pending::select_slot(&slots("1A 1A 1A 1A 1A 1A 1A 1A 1A 1A"), &'A'),
        None
    );
    assert_eq!(
        pending::select_slot(&slots("1- 1- 1- 0- 1- 1- 1- 1- 1- 0A"), &'A'),
        Some(9)
    );
    assert_eq!(
        pending::select_slot(&slots("1- 1- 1- 1- 1- 1- 1- 1- 1- 1-"), &'B'),
        None
    );
}

#[test]
fn alloc_oracle_skips_busy_slots_and_keeps_the_first_free() {
    assert_eq!(
        pending::select_slot(&slots("0B 0- 0A 0A 0- 0- 0- 0- 0- 0-"), &'A'),
        Some(2)
    );
    assert_eq!(
        pending::select_slot(&slots("1A 1- 0- 1B 0A 0- 1- 1- 1- 1-"), &'B'),
        Some(2)
    );
    assert_eq!(
        pending::select_slot(&slots("1- 1- 1- 1- 1- 1- 1- 0B 1- 1-"), &'A'),
        Some(7)
    );
    assert_eq!(
        pending::select_slot(&slots("1C 0B 1A 1B 0A 0A 1- 1- 0- 1-"), &'B'),
        Some(1)
    );
}

fn waits(spec: &str) -> Vec<Option<char>> {
    spec.split_whitespace()
        .map(|token| {
            let byte = token.as_bytes()[0];
            (byte != b'-').then_some(char::from(byte))
        })
        .collect()
}

#[test]
fn find_oracle_returns_the_first_matching_wait_or_none() {
    assert_eq!(
        pending::find_wait(&waits("- - - 3 - 5 - - - -"), &'5'),
        Some(5)
    );
    assert_eq!(
        pending::find_wait(&waits("- - - 3 - 5 - - - -"), &'4'),
        None
    );
    assert_eq!(
        pending::find_wait(&waits("1 2 3 4 5 6 7 8 9 -"), &'1'),
        Some(0)
    );
    assert_eq!(
        pending::find_wait(&waits("- - - - - - - - - -"), &'7'),
        None
    );
    assert_eq!(
        pending::find_wait(&waits("1 2 3 4 5 6 7 8 9 -"), &'9'),
        Some(8)
    );
}
