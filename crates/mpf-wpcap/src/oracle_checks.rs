//! Replays the Wine oracle (`target/canon/wpcap`) against this crate.
//! `driver.c` compiles the verbatim `pcap_statustostr`, `convert_length_to_ipv6_mask`,
//! `build_win32_name`, `build_win32_description`, `pcap_dispatch` and `pcap_loop`
//! code from `wpcap.c` and `unixlib.h` with `gcc -fsanitize=address,undefined -g`,
//! and replaces `pcap_next_ex` with a script. The rows are that output, verbatim.
//!
//! Excluded: mask lengths 0-7 and 136-200. ASan reports a stack-buffer-overflow
//! for them in the C code, so there is no defined oracle value.

use super::*;

struct StatusRow {
    status: i32,
    text: &'static str,
}

const STATUS_ROWS: &[StatusRow] = &[
    StatusRow {
        status: 1,
        text: "Generic warning",
    },
    StatusRow {
        status: 2,
        text: "That device doesn't support promiscuous mode",
    },
    StatusRow {
        status: 3,
        text: "That type of time stamp is not supported by that device",
    },
    StatusRow {
        status: -1,
        text: "Generic error",
    },
    StatusRow {
        status: -4,
        text: "The setting can't be changed after the pcap_t is activated",
    },
    StatusRow {
        status: -6,
        text: "Unknown error: -6",
    },
];

struct MaskRow {
    length: u32,
    hex: &'static str,
}

const MASK_ROWS: &[MaskRow] = &[
    MaskRow {
        length: 8,
        hex: "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00",
    },
    MaskRow {
        length: 9,
        hex: "80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00",
    },
    MaskRow {
        length: 12,
        hex: "f0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00",
    },
    MaskRow {
        length: 16,
        hex: "ff 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00",
    },
    MaskRow {
        length: 128,
        hex: "ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff 00",
    },
    MaskRow {
        length: 134,
        hex: "ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff fc",
    },
    MaskRow {
        length: 135,
        hex: "ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff fe",
    },
];

struct NameRow {
    source: Option<&'static str>,
    adapter: &'static str,
    text: &'static str,
}

const NAME_ROWS: &[NameRow] = &[
    NameRow {
        source: Some("rpcap://"),
        adapter: "{ABC}",
        text: "rpcap://\\Device\\NPF_{ABC}",
    },
    NameRow {
        source: None,
        adapter: "{ABC}",
        text: "\\Device\\NPF_{ABC}",
    },
];

struct DescriptionRow {
    name: &'static str,
    description: Option<&'static str>,
    text: &'static str,
}

const DESCRIPTION_ROWS: &[DescriptionRow] = &[
    DescriptionRow {
        name: "eth0",
        description: Some("Ethernet adapter"),
        text: "Ethernet adapter eth0",
    },
    DescriptionRow {
        name: "lo",
        description: None,
        text: "lo",
    },
];

#[derive(Clone, Copy)]
pub(super) enum Driver {
    Dispatch,
    PacketLoop,
}

struct PacketRow {
    name: &'static str,
    driver: Driver,
    count: i32,
    script: &'static [Result<u32, i32>],
    ret: i32,
    calls: &'static [u32],
    consumed: usize,
}

const PACKET_ROWS: &[PacketRow] = &[
    PacketRow {
        name: "dispatch d1",
        driver: Driver::Dispatch,
        count: 0,
        script: &[Ok(0), Ok(1), Err(0)],
        ret: 2,
        calls: &[0, 1],
        consumed: 3,
    },
    PacketRow {
        name: "dispatch d2",
        driver: Driver::Dispatch,
        count: 2,
        script: &[Ok(0), Ok(1), Ok(2)],
        ret: 2,
        calls: &[0, 1],
        consumed: 2,
    },
    PacketRow {
        name: "dispatch d3",
        driver: Driver::Dispatch,
        count: 0,
        script: &[Err(-2)],
        ret: -2,
        calls: &[],
        consumed: 1,
    },
    PacketRow {
        name: "dispatch d4",
        driver: Driver::Dispatch,
        count: 0,
        script: &[Ok(0), Err(-1)],
        ret: -1,
        calls: &[0],
        consumed: 2,
    },
    PacketRow {
        name: "loop l1",
        driver: Driver::PacketLoop,
        count: 0,
        script: &[Err(0), Ok(0), Err(0), Err(-2)],
        ret: 1,
        calls: &[0],
        consumed: 4,
    },
    PacketRow {
        name: "loop l2",
        driver: Driver::PacketLoop,
        count: 0,
        script: &[Err(-2)],
        ret: -2,
        calls: &[],
        consumed: 1,
    },
    PacketRow {
        name: "loop l3",
        driver: Driver::PacketLoop,
        count: 2,
        script: &[Err(0), Ok(0), Err(0), Ok(1), Ok(2)],
        ret: 2,
        calls: &[0, 1],
        consumed: 4,
    },
];

/// Runs a packet driver over `script`, returning its result, the callbacks it made
/// and how many scripted steps it consumed. The edge tests use it too, so each
/// generic instantiation of `dispatch` and `packet_loop` is covered by one set of closures.
pub(super) fn run_packets(
    driver: Driver,
    count: i32,
    script: &[Result<u32, i32>],
) -> (i32, Vec<u32>, usize) {
    let mut pos = 0;
    let mut calls = Vec::new();
    let next = || {
        let step = script[pos];
        pos += 1;
        step
    };
    let ret = match driver {
        Driver::Dispatch => dispatch(count, next, |id| calls.push(id)),
        Driver::PacketLoop => packet_loop(count, next, |id| calls.push(id)),
    };
    (ret, calls, pos)
}

#[test]
fn statustostr_matches_oracle() {
    for row in STATUS_ROWS {
        assert_eq!(statustostr(row.status), row.text, "status {}", row.status);
    }
}

#[test]
fn ipv6_prefix_mask_matches_oracle() {
    for row in MASK_ROWS {
        let mask = ipv6_prefix_mask(row.length).expect("in-range prefix length");
        let hex: Vec<String> = mask.iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(hex.join(" "), row.hex, "mask {}", row.length);
    }
}

#[test]
fn build_win32_name_matches_oracle() {
    for row in NAME_ROWS {
        assert_eq!(build_win32_name(row.source, row.adapter), row.text);
    }
}

#[test]
fn build_win32_description_matches_oracle() {
    for row in DESCRIPTION_ROWS {
        assert_eq!(build_win32_description(row.name, row.description), row.text);
    }
}

#[test]
fn packet_drivers_match_oracle() {
    for row in PACKET_ROWS {
        let (ret, calls, consumed) = run_packets(row.driver, row.count, row.script);
        assert_eq!(ret, row.ret, "{} return", row.name);
        assert_eq!(calls, row.calls, "{} callbacks", row.name);
        assert_eq!(consumed, row.consumed, "{} consumed", row.name);
    }
}
