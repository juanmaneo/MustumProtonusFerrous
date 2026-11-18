//! Checks the port against rows recorded from the verbatim Wine C code
//! (`dlls/dpwsockx/dpwsockx_main.c`, built with ASan and UBSan, no reports).
//! Each row is `(commands, replies)`: the `|`-separated commands run in order, each on its own
//! input, and the `|`-separated replies must match exactly.

use crate::caps::get_caps;
use crate::connection::{compare_connections, resolve_sender_addr, ConnectionKey};
use crate::header::{make_mixed, msg_size, msg_token, HEADER_SIZE};
use crate::tcp::{body_size, grown_buffer_size, tcp_receive_step, TcpStep};
use crate::udp::{parse_datagram, Datagram};
use crate::MsgHeader;

#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    ("mixed 20 0xfab00000|mixed 1048575 0xbab00000|mixed 0 0|mixed 1048576 0xfab00000", "mixed=fab00014 size=20 token=fab00000|mixed=babfffff size=1048575 token=bab00000|mixed=00000000 size=0 token=00000000|mixed=fab00000 size=0 token=fab00000"),
    ("cmp 4 4294967295 1 5 0 0|cmp 5 7 0 5 7 0", "cmp=-1|cmp=0"),
    ("cmp 5 8 0 5 7 0|cmp 5 7 0 5 8 0|cmp 5 7 1 5 7 0|cmp 5 7 0 5 7 1", "cmp=1|cmp=-1|cmp=1|cmp=-1"),
    ("tcp 0 5 10|tcp 0 4 10|tcp 0 6 6", "rearm len=5|rearm len=6|complete"),
    ("tcp 0 10 10|tcp 0 12 10", "complete|complete"),
    ("tcp 0 0 10|tcp 10054 5 10", "drop|drop"),
    ("header 0xfab00014 0|header 0xfab00064 100", "ok body=0 size=0|ok body=80 size=100"),
    ("header 0xfab00064 70|header 0xfab00064 40", "ok body=80 size=140|ok body=80 size=80"),
    ("header 0xfab0ffff 0", "ok body=65515 size=65515"),
    ("header 0xfab00013 0|header 0xfab00000 0", "drop|drop"),
    ("header 1048567 0|header 1048568 0", "ok body=1048547 size=1048547|drop"),
    ("msgbody 0xfab00030 0x0200a8c0 0x0100007f", "hm body=28 addr=0200a8c0 recv=20"),
    ("msgbody 0xfab00014 0 0x0100007f", "hm body=0 addr=0100007f recv=20"),
    ("udp|udp 0102", "ok hdr=- body=|ok hdr=- body=0102"),
    ("udp 1400b0fa0200b80b7f0000010000000000000000", "ok hdr=- body=1400b0fa0200b80b7f0000010000000000000000"),
    ("udp 1c00b0fa0200b80b7f0000010000000000000000706c617901020304", "ok hdr=fab0001c:0100007f body=706c617901020304"),
    ("udp 1400b0fa0200b80b7f0000010000000000000000706c6179|udp 1800b0fa0200b80b7f0000010000000000000000706c6179", "ok hdr=fab00014:0100007f body=|ok hdr=fab00018:0100007f body=706c6179"),
    ("udp 1300b0fa0200b80b7f0000010000000000000000706c617900000000|udp 3000b0fa0200b80b7f0000010000000000000000706c617900000000", "err|err"),
    ("caps 0|caps 0xfffffffe", "flags=00010060 q=0 hb=0 lat=500 local=65536 hdr=20 timeout=5000 maxbuf=65479 maxpl=65536|flags=00010060 q=0 hb=0 lat=500 local=65536 hdr=20 timeout=5000 maxbuf=65479 maxpl=65536"),
    ("caps 1|caps 0xffffffff", "flags=00010060 q=0 hb=0 lat=500 local=65536 hdr=20 timeout=5000 maxbuf=1048547 maxpl=64|flags=00010060 q=0 hb=0 lat=500 local=65536 hdr=20 timeout=5000 maxbuf=1048547 maxpl=64"),
    ("enum 20 0x0bb8|enum 100 80", "enum hdr=1400b0fa0200b80b000000000000000000000000|enum hdr=6400b0fa02005000000000000000000000000000"),
];

#[test]
fn oracle_rows_match_wine() {
    for (commands, expected) in ROWS {
        assert_eq!(replay(commands), *expected, "commands: {commands}");
    }
}

#[test]
fn unknown_command_is_bad() {
    assert_eq!(run_command("bogus"), "bad-op");
    assert_eq!(run_command(""), "bad-op");
}

fn replay(commands: &str) -> String {
    commands
        .split('|')
        .map(run_command)
        .collect::<Vec<_>>()
        .join("|")
}

fn run_command(command: &str) -> String {
    let words: Vec<&str> = command.split_whitespace().collect();
    let arg = |index: usize| words.get(index).copied().unwrap_or("");
    match words.first().copied().unwrap_or("") {
        "mixed" => mixed_reply(num(arg(1)), num(arg(2))),
        "cmp" => {
            let a = key(num(arg(1)), num(arg(2)), num(arg(3)));
            let b = key(num(arg(4)), num(arg(5)), num(arg(6)));
            format!("cmp={}", compare_connections(&a, &b))
        }
        "tcp" => tcp_reply(num(arg(1)), num(arg(2)), num(arg(3))),
        "header" => header_reply(num(arg(1)), num(arg(2))),
        "msgbody" => msgbody_reply(num(arg(1)), num(arg(2)), num(arg(3))),
        "udp" => udp_reply(&unhex(arg(1))),
        "caps" => caps_reply(num(arg(1))),
        "enum" => {
            let header = MsgHeader::remote(num(arg(1)), num(arg(2)) as u16);
            format!("enum hdr={}", hex(&header.to_bytes()))
        }
        _ => "bad-op".to_string(),
    }
}

fn mixed_reply(size: u32, token: u32) -> String {
    let mixed = make_mixed(size, token);
    format!(
        "mixed={mixed:08x} size={} token={:08x}",
        msg_size(mixed),
        msg_token(mixed)
    )
}

fn key(port: u32, addr: u32, guaranteed: u32) -> ConnectionKey {
    ConnectionKey {
        sin_port: port as u16,
        sin_addr: addr,
        guaranteed: guaranteed != 0,
    }
}

fn tcp_reply(error: u32, transferred: u32, remaining: u32) -> String {
    match tcp_receive_step(error, transferred, remaining) {
        TcpStep::Drop => "drop".to_string(),
        TcpStep::Complete => "complete".to_string(),
        TcpStep::Rearm { remaining } => format!("rearm len={remaining}"),
    }
}

fn header_reply(mixed: u32, buffer_size: u32) -> String {
    match body_size(mixed) {
        Ok(body) => {
            let size = grown_buffer_size(buffer_size, body).unwrap_or(buffer_size);
            format!("ok body={body} size={size}")
        }
        Err(_) => "drop".to_string(),
    }
}

fn msgbody_reply(mixed: u32, header_addr: u32, connection_addr: u32) -> String {
    let body = body_size(mixed).expect("header accepted by the header step");
    let addr = resolve_sender_addr(header_addr, connection_addr);
    format!("hm body={body} addr={addr:08x} recv={HEADER_SIZE}")
}

fn udp_reply(data: &[u8]) -> String {
    match parse_datagram(data) {
        Err(_) => "err".to_string(),
        Ok(Datagram {
            header: Some(header),
            body,
        }) => format!(
            "ok hdr={:08x}:{:08x} body={}",
            header.mixed,
            header.sin_addr,
            hex(body)
        ),
        Ok(Datagram { header: None, body }) => format!("ok hdr=- body={}", hex(body)),
    }
}

fn caps_reply(flags: u32) -> String {
    let caps = get_caps(flags);
    format!(
        "flags={:08x} q={} hb={} lat={} local={} hdr={} timeout={} maxbuf={} maxpl={}",
        caps.flags,
        caps.max_queue_size,
        caps.hundred_baud,
        caps.latency,
        caps.max_local_players,
        caps.header_length,
        caps.timeout,
        caps.max_buffer_size,
        caps.max_players
    )
}

/// A decimal or `0x`-prefixed number, read the way the C harness reads it with `strtoul`.
fn num(text: &str) -> u32 {
    match text.strip_prefix("0x") {
        Some(digits) => u32::from_str_radix(digits, 16),
        None => text.parse(),
    }
    .expect("number")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex byte"))
        .collect()
}
