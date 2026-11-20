//! Rows from the Wine oracle: the verbatim C driver, built with ASan and UBSan, run on each case
//! line. The oracle has 135 clean rows, generated under `target/canon/rometadata` (gitignored).
//! It also has 6 cases that the C asserts flag, which are left out. These 24 rows cover each
//! operation of the driver and each `None` outcome. `edge_checks.rs` covers the
//! remaining branches directly.

use crate::coded::{bit_width, coded_index_size, coded_value_as_token, index_size};
use crate::enumeration::TokenEnum;
use crate::heap::{
    blob_at, decode_compressed_uint, guid_offset, span_in_bounds, string_at, trim_string_heap,
};
use crate::pe::{rva_to_offset, Section};
use crate::typepath::split_type_name;

/// `(input line, expected output)`, in the driver's format. Each expected output was copied from
/// the oracle's `oracle.txt`.
const ROWS: &[(&str, &str)] = &[
    ("bits|2147483648", "31"),
    ("idxsize|65536", "4"),
    ("cis|5,-,7", "2"),
    ("coded|27|0,1,2,3,4,5,6,7,8,9,10,11,12", "b000001"),
    ("rva|-|5", "none"),
    ("rva|100:50:4000,200:10:8000|205", "ok:8005"),
    ("split|A.B.C", "A.B|C"),
    ("eq|A.B|A.BC", "0"),
    ("eq|System.X|System.X", "1"),
    ("count-range|1|4294967295", "4294967295"),
    ("count-list|ff,ee,dd,cc,bb", "5"),
    ("ent-range|2000000|1|3|2|10", "1:2000003:4"),
    (
        "ent-range|2000000|4294967295|4294967295|-|2",
        "2:ffffffff,2000000:1",
    ),
    (
        "ent-list|2000001,2000002,2000003|-|10",
        "3:2000001,2000002,2000003:3",
    ),
    (
        "ent-list|2000001,2000002,2000003|1|2",
        "2:2000002,2000003:3",
    ),
    ("blob|0301ab|1", "ok:1:ab"),
    ("str|68656c6c6f00|3", "ok:6c6f00"),
    ("guid|32|2147483649", "ok:0"),
    ("addr|16|8|8", "ok:8"),
    ("trim|006100", "ok:3"),
    ("trim|61", "err"),
    ("uint|dfffffff", "ok:536870911:4"),
    ("uint|e0", "none"),
    ("nope|1", "unknown"),
];

/// Runs one case line through the port, formatting the result as the C driver does.
fn run(line: &str) -> String {
    let fields: Vec<&str> = line.split('|').collect();
    match fields[0] {
        "bits" => bit_width(dec(fields[1])).to_string(),
        "idxsize" => index_size(dec(fields[1])).to_string(),
        "cis" => {
            let tables: Vec<Option<u32>> = fields[1]
                .split(',')
                .map(|item| if item == "-" { None } else { Some(dec(item)) })
                .collect();
            coded_index_size(&tables).to_string()
        }
        "coded" => {
            let tables: Vec<u32> = fields[2].split(',').map(dec).collect();
            let token = coded_value_as_token(dec(fields[1]), &tables)
                .expect("the oracle has no coded value with an out-of-range tag");
            format!("{token:x}")
        }
        "rva" => {
            let sections: Vec<Section> = if fields[1] == "-" {
                Vec::new()
            } else {
                fields[1].split(',').map(parse_section).collect()
            };
            or_none(rva_to_offset(&sections, dec(fields[2])).map(|offset| offset.to_string()))
        }
        "split" => {
            let path = split_type_name(fields[1].as_bytes());
            format!("{}|{}", text(path.namespace), text(path.type_name))
        }
        "eq" => {
            let left = split_type_name(fields[1].as_bytes());
            let right = split_type_name(fields[2].as_bytes());
            u8::from(left.equals(&right)).to_string()
        }
        "count-range" => TokenEnum::range(0, dec(fields[1]), dec(fields[2]))
            .count()
            .to_string(),
        "count-list" => TokenEnum::list(hex_list(fields[1])).count().to_string(),
        "ent-range" => entries(
            TokenEnum::range(hex(fields[1]), dec(fields[2]), dec(fields[3])),
            fields[4],
            dec(fields[5]),
        ),
        "ent-list" => entries(
            TokenEnum::list(hex_list(fields[1])),
            fields[2],
            dec(fields[3]),
        ),
        "blob" => {
            let heap = from_hex(fields[1]);
            or_none(
                blob_at(&heap, dec(fields[2]))
                    .map(|(size, tail)| format!("{size}:{}", to_hex(tail))),
            )
        }
        "str" => {
            let heap = from_hex(fields[1]);
            or_none(string_at(&heap, dec(fields[2])).map(to_hex))
        }
        "guid" => {
            or_none(guid_offset(dec(fields[2]), dec(fields[1])).map(|offset| offset.to_string()))
        }
        "addr" => {
            let offset = dec(fields[2]);
            let fits = span_in_bounds(u64::from(dec(fields[1])), offset, u64::from(dec(fields[3])));
            or_none(fits.then_some(offset).map(|offset| offset.to_string()))
        }
        "trim" => {
            let heap = from_hex(fields[1]);
            match trim_string_heap(&heap) {
                Some(trimmed) => format!("ok:{}", trimmed.len()),
                None => "err".to_string(),
            }
        }
        "uint" => {
            let encoded = from_hex(fields[1]);
            or_none(decode_compressed_uint(&encoded).map(|(value, len)| format!("{value}:{len}")))
        }
        _ => "unknown".to_string(),
    }
}

fn or_none(value: Option<String>) -> String {
    match value {
        Some(value) => format!("ok:{value}"),
        None => "none".to_string(),
    }
}

fn entries(mut henum: TokenEnum, reset: &str, buf_len: u32) -> String {
    if reset != "-" {
        henum.reset(dec(reset));
    }
    let mut buf = vec![0; buf_len as usize];
    let written = henum.get_entries(&mut buf);
    let tokens: Vec<String> = buf[..written]
        .iter()
        .map(|token| format!("{token:x}"))
        .collect();
    format!("{written}:{}:{}", tokens.join(","), henum.position())
}

fn parse_section(item: &str) -> Section {
    let fields: Vec<u32> = item.split(':').map(dec).collect();
    Section {
        virtual_address: fields[0],
        virtual_size: fields[1],
        raw_pointer: fields[2],
    }
}

fn dec(field: &str) -> u32 {
    field.parse().expect("decimal field")
}

fn hex(field: &str) -> u32 {
    u32::from_str_radix(field, 16).expect("hex field")
}

fn hex_list(spec: &str) -> Vec<u32> {
    spec.split(',').map(hex).collect()
}

fn text(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).expect("type names are ASCII")
}

fn nibble(digit: u8) -> u8 {
    (digit as char).to_digit(16).expect("hex digit") as u8
}

fn from_hex(spec: &str) -> Vec<u8> {
    let digits: Vec<u8> = spec.bytes().map(nibble).collect();
    digits
        .iter()
        .step_by(2)
        .zip(digits.iter().skip(1).step_by(2))
        .map(|(high, low)| (high << 4) | low)
        .collect()
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn oracle_rows_match() {
    for (line, expected) in ROWS {
        assert_eq!(run(line), *expected, "case {line}");
    }
}
