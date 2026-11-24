//! Edge-case tests for the `mpf-wpcap` port: the domain boundaries of the IPv6
//! mask, the status codes and formats the oracle rows do not reach, and the loop
//! exits of `dispatch` and `packet_loop`.

use super::oracle_checks::{run_packets, Driver};
use super::*;

#[test]
fn prefix_mask_rejects_lengths_outside_the_c_buffer() {
    for length in [0, 1, 7, 136, 200, u32::MAX] {
        assert_eq!(ipv6_prefix_mask(length), None, "length {length}");
    }
}

#[test]
fn prefix_mask_has_contiguous_leading_ones() {
    for length in 8..=135 {
        let bits = u128::from_be_bytes(ipv6_prefix_mask(length).expect("in-range length"));
        assert_eq!(bits.leading_ones(), length - 8, "length {length}");
        assert_eq!(bits.count_ones(), length - 8, "length {length}");
    }
}

#[test]
fn statustostr_names_the_remaining_error_codes() {
    assert_eq!(statustostr(PCAP_ERROR_BREAK), "Loop terminated by pcap_breakloop");
    assert_eq!(statustostr(PCAP_ERROR_NOT_ACTIVATED), "The pcap_t has not been activated");
    assert_eq!(statustostr(PCAP_ERROR_NO_SUCH_DEVICE), "No such device exists");
}

#[test]
fn statustostr_formats_unknown_codes_like_sprintf() {
    assert_eq!(statustostr(0), "Unknown error: 0");
    assert_eq!(statustostr(i32::MIN), "Unknown error: -2147483648");
    assert_eq!(statustostr(i32::MAX), "Unknown error: 2147483647");
}

#[test]
fn build_win32_name_without_pieces_keeps_the_prefix() {
    assert_eq!(build_win32_name(None, ""), "\\Device\\NPF_");
}

#[test]
fn build_win32_description_keeps_the_separator_for_empty_text() {
    assert_eq!(build_win32_description("lo", Some("")), " lo");
}

#[test]
fn dispatch_stops_at_the_count_limit() {
    assert_eq!(run_packets(Driver::Dispatch, 1, &[Ok(0), Ok(1)]), (1, vec![0], 1));
}

#[test]
fn dispatch_break_after_packets_returns_the_count() {
    assert_eq!(run_packets(Driver::Dispatch, 0, &[Ok(4), Err(-2)]), (1, vec![4], 2));
}

#[test]
fn dispatch_timeout_before_any_packet_returns_zero() {
    assert_eq!(run_packets(Driver::Dispatch, 0, &[Err(0)]), (0, vec![], 1));
}

#[test]
fn dispatch_reports_an_error_after_packets() {
    assert_eq!(run_packets(Driver::Dispatch, 0, &[Ok(0), Err(-5)]), (-5, vec![0], 2));
}

#[test]
fn packet_loop_skips_timeouts_until_a_packet() {
    let script = [Err(0), Err(0), Ok(7), Err(-2)];
    assert_eq!(run_packets(Driver::PacketLoop, 0, &script), (1, vec![7], 4));
}

#[test]
fn packet_loop_reports_an_error() {
    assert_eq!(run_packets(Driver::PacketLoop, 0, &[Err(-4)]), (-4, vec![], 1));
}

#[test]
fn packet_loop_stops_at_the_count_limit() {
    assert_eq!(run_packets(Driver::PacketLoop, 2, &[Ok(1), Ok(2), Ok(3)]), (2, vec![1, 2], 2));
}
