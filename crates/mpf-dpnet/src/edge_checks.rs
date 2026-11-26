//! Boundary and branch tests for the ported dpnet address logic.
//!
//! The oracle rows live in `oracle_checks.rs`. Every expectation here is
//! not oracle-derived unless its comment says otherwise; the expectations come
//! from reading the C code in `dlls/dpnet/address.c`.

use super::*;

fn w(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// UTF-16LE bytes of `text` plus its NUL terminator, as a STRING payload.
fn wz(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .chain([0])
        .flat_map(u16::to_le_bytes)
        .collect()
}

fn add(address: &mut Address, name: &str, datatype: u32, data: &[u8]) {
    address
        .add_component(&w(name), datatype, Some(data))
        .unwrap();
}

fn dword(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

/// The UTF-16 URL with its NUL terminator, for the given body text.
fn url(body: &str) -> Vec<u16> {
    w(&format!("x-directplay:/{body}\0"))
}

// Not oracle-derived: each wrong-size or malformed payload is rejected.
#[test]
fn size_mismatches_are_invalid_param() {
    let mut address = Address::default();
    for size in [0, 3, 5] {
        let data = vec![0u8; size];
        assert_eq!(
            address.add_component(&w("d"), DPNA_DATATYPE_DWORD, Some(&data)),
            Err(AddError::InvalidParam),
            "DWORD size {size}"
        );
    }
    for size in [0, 15, 17] {
        let data = vec![0u8; size];
        assert_eq!(
            address.add_component(&w("g"), DPNA_DATATYPE_GUID, Some(&data)),
            Err(AddError::InvalidParam),
            "GUID size {size}"
        );
    }
    assert_eq!(address.num_components(), 0);
}

// Not oracle-derived: STRING needs an even size, a NUL, and a size that
// matches the first NUL exactly.
#[test]
fn string_malformed_payloads_are_invalid_param() {
    let mut address = Address::default();
    // Odd size.
    assert_eq!(
        address.add_component(&w("s"), DPNA_DATATYPE_STRING, Some(&[0x61, 0x00, 0x00])),
        Err(AddError::InvalidParam)
    );
    // No NUL inside the buffer.
    assert_eq!(
        address.add_component(&w("s"), DPNA_DATATYPE_STRING, Some(&[0x61, 0x00])),
        Err(AddError::InvalidParam)
    );
    // NUL after one unit, but the buffer runs on past it.
    assert_eq!(
        address.add_component(
            &w("s"),
            DPNA_DATATYPE_STRING,
            Some(&[0x61, 0x00, 0x00, 0x00, 0x62, 0x00])
        ),
        Err(AddError::InvalidParam)
    );
    assert_eq!(address.num_components(), 0);
}

// Not oracle-derived: ANSI needs a NUL, and its size must equal strlen + 1.
#[test]
fn ansi_malformed_payloads_are_invalid_param() {
    let mut address = Address::default();
    assert_eq!(
        address.add_component(&w("a"), DPNA_DATATYPE_STRING_ANSI, Some(b"a\0\0")),
        Err(AddError::InvalidParam)
    );
    assert_eq!(
        address.add_component(&w("a"), DPNA_DATATYPE_STRING_ANSI, Some(b"a")),
        Err(AddError::InvalidParam)
    );
    assert_eq!(
        address.add_component(&w("a"), DPNA_DATATYPE_STRING_ANSI, Some(b"a\0b\0")),
        Err(AddError::InvalidParam)
    );
    assert_eq!(address.num_components(), 0);
}

// Not oracle-derived: the NULL check runs before the type check, so a NULL
// pointer is reported for every type, including unknown ones.
#[test]
fn null_data_is_checked_before_type() {
    let mut address = Address::default();
    for datatype in [DPNA_DATATYPE_DWORD, DPNA_DATATYPE_STRING, 9] {
        assert_eq!(
            address.add_component(&w("p"), datatype, None),
            Err(AddError::InvalidPointer),
            "type {datatype}"
        );
    }
    assert_eq!(address.num_components(), 0);
}

// Not oracle-derived: empty STRING and ANSI values render as "name=".
#[test]
fn empty_values_render_as_name_equals() {
    let mut address = Address::default();
    add(&mut address, "s", DPNA_DATATYPE_STRING, &wz(""));
    assert_eq!(address.get_url(64), Ok(url("s=")));

    let mut address = Address::default();
    add(&mut address, "e", DPNA_DATATYPE_STRING_ANSI, b"\0");
    assert_eq!(address.get_url(64), Ok(url("e=")));
}

// Not oracle-derived: an empty component name is valid and renders as "=value".
#[test]
fn empty_name_is_valid() {
    let mut address = Address::default();
    add(&mut address, "", DPNA_DATATYPE_DWORD, &dword(1));
    assert_eq!(address.get_url(64), Ok(url("=1")));
    assert_eq!(
        address.get_component_by_name(&w(""), 4),
        Ok(Retrieved {
            datatype: DPNA_DATATYPE_DWORD,
            size: 4,
            bytes: dword(1).to_vec(),
        })
    );
}

// Not oracle-derived: a name is stored up to its first NUL, so a later NUL and
// anything after it do not take part in matching.
#[test]
fn name_is_truncated_at_first_nul() {
    let mut address = Address::default();
    address
        .add_component(
            &[0x61, 0x62, 0x00, 0x63],
            DPNA_DATATYPE_DWORD,
            Some(&dword(5)),
        )
        .unwrap();
    assert_eq!(address.get_url(64), Ok(url("ab=5")));
    assert_eq!(
        address.get_component_by_name(&w("ab"), 4).map(|r| r.size),
        Ok(4)
    );
    assert_eq!(
        address
            .get_component_by_name(&[0x61, 0x62, 0x00, 0x7a], 4)
            .map(|r| r.size),
        Ok(4)
    );
    assert_eq!(
        address.get_component_by_name(&w("abc"), 4),
        Err(NameError::DoesNotExist)
    );
}

// Not oracle-derived: names match code unit by code unit, so case matters.
// Wine's lstrcmpW uses CompareStringW with flags 0, which is also
// case-sensitive, but this has not been checked against that call.
#[test]
fn names_are_case_sensitive() {
    let mut address = Address::default();
    add(&mut address, "A", DPNA_DATATYPE_DWORD, &dword(1));
    add(&mut address, "a", DPNA_DATATYPE_DWORD, &dword(2));
    assert_eq!(address.num_components(), 2);
    assert_eq!(address.get_url(64), Ok(url("A=1;a=2")));
}

// Not oracle-derived: a replaced component keeps its position in the list.
#[test]
fn replacement_keeps_position() {
    let mut address = Address::default();
    add(&mut address, "a", DPNA_DATATYPE_DWORD, &dword(1));
    add(&mut address, "b", DPNA_DATATYPE_DWORD, &dword(2));
    add(&mut address, "a", DPNA_DATATYPE_DWORD, &dword(3));
    assert_eq!(address.num_components(), 2);
    assert_eq!(address.get_url(64), Ok(url("a=3;b=2")));
}

// Not oracle-derived: DWORD renders as a signed 32-bit value, like %ld on Windows.
#[test]
fn dword_renders_as_signed_32_bit() {
    let mut address = Address::default();
    add(&mut address, "m", DPNA_DATATYPE_DWORD, &dword(0x8000_0000));
    assert_eq!(address.get_url(64), Ok(url("m=-2147483648")));

    let mut address = Address::default();
    add(&mut address, "m", DPNA_DATATYPE_DWORD, &dword(0x7FFF_FFFF));
    assert_eq!(address.get_url(64), Ok(url("m=2147483647")));

    let mut address = Address::default();
    add(&mut address, "m", DPNA_DATATYPE_DWORD, &dword(0));
    assert_eq!(address.get_url(64), Ok(url("m=0")));
}

// Not oracle-derived: non-ASCII STRING values pass through as UTF-16 code units.
#[test]
fn string_values_keep_non_ascii_units() {
    let mut address = Address::default();
    add(&mut address, "n", DPNA_DATATYPE_STRING, &wz("é€"));
    assert_eq!(address.get_url(64), Ok(url("n=é€")));
}

// Not oracle-derived: a binary entry first, or after a skipped entry, adds no
// separator, because the C test is on the length so far.
#[test]
fn leading_binary_and_unknown_add_no_separator() {
    let mut address = Address::default();
    add(&mut address, "b", DPNA_DATATYPE_BINARY, &[1, 2]);
    add(&mut address, "u", 9, &[1, 2]);
    add(&mut address, "c", DPNA_DATATYPE_DWORD, &dword(2));
    assert_eq!(address.get_url(64), Ok(url("c=2")));
}

// Not oracle-derived: a binary entry at the end leaves a trailing separator.
#[test]
fn trailing_binary_leaves_separator() {
    let mut address = Address::default();
    add(&mut address, "c", DPNA_DATATYPE_DWORD, &dword(2));
    add(&mut address, "b", DPNA_DATATYPE_BINARY, &[1, 2]);
    assert_eq!(address.get_url(64), Ok(url("c=2;")));
}

// Not oracle-derived: two binary entries between values give three separators.
#[test]
fn double_binary_gives_triple_separator() {
    let mut address = Address::default();
    add(&mut address, "a", DPNA_DATATYPE_DWORD, &dword(1));
    add(&mut address, "b", DPNA_DATATYPE_BINARY, &[1, 2]);
    add(&mut address, "d", DPNA_DATATYPE_BINARY, &[1, 2]);
    add(&mut address, "c", DPNA_DATATYPE_DWORD, &dword(2));
    assert_eq!(address.num_components(), 4);
    assert_eq!(address.get_url(64), Ok(url("a=1;;;c=2")));
}

// Not oracle-derived: a zero-size binary is valid and copies nothing.
#[test]
fn zero_size_binary_is_valid() {
    let mut address = Address::default();
    add(&mut address, "b", DPNA_DATATYPE_BINARY, &[]);
    assert_eq!(address.get_url(64), Ok(url("")));
    assert_eq!(
        address.get_component_by_name(&w("b"), 0),
        Ok(Retrieved {
            datatype: DPNA_DATATYPE_BINARY,
            size: 0,
            bytes: Vec::new(),
        })
    );
}

// Not oracle-derived: the URL capacity check is inclusive, and a one-short
// capacity reports the full length including the NUL.
#[test]
fn url_capacity_is_inclusive() {
    let mut address = Address::default();
    add(&mut address, "port", DPNA_DATATYPE_DWORD, &dword(80));
    let len = url("port=80").len();
    assert_eq!(address.get_url(len), Ok(url("port=80")));
    assert_eq!(address.get_url(len - 1), Err(len));
}

// Not oracle-derived: lookups by name. A capacity equal to the size succeeds;
// one unit short reports the size.
#[test]
fn by_name_capacity_boundary() {
    let mut address = Address::default();
    add(&mut address, "guid", DPNA_DATATYPE_GUID, &GUID_BYTES);
    assert_eq!(
        address
            .get_component_by_name(&w("guid"), 16)
            .map(|r| r.size),
        Ok(16)
    );
    assert_eq!(
        address.get_component_by_name(&w("guid"), 15),
        Err(NameError::BufferTooSmall { size: 16 })
    );
}

// Not oracle-derived: an ANSI value is returned with its NUL, and a binary value
// is returned as stored.
#[test]
fn by_name_returns_ansi_and_binary_bytes() {
    let mut address = Address::default();
    add(&mut address, "k", DPNA_DATATYPE_STRING_ANSI, b"v\0");
    add(&mut address, "b", DPNA_DATATYPE_BINARY, &[1, 2, 3]);
    assert_eq!(
        address.get_component_by_name(&w("k"), 2),
        Ok(Retrieved {
            datatype: DPNA_DATATYPE_STRING_ANSI,
            size: 2,
            bytes: b"v\0".to_vec(),
        })
    );
    assert_eq!(
        address.get_component_by_name(&w("b"), 2),
        Err(NameError::BufferTooSmall { size: 3 })
    );
    assert_eq!(
        address.get_component_by_name(&w("b"), 3),
        Ok(Retrieved {
            datatype: DPNA_DATATYPE_BINARY,
            size: 3,
            bytes: vec![1, 2, 3],
        })
    );
}

// Not oracle-derived: an empty address has no name to match, so every name is
// DoesNotExist.
#[test]
fn empty_address_has_no_names() {
    let address = Address::default();
    assert_eq!(
        address.get_component_by_name(&w("x"), 0),
        Err(NameError::DoesNotExist)
    );
    assert_eq!(
        address.get_component_by_index(0, 0, 0),
        Err(IndexError::DoesNotExist)
    );
}

// Not oracle-derived: index == count is DoesNotExist. The C bounds check is `>`,
// so it reads one slot past the array there; the port does not follow that read.
#[test]
fn index_at_count_is_does_not_exist() {
    let address = indexed_check();
    assert_eq!(address.num_components(), 2);
    assert_eq!(
        address.get_component_by_index(2, 64, 64),
        Err(IndexError::DoesNotExist)
    );
    assert_eq!(
        address.get_component_by_index(usize::MAX, 64, 64),
        Err(IndexError::DoesNotExist)
    );
}

// Not oracle-derived: index lookups. An exact data capacity succeeds.
#[test]
fn by_index_data_capacity_boundary() {
    let address = indexed_check();
    assert_eq!(
        address
            .get_component_by_index(1, 4, 4)
            .map(|(name, retrieved)| (name, retrieved.size)),
        Ok((w("port"), 4))
    );
}

// Not oracle-derived: name capacity one short of the name is BUFFERTOOSMALL,
// with name_len counting the NUL.
#[test]
fn by_index_name_capacity_one_short() {
    let address = indexed_check();
    assert_eq!(
        address.get_component_by_index(1, 3, 64),
        Err(IndexError::BufferTooSmall {
            name_len: 5,
            size: 4,
            datatype: DPNA_DATATYPE_DWORD,
        })
    );
}

// Not oracle-derived: an unknown type copies no bytes by index either, but its
// size and type are still reported.
#[test]
fn by_index_unknown_type_copies_nothing() {
    let mut address = Address::default();
    add(&mut address, "u", 9, &[1, 2]);
    assert_eq!(
        address.get_component_by_index(0, 64, 64),
        Ok((
            w("u"),
            Retrieved {
                datatype: 9,
                size: 2,
                bytes: Vec::new(),
            }
        ))
    );
    assert_eq!(
        address.get_component_by_index(0, 64, 1),
        Err(IndexError::BufferTooSmall {
            name_len: 2,
            size: 2,
            datatype: 9,
        })
    );
}

// Not oracle-derived: GUID byte conversion round-trips, and the fields match
// the GUID's Windows layout.
#[test]
fn guid_codec_round_trips() {
    let guid = Guid::from_le_bytes(GUID_BYTES);
    assert_eq!(guid.data1, 0x1234_5678);
    assert_eq!(guid.data2, 0x9ABC);
    assert_eq!(guid.data3, 0xDEF0);
    assert_eq!(guid.data4, [1, 2, 3, 4, 5, 6, 7, 0xff]);
    assert_eq!(guid.to_le_bytes(), GUID_BYTES);
}

/// The index-lookup fixture from the oracle rows, duplicated here so the
/// edge tests do not depend on oracle helpers.
fn indexed_check() -> Address {
    let mut address = Address::default();
    add(&mut address, "guid", DPNA_DATATYPE_GUID, &GUID_BYTES);
    add(&mut address, "port", DPNA_DATATYPE_DWORD, &dword(80));
    address
}

/// The GUID bytes for {12345678-9ABC-DEF0-0102-0304050607FF}.
const GUID_BYTES: [u8; 16] = [
    0x78, 0x56, 0x34, 0x12, 0xbc, 0x9a, 0xf0, 0xde, 1, 2, 3, 4, 5, 6, 7, 0xff,
];
