//! Tests against Wine's oracle vectors, then Rust-only edge cases.
//!
//! The vectors are the replies of Wine's verbatim `dmoreg.c` and `dmort.c`
//! bodies, built with ASan and UBSan by `target/canon/msdmo/build.sh`.
//! [`replay`] runs each command through the port and prints the result the
//! way the C harness does, so every vector is compared as the exact text.

use super::*;

#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("reserve 0 1 4", "TRUE 8"),
    ("reserve 8 9 4", "TRUE 16"),
    ("reserve 0 100 1", "TRUE 128"),
    ("reserve 0 1073741823 4", "TRUE 1073741823"),
    ("reserve 0 1073741824 4", "FALSE 0"),
    ("guid 00000000-0000-0000-0000000000000000", "00000000-0000-0000-0000-000000000000"),
    ("guid 0001a2b3-c4d5-e6f7-0809aabbccddeeff", "0001A2B3-C4D5-E6F7-0809-AABBCCDDEEFF"),
    ("equal 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920", "1"),
    ("equal 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000000-0000-0000-0000000000000000 00000000-0000-0000-0000000000000000", "1"),
    ("equal 00000000-0000-0000-0000000000000000 00000000-0000-0000-0000000000000000 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920", "1"),
    ("equal 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000002-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920", "0"),
    ("equal 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000001-0002-0003-0405060708090a0b 00000020-0021-0022-2324252627282930", "0"),
    ("any 1 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 2 00000002-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000000-0000-0000-0000000000000000 00000000-0000-0000-0000000000000000", "1"),
    ("any 1 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 1 00000002-0002-0003-0405060708090a0b 00000020-0021-0022-2324252627282930", "0"),
    ("types 00000001-0002-0003-0405060708090a0b 0 2 0 64 1 0 32", "0 00000001-0002-0003-0405-060708090A0B 2 1 64 32"),
    ("types 00000001-0002-0003-0405060708090a0b 2 2 0 64 1 0 32", "-2147467259 00000001-0002-0003-0405-060708090A0B 4294967295 4294967295 4294967295 4294967295"),
    ("types 00000001-0002-0003-0405060708090a0b 0 3 234 32 0 2 0", "0 00000001-0002-0003-0405-060708090A0B 1 0 96 0"),
    ("types 00000001-0002-0003-0405060708090a0b 0 134217729 0 0 0 0 0", "0 00000001-0002-0003-0405-060708090A0B 0 0 32 0"),
    ("init 0", "0 0 0"),
    ("init 5", "0 5 1"),
    ("copy 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000020-0021-0022-2324252627282930 1 0 16 4 1 01020304", "0 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000020-0021-0022-2324252627282930 1 0 16 4 1 01020304"),
    ("copy 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000020-0021-0022-2324252627282930 0 1 8 6 0 -", "0 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000020-0021-0022-2324252627282930 0 1 8 6 0 -"),
    ("copy 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000020-0021-0022-2324252627282930 1 0 0 0 1 -", "0 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000020-0021-0022-2324252627282930 1 0 0 0 0 -"),
    ("free 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 00000020-0021-0022-2324252627282930 1 0 16 4 1 01020304", "0 4 0"),
];

#[rustfmt::skip]
const EDGE: &[(&str, &str)] = &[
    ("reserve 16 9 4", "TRUE 16"),
    ("reserve 3 5 4", "TRUE 8"),
    ("reserve 0 4294967295 1", "TRUE 4294967295"),
    ("reserve 1 4294967295 2", "FALSE 1"),
    ("equal 00000000-0000-0000-0000000000000000 00000010-0011-0012-1314151617181920 00000005-0000-0000-0000000000000000 00000000-0000-0000-0000000000000000", "1"),
    ("any 0 1 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920", "0"),
    ("any 1 00000001-0002-0003-0405060708090a0b 00000010-0011-0012-1314151617181920 0", "0"),
    ("types 00000001-0002-0003-0405060708090a0b 0 2 234 96 3 0 32", "0 00000001-0002-0003-0405-060708090A0B 2 1 64 96"),
    ("types 00000001-0002-0003-0405060708090a0b 0 0 0 64 0 234 32", "0 00000001-0002-0003-0405-060708090A0B 0 0 0 0"),
    ("free 00000000-0000-0000-0000000000000000 00000000-0000-0000-0000000000000000 00000000-0000-0000-0000000000000000 0 0 0 0 0 -", "0 0 0"),
    ("null init", "-2147467261"),
    ("null free", "-2147467261"),
    ("null copy_dst", "-2147467261"),
    ("null copy_src", "-2147467261"),
    ("null copy_both", "-2147467261"),
    ("reserve 0 1 1073741824", "TRUE 8"),
    ("reserve 7 8 536870912", "FALSE 7"),
    ("reserve 0 536870913 8", "FALSE 0"),
    ("reserve 100 40 1", "TRUE 100"),
    ("reserve 0 1500000000 1", "TRUE 2147483648"),
    ("reserve 8 8 1", "TRUE 8"),
    ("types 00000001-0002-0003-0405060708090a0b 0 5 0 31 0 0 0", "0 00000001-0002-0003-0405-060708090A0B 0 0 160 0"),
];

const E_FAIL: i32 = 0x8000_4005u32 as i32;

fn num(text: &str) -> u32 {
    text.parse().unwrap()
}

fn parse_guid(token: &str) -> Guid {
    let parts: Vec<&str> = token.split('-').collect();
    let mut data4 = [0u8; 8];
    for (i, byte) in data4.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&parts[3][2 * i..2 * i + 2], 16).unwrap();
    }
    Guid {
        data1: u32::from_str_radix(parts[0], 16).unwrap(),
        data2: u16::from_str_radix(parts[1], 16).unwrap(),
        data3: u16::from_str_radix(parts[2], 16).unwrap(),
        data4,
    }
}

fn guid_token(guid: &Guid) -> String {
    let tail: String = guid.data4.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{:08x}-{:04x}-{:04x}-{tail}",
        guid.data1, guid.data2, guid.data3
    )
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

/// Reads a partial type list: a count, then that many type and subtype tokens.
fn partial_list<'a>(words: &'a [&'a str]) -> (Vec<PartialMediaType>, &'a [&'a str]) {
    let count = num(words[0]) as usize;
    let (pairs, rest) = words[1..].split_at(2 * count);
    let list = pairs
        .chunks(2)
        .map(|pair| PartialMediaType {
            type_: parse_guid(pair[0]),
            subtype: parse_guid(pair[1]),
        })
        .collect();
    (list, rest)
}

/// Reads a media type: major, sub and format types, the two flags, sample size, format size, present flag and format hex.
fn media_type_from(f: &[&str]) -> MediaType {
    let format_size = num(f[6]);
    let format = (num(f[7]) == 1).then(|| {
        if format_size > 0 {
            unhex(f[8])
        } else {
            Vec::new()
        }
    });
    MediaType {
        majortype: parse_guid(f[0]),
        subtype: parse_guid(f[1]),
        formattype: parse_guid(f[2]),
        fixed_size_samples: f[3].parse().unwrap(),
        temporal_compression: f[4].parse().unwrap(),
        sample_size: num(f[5]),
        cb_format: format_size,
        format,
    }
}

fn print_media(hres: i32, media: &MediaType) -> String {
    let format = match &media.format {
        Some(bytes) if media.cb_format > 0 => hex(bytes),
        _ => "-".to_string(),
    };
    format!(
        "{hres} {} {} {} {} {} {} {} {} {format}",
        guid_token(&media.majortype),
        guid_token(&media.subtype),
        guid_token(&media.formattype),
        media.fixed_size_samples,
        media.temporal_compression,
        media.sample_size,
        media.cb_format,
        media.format.is_some() as u8,
    )
}

/// The `types` command: `DMOGetTypes` after the key opened, with the registry
/// answering from the command's statuses and sizes.
fn replay_types(words: &[&str]) -> String {
    let clsid = guid_to_string(&parse_guid(words[1]));
    if num(words[2]) != 0 {
        return format!("{E_FAIL} {clsid} {0} {0} {0} {0}", u32::MAX);
    }
    let (in_count, in_status, in_size) = (num(words[3]), num(words[4]), num(words[5]));
    let (out_count, out_status, out_size) = (num(words[6]), num(words[7]), num(words[8]));
    let mut sizes = Vec::new();
    let (ret_in, ret_out) = dmo_get_types(in_count, out_count, |name, size| {
        sizes.push(size);
        let (status, configured) = if name == INPUT_TYPES {
            (in_status, in_size)
        } else {
            (out_status, out_size)
        };
        let reported = if status == ERROR_SUCCESS || status == ERROR_MORE_DATA {
            configured
        } else {
            size
        };
        (status, reported)
    });
    format!(
        "{S_OK} {clsid} {ret_in} {ret_out} {} {}",
        sizes[0], sizes[1]
    )
}

/// Runs one oracle command through the port and returns the harness's reply.
fn replay(command: &str) -> String {
    let words: Vec<&str> = command.split_whitespace().collect();
    match words[0] {
        "reserve" => {
            let (capacity, count, size) = (num(words[1]), num(words[2]), num(words[3]));
            match array_reserve(capacity, count, size) {
                Some(new_capacity) => format!("TRUE {new_capacity}"),
                None => format!("FALSE {capacity}"),
            }
        }
        "guid" => guid_to_string(&parse_guid(words[1])),
        "equal" => {
            let a = PartialMediaType {
                type_: parse_guid(words[1]),
                subtype: parse_guid(words[2]),
            };
            let b = PartialMediaType {
                type_: parse_guid(words[3]),
                subtype: parse_guid(words[4]),
            };
            (is_media_type_equal(&a, &b) as u8).to_string()
        }
        "any" => {
            let (a, rest) = partial_list(&words[1..]);
            let (b, _) = partial_list(rest);
            (any_types_match(&a, &b) as u8).to_string()
        }
        "types" => replay_types(&words),
        "init" => {
            let mut media = MediaType::default();
            let hres = mo_init_media_type(Some(&mut media), num(words[1]));
            format!(
                "{hres} {} {}",
                media.cb_format,
                media.format.is_some() as u8
            )
        }
        "copy" => {
            let src = media_type_from(&words[1..]);
            let mut dst = MediaType::default();
            let hres = mo_copy_media_type(Some(&mut dst), Some(&src));
            print_media(hres, &dst)
        }
        "free" => {
            let mut media = media_type_from(&words[1..]);
            let hres = mo_free_media_type(Some(&mut media));
            format!(
                "{hres} {} {}",
                media.cb_format,
                media.format.is_some() as u8
            )
        }
        "null" => {
            let mut media = MediaType::default();
            let hres = match words[1] {
                "init" => mo_init_media_type(None, 4),
                "free" => mo_free_media_type(None),
                "copy_dst" => mo_copy_media_type(None, Some(&media)),
                "copy_src" => mo_copy_media_type(Some(&mut media), None),
                _ => mo_copy_media_type(None, None),
            };
            hres.to_string()
        }
        other => panic!("unknown oracle command {other}"),
    }
}

#[test]
fn oracle_table_vectors_match_wine() {
    for (command, reply) in VECTORS {
        assert_eq!(replay(command), *reply, "{command}");
    }
}

#[test]
fn oracle_edge_vectors_match_wine() {
    for (command, reply) in EDGE {
        assert_eq!(replay(command), *reply, "{command}");
    }
}

#[test]
#[should_panic(expected = "unknown oracle command bogus")]
fn unknown_oracle_command_panics() {
    replay("bogus");
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn array_reserve_panics_on_zero_size_as_wine_divides_by_it() {
    array_reserve(0, 1, 0);
}

#[test]
#[should_panic(expected = "out of range")]
fn copy_panics_when_the_format_buffer_is_shorter_than_its_size() {
    let src = MediaType {
        cb_format: 4,
        format: Some(vec![1, 2]),
        ..MediaType::default()
    };
    let mut dst = MediaType::default();
    mo_copy_media_type(Some(&mut dst), Some(&src));
}

#[test]
fn is_media_type_equal_is_symmetric_over_wildcards() {
    let types = [
        PartialMediaType::default(),
        PartialMediaType {
            type_: parse_guid("00000001-0002-0003-0405060708090a0b"),
            subtype: GUID_NULL,
        },
        PartialMediaType {
            type_: GUID_NULL,
            subtype: parse_guid("00000010-0011-0012-1314151617181920"),
        },
        PartialMediaType {
            type_: parse_guid("00000002-0002-0003-0405060708090a0b"),
            subtype: parse_guid("00000010-0011-0012-1314151617181920"),
        },
    ];
    for a in &types {
        for b in &types {
            assert_eq!(is_media_type_equal(a, b), is_media_type_equal(b, a));
        }
    }
}

#[test]
fn returned_type_count_needs_a_success_or_more_data_status() {
    assert_eq!(returned_type_count(4, ERROR_SUCCESS, 96), 3);
    assert_eq!(returned_type_count(4, ERROR_MORE_DATA, 1000), 4);
    assert_eq!(returned_type_count(4, 2, 1000), 0);
    assert_eq!(returned_type_count(0, ERROR_SUCCESS, 1000), 0);
}

#[test]
fn media_type_and_guids_format_for_debugging() {
    let media = MediaType {
        format: Some(vec![9]),
        cb_format: 1,
        ..MediaType::default()
    };
    assert!(format!("{media:?}").starts_with("MediaType {"));
    assert!(format!("{:?}", PartialMediaType::default()).starts_with("PartialMediaType {"));
    assert_eq!(media.clone(), media);
}
