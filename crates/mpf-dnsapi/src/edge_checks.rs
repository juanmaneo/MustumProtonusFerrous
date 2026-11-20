//! Edge vectors from the same Wine build, all with oracle output as expected
//! values: names N01-N58, compares Q01-Q11, messages P01-P08, P10-P27 and
//! P29-P32 (P09 and P28 are not in the harness), message vectors X01-X13,
//! X15-X34 and X36-X51 (X14 and X35 are not in the generator), and F6.
//! F1-F5 are not oracle rows, since Wine reports undefined behaviour on them.
//! The hand-derived edges follow the tables.

use super::oracle_checks::{check_compares, check_messages, check_names, name_status, run, wide};
use crate::name::{name_compare, validate_name, NameFormat};
use crate::wire::{extract_records, RData, Record, Section};
use crate::Error;

const EDGE_NAMES: &[(&str, Option<&str>, u32, u32)] = &[
    ("N01", Some("."), 0, 0),
    ("N02", Some("a."), 0, 0),
    ("N03", Some(""), 0, 123),
    ("N04", Some(".a"), 0, 123),
    ("N05", Some("*"), 4, 0),
    ("N06", Some("*x"), 4, 9560),
    ("N07", Some("*.\u{e9}"), 4, 123),
    ("N08", Some("*. b"), 4, 123),
    ("N09", Some("1"), 4, 123),
    ("N10", Some("*.1"), 4, 0),
    ("N11", Some("*.a!"), 4, 123),
    ("N12", Some("_sip._tcp.example.com"), 5, 0),
    ("N13", Some("_"), 5, 9556),
    ("N14", Some("_a_"), 5, 0),
    ("N15", Some("a_b"), 5, 123),
    ("N16", Some("12"), 5, 123),
    ("N17", Some("_a b"), 5, 123),
    ("N18", Some("_\u{e9}"), 5, 123),
    ("N19", Some("_a!"), 5, 123),
    ("N20", Some("a b"), 1, 9560),
    ("N21", Some("a!"), 1, 9560),
    ("N22", Some("\u{e9}"), 1, 9556),
    ("N23", Some("a*"), 1, 9560),
    ("N24", Some("a.b"), 3, 123),
    ("N25", Some("ab"), 3, 0),
    ("N26", Some("a_b"), 3, 9556),
    ("N27", Some("a b"), 2, 9560),
    ("N28", Some("a*"), 2, 9560),
    ("N29", Some("a.\u{e9}"), 2, 9556),
    ("N30", Some("a_b"), 0, 9556),
    ("N31", Some("a/b"), 0, 9560),
    ("N32", Some("a-b.example"), 0, 0),
    ("N33", Some("a*b"), 0, 9560),
    ("N34", Some("a\u{22}b"), 0, 9560),
    ("N35", Some("\u{1f600}.com"), 0, 9556),
    ("N36", Some(".."), 0, 123),
    ("N37", Some("."), 1, 123),
    ("N38", Some(""), 4, 123),
    ("N39", Some("*.*"), 4, 0),
    ("N40", Some("xn--abc"), 1, 0),
    ("N41", Some(""), 5, 123),
    ("N42", Some("a"), 5, 123),
    ("N43", Some("a b"), 3, 9560),
    ("N44", Some("a_b"), 3, 9556),
    ("N45", Some("a*"), 3, 9560),
    ("N46", Some("a.b"), 2, 0),
    ("N47", Some("*"), 2, 9560),
    ("N48", Some("a-b"), 1, 0),
    ("N49", Some("\u{e9}"), 0, 9556),
    ("N50", Some("a"), 9, 0),
    ("N51", Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), 0, 0),
    ("N52", Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.a"), 0, 0),
    ("N53", Some("a.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), 0, 0),
    ("N54", Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaa"), 0, 0),
    ("N55", Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.a"), 0, 123),
    ("N56", Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aa"), 0, 123),
    ("N57", Some("a.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), 0, 123),
    ("N58", Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.a.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), 0, 123),
];

const EDGE_COMPARES: &[(&str, Option<&str>, Option<&str>, bool)] = &[
    ("Q01", None, None, true),
    ("Q02", Some(""), None, false),
    ("Q03", Some("\u{df}"), Some("SS"), false),
    ("Q04", Some("a"), Some("a"), true),
    ("Q05", Some(""), Some("."), true),
    ("Q06", Some("\u{1c6}"), Some("\u{1c4}"), true),
    ("Q07", Some("A\u{0}x"), Some("a"), true),
    ("Q08", Some("a..b"), Some("a..b."), true),
    ("Q09", Some("ab"), Some("a"), false),
    ("Q10", Some("\u{e9}"), Some("\u{c9}"), true),
    ("Q11", Some("\u{3c3}"), Some("\u{3a3}"), true),
];

const EDGE_MESSAGES: &[(&str, &str, u32, &[&str])] = &[
    ("P01", "12340100000000000000000000", 9502, &[]),
    ("P02", "1234010000000100000000000461626364000001000100000000000401020304", 0, &[
        "sec=1 type=1 ttl=0 name=abcd a=01020304",
    ]),
    ("P03", "123481800000010000000000016b000019000100000e10000601000308aabb", 0, &[
        "sec=1 type=25 ttl=3600 name=k flags=256 proto=3 alg=8 key=aabb",
    ]),
    ("P04", "123481800000010000000000016b000019000100000e100003010003", 9502, &[]),
    ("P05", "12348180000001000000000000000a0001000000000003dead00", 0, &[
        "sec=1 type=10 ttl=0 name=. null=dead00",
    ]),
    ("P06", "1234818000000100000000000000291000000000000002abcd", 0, &[
        "sec=1 type=41 ttl=0 name=. opt=abcd",
    ]),
    ("P07", "12348180000001000000000000001800010000000000170001080200000e1000000100000000801234017800aabb", 0, &[
        "sec=1 type=24 ttl=0 name=. cov=1 alg=8 labels=2 ottl=3600 exp=256 signed=128 tag=4660 signer=x sig=aabb",
    ]),
    ("P08", "123481800000010000000000000006000100000000001e026e730004726f6f74000000000100000002000000030000000400000005", 0, &[
        "sec=1 type=6 ttl=0 name=. primary=ns admin=root serial=1 refresh=2 retry=3 expire=4 minttl=5",
    ]),
    ("P10", "12348180000002000000000000000e0001000000000006016d000165000000110001000000000006017200017000", 0, &[
        "sec=1 type=14 ttl=0 name=. mailbox=m errors=e",
        "sec=1 type=17 ttl=0 name=. mailbox=r errors=p",
    ]),
    ("P11", "12348180000001000000000000000e0001000000000003016d00", 0, &[
        "sec=1 type=14 ttl=0 name=. mailbox=m errors=.",
    ]),
    ("P12", "1234818000000200000000000000120001000000000005000201610000001500010000000000050005016200", 0, &[
        "sec=1 type=18 ttl=0 name=. pref=2 exchange=a",
        "sec=1 type=21 ttl=0 name=. pref=5 exchange=b",
    ]),
    ("P13", "12348180000001000000000000000f000100000000000100", 9502, &[]),
    ("P14", "123481800000010000000000017000000c0001000000000002c00c", 0, &[
        "sec=1 type=12 ttl=0 name=p host=p",
    ]),
    ("P15", "1234818000000100000000000000020001000000000002c017", 9502, &[]),
    ("P16", "12348180000001000000000040000001000100000000000401020304", 9502, &[]),
    ("P17", "123481800000010000000000046162636400000100010000000000040102030400", 9502, &[]),
    ("P18", "123481800000010001000100046162636400000100010000003c0004010203040000060001000000000002ffff000001000100000000000405060708", 0, &[
        "sec=1 type=1 ttl=60 name=abcd a=01020304",
        "sec=3 type=1 ttl=0 name=. a=05060708",
    ]),
    ("P19", "12348180020001000000000001610000010001016200001c000102777700000100010000003c000409090909", 0, &[
        "sec=1 type=1 ttl=60 name=ww a=09090909",
    ]),
    ("P20", "1234818001000000000000000161000001", 9502, &[]),
    ("P21", "1234818000000100010000000461626364000001000100000000000401020304", 9502, &[]),
    ("P22", "1234818000000100000000003f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161613f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161613f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161613d61616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161016100000a000100000000000401020304", 0, &[
        "sec=1 type=10 ttl=0 name=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.a null=01020304",
    ]),
    ("P23", "1234818000000100000000003f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161613f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161613f6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161613d6161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616102616100000a000100000000000401020304", 9502, &[]),
    ("P24", "12348180000001000000000000001000010000000000050003610062", 0, &[
        "sec=1 type=16 ttl=0 name=. strings=2 [] [a]",
    ]),
    ("P25", "1234818000000100000000000000100001000000000003056162", 9502, &[]),
    ("P26", "12348180000003000000000000000d0001000000000000000014000100000000000201780000130001000000000000", 0, &[
        "sec=1 type=13 ttl=0 name=. strings=0",
        "sec=1 type=20 ttl=0 name=. strings=1 [x]",
        "sec=1 type=19 ttl=0 name=. strings=0",
    ]),
    ("P27", "1234818000000100000000000000050001000000000002c00c", 0, &[
        "sec=1 type=5 ttl=0 name=. host=.",
    ]),
    ("P29", "1234818000000100000000000461006263000001000100000000000401020304", 0, &[
        "sec=1 type=1 ttl=0 name=a a=01020304",
    ]),
    ("P30", "123481800000010000000000040078797a000001000100000000000401020304", 0, &[
        "sec=1 type=1 ttl=0 name= a=01020304",
    ]),
    ("P31", "123481800000010000000000c0", 9502, &[]),
    ("P32", "123481800100010000000000c00c00010001027777000001000100000000000401020304", 0, &[
        "sec=1 type=1 ttl=0 name=ww a=01020304",
    ]),
    ("X01", "123481800000010000000000c00c0001000100000000000401020304", 9502, &[]),
    ("X02", "1234818000000100000000000261000162000001000100000000000401020304", 0, &[
        "sec=1 type=1 ttl=0 name=a a=01020304",
    ]),
    ("X03", "12348180000001000000010002616201630000010001000000000004010203040000010001000000000010", 9502, &[]),
    ("X04", "12348180000001000100000002616201630000010001000000000004010203040000010001", 9502, &[]),
    ("X05", "123401000100000000000000016100", 9502, &[]),
    ("X06", "12340100010000000000000040610000010001", 9502, &[]),
    ("X07", "123401000000000001000000000001000100000000", 9502, &[]),
    ("X08", "12348180000001000000000000000100", 9502, &[]),
    ("X09", "123481800000010000000000000001000100000000001001020304", 9502, &[]),
    ("X10", "1234818000000100000000000000010001000000000003010203", 9502, &[]),
    ("X11", "12348180000001000000000000001c000100000000000f000000000000000000000000000000", 9502, &[]),
    ("X12", "12348180000001000000000000001800010000000000110000000000000000000000000000000000", 9502, &[]),
    ("X13", "12348180000001000000000000001800010000000000130001080200000e100000010000000080123440", 9502, &[]),
    ("X15", "123481800000010000000000000021000100000000000400010002", 9502, &[]),
    ("X16", "123481800000010000000000000021000100000000000700010002000340", 9502, &[]),
    ("X17", "1234818000000100000000000000050001000000000000", 0, &[
        "sec=1 type=5 ttl=0 name=. host=.",
    ]),
    ("X18", "12348180000001000000000000000e0001000000000004016d0040", 9502, &[]),
    ("X19", "12348180000002000000000000000e0001000000000002016d000001000100000000000401020304", 0, &[
        "sec=1 type=14 ttl=0 name=. mailbox=m errors=.",
        "sec=1 type=1 ttl=0 name=. a=01020304",
    ]),
    ("X20", "12348180000002000000000000000f0001000000000004000a016d0161000001000100000000000401020304", 0, &[
        "sec=1 type=15 ttl=0 name=. pref=10 exchange=m.a",
        "sec=1 type=1 ttl=0 name=a a=01020304",
    ]),
    ("X21", "12340100000000000000000000", 9502, &[]),
    ("X22", "123481800000010000000000000019000100000000000400010308", 0, &[
        "sec=1 type=25 ttl=0 name=. flags=1 proto=3 alg=8 key=",
    ]),
    ("X23", "1234818000000100000000000000290001000000000000", 0, &[
        "sec=1 type=41 ttl=0 name=. opt=",
    ]),
    ("X24", "12348180000001000000000000001d0001000000000000", 9004, &[]),
    ("X25", "123481800000010000000000000006000100000000000100", 9502, &[]),
    ("X26", "12348180000001000000000002616201630000010001000000000004010203040000", 9502, &[]),
    ("X27", "1234818000000100000000000000100001000000000000", 0, &[
        "sec=1 type=16 ttl=0 name=. strings=0",
    ]),
    ("X28", "12348180000001000000000000001000010000000000050003616200", 0, &[
        "sec=1 type=16 ttl=0 name=. strings=2 [] [ab]",
    ]),
    ("X29", "1234818000000100000000000000100001000000000006036162630161", 0, &[
        "sec=1 type=16 ttl=0 name=. strings=2 [abc] [a]",
    ]),
    ("X30", "12348180000002000000000002616201630000010001000000000004010203040000ff000100000000000100", 9004, &[]),
    ("X31", "12348180000001000000000000000f000100000000000100", 9502, &[]),
    ("X32", "123481800000010000000000000006000100000000001600000000000100000002000000030000000400000005", 0, &[
        "sec=1 type=6 ttl=0 name=. primary=. admin=. serial=1 refresh=2 retry=3 expire=4 minttl=5",
    ]),
    ("X33", "1234818000000100000000000000060001000000000012000000000001000000020000000300000004", 9502, &[]),
    ("X34", "1234818000000100000000000000100001000000000003056162", 9502, &[]),
    ("X36", "123481800000010000000000026162016300000100010000000000050102030400", 0, &[
        "sec=1 type=1 ttl=0 name=ab.c a=01020304",
    ]),
    ("X37", "123481800000010000000000c0ff0001000100000000000401020304", 9502, &[]),
    ("X38", "123401000100000000000000c00c00010001", 0, &[]),
    ("X39", "123401000000000001000000c00c00010001000000000000", 0, &[]),
    ("X40", "123401000100000000000000000001", 9502, &[]),
    ("X41", "12348180000001000000000000000e000100000000000140", 9502, &[]),
    ("X42", "12348180000001000000000000000f0001000000000003000a40", 9502, &[]),
    ("X43", "123481800000010000000000000006000100000000000140", 9502, &[]),
    ("X44", "12348180000001000000000000000600010000000000020040", 9502, &[]),
    ("X45", "1234818000000100000000000100026162000001000100000000000401020304", 0, &[
        "sec=1 type=1 ttl=0 name= a=01020304",
    ]),
    ("X46", "123401000100000000000000400000010001", 9502, &[]),
    ("X47", "123401000000000001000000800000010001000000000000", 9502, &[]),
    ("X48", "12340100010000000000000000", 9502, &[]),
    ("X49", "1234818000000100000001000261620163000001000100000000000401020304", 9502, &[]),
    ("X50", "12340100000000000100000000000100010000000000", 9502, &[]),
    ("X51", "123401000000000001000000000001000100000000000501", 9502, &[]),
    ("F6", "123481800000010000000000000006000100000000001a026e730004726f6f740000000001000000020000000300000004", 9502, &[]),
];

#[test]
fn edge_names() {
    check_names(EDGE_NAMES);
}

#[test]
fn edge_compare() {
    check_compares(EDGE_COMPARES);
}

#[test]
fn edge_messages() {
    check_messages(EDGE_MESSAGES);
}

fn dotted(sizes: &[usize]) -> String {
    sizes
        .iter()
        .map(|&n| "a".repeat(n))
        .collect::<Vec<_>>()
        .join(".")
}

fn wire_name(sizes: &[usize]) -> Vec<u8> {
    let mut out = Vec::new();
    for &n in sizes {
        out.push(n as u8);
        out.extend("a".repeat(n).bytes());
    }
    out.push(0);
    out
}

/// A message with the flags of a response or a query, the counts of the
/// question, answer, authority and additional sections, and the body.
fn message(response: bool, counts: [u8; 4], body: &[u8]) -> Vec<u8> {
    let flags = if response { [0x81, 0x80] } else { [0x01, 0x00] };
    let mut msg = vec![0x12, 0x34, flags[0], flags[1]];
    for count in counts {
        msg.extend([count, 0]);
    }
    msg.extend_from_slice(body);
    msg
}

/// A response with one answer record of class IN and TTL 0.
fn answer(owner: &[u8], rtype: u16, rdata: &[u8]) -> Vec<u8> {
    let mut msg = message(true, [0, 1, 0, 0], &[]);
    msg.extend_from_slice(owner);
    msg.extend(rtype.to_be_bytes());
    msg.extend([0, 1, 0, 0, 0, 0]);
    msg.extend(
        u16::try_from(rdata.len())
            .expect("rdata length")
            .to_be_bytes(),
    );
    msg.extend_from_slice(rdata);
    msg
}

/// A name as Wine's `WCHAR` string, or `None` for a missing name.
type WideName = Option<Vec<u16>>;

fn check_bytes(msg: &[u8], status: u32, lines: &[&str]) {
    let want: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
    assert_eq!(run(msg), (status, want));
}

#[test]
fn hand_names() {
    let cases: &[(WideName, u32, u32)] = &[
        (wide(Some("*.")), 4, 0),
        (wide(Some("a\0!")), 0, 0),
        (Some(vec![0xd800]), 0, 9556),
        (wide(Some("\u{131}")), 0, 9556),
        (wide(Some("a*")), 0, 9560),
        (wide(Some("a_b")), 0, 9556),
        (wide(Some("a!")), 0, 9560),
        (wide(Some("a b")), 9, 0),
        (wide(Some("a b")), 1, 9560),
        (wide(Some("a.b")), 1, 123),
        (wide(Some("a{")), 3, 9560),
        (wide(Some("a*")), 2, 9560),
        (wide(Some(".a")), 0, 123),
        (wide(Some(".")), 0, 0),
        (wide(Some("a.")), 0, 0),
        (wide(Some("a..b")), 0, 123),
        (wide(Some("")), 0, 123),
        (wide(Some("\0")), 0, 123),
        (None, 0, 123),
        (wide(Some("123")), 4, 123),
        (wide(Some("a")), 4, 123),
        (wide(Some("*a")), 4, 9560),
        (wide(Some("*.\u{e9}")), 4, 123),
        (wide(Some("*.a b")), 4, 123),
        (wide(Some("*.a!")), 4, 123),
        (wide(Some("*")), 4, 0),
        (wide(Some("*.a")), 4, 0),
        (wide(Some("_")), 5, 9556),
        (wide(Some("a")), 5, 123),
        (wide(Some("_a b")), 5, 123),
        (wide(Some("123")), 5, 123),
        (wide(Some("_sip._tcp.example.com")), 5, 0),
        (wide(Some(dotted(&[64]).as_str())), 0, 123),
        (wide(Some(dotted(&[63]).as_str())), 0, 0),
        (wide(Some(dotted(&[1, 63]).as_str())), 0, 123),
        (wide(Some(dotted(&[1, 62]).as_str())), 0, 0),
        (wide(Some(dotted(&[62, 62, 62, 62, 4]).as_str())), 0, 123),
        (wide(Some(dotted(&[62, 62, 62, 62, 3]).as_str())), 0, 0),
    ];
    for (name, format, status) in cases {
        assert_eq!(
            name_status(validate_name(name.as_deref(), *format)),
            *status,
            "{name:?} {format}"
        );
    }
}

#[test]
fn hand_compares() {
    let cases: &[(WideName, WideName, bool)] = &[
        (None, None, true),
        (None, wide(Some("a")), false),
        (wide(Some("a")), None, false),
        (wide(Some("")), wide(Some(".")), true),
        (wide(Some("a")), wide(Some("ab")), false),
        (wide(Some("a.")), wide(Some("A")), true),
        (wide(Some("a..")), wide(Some("A")), true),
        (wide(Some("a\0x.")), wide(Some("A")), true),
        // Wine folds each unit with the simple uppercase mapping, which makes this
        // pair equal; this port uses the full mapping and does not (see lib.rs).
        (wide(Some("\u{1f80}")), wide(Some("\u{1f88}")), false),
        (Some(vec![0xd800]), Some(vec![0xdc00]), false),
        (Some(vec![0xd800]), Some(vec![0xd800]), true),
        (wide(Some("\u{1c5}")), wide(Some("\u{1c4}")), true),
        (wide(Some("\u{17f}")), wide(Some("S")), true),
        (wide(Some("\u{131}")), wide(Some("I")), true),
        (wide(Some("\u{130}")), wide(Some("i")), false),
        (wide(Some("\u{b5}")), wide(Some("\u{39c}")), true),
    ];
    for (a, b, same) in cases {
        assert_eq!(
            name_compare(a.as_deref(), b.as_deref()),
            *same,
            "{a:?} {b:?}"
        );
    }
}

#[test]
fn hand_framing() {
    assert_eq!(extract_records(&[0u8; 12]), Err(Error::InvalidParameter));
    assert_eq!(extract_records(&[0u8; 65536]), Err(Error::InvalidParameter));
    assert_eq!(extract_records(&[0u8; 65535]), Err(Error::BadPacket));
    let mut trailing = answer(&[0], 1, &[1, 2, 3, 4]);
    trailing.push(0);
    assert_eq!(extract_records(&trailing), Err(Error::BadPacket));
    let mut short = answer(&[0], 1, &[1, 2, 3, 4]);
    short.truncate(short.len() - 2);
    assert_eq!(extract_records(&short), Err(Error::BadPacket));
}

#[test]
fn hand_questions_and_authorities() {
    check_bytes(&message(false, [1, 0, 0, 0], &[0, 0, 1, 0, 1]), 0, &[]);
    check_bytes(
        &message(false, [1, 0, 0, 0], &[0xc0, 0x0c, 0, 1, 0, 1]),
        0,
        &[],
    );
    check_bytes(&message(false, [1, 0, 0, 0], &[0x40]), 9502, &[]);
    check_bytes(&message(false, [1, 0, 0, 0], &[5, b'a']), 9502, &[]);
    check_bytes(&message(false, [1, 0, 0, 0], &[0]), 9502, &[]);
    check_bytes(&message(false, [1, 0, 0, 0], &[0, 0, 1, 0]), 9502, &[]);
    check_bytes(
        &message(false, [0, 0, 1, 0], &[0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0]),
        0,
        &[],
    );
    check_bytes(
        &message(false, [0, 0, 1, 0], &[0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 5]),
        9502,
        &[],
    );
    check_bytes(
        &message(false, [0, 0, 1, 0], &[0, 0, 2, 0, 1, 0, 0, 0, 0]),
        9502,
        &[],
    );
    check_bytes(&message(false, [0, 0, 1, 0], &[0x40]), 9502, &[]);
}

#[test]
fn hand_answers() {
    check_bytes(
        &message(true, [0, 1, 0, 0], &[0, 0, 1, 0, 1, 0, 0, 0, 0]),
        9502,
        &[],
    );
    check_bytes(&answer(&[0], 99, &[1, 2]), 9004, &[]);
    check_bytes(
        &message(true, [0, 1, 0, 0], &[10, b'a', b'b', b'c']),
        9502,
        &[],
    );
    assert_eq!(
        extract_records(&answer(&[0], 14, &[0xc0])),
        Err(Error::BadPacket)
    );
    assert_eq!(
        extract_records(&answer(&[0xc0, 0xff], 1, &[1, 2, 3, 4])),
        Err(Error::BadPacket)
    );
    assert_eq!(
        extract_records(&answer(&[0xc0, 0x0c], 1, &[1, 2, 3, 4])),
        Err(Error::BadPacket)
    );
    assert_eq!(
        extract_records(&answer(&wire_name(&[63; 5]), 1, &[1, 2, 3, 4])),
        Err(Error::BadPacket)
    );
    let long = Record {
        name: dotted(&[63; 4]).into_bytes(),
        section: Section::Answer,
        rtype: 1,
        ttl: 0,
        data: RData::A([1, 2, 3, 4]),
    };
    assert_eq!(
        extract_records(&answer(&wire_name(&[63; 4]), 1, &[1, 2, 3, 4])),
        Ok(vec![long])
    );
    let msg = message(
        true,
        [1, 1, 0, 0],
        &[
            0, 0, 1, 0, 1, 0xc0, 0x0c, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 1, 2, 3, 4,
        ],
    );
    let root = Record {
        name: b".".to_vec(),
        section: Section::Answer,
        rtype: 1,
        ttl: 0,
        data: RData::A([1, 2, 3, 4]),
    };
    assert_eq!(extract_records(&msg), Ok(vec![root]));
}

#[test]
fn hand_rdata_bounds() {
    let cases: &[(u16, &[u8])] = &[
        (1, &[1, 2, 3]),
        (28, &[0; 15]),
        (25, &[0, 1, 2]),
        (14, &[0x40]),
        (14, &[0, 0x40]),
        (17, &[0, 0x40]),
        (15, &[0]),
        (15, &[0, 10, 0x40]),
        (5, &[0x40]),
        (5, &[5, b'a']),
        (24, &[0; 17]),
        (24, &[0x40; 19]),
        (6, &[0x40]),
        (6, &[0, 0x40]),
        (33, &[0; 5]),
        (33, &[0, 0, 0, 0, 0, 0, 0x40]),
        (16, &[5, b'a']),
    ];
    for &(rtype, rdata) in cases {
        assert_eq!(
            extract_records(&answer(&[0], rtype, rdata)),
            Err(Error::BadPacket),
            "type {rtype} rdata {rdata:?}"
        );
    }
}

#[test]
fn hand_flagged_inputs() {
    // A SIG whose signer name runs past its rdata into the message.
    let mut rdata = [0u8; 19];
    rdata[18] = 1;
    let mut sig = answer(&[0], 24, &rdata);
    sig.extend([b'a', 0]);
    check_bytes(&sig, 9502, &[]);
    check_bytes(
        &message(
            true,
            [0, 1, 0, 0],
            &[0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 1, 2, 3, 4],
        ),
        0,
        &["sec=1 type=1 ttl=0 name=. a=01020304"],
    );
    check_bytes(
        &answer(
            &[0],
            28,
            &[0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        ),
        0,
        &["sec=1 type=28 ttl=0 name=. aaaa=20010db8000000000000000000000001"],
    );
    check_bytes(
        &message(
            true,
            [0, 1, 0, 0],
            &[0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 2, 0x3f, 0xff],
        ),
        9502,
        &[],
    );
}

#[test]
fn hand_values() {
    let values = [
        (RData::A([1, 2, 3, 4]), "A("),
        (RData::Aaaa([0; 16]), "Aaaa("),
        (
            RData::Key {
                flags: 1,
                protocol: 2,
                algorithm: 3,
                key: vec![4],
            },
            "Key {",
        ),
        (
            RData::Minfo {
                mailbox: b"m".to_vec(),
                errors_mailbox: b"e".to_vec(),
            },
            "Minfo {",
        ),
        (
            RData::Mx {
                preference: 10,
                exchange: b"mx".to_vec(),
            },
            "Mx {",
        ),
        (RData::Name(b"host".to_vec()), "Name("),
        (RData::Null(vec![1]), "Null("),
        (RData::Opt(vec![2]), "Opt("),
        (
            RData::Sig {
                type_covered: 1,
                algorithm: 2,
                labels: 3,
                original_ttl: 4,
                expiration: 5,
                time_signed: 6,
                key_tag: 7,
                signer: b"s".to_vec(),
                signature: vec![9],
            },
            "Sig {",
        ),
        (
            RData::Soa {
                primary: b"p".to_vec(),
                admin: b"a".to_vec(),
                serial: 1,
                refresh: 2,
                retry: 3,
                expire: 4,
                minimum: 5,
            },
            "Soa {",
        ),
        (
            RData::Srv {
                priority: 1,
                weight: 2,
                port: 3,
                target: b"t".to_vec(),
            },
            "Srv {",
        ),
        (RData::Strings(vec![b"txt".to_vec()]), "Strings("),
    ];
    for (data, prefix) in values {
        assert!(format!("{data:?}").starts_with(prefix));
        assert_eq!(data.clone(), data);
    }

    let key = RData::Key {
        flags: 1,
        protocol: 2,
        algorithm: 3,
        key: vec![4],
    };
    assert_ne!(
        key,
        RData::Key {
            flags: 0,
            protocol: 2,
            algorithm: 3,
            key: vec![4]
        }
    );
    assert_ne!(
        key,
        RData::Key {
            flags: 1,
            protocol: 0,
            algorithm: 3,
            key: vec![4]
        }
    );
    assert_ne!(
        key,
        RData::Key {
            flags: 1,
            protocol: 2,
            algorithm: 0,
            key: vec![4]
        }
    );
    assert_ne!(
        key,
        RData::Key {
            flags: 1,
            protocol: 2,
            algorithm: 3,
            key: vec![0]
        }
    );

    let minfo = RData::Minfo {
        mailbox: b"m".to_vec(),
        errors_mailbox: b"e".to_vec(),
    };
    assert_ne!(
        minfo,
        RData::Minfo {
            mailbox: b"x".to_vec(),
            errors_mailbox: b"e".to_vec()
        }
    );
    assert_ne!(
        minfo,
        RData::Minfo {
            mailbox: b"m".to_vec(),
            errors_mailbox: b"x".to_vec()
        }
    );

    let mx = RData::Mx {
        preference: 10,
        exchange: b"mx".to_vec(),
    };
    assert_ne!(
        mx,
        RData::Mx {
            preference: 11,
            exchange: b"mx".to_vec()
        }
    );
    assert_ne!(
        mx,
        RData::Mx {
            preference: 10,
            exchange: b"my".to_vec()
        }
    );

    let sig = RData::Sig {
        type_covered: 1,
        algorithm: 2,
        labels: 3,
        original_ttl: 4,
        expiration: 5,
        time_signed: 6,
        key_tag: 7,
        signer: b"s".to_vec(),
        signature: vec![9],
    };
    assert_ne!(
        sig,
        RData::Sig {
            type_covered: 0,
            algorithm: 2,
            labels: 3,
            original_ttl: 4,
            expiration: 5,
            time_signed: 6,
            key_tag: 7,
            signer: b"s".to_vec(),
            signature: vec![9]
        }
    );
    assert_ne!(
        sig,
        RData::Sig {
            type_covered: 1,
            algorithm: 0,
            labels: 3,
            original_ttl: 4,
            expiration: 5,
            time_signed: 6,
            key_tag: 7,
            signer: b"s".to_vec(),
            signature: vec![9]
        }
    );
    assert_ne!(
        sig,
        RData::Sig {
            type_covered: 1,
            algorithm: 2,
            labels: 0,
            original_ttl: 4,
            expiration: 5,
            time_signed: 6,
            key_tag: 7,
            signer: b"s".to_vec(),
            signature: vec![9]
        }
    );
    assert_ne!(
        sig,
        RData::Sig {
            type_covered: 1,
            algorithm: 2,
            labels: 3,
            original_ttl: 0,
            expiration: 5,
            time_signed: 6,
            key_tag: 7,
            signer: b"s".to_vec(),
            signature: vec![9]
        }
    );
    assert_ne!(
        sig,
        RData::Sig {
            type_covered: 1,
            algorithm: 2,
            labels: 3,
            original_ttl: 4,
            expiration: 0,
            time_signed: 6,
            key_tag: 7,
            signer: b"s".to_vec(),
            signature: vec![9]
        }
    );
    assert_ne!(
        sig,
        RData::Sig {
            type_covered: 1,
            algorithm: 2,
            labels: 3,
            original_ttl: 4,
            expiration: 5,
            time_signed: 0,
            key_tag: 7,
            signer: b"s".to_vec(),
            signature: vec![9]
        }
    );
    assert_ne!(
        sig,
        RData::Sig {
            type_covered: 1,
            algorithm: 2,
            labels: 3,
            original_ttl: 4,
            expiration: 5,
            time_signed: 6,
            key_tag: 0,
            signer: b"s".to_vec(),
            signature: vec![9]
        }
    );
    assert_ne!(
        sig,
        RData::Sig {
            type_covered: 1,
            algorithm: 2,
            labels: 3,
            original_ttl: 4,
            expiration: 5,
            time_signed: 6,
            key_tag: 7,
            signer: b"t".to_vec(),
            signature: vec![9]
        }
    );
    assert_ne!(
        sig,
        RData::Sig {
            type_covered: 1,
            algorithm: 2,
            labels: 3,
            original_ttl: 4,
            expiration: 5,
            time_signed: 6,
            key_tag: 7,
            signer: b"s".to_vec(),
            signature: vec![0]
        }
    );

    let soa = RData::Soa {
        primary: b"p".to_vec(),
        admin: b"a".to_vec(),
        serial: 1,
        refresh: 2,
        retry: 3,
        expire: 4,
        minimum: 5,
    };
    assert_ne!(
        soa,
        RData::Soa {
            primary: b"q".to_vec(),
            admin: b"a".to_vec(),
            serial: 1,
            refresh: 2,
            retry: 3,
            expire: 4,
            minimum: 5
        }
    );
    assert_ne!(
        soa,
        RData::Soa {
            primary: b"p".to_vec(),
            admin: b"q".to_vec(),
            serial: 1,
            refresh: 2,
            retry: 3,
            expire: 4,
            minimum: 5
        }
    );
    assert_ne!(
        soa,
        RData::Soa {
            primary: b"p".to_vec(),
            admin: b"a".to_vec(),
            serial: 0,
            refresh: 2,
            retry: 3,
            expire: 4,
            minimum: 5
        }
    );
    assert_ne!(
        soa,
        RData::Soa {
            primary: b"p".to_vec(),
            admin: b"a".to_vec(),
            serial: 1,
            refresh: 0,
            retry: 3,
            expire: 4,
            minimum: 5
        }
    );
    assert_ne!(
        soa,
        RData::Soa {
            primary: b"p".to_vec(),
            admin: b"a".to_vec(),
            serial: 1,
            refresh: 2,
            retry: 0,
            expire: 4,
            minimum: 5
        }
    );
    assert_ne!(
        soa,
        RData::Soa {
            primary: b"p".to_vec(),
            admin: b"a".to_vec(),
            serial: 1,
            refresh: 2,
            retry: 3,
            expire: 0,
            minimum: 5
        }
    );
    assert_ne!(
        soa,
        RData::Soa {
            primary: b"p".to_vec(),
            admin: b"a".to_vec(),
            serial: 1,
            refresh: 2,
            retry: 3,
            expire: 4,
            minimum: 0
        }
    );

    let srv = RData::Srv {
        priority: 1,
        weight: 2,
        port: 3,
        target: b"t".to_vec(),
    };
    assert_ne!(
        srv,
        RData::Srv {
            priority: 0,
            weight: 2,
            port: 3,
            target: b"t".to_vec()
        }
    );
    assert_ne!(
        srv,
        RData::Srv {
            priority: 1,
            weight: 0,
            port: 3,
            target: b"t".to_vec()
        }
    );
    assert_ne!(
        srv,
        RData::Srv {
            priority: 1,
            weight: 2,
            port: 0,
            target: b"t".to_vec()
        }
    );
    assert_ne!(
        srv,
        RData::Srv {
            priority: 1,
            weight: 2,
            port: 3,
            target: b"u".to_vec()
        }
    );

    assert_ne!(RData::A([1, 2, 3, 4]), RData::A([1, 2, 3, 5]));
    assert_ne!(RData::Aaaa([0; 16]), RData::Aaaa([1; 16]));
    assert_ne!(RData::Name(b"a".to_vec()), RData::Name(b"b".to_vec()));
    assert_ne!(RData::Null(vec![1]), RData::Null(vec![2]));
    assert_ne!(RData::Opt(vec![1]), RData::Opt(vec![2]));
    assert_ne!(
        RData::Strings(vec![b"a".to_vec()]),
        RData::Strings(vec![b"b".to_vec()])
    );
    assert_ne!(RData::A([1, 2, 3, 4]), RData::Name(b"a".to_vec()));

    let record = Record {
        name: b"a.".to_vec(),
        section: Section::Answer,
        rtype: 1,
        ttl: 2,
        data: RData::A([1, 2, 3, 4]),
    };
    assert!(format!("{record:?}").starts_with("Record {"));
    assert_eq!(record.clone(), record);
    assert_ne!(
        record,
        Record {
            name: b"b.".to_vec(),
            ..record.clone()
        }
    );
    assert_ne!(
        record,
        Record {
            section: Section::Additional,
            ..record.clone()
        }
    );
    assert_ne!(
        record,
        Record {
            rtype: 2,
            ..record.clone()
        }
    );
    assert_ne!(
        record,
        Record {
            ttl: 3,
            ..record.clone()
        }
    );
    assert_ne!(
        record,
        Record {
            data: RData::A([0; 4]),
            ..record.clone()
        }
    );
}

#[test]
fn hand_small_values() {
    for (section, name) in [
        (Section::Answer, "Answer"),
        (Section::Additional, "Additional"),
    ] {
        assert_eq!(format!("{:?}", Clone::clone(&section)), name);
    }
    assert_ne!(Section::Answer, Section::Additional);

    let formats = [
        (0, NameFormat::Domain, "Domain"),
        (1, NameFormat::DomainLabel, "DomainLabel"),
        (2, NameFormat::HostnameFull, "HostnameFull"),
        (3, NameFormat::HostnameLabel, "HostnameLabel"),
        (4, NameFormat::Wildcard, "Wildcard"),
        (5, NameFormat::SrvRecord, "SrvRecord"),
    ];
    for (raw, format, name) in formats {
        assert_eq!(NameFormat::from_raw(raw), Some(format));
        assert_eq!(format!("{:?}", Clone::clone(&format)), name);
    }
    assert_eq!(NameFormat::from_raw(6), None);
    assert_ne!(NameFormat::Domain, NameFormat::Wildcard);

    let errors = [
        (Error::InvalidParameter, 87, "InvalidParameter"),
        (Error::InvalidName, 123, "InvalidName"),
        (Error::NotImplemented, 9004, "NotImplemented"),
        (Error::BadPacket, 9502, "BadPacket"),
        (Error::NonRfcName, 9556, "NonRfcName"),
        (Error::InvalidNameChar, 9560, "InvalidNameChar"),
    ];
    for (error, code, name) in errors {
        assert_eq!(error.code(), code);
        assert_eq!(format!("{:?}", Clone::clone(&error)), name);
    }
    assert_ne!(Error::BadPacket, Error::NonRfcName);
}
