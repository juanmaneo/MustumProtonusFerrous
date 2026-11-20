//! Replays the oracle rows: Wine's `msvcirt` functions compiled from verbatim C with
//! AddressSanitizer and UBSan, behind a harness that prints each result. A row is
//! `operation arguments => output`. Flags and counts are decimal, bytes are hex, and
//! `-` stands for the empty byte string. No sanitizer-flagged vector was excluded.

use crate::pad::writepad;
use crate::print::print_integer;
use crate::read::{read_integer, read_unsigned_integer};
use crate::scan::scan_integer;
use std::fmt::Debug;
use std::str::FromStr;

const ROWS: &[&str] = &[
    "scan 0 31323320 => base=0 state=0 consumed=3 token=313233",
    "scan 0 3078314667 => base=16 state=0 consumed=4 token=30783146",
    "scan 0 2d303137 => base=8 state=1 consumed=4 token=2d303137",
    "scan 0 7831 => base=0 state=2 consumed=0 token=-",
    "scan 0 37 => base=0 state=1 consumed=1 token=37",
    "scan 0 3078 => base=16 state=2 consumed=0 token=-",
    "scan 16 30783132 => base=10 state=0 consumed=1 token=30",
    "scan 64 2d305861426364 => base=16 state=1 consumed=7 token=2d305861426364",
    "scan 32 3738 => base=8 state=0 consumed=1 token=37",
    "scan 0 31323334353637383930313233343536373839 => base=0 state=0 consumed=15 token=313233343536373839303132333435",
    "readint 0 -32768 32767 0 343030303020 => value=32767 state=2 consumed=5",
    "readint 0 -2147483648 2147483647 1 3939393939393939393939 => value=2147483647 state=3 consumed=11",
    "readint 0 -2147483648 2147483647 0 3939393939393939393939 => value=2147483647 state=1 consumed=11",
    "readint 16 -5 5 0 2d => value=0 state=2 consumed=0",
    "readuint 0 -32768 65535 3730303030 => value=65535 state=3 consumed=5",
    "readuint 0 -32768 65535 2d35 => value=4294967291 state=1 consumed=2",
    "readuint 0 -32768 65535 34323934393334363030 => value=4294934600 state=1 consumed=10",
    "readuint 0 -2147483648 4294967295 34323934393637323936 => value=4294967295 state=3 consumed=10",
    "print 160 -1 0 0 0 32 => out=303337373737373737373737",
    "print 16 -1 1 1 0 32 => out=3635353335",
    "print 64 -1 0 1 0 32 => out=66666666",
    "print 1024 -3 1 0 0 32 => out=2b34323934393637323933",
    "print 1024 5 0 0 6 48 => out=303030302b35",
    "print 200 255 0 0 8 46 => out=30782e2e2e2e6666",
    "pad 2 8 42 3132 - => out=31322a2a2a2a2a2a",
];

#[test]
fn oracle_rows_match_wine() {
    for row in ROWS {
        let (input, expected) = row.split_once(" => ").expect("row separator");
        assert_eq!(run_line(input), expected, "row: {row}");
    }
}

/// Runs one harness operation on the port and formats the result as the harness does.
pub(crate) fn run_line(line: &str) -> String {
    let fields: Vec<&str> = line.split_whitespace().collect();
    match fields.as_slice() {
        ["scan", flags, input] => {
            let input = unhex(input);
            let scan = scan_integer(&input, num(flags));
            format!(
                "base={} state={} consumed={} token={}",
                scan.num_base,
                scan.state,
                scan.consumed,
                hex(&input[..scan.consumed])
            )
        }
        ["readint", flags, min, max, set, input] => {
            let read = read_integer(
                &unhex(input),
                num(flags),
                num(min),
                num(max),
                num::<i32>(set) != 0,
            );
            format!(
                "value={} state={} consumed={}",
                read.value, read.state, read.consumed
            )
        }
        ["readuint", flags, min, max, input] => {
            let read = read_unsigned_integer(&unhex(input), num(flags), num(min), num(max));
            format!(
                "value={} state={} consumed={}",
                read.value, read.state, read.consumed
            )
        }
        ["pad", flags, width, fill, str1, str2] => {
            let out = writepad(
                num(flags),
                num(width),
                num(fill),
                &unhex(str1),
                &unhex(str2),
            );
            format!("out={}", hex(&out))
        }
        ["print", flags, n, unsig, shrt, width, fill] => {
            let out = print_integer(
                num(n),
                num(flags),
                num::<i32>(unsig) != 0,
                num::<i32>(shrt) != 0,
                num(width),
                num(fill),
            );
            format!("out={}", hex(&out))
        }
        _ => panic!("unknown harness operation: {line}"),
    }
}

fn num<T>(field: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    field.parse().expect("decimal field")
}

fn unhex(field: &str) -> Vec<u8> {
    if field == "-" {
        return Vec::new();
    }
    (0..field.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&field[i..i + 2], 16).expect("hex byte"))
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".to_string();
    }
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
