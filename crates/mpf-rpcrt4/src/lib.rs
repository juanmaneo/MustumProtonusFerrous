//! UUID string conversions from Wine's `dlls/rpcrt4/rpcrt4_main.c`.
//!
//! This is a string-only subset. It mirrors Wine's case-insensitive hex parser
//! and 8-4-4-4-12 formatting. `None` models the C API's null string, which
//! parses as the nil UUID. Allocation and RPC_STATUS constants are not ABI
//! compatible; errors are represented by `UuidError`.

/// The RPC UUID string parsing error (`RPC_S_INVALID_STRING_UUID`, 1705).
pub const RPC_S_INVALID_STRING_UUID: u32 = 1705;

/// UUID fields in Windows `UUID` layout.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Uuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// Invalid UUID input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UuidError;

/// Parses Wine-compatible UUID text; `None` yields the nil UUID.
pub fn from_string(input: Option<&str>) -> Result<Uuid, UuidError> {
    let Some(input) = input else {
        return Ok(Uuid::default());
    };
    let bytes = input.as_bytes();
    if bytes.len() != 36 {
        return Err(UuidError);
    }
    if bytes[8] != b'-' || bytes[13] != b'-' || bytes[18] != b'-' || bytes[23] != b'-' {
        return Err(UuidError);
    }
    let mut nibbles = [0u8; 32];
    let mut out = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        if i == 8 || i == 13 || i == 18 || i == 23 {
            continue;
        }
        nibbles[out] = hex(byte).ok_or(UuidError)?;
        out += 1;
    }
    let data1 = nibbles[0..8]
        .iter()
        .fold(0u32, |v, n| (v << 4) | u32::from(*n));
    let data2 = nibbles[8..12]
        .iter()
        .fold(0u16, |v, n| (v << 4) | u16::from(*n));
    let data3 = nibbles[12..16]
        .iter()
        .fold(0u16, |v, n| (v << 4) | u16::from(*n));
    let mut data4 = [0; 8];
    for i in 0..8 {
        data4[i] = (nibbles[16 + i * 2] << 4) | nibbles[17 + i * 2];
    }
    Ok(Uuid {
        data1,
        data2,
        data3,
        data4,
    })
}

fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Formats a UUID as lowercase 8-4-4-4-12 text.
pub fn to_string(uuid: Option<&Uuid>) -> String {
    let uuid = uuid.copied().unwrap_or_default();
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        uuid.data1,
        uuid.data2,
        uuid.data3,
        uuid.data4[0],
        uuid.data4[1],
        uuid.data4[2],
        uuid.data4[3],
        uuid.data4[4],
        uuid.data4[5],
        uuid.data4[6],
        uuid.data4[7]
    )
}

/// Parses NUL-terminated UTF-16 UUID text. An empty slice models a null pointer.
pub fn from_string_wide(input: &[u16]) -> Result<Uuid, UuidError> {
    let len = input.iter().position(|c| *c == 0).unwrap_or(input.len());
    let bytes: Option<Vec<u8>> = input[..len].iter().map(|c| u8::try_from(*c).ok()).collect();
    if input.is_empty() {
        return Ok(Uuid::default());
    }
    match bytes {
        Some(bytes) => match std::str::from_utf8(&bytes) {
            Ok(text) => from_string(Some(text)),
            Err(_) => Err(UuidError),
        },
        None => Err(UuidError),
    }
}

/// Formats as NUL-terminated UTF-16, matching `UuidToStringW` output shape.
pub fn to_string_wide(uuid: Option<&Uuid>) -> Vec<u16> {
    to_string(uuid)
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}

/// Orders UUIDs field by field like `UuidCompare`; `None` models the nil UUID.
pub fn uuid_compare(a: Option<&Uuid>, b: Option<&Uuid>) -> i32 {
    let a = a.copied().unwrap_or_default();
    let b = b.copied().unwrap_or_default();
    (a.data1, a.data2, a.data3, a.data4).cmp(&(b.data1, b.data2, b.data3, b.data4)) as i32
}

/// `UuidEqual`: true when `uuid_compare` returns zero.
pub fn uuid_equal(a: Option<&Uuid>, b: Option<&Uuid>) -> bool {
    uuid_compare(a, b) == 0
}

/// `UuidIsNil`: `None` and the all-zero UUID are both nil.
pub fn uuid_is_nil(uuid: Option<&Uuid>) -> bool {
    uuid.copied().unwrap_or_default() == Uuid::default()
}

fn uuid_bytes(uuid: &Uuid) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[0..4].copy_from_slice(&uuid.data1.to_le_bytes());
    bytes[4..6].copy_from_slice(&uuid.data2.to_le_bytes());
    bytes[6..8].copy_from_slice(&uuid.data3.to_le_bytes());
    bytes[8..16].copy_from_slice(&uuid.data4);
    bytes
}

/// `UuidHash` (FreeDCE checksum over the 16-byte memory image).
///
/// The C code keeps `c0`/`c1` in `short` (wrapping) and uses truncated `%`
/// followed by a `+255` fixup for negatives, which is `rem_euclid(255)`.
pub fn uuid_hash(uuid: Option<&Uuid>) -> u16 {
    let uuid = uuid.copied().unwrap_or_default();
    let mut c0: i16 = 0;
    let mut c1: i16 = 0;
    for byte in uuid_bytes(&uuid) {
        c0 = c0.wrapping_add(i16::from(byte));
        c1 = c1.wrapping_add(c0);
    }
    let x = (-i32::from(c1)).rem_euclid(255);
    let y = (i32::from(c1) - i32::from(c0)).rem_euclid(255);
    (y * 256 + x) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "00112233-4455-6677-8899-aabbccddeeff";

    #[test]
    fn parse_and_format_roundtrip() {
        let uuid = from_string(Some(SAMPLE)).unwrap();
        assert_eq!(uuid.data1, 0x0011_2233);
        assert_eq!(uuid.data2, 0x4455);
        assert_eq!(uuid.data3, 0x6677);
        assert_eq!(uuid.data4, [0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
        assert_eq!(to_string(Some(&uuid)), SAMPLE);
        assert_eq!(from_string(Some(&to_string(Some(&uuid)))).unwrap(), uuid);
        assert_eq!(
            from_string(Some("00112233-4455-6677-8899-AABBCCDDEEFF")),
            Ok(uuid)
        );
    }

    #[test]
    fn null_formats_and_parses_as_nil() {
        assert_eq!(from_string(None), Ok(Uuid::default()));
        assert_eq!(to_string(None), "00000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn rejects_wrong_length_and_separators() {
        assert_eq!(from_string(Some("short")), Err(UuidError));
        assert_eq!(
            from_string(Some("00112233_4455-6677-8899-aabbccddeeff")),
            Err(UuidError)
        );
    }

    #[test]
    fn rejects_invalid_digits_at_each_field_kind() {
        for index in [0, 9, 14, 19, 24, 35] {
            let mut value = SAMPLE.as_bytes().to_vec();
            value[index] = b'g';
            assert_eq!(
                from_string(Some(std::str::from_utf8(&value).unwrap())),
                Err(UuidError)
            );
        }
        assert_eq!(
            from_string(Some("00112233-4455-6677-8899-GABBCCDDEEFF")),
            Err(UuidError)
        );
    }

    #[test]
    fn wide_roundtrip_and_invalid_code_unit() {
        let wide: Vec<u16> = SAMPLE.encode_utf16().chain([0]).collect();
        let uuid = from_string_wide(&wide).unwrap();
        assert_eq!(to_string_wide(Some(&uuid)), wide);
        assert_eq!(from_string_wide(&[0x100]), Err(UuidError));
        assert_eq!(from_string_wide(&[0x80]), Err(UuidError));
        assert_eq!(from_string_wide(&[]), Ok(Uuid::default()));
        assert_eq!(from_string_wide(&[0]), Err(UuidError));
    }

    #[test]
    fn rpc_status_constant_matches_wine() {
        assert_eq!(RPC_S_INVALID_STRING_UUID, 1705);
    }

    const NIL: Uuid = Uuid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };

    fn uuid_from_memory(bytes: [u8; 16]) -> Uuid {
        Uuid {
            data1: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            data2: u16::from_le_bytes([bytes[4], bytes[5]]),
            data3: u16::from_le_bytes([bytes[6], bytes[7]]),
            data4: [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ],
        }
    }

    #[test]
    fn compare_orders_by_each_field() {
        let fields = [
            Uuid { data1: 1, ..NIL },
            Uuid { data2: 1, ..NIL },
            Uuid { data3: 1, ..NIL },
            Uuid {
                data4: [0, 0, 0, 0, 0, 0, 0, 1],
                ..NIL
            },
        ];
        for field in fields {
            assert_eq!(uuid_compare(Some(&NIL), Some(&field)), -1);
            assert_eq!(uuid_compare(Some(&field), Some(&NIL)), 1);
        }
        assert_eq!(uuid_compare(None, None), 0);
        assert_eq!(uuid_compare(None, Some(&NIL)), 0);
        let high = Uuid {
            data1: 0x8000_0000,
            ..NIL
        };
        assert_eq!(uuid_compare(Some(&high), None), 1);
        let hi_byte = Uuid {
            data4: [0, 0, 0, 0, 0, 0, 0, 0x80],
            ..NIL
        };
        let lo_byte = Uuid {
            data4: [0, 0, 0, 0, 0, 0, 0, 0x7f],
            ..NIL
        };
        assert_eq!(uuid_compare(Some(&hi_byte), Some(&lo_byte)), 1);
    }

    #[test]
    fn equal_and_nil_predicates() {
        let nonzero = Uuid { data3: 1, ..NIL };
        assert!(uuid_equal(None, Some(&NIL)));
        assert!(uuid_equal(Some(&nonzero), Some(&nonzero)));
        assert!(!uuid_equal(Some(&nonzero), None));
        assert!(uuid_is_nil(None));
        assert!(uuid_is_nil(Some(&NIL)));
        assert!(!uuid_is_nil(Some(&nonzero)));
    }

    #[test]
    fn hash_matches_freedce_checksum_vectors() {
        let vectors: [([u8; 16], u16); 6] = [
            ([0; 16], 0),
            ([0xff; 16], 65025),
            (
                [
                    0x04, 0x03, 0x02, 0x01, 0x06, 0x05, 0x08, 0x07, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                    0x0e, 0x0f, 0x0a,
                ],
                46790,
            ),
            ([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 4079),
            ([0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0),
            (
                [
                    0x78, 0x56, 0x34, 0x12, 0xbc, 0x9a, 0xf0, 0xde, 0x11, 0x22, 0x33, 0x44, 0x55,
                    0x66, 0x77, 0x88,
                ],
                30438,
            ),
        ];
        for (bytes, expected) in vectors {
            assert_eq!(uuid_hash(Some(&uuid_from_memory(bytes))), expected);
        }
        assert_eq!(uuid_hash(None), 0);
    }
}
