//! Oracle rows from Wine's `dlls/dnsapi`, built verbatim under ASan and UBSan:
//! name validation (R01-R14), name compare (R15-R19) and message parsing
//! (R20-R25). Each expected value is the output of that build, not a value
//! derived by hand. Names are the harness's Rust literals, unchanged.

use crate::name::{name_compare, validate_name};
use crate::wire::{extract_records, RData, Record, Section};
use crate::Error;

const NAMES: &[(&str, Option<&str>, u32, u32)] = &[
    ("R01", Some("example.com"), 0, 0),
    ("R02", Some("exa mple.com"), 0, 9560),
    ("R03", Some("my_host.example"), 0, 9556),
    ("R04", Some("caf\u{e9}.example"), 0, 9556),
    ("R05", Some("a.b"), 1, 123),
    ("R06", Some("*.example.com"), 4, 0),
    ("R07", Some("www.*.example"), 4, 123),
    ("R08", Some("_"), 5, 9556),
    (
        "R09",
        Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        0,
        123,
    ),
    (
        "R10",
        Some("a.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        0,
        123,
    ),
    ("R11", None, 2, 123),
    ("R12", Some("a..b"), 0, 123),
    ("R13", Some("a.b"), 9, 0),
    ("R14", Some("a\u{0}b"), 0, 0),
];

const COMPARES: &[(&str, Option<&str>, Option<&str>, bool)] = &[
    ("R15", Some("Example.COM."), Some("example.com"), true),
    ("R16", Some("CAF\u{c9}"), Some("caf\u{e9}"), true),
    ("R17", Some("a.b"), Some("a.bc"), false),
    ("R18", None, Some(""), false),
    ("R19", Some("."), Some(""), true),
];

const MESSAGES: &[(&str, &str, u32, &[&str])] = &[
    ("R20", "123481800100030000000000076578616d706c6503636f6d0000010001027777c00c000100010000012c000401020304c00c002100010000003c000c000a000501bb03737276c00c03777777c00c00100001000000010009026869057468657265", 0, &[
        "sec=1 type=1 ttl=300 name=ww.example.com a=01020304",
        "sec=1 type=33 ttl=60 name=example.com prio=10 weight=5 port=443 target=srv.example.com",
        "sec=1 type=16 ttl=1 name=www.example.com strings=2 [hi] [there]",
    ]),
    ("R21", "123481800000010000000200076578616d706c6503636f6d00000f000100000e100007000a026d78c00c016dc00c001c000100000005001020010db8000000000000000000000001c00c000500010000000a000603777777c00c", 0, &[
        "sec=1 type=15 ttl=3600 name=example.com pref=10 exchange=mx.example.com",
        "sec=3 type=28 ttl=5 name=m.example.com aaaa=20010db8000000000000000000000001",
        "sec=3 type=5 ttl=10 name=example.com host=www.example.com",
    ]),
    ("R22", "12348180000001000000000000004200010000000000020102", 9004, &[]),
    ("R23", "12348180000001000000000000000100010000000000040102", 9502, &[]),
    ("R24", "123481800000000000000000", 87, &[]),
    ("R25", "12348180010000000000000001610000010001", 9502, &[]),
];

pub(crate) fn wide(s: Option<&str>) -> Option<Vec<u16>> {
    s.map(|s| s.encode_utf16().collect())
}

pub(crate) fn hex_bytes(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex digits"))
        .collect()
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub(crate) fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

pub(crate) fn name_status(result: Result<(), Error>) -> u32 {
    match result {
        Ok(()) => 0,
        Err(e) => e.code(),
    }
}

/// The line that the harness prints for a record: section, type, ttl and name,
/// then the fields of its rdata.
pub(crate) fn dump(record: &Record) -> String {
    let section = match record.section {
        Section::Answer => 1,
        Section::Additional => 3,
    };
    let head = format!(
        "sec={section} type={} ttl={} name={}",
        record.rtype,
        record.ttl,
        text(&record.name)
    );
    let body = match &record.data {
        RData::A(addr) => format!(" a={}", hex(addr)),
        RData::Aaaa(addr) => format!(" aaaa={}", hex(addr)),
        RData::Key {
            flags,
            protocol,
            algorithm,
            key,
        } => format!(" flags={flags} proto={protocol} alg={algorithm} key={}", hex(key)),
        RData::Minfo {
            mailbox,
            errors_mailbox,
        } => format!(" mailbox={} errors={}", text(mailbox), text(errors_mailbox)),
        RData::Mx {
            preference,
            exchange,
        } => format!(" pref={preference} exchange={}", text(exchange)),
        RData::Name(host) => format!(" host={}", text(host)),
        RData::Null(data) => format!(" null={}", hex(data)),
        RData::Opt(data) => format!(" opt={}", hex(data)),
        RData::Sig {
            type_covered,
            algorithm,
            labels,
            original_ttl,
            expiration,
            time_signed,
            key_tag,
            signer,
            signature,
        } => format!(
            " cov={type_covered} alg={algorithm} labels={labels} ottl={original_ttl} exp={expiration} signed={time_signed} tag={key_tag} signer={} sig={}",
            text(signer),
            hex(signature)
        ),
        RData::Soa {
            primary,
            admin,
            serial,
            refresh,
            retry,
            expire,
            minimum,
        } => format!(
            " primary={} admin={} serial={serial} refresh={refresh} retry={retry} expire={expire} minttl={minimum}",
            text(primary),
            text(admin)
        ),
        RData::Srv {
            priority,
            weight,
            port,
            target,
        } => format!(
            " prio={priority} weight={weight} port={port} target={}",
            text(target)
        ),
        RData::Strings(strings) => {
            let mut out = format!(" strings={}", strings.len());
            for s in strings {
                out.push_str(&format!(" [{}]", text(s)));
            }
            out
        }
    };
    format!("{head}{body}")
}

/// Runs the parser the way the harness does: the status, then the dump line of
/// each record when the message parses.
pub(crate) fn run(msg: &[u8]) -> (u32, Vec<String>) {
    match extract_records(msg) {
        Ok(records) => (0, records.iter().map(dump).collect()),
        Err(e) => (e.code(), Vec::new()),
    }
}

pub(crate) fn check_names(rows: &[(&str, Option<&str>, u32, u32)]) {
    for &(id, name, format, status) in rows {
        let name = wide(name);
        assert_eq!(
            name_status(validate_name(name.as_deref(), format)),
            status,
            "{id}"
        );
    }
}

pub(crate) fn check_compares(rows: &[(&str, Option<&str>, Option<&str>, bool)]) {
    for &(id, a, b, same) in rows {
        let (a, b) = (wide(a), wide(b));
        assert_eq!(name_compare(a.as_deref(), b.as_deref()), same, "{id}");
    }
}

pub(crate) fn check_messages(rows: &[(&str, &str, u32, &[&str])]) {
    for &(id, msg, status, lines) in rows {
        let want: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        assert_eq!(run(&hex_bytes(msg)), (status, want), "{id}");
    }
}

#[test]
fn oracle_names() {
    check_names(NAMES);
}

#[test]
fn oracle_compare() {
    check_compares(COMPARES);
}

#[test]
fn oracle_messages() {
    check_messages(MESSAGES);
}
