//! Oracle-derived test vectors for the ported wintypes functions.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from `dlls/wintypes` (`main.c`,
//! `buffer.c`, `map.c`) with `gcc -fsanitize=address,undefined -g` under `target/canon/wintypes/`
//! and running the harness. Only the OS primitives (HSTRING, wcsicmp, ordinal comparison,
//! CoTaskMem) are stubbed. The run was clean under both sanitizers, so no vector was excluded.

use super::*;

struct TypeRow {
    input: &'static str,
    hresult: u32,
    parts: &'static [&'static str],
}

/// `RoParseTypeName` rows from the verbatim `parse_type` oracle.
const TYPE_ROWS: [TypeRow; 14] = [
    TypeRow {
        input: "Windows.Foundation.Point",
        hresult: 0,
        parts: &["Windows.Foundation.Point"],
    },
    TypeRow {
        input: "",
        hresult: 0x8007_0057,
        parts: &[],
    },
    TypeRow {
        input: " A",
        hresult: 0x8000_0011,
        parts: &[],
    },
    TypeRow {
        input: "A B",
        hresult: 0x8000_0011,
        parts: &[],
    },
    TypeRow {
        input: "Windows.Foundation.Collections.IVector`1<Int32>",
        hresult: 0,
        parts: &["Windows.Foundation.Collections.IVector`1", "Int32"],
    },
    TypeRow {
        input: "IMap`2<String, Int32> ",
        hresult: 0,
        parts: &["IMap`2", "String", "Int32"],
    },
    TypeRow {
        input: "IVector`1<IMap`2<String, Int32>>",
        hresult: 0,
        parts: &["IVector`1", "IMap`2", "String", "Int32"],
    },
    TypeRow {
        input: "A`0<B>",
        hresult: 0x8000_0011,
        parts: &[],
    },
    // Wine accepts a missing second parameter here and returns only the name part.
    TypeRow {
        input: "A`2<B>",
        hresult: 0,
        parts: &["A`2"],
    },
    TypeRow {
        input: "A`1<B",
        hresult: 0x8000_0011,
        parts: &[],
    },
    TypeRow {
        input: "A`1<B>>",
        hresult: 0x8000_0011,
        parts: &[],
    },
    TypeRow {
        input: "A`1<>",
        hresult: 0x8000_0011,
        parts: &[],
    },
    TypeRow {
        input: "A\0B",
        hresult: 0,
        parts: &["A"],
    },
    TypeRow {
        input: "A`2<B`1<C>,D>",
        hresult: 0,
        parts: &["A`2", "B`1", "C", "D"],
    },
];

struct ContractRow {
    name: &'static str,
    version: u32,
    present: bool,
}

/// `is_api_contract_present` rows from the verbatim oracle.
const CONTRACT_ROWS: [ContractRow; 4] = [
    ContractRow {
        name: "Windows.Foundation.UniversalApiContract",
        version: 10,
        present: true,
    },
    ContractRow {
        name: "Windows.Foundation.UniversalApiContract",
        version: 11,
        present: false,
    },
    ContractRow {
        name: "windows.foundation.UNIVERSALAPICONTRACT",
        version: 0,
        present: true,
    },
    ContractRow {
        name: "Windows.Foundation.UniversalApiContract\0x",
        version: 1,
        present: true,
    },
];

struct BufferRow {
    capacity: u32,
    value: u32,
    hresult: u32,
    length: u32,
}

/// `buffer_put_Length` rows from the verbatim oracle; each row starts from a fresh buffer.
const BUFFER_ROWS: [BufferRow; 3] = [
    BufferRow {
        capacity: 4,
        value: 4,
        hresult: 0,
        length: 4,
    },
    BufferRow {
        capacity: 4,
        value: 5,
        hresult: 0x8007_0057,
        length: 0,
    },
    BufferRow {
        capacity: 0,
        value: 0,
        hresult: 0,
        length: 0,
    },
];

struct MapRow {
    keys: &'static [&'static str],
    key: &'static str,
    index: usize,
    order: Ordering,
}

/// `map_entries_lower_bound` rows from the verbatim oracle.
const MAP_ROWS: [MapRow; 4] = [
    MapRow {
        keys: &[],
        key: "a",
        index: 0,
        order: Ordering::Less,
    },
    MapRow {
        keys: &["a", "c", "e"],
        key: "c",
        index: 1,
        order: Ordering::Equal,
    },
    MapRow {
        keys: &["a", "c", "e"],
        key: "b",
        index: 1,
        order: Ordering::Less,
    },
    MapRow {
        keys: &["b"],
        key: "a",
        index: 0,
        order: Ordering::Greater,
    },
];

fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn type_outcome(input: &str) -> (u32, Vec<String>) {
    match parse_type_name(input) {
        Ok(parts) => (0, parts),
        Err(error) => (error.hresult(), Vec::new()),
    }
}

#[test]
fn oracle_type_name_rows() {
    for row in &TYPE_ROWS {
        let (hresult, parts) = type_outcome(row.input);
        assert_eq!(hresult, row.hresult, "hresult for {:?}", row.input);
        let expected: Vec<String> = row.parts.iter().map(|p| p.to_string()).collect();
        assert_eq!(parts, expected, "parts for {:?}", row.input);
    }
}

#[test]
fn oracle_api_contract_rows() {
    for row in &CONTRACT_ROWS {
        assert_eq!(
            is_api_contract_present(row.name, row.version),
            row.present,
            "presence of {:?} version {}",
            row.name,
            row.version
        );
    }
}

#[test]
fn oracle_buffer_put_length_rows() {
    for row in &BUFFER_ROWS {
        let mut buffer = Buffer::new(row.capacity);
        let hresult = match buffer.set_length(row.value) {
            Ok(()) => 0,
            Err(error) => error.hresult(),
        };
        assert_eq!(
            hresult, row.hresult,
            "hresult for put_Length({})",
            row.value
        );
        assert_eq!(
            buffer.length(),
            row.length,
            "length after put_Length({})",
            row.value
        );
    }
}

#[test]
fn oracle_map_lower_bound_rows() {
    for row in &MAP_ROWS {
        let keys: Vec<Vec<u16>> = row.keys.iter().map(|k| utf16(k)).collect();
        assert_eq!(
            map_lower_bound(&keys, &utf16(row.key)),
            (row.index, row.order),
            "lower bound of {:?} in {:?}",
            row.key,
            row.keys
        );
    }
}
