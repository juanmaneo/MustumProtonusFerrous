//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/wintab32/harness.c`, built with ASan and UBSan, no reports).
//! Each row is `(commands, replies)`: the `|`-separated commands run in order on one fresh
//! tablet state, and the `|`-separated replies must match exactly.

use std::fmt::Debug;
use std::str::FromStr;

use crate::context::Context;
use crate::info::{is_logcontext_category, is_string_field};
use crate::packet::{encode, encode_all, encoded_len, fill, Packet};
use crate::scale::{mul_div, scale_for_context, Axis};
use crate::Accepted;

/// The handle of the first context opened in a row (`gTopContext` starts at 0xc00).
const FIRST_HANDLE: u32 = 0xc00;
/// The fields the harness's `push` command sets besides the ones it takes as arguments.
const SAMPLE_TIME: i32 = -5;
const SAMPLE_ORIENTATION: [i32; 3] = [7, -8, 9];
const SAMPLE_ROTATION: [i32; 3] = [10, 11, -12];

/// Rows recorded from the verbatim Wine C code: `(commands, replies)`.
#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    ("cat 1 1|cat 1 6|cat 2 1|cat 3 1|cat 3 2|cat 4 1|cat 400 1|cat 500 1|cat 200 1|cat 200 2|cat 209 6|cat 210 1|cat 199 1|cat 100 1|cat 100 2|cat 100 19|cat 100 20", "ctx=0 str=1|ctx=0 str=0|ctx=0 str=0|ctx=1 str=1|ctx=1 str=0|ctx=1 str=1|ctx=1 str=1|ctx=0 str=0|ctx=0 str=1|ctx=0 str=0|ctx=0 str=1|ctx=0 str=0|ctx=0 str=0|ctx=0 str=1|ctx=0 str=0|ctx=0 str=1|ctx=0 str=0"),
    ("muldiv 3 4 5|muldiv -3 4 5|muldiv 3 -4 5|muldiv 3 4 -5|muldiv -3 -4 5|muldiv 5 1 2|muldiv -5 1 2|muldiv 7 3 0|muldiv 0 99 1|muldiv 7 -3 2|muldiv -7 -3 2|muldiv 10 10 3", "2|-2|-2|-2|2|3|-3|-1|0|-11|11|33"),
    ("muldiv 2147483647 2 1|muldiv -2147483647 2 1|muldiv 65536 65536 1|muldiv 2147483647 1 1|muldiv -2147483647 1 1|muldiv 1000000 3000 1000|muldiv 2147483647 -1 -2147483647|muldiv -2147483647 -2 1", "-1|-1|-1|2147483647|-2147483647|3000000|1|-1"),
    ("scale 50 0 100 0 200|scale 50 10 100 5 200|scale 10 50 100 0 200|scale 30 0 -100 0 -200|scale 30 0 -100 0 200|scale 30 0 100 0 -200|scale 5 0 0 0 100|scale 0 0 100 -5 100", "100|85|4294967216|60|140|140|4294967295|4294967291"),
    ("scale 4294967295 0 100 0 100|scale 0 -1 100 0 100|scale 1000 500 1000 0 -10|scale 2000 0 1000 0 10|scale 1 0 -5 3 -7|scale 7 2 0 0 0|scale 3 0 3 0 2|scale 1 0 3 0 2", "4294967295|1|5|20|4|4294967295|2|1"),
    ("open 1 0 190|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 0 0 0 0 0|packet 77|list", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=0:50:400:16|ret=1 hex=4d0000003200000090010000|ser=-"),
    ("open 1 0 190|axes 0 1000 0 -500 0 1000 0 -500|push 100 200 16 5 77 0 0 0 0 0|get 3", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=0:450:100:16|ret=1 hex=4d000000c201000064000000000000000000000000000000000000000000000000000000"),
    ("open 1 0 190|axes 0 1000 0 500 0 -1000 0 500|push 100 2000 16 5 77 0 0 0 0 0|get 2", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=0:50:1000:16|ret=1 hex=4d00000032000000e8030000000000000000000000000000"),
    ("open 1 0 3fff|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 6 7 8 9 10|packet 77", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=6:50:400:16|ret=1 hex=000c00000000000006000000fbffffff100000004d0000000500000007000000320000009001000008000000090000000a00000007000000f8ffffff090000000a0000000b000000f4ffffff"),
    ("open 1 0 1|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 6 7 8 9 10|packet 77", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=6:50:400:0|ret=1 hex=000c000000000000"),
    ("open 1 0 2|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 6 7 8 9 10|packet 77", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=6:50:400:0|ret=1 hex=06000000"),
    ("open 1 0 3000|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 6 7 8 9 10|packet 77", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=6:50:400:0|ret=1 hex=07000000f8ffffff090000000a0000000b000000f4ffffff"),
    ("open 1 0 e04|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 6 7 8 9 10|packet 77", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=6:50:400:0|ret=1 hex=fbffffff08000000090000000a000000"),
    ("open 1 0 78|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 6 7 8 9 10|packet 77", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=6:50:400:16|ret=1 hex=100000004d0000000500000007000000"),
    ("open 1 0 190|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 5 0 0 0 0 0|push 101 201 16 5 6 0 0 0 0 0|push 102 202 16 5 7 0 0 0 0 0|push 103 203 16 5 8 0 0 0 0 0|push 104 204 16 5 9 0 0 0 0 0|packet 6|list|packet 9|list|packet 99", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=0:50:400:16|ret=1 q=2 act=5 posted=0 last=0:51:399:16|ret=1 q=3 act=5 posted=0 last=0:51:399:16|ret=1 q=4 act=5 posted=0 last=0:52:398:16|ret=1 q=5 act=5 posted=0 last=0:52:398:16|ret=2 hex=06000000330000008f010000|ser=7,8,9|ret=3 hex=09000000340000008e010000|ser=-|ret=0 hex=000000000000000000000000"),
    ("open 1 0 190|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 5 0 0 0 0 0|push 101 201 16 5 6 0 0 0 0 0|push 102 202 16 5 7 0 0 0 0 0|push 103 203 16 5 8 0 0 0 0 0|push 104 204 16 5 9 0 0 0 0 0|get 2|list|get 5|list", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=0:50:400:16|ret=1 q=2 act=5 posted=0 last=0:51:399:16|ret=1 q=3 act=5 posted=0 last=0:51:399:16|ret=1 q=4 act=5 posted=0 last=0:52:398:16|ret=1 q=5 act=5 posted=0 last=0:52:398:16|ret=2 hex=05000000320000009001000006000000330000008f010000|ser=7,8,9|ret=3 hex=07000000330000008f01000008000000340000008e01000009000000340000008e010000000000000000000000000000000000000000000000000000|ser=-"),
    ("open 1 0 190|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 5 0 0 0 0 0|push 101 201 16 5 6 0 0 0 0 0|push 102 202 16 5 7 0 0 0 0 0|push 103 203 16 5 8 0 0 0 0 0|push 104 204 16 5 9 0 0 0 0 0|peek 2|list|peek 9|list", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=0:50:400:16|ret=1 q=2 act=5 posted=0 last=0:51:399:16|ret=1 q=3 act=5 posted=0 last=0:51:399:16|ret=1 q=4 act=5 posted=0 last=0:52:398:16|ret=1 q=5 act=5 posted=0 last=0:52:398:16|ret=2 hex=05000000320000009001000006000000330000008f010000|ser=5,6,7,8,9|ret=5 hex=05000000320000009001000006000000330000008f01000007000000330000008f01000008000000340000008e01000009000000340000008e010000|ser=5,6,7,8,9"),
    ("open 1 0 190|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 5 0 0 0 0 0|push 101 201 16 5 6 0 0 0 0 0|push 102 202 16 5 7 0 0 0 0 0|push 103 203 16 5 8 0 0 0 0 0|push 104 204 16 5 9 0 0 0 0 0|dget 6 7|list|dpeek 5 8|list|dget 5 9|list", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=0:50:400:16|ret=1 q=2 act=5 posted=0 last=0:51:399:16|ret=1 q=3 act=5 posted=0 last=0:51:399:16|ret=1 q=4 act=5 posted=0 last=0:52:398:16|ret=1 q=5 act=5 posted=0 last=0:52:398:16|ret=2 n=2 hex=06000000330000008f01000007000000330000008f010000|ser=5,8,9|ret=2 n=2 hex=05000000320000009001000008000000340000008e010000|ser=5,8,9|ret=3 n=3 hex=05000000320000009001000008000000340000008e01000009000000340000008e010000|ser=-"),
    ("open 1 0 190|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 5 0 0 0 0 0|push 101 201 16 5 6 0 0 0 0 0|push 102 202 16 5 7 0 0 0 0 0|push 103 203 16 5 8 0 0 0 0 0|push 104 204 16 5 9 0 0 0 0 0|dget 42 6|dpeek 42 6|dpeek 9 5|dget 7 7|list", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=0:50:400:16|ret=1 q=2 act=5 posted=0 last=0:51:399:16|ret=1 q=3 act=5 posted=0 last=0:51:399:16|ret=1 q=4 act=5 posted=0 last=0:52:398:16|ret=1 q=5 act=5 posted=0 last=0:52:398:16|ret=0 n=0 hex=|ret=0 n=0 hex=|ret=0 n=0 hex=|ret=1 n=1 hex=07000000330000008f010000|ser=5,6,8,9"),
    ("open 1 0 190|qb|push 100 200 16 5 5 0 0 0 0 0|push 101 201 16 5 6 0 0 0 0 0|push 102 202 16 5 7 0 0 0 0 0|qb|packet 5|qb", "h=3072|fail|ret=1 q=1 act=5 posted=0 last=0:4294967295:4294967295:16|ret=1 q=2 act=5 posted=0 last=0:4294967295:4294967295:16|ret=1 q=3 act=5 posted=0 last=0:4294967295:4294967295:16|ok old=5 new=7|ret=1 hex=05000000ffffffffffffffff|ok old=6 new=7"),
    ("open 1 0 192|qsize 2|push 100 200 16 5 1 0 0 0 0 0|push 101 201 16 5 2 0 0 0 0 0|push 102 202 16 5 3 0 0 0 0 0|list|get 3", "h=3072|qsize=2|ret=1 q=1 act=5 posted=0 last=0:4294967295:4294967295:16|ret=1 q=2 act=5 posted=0 last=0:4294967295:4294967295:16|ret=1 q=2 act=5 posted=0 last=2:4294967295:4294967295:16|ser=1,2|ret=2 hex=0000000001000000ffffffffffffffff0200000002000000ffffffffffffffff00000000000000000000000000000000"),
    ("open 0 0 190|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 0 0 0 0 0|enable 1|push 100 200 16 5 78 0 0 0 0 0|enable 0|list|enable 0", "h=3072|ok|ret=0 q=0 act=4294967295 posted=0 last=-|st=4 en=1 q=0|ret=1 q=1 act=5 posted=0 last=0:50:400:16|st=1 en=0 q=0|ser=-|st=1 en=0 q=0"),
    ("open 1 0 190|overlap 1|overlap 1|overlap 0|overlap 1|enable 0|overlap 0|overlap 1", "h=3072|st=4 q=0|st=4 q=0|st=2 q=0|st=4 q=0|st=1 en=0 q=0|st=2 q=0|st=4 q=0"),
    ("open 1 c 190|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 0 0 0 0 0|push 100 200 16 5 78 0 0 0 0 0|push 100 200 16 6 79 0 0 0 0 0", "h=3072|ok|ret=1 q=1 act=5 posted=1 last=0:50:400:16|ret=1 q=2 act=5 posted=0 last=0:50:400:16|ret=1 q=3 act=6 posted=1 last=0:50:400:16"),
    ("open 1 8 190|axes 0 1000 0 500 0 1000 0 500|push 100 200 16 5 77 0 0 0 0 0|push 100 200 16 6 78 0 0 0 0 0", "h=3072|ok|ret=1 q=1 act=5 posted=0 last=0:50:400:16|ret=1 q=2 act=6 posted=0 last=0:50:400:16"),
];

#[test]
fn oracle_rows_match_wine() {
    for (commands, expected) in ROWS {
        assert_eq!(replay(commands), *expected, "commands: {commands}");
    }
}

#[test]
fn unknown_command_is_bad() {
    assert_eq!(Replay::default().run("bogus"), "bad");
}

fn replay(commands: &str) -> String {
    let mut state = Replay::default();
    commands
        .split('|')
        .map(|command| state.run(command))
        .collect::<Vec<_>>()
        .join("|")
}

/// The tablet state of one row: the context the last `open` command created.
#[derive(Default)]
struct Replay {
    current: Option<Context>,
}

impl Replay {
    fn run(&mut self, command: &str) -> String {
        let words: Vec<&str> = command.split(' ').collect();
        match words.as_slice() {
            ["cat", category, index] => format!(
                "ctx={} str={}",
                u8::from(is_logcontext_category(arg(category))),
                u8::from(is_string_field(arg(category), arg(index)))
            ),
            ["muldiv", a, b, c] => mul_div(arg(a), arg(b), arg(c)).to_string(),
            ["scale", input, in_org, in_ext, out_org, out_ext] => scale_for_context(
                arg(input),
                Axis {
                    in_org: arg(in_org),
                    in_ext: arg(in_ext),
                    out_org: arg(out_org),
                    out_ext: arg(out_ext),
                },
            )
            .to_string(),
            ["open", enable, options, pk_data] => {
                self.current = Some(Context::open(
                    FIRST_HANDLE,
                    arg::<u32>(enable) != 0,
                    hex_arg(options),
                    hex_arg(pk_data),
                ));
                format!("h={FIRST_HANDLE}")
            }
            ["axes", in_org_x, in_ext_x, out_org_x, out_ext_x, in_org_y, in_ext_y, out_org_y, out_ext_y] =>
            {
                let x_axis = Axis {
                    in_org: arg(in_org_x),
                    in_ext: arg(in_ext_x),
                    out_org: arg(out_org_x),
                    out_ext: arg(out_ext_x),
                };
                let y_axis = Axis {
                    in_org: arg(in_org_y),
                    in_ext: arg(in_ext_y),
                    out_org: arg(out_org_y),
                    out_ext: arg(out_ext_y),
                };
                self.context().set_axes(x_axis, y_axis);
                "ok".to_string()
            }
            ["qsize", size] => {
                let queue = self.context().queue_mut();
                queue.set_capacity(arg(size));
                format!("qsize={}", queue.capacity())
            }
            ["enable", on] => {
                let context = self.context();
                context.set_enabled(arg::<u32>(on) != 0);
                format!(
                    "st={} en={} q={}",
                    context.status(),
                    u8::from(context.enabled()),
                    context.queue().packets().len()
                )
            }
            ["overlap", on] => {
                let context = self.context();
                context.set_overlap(arg::<u32>(on) != 0);
                format!(
                    "st={} q={}",
                    context.status(),
                    context.queue().packets().len()
                )
            }
            ["push", x, y, changed, cursor, serial, status, buttons, z, normal_pressure, tangent_pressure] =>
            {
                let packet = Packet {
                    x: arg(x),
                    y: arg(y),
                    changed: arg(changed),
                    cursor: arg(cursor),
                    serial: arg(serial),
                    status: arg(status),
                    buttons: arg(buttons),
                    z: arg(z),
                    normal_pressure: arg(normal_pressure),
                    tangent_pressure: arg(tangent_pressure),
                    time: SAMPLE_TIME,
                    orientation: SAMPLE_ORIENTATION,
                    rotation: SAMPLE_ROTATION,
                    ..Packet::default()
                };
                let context = self.context();
                let accepted = context.accept(packet);
                let queued = context.queue().packets();
                let last = match queued.last() {
                    Some(p) => format!("{}:{}:{}:{}", p.status, p.x, p.y, p.changed),
                    None => "-".to_string(),
                };
                let posted = matches!(accepted, Some(Accepted::Queued { csr_message: true }));
                format!(
                    "ret={} q={} act={} posted={} last={last}",
                    u8::from(accepted.is_some()),
                    queued.len(),
                    context.active_cursor(),
                    u8::from(posted)
                )
            }
            ["packet", serial] => {
                let context = self.context();
                let mask = context.pk_data();
                match context.queue_mut().take_through(arg(serial)) {
                    Some((removed, packet)) => {
                        format!("ret={removed} hex={}", hex(&encode(mask, &packet)))
                    }
                    None => format!("ret=0 hex={}", hex(&vec![0u8; encoded_len(mask)])),
                }
            }
            ["get", max] => {
                let context = self.context();
                let mask = context.pk_data();
                let max: usize = arg(max);
                let taken = context.queue_mut().take_front(max);
                format!("ret={} hex={}", taken.len(), hex(&fill(mask, &taken, max)))
            }
            ["peek", max] => {
                let context = self.context();
                let packets = context.queue().peek_front(arg(max));
                format!(
                    "ret={} hex={}",
                    packets.len(),
                    hex(&encode_all(context.pk_data(), packets))
                )
            }
            ["dget", begin, end] => {
                let context = self.context();
                let mask = context.pk_data();
                let range = context
                    .queue()
                    .data_range(arg(begin), arg(end))
                    .unwrap_or(0..0);
                let taken = context.queue_mut().take_range(range);
                let count = taken.len();
                format!(
                    "ret={count} n={count} hex={}",
                    hex(&encode_all(mask, &taken))
                )
            }
            ["dpeek", begin, end] => {
                let context = self.context();
                let mask = context.pk_data();
                let range = context
                    .queue()
                    .data_range(arg(begin), arg(end))
                    .unwrap_or(0..0);
                let packets = &context.queue().packets()[range];
                let count = packets.len();
                format!(
                    "ret={count} n={count} hex={}",
                    hex(&encode_all(mask, packets))
                )
            }
            ["qb"] => match self.context().queue().ends() {
                Some((old, new)) => format!("ok old={old} new={new}"),
                None => "fail".to_string(),
            },
            ["list"] => {
                let serials: Vec<String> = self
                    .context()
                    .queue()
                    .packets()
                    .iter()
                    .map(|p| p.serial.to_string())
                    .collect();
                if serials.is_empty() {
                    "ser=-".to_string()
                } else {
                    format!("ser={}", serials.join(","))
                }
            }
            _ => "bad".to_string(),
        }
    }

    fn context(&mut self) -> &mut Context {
        self.current
            .as_mut()
            .expect("every row opens a context before its packet commands")
    }
}

fn arg<T>(text: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    text.parse().expect("decimal argument")
}

fn hex_arg(text: &str) -> u32 {
    u32::from_str_radix(text, 16).expect("hex argument")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
