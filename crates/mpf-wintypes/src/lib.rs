//! Port of the portable logic in Wine's `dlls/wintypes`.
//!
//! Ported:
//! - `RoParseTypeName` parser (`parse_type` and `parse_part` in `main.c`): [`parse_type_name`]
//! - `is_api_contract_present` (`main.c`, used by `IsApiContractPresentByMajor`): [`is_api_contract_present`]
//! - `buffer_put_Length` argument check (`buffer.c`): [`Buffer::set_length`]
//! - `map_entries_lower_bound` binary search (`map.c`): [`map_lower_bound`]
//!
//! Skipped:
//! - `RoIsApiContractMajorVersionPresent` and `IsApiContractPresentByMajorAndMinor`: FIXME stubs.
//! - `RoResolveNamespace`: reads metadata files from disk.
//! - `map.c` pair, iterator and view objects, and `map_entry_init` / `map_entry_copy` /
//!   `map_entry_clear`: HSTRING duplication and IInspectable reference counting.
//! - `propertyset.c`, `storage.c`, and the property value and IReference objects in `main.c`:
//!   COM objects that own IInspectable, HSTRING or IBuffer storage.

use std::cmp::Ordering;

/// HRESULT failures returned by the ported checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// `E_INVALIDARG`.
    InvalidArg,
    /// `RO_E_METADATA_INVALID_TYPE_FORMAT`.
    InvalidTypeFormat,
}

impl Error {
    /// The HRESULT Wine returns for this error.
    pub fn hresult(self) -> u32 {
        match self {
            Error::InvalidArg => 0x8007_0057,
            Error::InvalidTypeFormat => 0x8000_0011,
        }
    }
}

/// `RoParseTypeName`: splits a type name such as ``IMap`2<String, Int32>`` into the name of
/// each parameterized type followed by its parameters, in order.
///
/// As in the C code, input is read up to its first NUL code unit, but the empty and
/// leading-space checks look at the whole string. Wine does not check that a parameterized
/// type has as many parameters as its arity: ``A`2<B>`` is accepted and yields only
/// ``["A`2"]``. The port keeps that behaviour to match the oracle.
pub fn parse_type_name(type_name: &str) -> Result<Vec<String>, Error> {
    if type_name.is_empty() {
        return Err(Error::InvalidArg);
    }
    if type_name.starts_with(' ') {
        return Err(Error::InvalidTypeFormat);
    }
    let mut parts = Vec::new();
    parse_type(truncate_at_nul(type_name), &mut parts)?;
    Ok(parts)
}

fn truncate_at_nul(s: &str) -> &str {
    s.find('\0').map_or(s, |i| &s[..i])
}

/// `parse_part`: trims spaces, then rejects an empty part or one with a space inside.
fn parse_part(input: &str) -> Result<String, Error> {
    let part = input.trim_matches(' ');
    if part.is_empty() || part.contains(' ') {
        return Err(Error::InvalidTypeFormat);
    }
    Ok(part.to_owned())
}

/// `parse_type`. Indices are byte offsets. Every slice boundary is either the start, the end, or
/// just past an ASCII delimiter, so the slices always fall on char boundaries.
fn parse_type(input: &str, parts: &mut Vec<String>) -> Result<(), Error> {
    let bytes = input.as_bytes();
    let end = bytes.len();
    let mut ptr = bytes
        .iter()
        .position(|&c| matches!(c, b'`' | b'<' | b'>' | b','))
        .unwrap_or(end);

    if ptr > 0 && ptr < end && bytes[ptr] == b'`' {
        ptr += 1;
        let parameter_count = match bytes.get(ptr) {
            Some(&c @ b'1'..=b'9') => u32::from(c - b'0'),
            _ => return Err(Error::InvalidTypeFormat),
        };
        ptr += 1;
        if bytes.get(ptr) != Some(&b'<') {
            return Err(Error::InvalidTypeFormat);
        }
        parts.push(parse_part(&input[..ptr])?);
        ptr += 1;

        let mut nested_level: u32 = 1;
        for i in 0..parameter_count {
            let part_start = ptr;
            while ptr < end {
                match bytes[ptr] {
                    b'<' => {
                        nested_level += 1;
                        ptr += 1;
                    }
                    b'>' if i == parameter_count - 1 && nested_level == 1 => {
                        parse_type(&input[part_start..ptr], parts)?;
                        nested_level -= 1;
                        ptr += 1;
                        break;
                    }
                    b'>' => {
                        nested_level = nested_level.wrapping_sub(1);
                        ptr += 1;
                    }
                    b',' if nested_level == 1 => {
                        parse_type(&input[part_start..ptr], parts)?;
                        ptr += 1;
                        break;
                    }
                    _ => ptr += 1,
                }
            }
        }

        if nested_level != 0 || bytes[ptr..].iter().any(|&c| c != b' ') {
            return Err(Error::InvalidTypeFormat);
        }
        return Ok(());
    }

    if ptr != end {
        return Err(Error::InvalidTypeFormat);
    }
    parts.push(parse_part(input)?);
    Ok(())
}

/// Contracts known to `is_api_contract_present`, with the highest major version each one reports.
const PRESENT_CONTRACTS: [(&str, u32); 1] = [("Windows.Foundation.UniversalApiContract", 10)];

/// `is_api_contract_present`: finds the contract by ASCII case-insensitive name, then reports
/// whether `version` is at most its maximum major version. Input is read up to its first NUL.
pub fn is_api_contract_present(contract_name: &str, version: u32) -> bool {
    let name = truncate_at_nul(contract_name);
    PRESENT_CONTRACTS
        .iter()
        .find(|(contract, _)| contract.eq_ignore_ascii_case(name))
        .is_some_and(|&(_, max_major)| version <= max_major)
}

/// The length state of a `Windows.Storage.Streams.Buffer` (`buffer.c`). The capacity is fixed at
/// creation and the length starts at zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Buffer {
    capacity: u32,
    length: u32,
}

impl Buffer {
    /// Creates an empty buffer with the given capacity.
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            length: 0,
        }
    }

    /// The fixed capacity.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// The current length.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// `buffer_put_Length`: rejects a value above the capacity and leaves the length unchanged.
    pub fn set_length(&mut self, value: u32) -> Result<(), Error> {
        if value > self.capacity {
            return Err(Error::InvalidArg);
        }
        self.length = value;
        Ok(())
    }
}

/// `map_entries_lower_bound`: binary search over keys sorted in ordinal (UTF-16 code unit) order.
///
/// Returns the index of the first key not less than `key`, or `keys.len()` if there is none, and
/// the result of the last comparison made. Wine stores that comparison in `*ret` and callers treat
/// `Ordering::Equal` as found, so a miss can still report `Less` or `Greater`.
pub fn map_lower_bound<K: AsRef<[u16]>>(keys: &[K], key: &[u16]) -> (usize, Ordering) {
    let mut begin = 0;
    let mut end = keys.len();
    let mut order = Ordering::Less;
    while begin < end {
        let mid = begin + (end - begin) / 2;
        order = keys[mid].as_ref().cmp(key);
        match order {
            Ordering::Less => begin = mid + 1,
            Ordering::Greater => end = mid,
            Ordering::Equal => {
                begin = mid;
                end = mid;
            }
        }
    }
    (begin, order)
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
