//! Fixtures shared by the oracle and edge tests.
//!
//! The tables mirror the ones `driver.c` builds for Wine: one-name types first, then
//! multi-name types. Both sides are generated from the same lists in
//! `target/canon/adsldp/gen_all.py`.

use crate::path::parse_ldap_path;
use crate::schema::{get_schema_type, AdsType, AttributeType};
use crate::HResult;

/// One oracle row for `lookup`: name, table, start, single count, multi count, result.
pub type SchemaRow = (&'static str, Table, usize, usize, usize, AdsType);
/// One oracle row for `check_path`: text and the host, port and object, or the error.
pub type PathRow = (
    &'static str,
    Result<(Option<&'static str>, u32, Option<&'static str>), HResult>,
);

/// Which of the two schema tables a row reads.
#[derive(Clone, Copy)]
pub enum Table {
    Full,
    Unsorted,
}

pub fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

pub fn narrow(units: &[u16]) -> String {
    String::from_utf16_lossy(units)
}

/// Runs `get_schema_type` on the `single + multiple` entries of `table` from `start`, the way
/// Wine's driver calls it.
pub fn lookup(table: Table, start: usize, single: usize, multiple: usize, name: &str) -> AdsType {
    let all = match table {
        Table::Full => full_table(),
        Table::Unsorted => unsorted_table(),
    };
    let entries = &all[start..start + single + multiple];
    let (singles, multiples) = entries.split_at(single);
    get_schema_type(&wide(name), singles, multiples)
}

/// Checks `parse_ldap_path` on `text` against an oracle row.
pub fn check_path(text: &str, expected: Result<(Option<&str>, u32, Option<&str>), HResult>) {
    let got =
        parse_ldap_path(&wide(text)).map(|p| (p.host.map(narrow), p.port, p.object.map(narrow)));
    let expected = expected
        .map(|(host, port, object)| (host.map(str::to_owned), port, object.map(str::to_owned)));
    assert_eq!(got, expected, "{text:?}");
}

fn full_table() -> Vec<AttributeType> {
    vec![
        AttributeType {
            names: vec![wide("cn")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.15")),
        },
        AttributeType {
            names: vec![wide("distinguishedName")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.12")),
        },
        AttributeType {
            names: vec![wide("isDeleted")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.7")),
        },
        AttributeType {
            names: vec![wide("lastLogon")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.24")),
        },
        AttributeType {
            names: vec![wide("noSyntax")],
            syntax: None,
        },
        AttributeType {
            names: vec![wide("nTSecurityDescriptor")],
            syntax: Some(wide("1.2.840.113556.1.4.907")),
        },
        AttributeType {
            names: vec![wide("objectSid")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.40")),
        },
        AttributeType {
            names: vec![wide("serverState")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.27")),
        },
        AttributeType {
            names: vec![wide("unknownSyntax")],
            syntax: Some(wide("1.2.3.4")),
        },
        AttributeType {
            names: vec![wide("uSNChanged")],
            syntax: Some(wide("1.2.840.113556.1.4.906")),
        },
        AttributeType {
            names: vec![wide("ou"), wide("organizationalUnitName")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.15")),
        },
        AttributeType {
            names: vec![wide("bootParameter"), wide("bp")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.40")),
        },
        AttributeType {
            names: vec![wide("dnBinary"), wide("dnb")],
            syntax: Some(wide("1.2.840.113556.1.4.903")),
        },
        AttributeType {
            names: vec![wide("lastModified"), wide("lm")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.24")),
        },
    ]
}

fn unsorted_table() -> Vec<AttributeType> {
    vec![
        AttributeType {
            names: vec![wide("zz")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.7")),
        },
        AttributeType {
            names: vec![wide("bb")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.27")),
        },
        AttributeType {
            names: vec![wide("aa")],
            syntax: Some(wide("1.3.6.1.4.1.1466.115.121.1.7")),
        },
    ]
}
