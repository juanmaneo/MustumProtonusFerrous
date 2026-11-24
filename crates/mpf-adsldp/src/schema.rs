//! Port of `get_schema_type` from `schema.c`: finds an attribute type by name and maps its syntax
//! OID to an ADS type.

use std::cmp::Ordering;

use crate::{cstr, icase_cmp};

/// One entry of the attribute type table, as `struct attribute_type` in `adsldp_private.h`. The C
/// struct keeps all names of an entry in one buffer; here each name is its own vector. Entries
/// with a single name come first and are sorted by that name.
pub struct AttributeType {
    pub names: Vec<Vec<u16>>,
    pub syntax: Option<Vec<u16>>,
}

/// `ADSTYPEENUM` from `iads.h`, limited to the types `get_schema_type` returns.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AdsType {
    DnString = 1,
    CaseIgnoreString = 3,
    Boolean = 6,
    Integer = 7,
    OctetString = 8,
    UtcTime = 9,
    LargeInteger = 10,
    NtSecurityDescriptor = 25,
    DnWithBinary = 27,
}

/// Syntax OIDs that `get_schema_type` maps, in the order `schema.c` tests them.
const SYNTAX_TYPES: &[(&str, AdsType)] = &[
    ("1.3.6.1.4.1.1466.115.121.1.7", AdsType::Boolean),
    ("1.3.6.1.4.1.1466.115.121.1.12", AdsType::DnString),
    ("1.3.6.1.4.1.1466.115.121.1.15", AdsType::CaseIgnoreString),
    ("1.3.6.1.4.1.1466.115.121.1.24", AdsType::UtcTime),
    ("1.3.6.1.4.1.1466.115.121.1.26", AdsType::CaseIgnoreString),
    ("1.3.6.1.4.1.1466.115.121.1.27", AdsType::Integer),
    ("1.3.6.1.4.1.1466.115.121.1.38", AdsType::CaseIgnoreString),
    ("1.3.6.1.4.1.1466.115.121.1.40", AdsType::OctetString),
    ("1.2.840.113556.1.4.903", AdsType::DnWithBinary),
    ("1.2.840.113556.1.4.906", AdsType::LargeInteger),
    ("1.2.840.113556.1.4.907", AdsType::NtSecurityDescriptor),
];

/// Returns the ADS type of the attribute called `name`. `singles` are searched by binary search
/// and `multiples` linearly. An unknown name or syntax gives `CaseIgnoreString`.
pub fn get_schema_type(
    name: &[u16],
    singles: &[AttributeType],
    multiples: &[AttributeType],
) -> AdsType {
    find_sorted(name, singles)
        .or_else(|| find_linear(name, multiples))
        .and_then(|attr| attr.syntax.as_deref())
        .map_or(AdsType::CaseIgnoreString, syntax_type)
}

fn syntax_type(syntax: &[u16]) -> AdsType {
    let oid = cstr(syntax);
    SYNTAX_TYPES
        .iter()
        .find(|(text, _)| text.encode_utf16().eq(oid.iter().copied()))
        .map_or(AdsType::CaseIgnoreString, |&(_, ty)| ty)
}

/// Binary search by the first name. The probes match `find_schema_type_sorted`, so on a table
/// that is not sorted the search can miss a name that is present.
fn find_sorted<'a>(name: &[u16], entries: &'a [AttributeType]) -> Option<&'a AttributeType> {
    let mut min: isize = 0;
    let mut max = entries.len() as isize - 1;
    while min <= max {
        let idx = (min + max) / 2;
        let attr = &entries[idx as usize];
        match icase_cmp(name, &attr.names[0]) {
            Ordering::Equal => return Some(attr),
            Ordering::Greater => min = idx + 1,
            Ordering::Less => max = idx - 1,
        }
    }
    None
}

fn find_linear<'a>(name: &[u16], entries: &'a [AttributeType]) -> Option<&'a AttributeType> {
    entries.iter().find(|attr| {
        attr.names
            .iter()
            .any(|candidate| icase_cmp(candidate, name).is_eq())
    })
}
