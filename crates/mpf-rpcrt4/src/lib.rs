//! UUID string conversions from Wine's `dlls/rpcrt4/rpcrt4_main.c`.
//!
//! This is a string-only subset. It mirrors Wine's lowercase-only hex parser
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
            from_string(Some("00112233-4455-6677-8899-AABBCCDDEEFF")),
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
}
