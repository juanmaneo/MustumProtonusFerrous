//! Oracle-derived test vectors for the ported dpnet address logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/dpnet/address.c` (plus the `struct component` layout from
//! `dpnet_private.h`) with `gcc -fsanitize=address,undefined -fshort-wchar -g`
//! under `target/canon/dpnet/`, and running `harness.c`. The harness printed 24
//! rows (u01-u10, a01-a05, n01-n04, i01-i05). ASan and UBSan flagged none, so
//! none are excluded.
//!
//! `-fshort-wchar` makes `WCHAR` 16-bit, as on Windows. Rows whose C call passes a
//! capacity smaller than the name or data (a02, i05) are noted at their test.
//! Lengths quoted in comments include the terminating NUL.

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

/// `GUID {12345678-9ABC-DEF0-0102-0304050607FF}` in `GUID` memory layout.
const GUID_BYTES: [u8; 16] = [
    0x78, 0x56, 0x34, 0x12, 0xbc, 0x9a, 0xf0, 0xde, 1, 2, 3, 4, 5, 6, 7, 0xff,
];

/// The address built by the `n` rows: host=STRING "ab", port=DWORD, u=type 9.
fn named_address() -> Address {
    let mut address = Address::default();
    add(&mut address, "host", DPNA_DATATYPE_STRING, &wz("ab"));
    add(
        &mut address,
        "port",
        DPNA_DATATYPE_DWORD,
        &0x0102_0304u32.to_le_bytes(),
    );
    add(&mut address, "u", 9, &[0x11, 0x22]);
    address
}

/// The address built by the `i` rows: guid=GUID, port=DWORD 80.
fn indexed_address() -> Address {
    let mut address = Address::default();
    add(&mut address, "guid", DPNA_DATATYPE_GUID, &GUID_BYTES);
    add(
        &mut address,
        "port",
        DPNA_DATATYPE_DWORD,
        &80u32.to_le_bytes(),
    );
    address
}

#[test]
fn oracle_get_url_rows() {
    let empty = Address::default();
    // u01: empty, capacity 0 -> DPNERR_BUFFERTOOSMALL, len 15.
    assert_eq!(empty.get_url(0), Err(15));
    // u02: empty, capacity 15 -> DPN_OK, len 15.
    assert_eq!(empty.get_url(15), Ok(w("x-directplay:/\0")));

    // u03: DWORD port=80 -> len 22.
    let mut address = Address::default();
    add(
        &mut address,
        "port",
        DPNA_DATATYPE_DWORD,
        &80u32.to_le_bytes(),
    );
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/port=80\0")));

    // u04: DWORD n=0xFFFFFFFF -> "-1" (32-bit %ld), len 19.
    let mut address = Address::default();
    add(
        &mut address,
        "n",
        DPNA_DATATYPE_DWORD,
        &0xFFFF_FFFFu32.to_le_bytes(),
    );
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/n=-1\0")));

    // u05: STRING host=example.org -> len 31.
    let mut address = Address::default();
    add(
        &mut address,
        "host",
        DPNA_DATATYPE_STRING,
        &wz("example.org"),
    );
    assert_eq!(
        address.get_url(64),
        Ok(w("x-directplay:/host=example.org\0"))
    );

    // u06: GUID -> len 62, with the %7B/%7D escapes.
    let mut address = Address::default();
    add(&mut address, "guid", DPNA_DATATYPE_GUID, &GUID_BYTES);
    assert_eq!(
        address.get_url(64),
        Ok(w(
            "x-directplay:/guid=%7B12345678-9ABC-DEF0-0102-0304050607FF%7D\0"
        ))
    );

    // u07: ANSI k=v -> len 18.
    let mut address = Address::default();
    add(&mut address, "k", DPNA_DATATYPE_STRING_ANSI, b"v\0");
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/k=v\0")));

    // u08: DWORD a=1, BINARY b, DWORD c=2. The binary entry leaves ";;" -> len 23.
    let mut address = Address::default();
    add(&mut address, "a", DPNA_DATATYPE_DWORD, &1u32.to_le_bytes());
    add(&mut address, "b", DPNA_DATATYPE_BINARY, &[0xde, 0xad]);
    add(&mut address, "c", DPNA_DATATYPE_DWORD, &2u32.to_le_bytes());
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/a=1;;c=2\0")));

    // u09: BINARY b, DWORD c=2. No leading separator -> len 18.
    let mut address = Address::default();
    add(&mut address, "b", DPNA_DATATYPE_BINARY, &[0xde, 0xad]);
    add(&mut address, "c", DPNA_DATATYPE_DWORD, &2u32.to_le_bytes());
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/c=2\0")));

    // u10: DWORD a=1, capacity one unit short of len 18 -> DPNERR_BUFFERTOOSMALL.
    let mut address = Address::default();
    add(&mut address, "a", DPNA_DATATYPE_DWORD, &1u32.to_le_bytes());
    assert_eq!(address.get_url(17), Err(18));
}

#[test]
fn oracle_add_component_rows() {
    // a01: DWORD with size 3 -> DPNERR_INVALIDPARAM, count 0, URL len 15.
    let mut address = Address::default();
    let data = 7u32.to_le_bytes();
    assert_eq!(
        address.add_component(&w("size3"), DPNA_DATATYPE_DWORD, Some(&data[..3])),
        Err(AddError::InvalidParam)
    );
    assert_eq!(address.num_components(), 0);
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/\0")));

    // a02: STRING "ab" with size 4 (needs 6) -> DPNERR_INVALIDPARAM, count 0.
    // The C code finds the NUL one unit past the 4 bytes; the slice here has none.
    // Both paths reject the size.
    let mut address = Address::default();
    assert_eq!(
        address.add_component(&w("s"), DPNA_DATATYPE_STRING, Some(&wz("ab")[..4])),
        Err(AddError::InvalidParam)
    );
    assert_eq!(address.num_components(), 0);
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/\0")));

    // a03: NULL data -> DPNERR_INVALIDPOINTER, count 0.
    let mut address = Address::default();
    assert_eq!(
        address.add_component(&w("p"), DPNA_DATATYPE_DWORD, None),
        Err(AddError::InvalidPointer)
    );
    assert_eq!(address.num_components(), 0);
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/\0")));

    // a04: STRING a=old, then DWORD a=7 replaces it in place -> count 1, len 18.
    let mut address = Address::default();
    add(&mut address, "a", DPNA_DATATYPE_STRING, &wz("old"));
    add(&mut address, "a", DPNA_DATATYPE_DWORD, &7u32.to_le_bytes());
    assert_eq!(address.num_components(), 1);
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/a=7\0")));

    // a05: unknown type 9, size 2 -> DPN_OK, count 1, URL len 15 (header only).
    let mut address = Address::default();
    assert_eq!(address.add_component(&w("u"), 9, Some(&[1, 2])), Ok(()));
    assert_eq!(address.num_components(), 1);
    assert_eq!(address.get_url(64), Ok(w("x-directplay:/\0")));
}

#[test]
fn oracle_get_component_by_name_rows() {
    let address = named_address();

    // n01: "missing" -> DPNERR_DOESNOTEXIST.
    assert_eq!(
        address.get_component_by_name(&w("missing"), 10),
        Err(NameError::DoesNotExist)
    );
    // n02: "port" with capacity 2 -> DPNERR_BUFFERTOOSMALL, size 4.
    assert_eq!(
        address.get_component_by_name(&w("port"), 2),
        Err(NameError::BufferTooSmall { size: 4 })
    );
    // n03: "host" -> size 6, type 1, bytes 61 00 62 00 00 00.
    assert_eq!(
        address.get_component_by_name(&w("host"), 64),
        Ok(Retrieved {
            datatype: DPNA_DATATYPE_STRING,
            size: 6,
            bytes: vec![0x61, 0x00, 0x62, 0x00, 0x00, 0x00],
        })
    );
    // n04: "u" (type 9) -> size 2, type 9, no bytes copied.
    assert_eq!(
        address.get_component_by_name(&w("u"), 64),
        Ok(Retrieved {
            datatype: 9,
            size: 2,
            bytes: Vec::new(),
        })
    );
}

#[test]
fn oracle_get_component_by_index_rows() {
    let address = indexed_address();

    // i01: index 3 with 2 components -> DPNERR_DOESNOTEXIST.
    assert_eq!(
        address.get_component_by_index(3, 64, 64),
        Err(IndexError::DoesNotExist)
    );
    // i02: index 0, name capacity 3 for "guid" -> BUFFERTOOSMALL, name_len 5, size 16, type 3.
    assert_eq!(
        address.get_component_by_index(0, 3, 64),
        Err(IndexError::BufferTooSmall {
            name_len: 5,
            size: 16,
            datatype: DPNA_DATATYPE_GUID,
        })
    );
    // i03: index 1, data capacity 3 for DWORD -> BUFFERTOOSMALL, name_len 5, size 4, type 2.
    assert_eq!(
        address.get_component_by_index(1, 64, 3),
        Err(IndexError::BufferTooSmall {
            name_len: 5,
            size: 4,
            datatype: DPNA_DATATYPE_DWORD,
        })
    );
    // i04: index 0 -> name "guid", name_len 5, size 16, type 3, bytes as GUID memory.
    assert_eq!(
        address.get_component_by_index(0, 64, 64),
        Ok((
            w("guid"),
            Retrieved {
                datatype: DPNA_DATATYPE_GUID,
                size: 16,
                bytes: GUID_BYTES.to_vec(),
            }
        ))
    );
    // i05: index 1, name capacity 4 equal to "port"'s length -> DPN_OK.
    // The C code also writes the NUL one unit past the 4-unit capacity; the
    // harness buffer is 64 units, so ASan does not see it.
    assert_eq!(
        address.get_component_by_index(1, 4, 64),
        Ok((
            w("port"),
            Retrieved {
                datatype: DPNA_DATATYPE_DWORD,
                size: 4,
                bytes: vec![0x50, 0x00, 0x00, 0x00],
            }
        ))
    );
}
