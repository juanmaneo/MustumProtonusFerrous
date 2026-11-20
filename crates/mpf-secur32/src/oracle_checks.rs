//! Rows replayed from `target/canon/secur32/oracle.txt`, produced by compiling the verbatim
//! `parse_alpn_protocol_list` helper from Wine's `schannel_gnutls.c` with ASan/UBSan. Each row
//! lists the input bytes and the identifiers Wine reports for them; `None` means Wine returns
//! zero identifiers for malformed input.

use super::parse_alpn_protocol_list;
use std::iter::once;

fn owned(input: &[u8]) -> Option<Vec<Vec<u8>>> {
    parse_alpn_protocol_list(input)
        .map(|protocols| protocols.into_iter().map(<[u8]>::to_vec).collect())
}

#[test]
fn oracle_rows() {
    type Row = (&'static str, Vec<u8>, Option<Vec<Vec<u8>>>);
    let rows: Vec<Row> = vec![
        ("0 empty", vec![], Some(vec![])),
        ("1 zero length", vec![0], None),
        ("2 length without payload", vec![1], None),
        ("3 one byte", vec![1, b'a'], Some(vec![b"a".to_vec()])),
        ("4 h2", vec![2, b'h', b'2'], Some(vec![b"h2".to_vec()])),
        (
            "5 http/1.1",
            vec![8, b'h', b't', b't', b'p', b'/', b'1', b'.', b'1'],
            Some(vec![b"http/1.1".to_vec()]),
        ),
        (
            "6 two one-byte identifiers",
            vec![1, b'a', 1, b'b'],
            Some(vec![b"a".to_vec(), b"b".to_vec()]),
        ),
        (
            "7 zero length after valid identifier",
            vec![1, b'a', 0],
            None,
        ),
        (
            "8 truncated second identifier",
            vec![1, b'a', 2, b'b'],
            None,
        ),
        (
            "9 trailing length without payload",
            vec![2, b'a', b'b', 1],
            None,
        ),
        (
            "10 three identifiers",
            vec![1, b'a', 1, b'b', 1, b'c'],
            Some(vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]),
        ),
        (
            "11 255 prefix with 254 payload bytes",
            once(255).chain(1..=254).collect(),
            None,
        ),
        (
            "12 255 prefix with 253 payload bytes",
            once(255).chain(1..=253).collect(),
            None,
        ),
        (
            "13 255 prefix with 255 payload bytes",
            once(255).chain(1..=255).collect(),
            Some(vec![(1..=255).collect()]),
        ),
        (
            "14 three-byte and one-byte identifiers",
            vec![3, b'a', b'b', b'c', 1, b'x'],
            Some(vec![b"abc".to_vec(), b"x".to_vec()]),
        ),
        ("15 NUL identifier byte", vec![1, 0], Some(vec![vec![0]])),
        (
            "16 high identifier byte",
            vec![1, 255],
            Some(vec![vec![255]]),
        ),
        (
            "17 mixed identifier lengths",
            vec![1, b'x', 2, b'y', b'z', 1, b'q'],
            Some(vec![b"x".to_vec(), b"yz".to_vec(), b"q".to_vec()]),
        ),
        ("18 zero length after identifier", vec![1, b'z', 0], None),
        (
            "19 truncated identifier after valid prefix",
            vec![1, b'z', 3, b'a', b'b'],
            None,
        ),
    ];

    assert_eq!(rows.len(), 20);
    for (name, input, expected) in rows {
        assert_eq!(owned(&input), expected, "oracle row {name}");
    }
}
