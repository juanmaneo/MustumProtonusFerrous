//! Boundary checks beyond the recorded Wine rows in `oracle_checks.rs`.

use crate::caps::{get_caps, DPGETCAPS_GUARANTEED};
use crate::connection::{compare_connections, resolve_sender_addr, ConnectionKey};
use crate::header::{make_mixed, msg_size, msg_token, AF_INET, HEADER_SIZE, TOKEN_REMOTE};
use crate::tcp::{
    body_size, grown_buffer_size, tcp_receive_step, HeaderError, TcpStep,
    GUARANTEED_MAX_BUFFER_SIZE,
};
use crate::udp::{parse_datagram, Datagram, DatagramError, SIGNATURE};
use crate::MsgHeader;

/// A datagram made of a header with `mixed` as its size word, followed by `tail`.
fn framed(mixed: u32, tail: &[u8]) -> Vec<u8> {
    let header = MsgHeader {
        mixed,
        sin_family: AF_INET,
        sin_port: 0,
        sin_addr: 0,
    };
    let mut data = header.to_bytes().to_vec();
    data.extend_from_slice(tail);
    data
}

fn key(port: u16, addr: u32, guaranteed: bool) -> ConnectionKey {
    ConnectionKey {
        sin_port: port,
        sin_addr: addr,
        guaranteed,
    }
}

#[test]
fn header_bytes_round_trip() {
    let header = MsgHeader {
        mixed: 0xFAB0_0014,
        sin_family: AF_INET,
        sin_port: 0x0BB8,
        sin_addr: 0x0100_007F,
    };
    let bytes = header.to_bytes();
    assert_eq!(
        bytes[..12],
        [0x14, 0x00, 0xB0, 0xFA, 0x02, 0x00, 0xB8, 0x0B, 0x7F, 0x00, 0x00, 0x01]
    );
    assert_eq!(bytes[12..], [0u8; 8]);
    assert_eq!(MsgHeader::from_bytes(&bytes), Some(header));
}

#[test]
fn sin_zero_padding_is_ignored() {
    let mut bytes = MsgHeader::remote(20, 1).to_bytes();
    bytes[12..].copy_from_slice(&[0xAA; 8]);
    assert_eq!(
        MsgHeader::from_bytes(&bytes),
        Some(MsgHeader::remote(20, 1))
    );
}

#[test]
fn short_bytes_are_not_a_header() {
    assert_eq!(MsgHeader::from_bytes(&[0; 19]), None);
    assert!(MsgHeader::from_bytes(&[0; 20]).is_some());
}

#[test]
fn remote_header_matches_enum_sessions() {
    assert_eq!(
        MsgHeader::remote(20, 0x1234),
        MsgHeader {
            mixed: TOKEN_REMOTE | 20,
            sin_family: AF_INET,
            sin_port: 0x1234,
            sin_addr: 0,
        }
    );
}

#[test]
fn mixed_word_splits_into_size_and_token() {
    let mixed = make_mixed(0xF_FFFF, TOKEN_REMOTE);
    assert_eq!(msg_size(mixed), 0xF_FFFF);
    assert_eq!(msg_token(mixed), TOKEN_REMOTE);
    assert_eq!(msg_size(0xFFFF_FFFF), 0xF_FFFF);
    assert_eq!(msg_token(0xFFFF_FFFF), 0xFFF0_0000);
}

#[test]
fn oversized_message_size_sets_token_bits() {
    // Wine ORs the size in without masking, so bit 20 lands in the token.
    let mixed = make_mixed(0x10_0000, 0);
    assert_eq!(mixed, 0x10_0000);
    assert_eq!(msg_size(mixed), 0);
    assert_eq!(msg_token(mixed), 0x10_0000);
}

#[test]
fn body_size_bounds() {
    assert_eq!(body_size(TOKEN_REMOTE | 20), Ok(0));
    assert_eq!(body_size(19), Err(HeaderError::TooShort));
    assert_eq!(body_size(0), Err(HeaderError::TooShort));
    assert_eq!(
        body_size(TOKEN_REMOTE | 1_048_567),
        Ok(GUARANTEED_MAX_BUFFER_SIZE)
    );
    assert_eq!(body_size(1_048_568), Err(HeaderError::TooLong));
    assert_eq!(body_size(0xFFFF_FFFF), Err(HeaderError::TooLong));
}

#[test]
fn buffer_growth_rule() {
    assert_eq!(grown_buffer_size(100, 100), None);
    assert_eq!(grown_buffer_size(200, 100), None);
    assert_eq!(grown_buffer_size(0, 0), None);
    assert_eq!(grown_buffer_size(60, 100), Some(120));
    assert_eq!(grown_buffer_size(10, 100), Some(100));
    assert_eq!(grown_buffer_size(u32::MAX - 1, u32::MAX), Some(u32::MAX));
}

#[test]
fn tcp_read_outcomes() {
    assert_eq!(tcp_receive_step(10054, 5, 10), TcpStep::Drop);
    assert_eq!(tcp_receive_step(0, 0, 0), TcpStep::Drop);
    assert_eq!(tcp_receive_step(0, 3, 3), TcpStep::Complete);
    assert_eq!(tcp_receive_step(0, 1, 4), TcpStep::Rearm { remaining: 3 });
}

#[test]
fn explicit_sender_address_is_kept() {
    assert_eq!(resolve_sender_addr(0x0200_A8C0, 0x0100_007F), 0x0200_A8C0);
    assert_eq!(resolve_sender_addr(0, 0), 0);
}

#[test]
fn connections_order_by_port_then_address_then_guaranteed() {
    assert_eq!(
        compare_connections(&key(1, u32::MAX, true), &key(2, 0, false)),
        -1
    );
    assert_eq!(compare_connections(&key(1, 5, true), &key(1, 5, false)), 1);
    assert_eq!(compare_connections(&key(1, 5, false), &key(1, 5, false)), 0);

    let mut keys = vec![
        key(2, 0, false),
        key(1, 9, true),
        key(1, 9, false),
        key(1, 2, true),
    ];
    keys.sort();
    assert_eq!(
        keys,
        vec![
            key(1, 2, true),
            key(1, 9, false),
            key(1, 9, true),
            key(2, 0, false)
        ]
    );
}

#[test]
fn signature_needs_all_four_bytes() {
    let data = framed(TOKEN_REMOTE | 24, &SIGNATURE[..3]);
    assert_eq!(data.len(), 23);
    assert_eq!(
        parse_datagram(&data),
        Ok(Datagram {
            header: None,
            body: &data,
        })
    );
}

#[test]
fn framed_datagram_bounds() {
    let exact = framed(TOKEN_REMOTE | 24, SIGNATURE);
    let parsed = parse_datagram(&exact).expect("exact size");
    assert_eq!(parsed.body, SIGNATURE);
    assert_eq!(
        parsed.header.map(|header| header.mixed),
        Some(TOKEN_REMOTE | 24)
    );

    let empty = framed(TOKEN_REMOTE | 20, SIGNATURE);
    assert!(parse_datagram(&empty).expect("empty body").body.is_empty());

    let trailing = framed(TOKEN_REMOTE | 24, b"playXYZW");
    assert_eq!(
        parse_datagram(&trailing).expect("trailing bytes").body,
        SIGNATURE
    );

    let too_short = framed(TOKEN_REMOTE | 19, SIGNATURE);
    assert_eq!(parse_datagram(&too_short), Err(DatagramError::TooShort));

    let truncated = framed(TOKEN_REMOTE | 25, SIGNATURE);
    assert_eq!(parse_datagram(&truncated), Err(DatagramError::Truncated));
}

#[test]
fn caps_read_only_the_guaranteed_bit() {
    let guaranteed = get_caps(DPGETCAPS_GUARANTEED | 0x8000_0000);
    assert_eq!(guaranteed.max_buffer_size, GUARANTEED_MAX_BUFFER_SIZE);
    assert_eq!(guaranteed.max_players, 64);

    let plain = get_caps(0xFFFF_FFFE);
    assert_eq!(plain.max_buffer_size, 65_479);
    assert_eq!(plain.max_players, 65_536);
    assert_eq!(plain.flags, 0x0001_0060);
    assert_eq!(plain.header_length, HEADER_SIZE as u32);
}
