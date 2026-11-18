//! Headless subset of Wine's `dlls/bluetoothapis/sdp.c`: SDP data element and attribute parsing over byte buffers.
//!
//! The four ported exports are pure functions of the buffers they are given. They never touch a radio, a device handle, the Bluetooth stack or process state.
//!
//! Ported (4 of 97 `bluetoothapis.spec` entries):
//! - `BluetoothSdpGetElementData`: `bluetooth_sdp_get_element_data`.
//! - `BluetoothSdpGetContainerElementData`: `bluetooth_sdp_get_container_element_data`. The cursor is an offset into the given buffer, not a pointer, and `None` is Wine's NULL handle.
//! - `BluetoothSdpEnumAttributes`: `bluetooth_sdp_enum_attributes`. The callback receives the attribute value element, not the id.
//! - `BluetoothSdpGetAttributeValue`: `bluetooth_sdp_get_attribute_value`.
//!
//! Return codes are Wine's (`ERROR_INVALID_PARAMETER` 87, `ERROR_INVALID_DATA` 13, `ERROR_FILE_NOT_FOUND` 2, `ERROR_NO_MORE_ITEMS` 259). `SetLastError` is not modelled. `bluetooth_sdp_enum_attributes` returns `Result<(), u32>`, and its `Err` carries the code Wine would pass to `SetLastError`.
//!
//! Deliberate differences from Wine, all on inputs where Wine reads outside the buffer it was given:
//! - Elements never extend past the slice given. Wine bounds a cursor element by `stream_size` measured from the cursor, so it can read past the buffer (oracle `A084` silently; `C058`, `C060`, `C063` and `N087` are reported by AddressSanitizer). Wine's NULL-cursor path reads one byte past the end of the buffer (`C060`).
//! - An empty sequence (`35 00`) has no children. `bluetooth_sdp_get_container_element_data` returns `ERROR_NO_MORE_ITEMS` and the enumeration succeeds with no callbacks. Wine reads past the body (`X03`, `X05`, AddressSanitizer), or for a bare container returns the bytes after it as the first child (`X04`).
//! - `BluetoothSdpEnumAttributes(NULL, ...)` returns `ERROR_INVALID_PARAMETER` as a BOOL, which is TRUE. This port returns `Err(87)`.
//!
//! Oracle ids such as `E003` or `C051` name rows of the Wine C oracle built under `target/canon/bluetoothapis` (gitignored); they are not checked in.
//!
//! Skipped (93 of 97):
//! - No C body in the reference tree, 73 entries, for example `BluetoothAddressToString`, `BluetoothSdpGetString` and `BthpMapStatusToErr`. Porting them would invent behaviour.
//! - Radio and device handles, 12 entries: `BluetoothFindFirstRadio`, `BluetoothFindNextRadio`, `BluetoothFindRadioClose`, `BluetoothGetRadioInfo`, `BluetoothFindFirstDevice`, `BluetoothFindNextDevice`, `BluetoothFindDeviceClose`, `BluetoothGetDeviceInfo`, `BluetoothIsConnectable`, `BluetoothIsDiscoverable`, `BluetoothEnableIncomingConnections`, `BluetoothEnableDiscovery`. They need radio or device handles and the Bluetooth stack.
//! - Authentication registration and responses, 4 entries: `BluetoothRegisterForAuthenticationEx`, `BluetoothUnregisterAuthentication`, `BluetoothSendAuthenticationResponseEx`, `BluetoothAuthenticateDeviceEx`. They register with the authentication system.
//! - `BluetoothRemoveDevice`, 1 entry. It finds a radio and calls `DeviceIoControl`.
//! - GATT, 3 entries: `BluetoothGATTGetServices`, `BluetoothGATTGetCharacteristics`, `BluetoothGATTGetCharacteristicValue`. They query services on a device through the stack.

#![forbid(unsafe_code)]

/// Success.
pub const ERROR_SUCCESS: u32 = 0;
/// `BluetoothSdpGetAttributeValue`: the attribute id is not in the sequence.
pub const ERROR_FILE_NOT_FOUND: u32 = 2;
/// `BluetoothSdpEnumAttributes`: the sequence or one of its entries is malformed.
pub const ERROR_INVALID_DATA: u32 = 13;
/// A null argument, a truncated or unsupported element, or a bad cursor.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// The cursor is at the end of the container.
pub const ERROR_NO_MORE_ITEMS: u32 = 259;

const TYPE_NIL: u8 = 0;
const TYPE_UINT: u8 = 1;
const TYPE_INT: u8 = 2;
const TYPE_UUID: u8 = 3;
const TYPE_STRING: u8 = 4;
const TYPE_BOOLEAN: u8 = 5;
const TYPE_SEQUENCE: u8 = 6;
const TYPE_ALTERNATIVE: u8 = 7;
const TYPE_URL: u8 = 8;

/// A decoded SDP data element.
///
/// Strings and URLs borrow their payload. Sequences and alternatives borrow the whole element, header included, so the caller can enumerate its children.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdpElement<'a> {
    Nil,
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Uuid16(u16),
    Uuid32(u32),
    Uuid128(u128),
    /// The raw octet, as in Wine's `booleanVal`.
    Boolean(u8),
    String(&'a [u8]),
    Url(&'a [u8]),
    Sequence(&'a [u8]),
    Alternative(&'a [u8]),
}

/// Port of `BluetoothSdpGetElementData`. Decodes the element at the start of `stream` into `data`.
///
/// On a rejected element `data` is set to `Nil`. A missing or empty `stream`, or a missing `data`, leaves `data` untouched.
pub fn bluetooth_sdp_get_element_data<'a>(
    stream: Option<&'a [u8]>,
    data: Option<&mut SdpElement<'a>>,
) -> u32 {
    let (Some(stream), Some(data)) = (stream, data) else {
        return ERROR_INVALID_PARAMETER;
    };
    if stream.is_empty() {
        return ERROR_INVALID_PARAMETER;
    }
    match read_into(stream, data) {
        Ok(_) => ERROR_SUCCESS,
        Err(code) => code,
    }
}

/// Port of `BluetoothSdpGetContainerElementData`. Decodes the next child of the sequence or alternative in `stream`.
///
/// `handle` is `None` before the first child, and afterwards holds the offset of the following child. It is only updated on success. `stream` must end where the container ends.
pub fn bluetooth_sdp_get_container_element_data<'a>(
    stream: Option<&'a [u8]>,
    handle: Option<&mut Option<usize>>,
    data: Option<&mut SdpElement<'a>>,
) -> u32 {
    let (Some(stream), Some(handle), Some(data)) = (stream, handle, data) else {
        return ERROR_INVALID_PARAMETER;
    };
    match container_step(stream, *handle, data) {
        Ok(next) => {
            *handle = Some(next);
            ERROR_SUCCESS
        }
        Err(code) => code,
    }
}

/// Port of `BluetoothSdpEnumAttributes`. Calls `callback` with each attribute id and the bytes of its value element, until the callback returns `false` or the list ends.
///
/// The container must be a sequence or alternative of `(uint16 id, value)` pairs. A malformed entry fails with `ERROR_INVALID_DATA`, but only if it is reached before the callback stops the walk. A missing `stream` or `callback` returns `ERROR_INVALID_PARAMETER`.
pub fn bluetooth_sdp_enum_attributes<'a>(
    stream: Option<&'a [u8]>,
    callback: Option<&mut dyn FnMut(u16, &'a [u8]) -> bool>,
) -> Result<(), u32> {
    let (Some(stream), Some(callback)) = (stream, callback) else {
        return Err(ERROR_INVALID_PARAMETER);
    };
    let mut data = SdpElement::Nil;
    if bluetooth_sdp_get_element_data(Some(stream), Some(&mut data)) != ERROR_SUCCESS {
        return Err(ERROR_INVALID_DATA);
    }
    let (SdpElement::Sequence(list) | SdpElement::Alternative(list)) = data else {
        return Err(ERROR_INVALID_DATA);
    };

    let mut cursor = None;
    loop {
        let mut attr_id = SdpElement::Nil;
        let after_id = match container_step(list, cursor, &mut attr_id) {
            Ok(next) => next,
            Err(ERROR_NO_MORE_ITEMS) => return Ok(()),
            Err(_) => return Err(ERROR_INVALID_DATA),
        };
        let SdpElement::Uint16(id) = attr_id else {
            return Err(ERROR_INVALID_DATA);
        };

        let mut value = SdpElement::Nil;
        let after_value =
            container_step(list, Some(after_id), &mut value).map_err(|_| ERROR_INVALID_DATA)?;
        if !callback(id, &list[after_id..after_value]) {
            return Ok(());
        }
        cursor = Some(after_value);
    }
}

/// Port of `BluetoothSdpGetAttributeValue`. Decodes the value of the first attribute whose id is `attr_id`.
///
/// A malformed list fails with `ERROR_INVALID_PARAMETER`, an absent id with `ERROR_FILE_NOT_FOUND`.
pub fn bluetooth_sdp_get_attribute_value<'a>(
    stream: Option<&'a [u8]>,
    attr_id: u16,
    data: Option<&mut SdpElement<'a>>,
) -> u32 {
    let (Some(stream), Some(data)) = (stream, data) else {
        return ERROR_INVALID_PARAMETER;
    };
    let mut found = None;
    let mut find = |id: u16, value: &'a [u8]| {
        if id == attr_id {
            found = Some(value);
            false
        } else {
            true
        }
    };
    if bluetooth_sdp_enum_attributes(Some(stream), Some(&mut find)).is_err() {
        return ERROR_INVALID_PARAMETER;
    }
    match found {
        Some(value) => bluetooth_sdp_get_element_data(Some(value), Some(data)),
        None => ERROR_FILE_NOT_FOUND,
    }
}

/// Decodes the child at `cursor`, or the first child when `cursor` is `None`. Returns the offset just past it.
fn container_step<'a>(
    stream: &'a [u8],
    cursor: Option<usize>,
    data: &mut SdpElement<'a>,
) -> Result<usize, u32> {
    let Some((&header, _)) = stream.split_first() else {
        return Err(ERROR_INVALID_PARAMETER);
    };
    let start = match cursor {
        None => first_child_offset(stream, header)?,
        Some(position) if position > stream.len() => return Err(ERROR_INVALID_PARAMETER),
        Some(position) if position == stream.len() => return Err(ERROR_NO_MORE_ITEMS),
        Some(position) => position,
    };
    Ok(start + read_into(&stream[start..], data)?)
}

/// Offset of the first child of a sequence or alternative whose header byte is `header`.
///
/// Returns `ERROR_NO_MORE_ITEMS` when the container is empty.
fn first_child_offset(stream: &[u8], header: u8) -> Result<usize, u32> {
    if !matches!(header >> 3, TYPE_SEQUENCE | TYPE_ALTERNATIVE) {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let width = match header & 0x7 {
        5 => 1,
        6 => 2,
        7 => 4,
        _ => return Err(ERROR_INVALID_PARAMETER),
    };
    let length_field = stream.get(1..1 + width).ok_or(ERROR_INVALID_PARAMETER)?;
    if be_value(length_field) == 0 {
        return Err(ERROR_NO_MORE_ITEMS);
    }
    Ok(1 + width)
}

/// Decodes the element at the start of `stream` into `data`. On failure `data` is set to `Nil`.
///
/// `stream` must not be empty.
fn read_into<'a>(stream: &'a [u8], data: &mut SdpElement<'a>) -> Result<usize, u32> {
    match read_element(stream) {
        Ok((element, consumed)) => {
            *data = element;
            Ok(consumed)
        }
        Err(code) => {
            *data = SdpElement::Nil;
            Err(code)
        }
    }
}

/// Decodes one element from the start of `stream`, returning it with its encoded length.
fn read_element<'a>(stream: &'a [u8]) -> Result<(SdpElement<'a>, usize), u32> {
    let Some((&header, body)) = stream.split_first() else {
        return Err(ERROR_INVALID_PARAMETER);
    };
    let ty = header >> 3;
    let size_desc = header & 0x7;
    match ty {
        TYPE_NIL if size_desc == 0 => Ok((SdpElement::Nil, 1)),
        TYPE_UINT | TYPE_INT | TYPE_UUID => read_fixed(ty, size_desc, body),
        TYPE_BOOLEAN if size_desc == 0 => {
            let &value = body.first().ok_or(ERROR_INVALID_PARAMETER)?;
            Ok((SdpElement::Boolean(value), 2))
        }
        TYPE_STRING | TYPE_URL | TYPE_SEQUENCE | TYPE_ALTERNATIVE => {
            read_variable(stream, body, ty, size_desc)
        }
        _ => Err(ERROR_INVALID_PARAMETER),
    }
}

/// Payload width of a fixed-size integer or UUID element, or `None` when the size descriptor is not allowed for that type.
///
/// Every accepted combination has width `2^size_desc`.
fn fixed_width(ty: u8, size_desc: u8) -> Option<usize> {
    match (ty, size_desc) {
        (TYPE_UINT | TYPE_INT, 0..=4) | (TYPE_UUID, 1 | 2 | 4) => Some(1 << size_desc),
        _ => None,
    }
}

/// Decodes an integer or UUID element. `body` is the bytes after the header.
fn read_fixed<'a>(ty: u8, size_desc: u8, body: &[u8]) -> Result<(SdpElement<'a>, usize), u32> {
    let width = fixed_width(ty, size_desc).ok_or(ERROR_INVALID_PARAMETER)?;
    let value = be_value(body.get(..width).ok_or(ERROR_INVALID_PARAMETER)?);
    let element = match (ty, width) {
        (TYPE_UINT, 1) => SdpElement::Uint8(value as u8),
        (TYPE_UINT, 2) => SdpElement::Uint16(value as u16),
        (TYPE_UINT, 4) => SdpElement::Uint32(value as u32),
        (TYPE_UINT, 8) => SdpElement::Uint64(value as u64),
        (TYPE_UINT, _) => SdpElement::Uint128(value),
        (TYPE_INT, 1) => SdpElement::Int8(value as u8 as i8),
        (TYPE_INT, 2) => SdpElement::Int16(value as u16 as i16),
        (TYPE_INT, 4) => SdpElement::Int32(value as u32 as i32),
        (TYPE_INT, 8) => SdpElement::Int64(value as u64 as i64),
        (TYPE_INT, _) => SdpElement::Int128(value as i128),
        (_, 2) => SdpElement::Uuid16(value as u16),
        (_, 4) => SdpElement::Uuid32(value as u32),
        _ => SdpElement::Uuid128(value),
    };
    Ok((element, 1 + width))
}

/// Decodes a string, URL, sequence or alternative. `stream` starts at the header and `body` is the rest of it.
///
/// The length field is 1, 2 or 4 big-endian bytes, selected by `size_desc` 5, 6 or 7. The payload must lie inside `stream`.
fn read_variable<'a>(
    stream: &'a [u8],
    body: &'a [u8],
    ty: u8,
    size_desc: u8,
) -> Result<(SdpElement<'a>, usize), u32> {
    let width = match size_desc {
        5 => 1,
        6 => 2,
        7 => 4,
        _ => return Err(ERROR_INVALID_PARAMETER),
    };
    let length = be_value(body.get(..width).ok_or(ERROR_INVALID_PARAMETER)?) as usize;
    let payload = &body[width..];
    if payload.len() < length {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let total = 1 + width + length;
    let element = match ty {
        TYPE_STRING => SdpElement::String(&payload[..length]),
        TYPE_URL => SdpElement::Url(&payload[..length]),
        TYPE_SEQUENCE => SdpElement::Sequence(&stream[..total]),
        _ => SdpElement::Alternative(&stream[..total]),
    };
    Ok((element, total))
}

/// Big-endian value of up to 16 bytes.
fn be_value(bytes: &[u8]) -> u128 {
    bytes
        .iter()
        .fold(0, |value, &byte| (value << 8) | u128::from(byte))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The oracle's record buffer (`N068`): attributes 0, 1, 4 and 4 again.
    const RECORD: &str = "351e0900000a0001000209000125036162630900043504080108020900040809";
    const SENTINEL: SdpElement<'static> = SdpElement::Uint8(0xee);

    fn hex(text: &str) -> Vec<u8> {
        (0..text.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
            .collect()
    }

    /// (id, input, cursor, return, next cursor, element) for the container vectors.
    type ContainerRow = (
        &'static str,
        &'static str,
        Option<usize>,
        u32,
        Option<usize>,
        SdpElement<'static>,
    );
    /// (id, input, entries, result) for the enumeration vectors.
    type EnumRow = (
        &'static str,
        &'static str,
        &'static [(u16, &'static [u8])],
        Result<(), u32>,
    );
    type Entries = Vec<(u16, Vec<u8>)>;

    fn enum_entries(bytes: &[u8]) -> (Result<(), u32>, Entries) {
        let mut seen = Vec::new();
        let mut record = |id: u16, value: &[u8]| {
            seen.push((id, value.to_vec()));
            true
        };
        let result = bluetooth_sdp_enum_attributes(Some(bytes), Some(&mut record));
        (result, seen)
    }

    /// Element vectors from the oracle `E` rows (`out.txt`).
    #[test]
    fn element_vectors_match_oracle() {
        let cases: Vec<(&str, String, u32, SdpElement)> = vec![
            ("E003", "082a".into(), 0, SdpElement::Uint8(0x2a)),
            ("E004", "090102".into(), 0, SdpElement::Uint16(0x0102)),
            (
                "E005",
                "0a01020304".into(),
                0,
                SdpElement::Uint32(0x0102_0304),
            ),
            (
                "E006",
                "0b0102030405060708".into(),
                0,
                SdpElement::Uint64(0x0102_0304_0506_0708),
            ),
            (
                "E007",
                "0c000102030405060708090a0b0c0d0e0f".into(),
                0,
                SdpElement::Uint128(0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f),
            ),
            ("E008", "10ff".into(), 0, SdpElement::Int8(-1)),
            ("E010", "12ffffffff".into(), 0, SdpElement::Int32(-1)),
            (
                "E012",
                format!("14{}", "ff".repeat(16)),
                0,
                SdpElement::Int128(-1),
            ),
            ("E013", "1912ab".into(), 0, SdpElement::Uuid16(0x12ab)),
            (
                "E014",
                "1a01020304".into(),
                0,
                SdpElement::Uuid32(0x0102_0304),
            ),
            (
                "E015",
                "1c000102030405060708090a0b0c0d0e0f".into(),
                0,
                SdpElement::Uuid128(0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f),
            ),
            ("E016", "2801".into(), 0, SdpElement::Boolean(1)),
            ("E018", "2503616263".into(), 0, SdpElement::String(b"abc")),
            ("E021", "45026162".into(), 0, SdpElement::Url(b"ab")),
            (
                "E022",
                "350408010802".into(),
                0,
                SdpElement::Sequence(b"\x35\x04\x08\x01\x08\x02"),
            ),
            (
                "E025",
                "3d0100".into(),
                0,
                SdpElement::Alternative(b"\x3d\x01\x00"),
            ),
            (
                "E026",
                "3f0000".into(),
                ERROR_INVALID_PARAMETER,
                SdpElement::Nil,
            ),
        ];
        for (id, input, ret, expected) in &cases {
            let bytes = hex(input);
            let mut data = SENTINEL;
            assert_eq!(
                bluetooth_sdp_get_element_data(Some(&bytes), Some(&mut data)),
                *ret,
                "{id}"
            );
            assert_eq!(data, *expected, "{id}");
        }
    }

    /// Container vectors from the oracle `C` rows. Each row is (id, input, cursor, return, next cursor, element).
    #[test]
    fn container_vectors_match_oracle() {
        let cases: [ContainerRow; 12] = [
            (
                "C051",
                "3506080108020803",
                None,
                0,
                Some(4),
                SdpElement::Uint8(1),
            ),
            (
                "C052",
                "3506080108020803",
                Some(3),
                ERROR_INVALID_PARAMETER,
                Some(3),
                SdpElement::Nil,
            ),
            (
                "C059",
                "36000208010802",
                None,
                0,
                Some(5),
                SdpElement::Uint8(1),
            ),
            (
                "C053",
                "3506080108020803",
                Some(5),
                ERROR_INVALID_PARAMETER,
                Some(5),
                SdpElement::Nil,
            ),
            (
                "C054",
                "3506080108020803",
                Some(7),
                ERROR_INVALID_PARAMETER,
                Some(7),
                SdpElement::Nil,
            ),
            (
                "C055",
                "3506080108020803",
                Some(1),
                ERROR_INVALID_PARAMETER,
                Some(1),
                SdpElement::Nil,
            ),
            ("C061", "35010801", None, 0, Some(4), SdpElement::Uint8(1)),
            ("C062", "3503080108", None, 0, Some(4), SdpElement::Uint8(1)),
            (
                "C064",
                "35010801",
                Some(3),
                ERROR_INVALID_PARAMETER,
                Some(3),
                SdpElement::Nil,
            ),
            (
                "C065",
                "35010801",
                Some(2),
                0,
                Some(4),
                SdpElement::Uint8(1),
            ),
            (
                "C066",
                "35020809",
                Some(1),
                ERROR_INVALID_PARAMETER,
                Some(1),
                SdpElement::Nil,
            ),
            (
                "C067",
                "35020101",
                None,
                ERROR_INVALID_PARAMETER,
                None,
                SdpElement::Nil,
            ),
        ];
        for (id, input, cursor, ret, next, expected) in cases {
            let bytes = hex(input);
            let mut handle = cursor;
            let mut data = SENTINEL;
            let code = bluetooth_sdp_get_container_element_data(
                Some(&bytes),
                Some(&mut handle),
                Some(&mut data),
            );
            assert_eq!(code, ret, "{id}");
            assert_eq!(handle, next, "{id}");
            assert_eq!(data, expected, "{id}");
        }
    }

    /// Enumeration vectors from the oracle `N` rows: (id, input, entries seen, result).
    #[test]
    fn enum_vectors_match_oracle() {
        let cases: [EnumRow; 8] = [
            ("N074", "35050901000805", &[(0x0100, &[0x08, 0x05])], Ok(())),
            ("N075", "350408010805", &[], Err(ERROR_INVALID_DATA)),
            ("N076", "3503090100", &[], Err(ERROR_INVALID_DATA)),
            (
                "N077",
                "35080901000805090200",
                &[(0x0100, &[0x08, 0x05])],
                Err(ERROR_INVALID_DATA),
            ),
            ("N078", "090100", &[], Err(ERROR_INVALID_DATA)),
            ("N079", "", &[], Err(ERROR_INVALID_DATA)),
            (
                "N085",
                "3505090001080100",
                &[(0x0001, &[0x08, 0x01])],
                Ok(()),
            ),
            ("N086", "3600010801", &[], Err(ERROR_INVALID_DATA)),
        ];
        for (id, input, entries, result) in cases {
            let (actual, seen) = enum_entries(&hex(input));
            assert_eq!(actual, result, "{id}");
            let expected: Vec<(u16, Vec<u8>)> = entries
                .iter()
                .map(|(attr, value)| (*attr, value.to_vec()))
                .collect();
            assert_eq!(seen, expected, "{id}");
        }
    }

    /// Attribute vectors from the oracle `A` rows: (id, input, attribute, return, element).
    #[test]
    fn attribute_vectors_match_oracle() {
        let cases: [(&str, &str, u16, u32, SdpElement); 2] = [
            ("A069", RECORD, 0, 0, SdpElement::Uint32(0x0001_0002)),
            ("A072", RECORD, 0xffff, ERROR_FILE_NOT_FOUND, SENTINEL),
        ];
        for (id, input, attr, ret, expected) in cases {
            let bytes = hex(input);
            let mut data = SENTINEL;
            let code = bluetooth_sdp_get_attribute_value(Some(&bytes), attr, Some(&mut data));
            assert_eq!(code, ret, "{id}");
            assert_eq!(data, expected, "{id}");
        }
    }

    #[test]
    fn enum_lists_every_record_entry() {
        let (result, seen) = enum_entries(&hex(RECORD));
        assert_eq!(result, Ok(()));
        let expected: [(u16, &str); 4] = [
            (0x0000, "0a00010002"),
            (0x0001, "2503616263"),
            (0x0004, "350408010802"),
            (0x0004, "0809"),
        ];
        let expected: Vec<(u16, Vec<u8>)> = expected
            .iter()
            .map(|(attr, value)| (*attr, hex(value)))
            .collect();
        assert_eq!(seen, expected);
    }

    #[test]
    fn element_missing_arguments_leave_data_untouched() {
        let mut data = SENTINEL;
        assert_eq!(
            bluetooth_sdp_get_element_data(None, Some(&mut data)),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!(
            bluetooth_sdp_get_element_data(Some(&[]), Some(&mut data)),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!(
            bluetooth_sdp_get_element_data(Some(&[0x08]), None),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!(data, SENTINEL);
    }

    /// Every rejection in the oracle (`E029`..`E049`, plus `E041`'s 16-byte case) sets the element to `Nil`.
    #[test]
    fn element_rejections_set_nil() {
        let full_uint128 = format!("0c{}", "00".repeat(15));
        let inputs = [
            "01",
            "0d00",
            "0f00",
            "18",
            "1b0000000000000000",
            "1d00",
            "2400",
            "2900",
            "48",
            "f8",
            "0901",
            full_uint128.as_str(),
            "25056162",
            "35050801",
            "25",
            "2600",
            "270000",
            "28",
            "3c00",
            "03",
        ];
        for input in inputs {
            let bytes = hex(input);
            let mut data = SENTINEL;
            assert_eq!(
                bluetooth_sdp_get_element_data(Some(&bytes), Some(&mut data)),
                ERROR_INVALID_PARAMETER,
                "{input}"
            );
            assert_eq!(data, SdpElement::Nil, "{input}");
        }
    }

    /// Oracle rows for the remaining integer widths, boolean values, zero-length payloads and trailing bytes.
    #[test]
    fn element_payload_variants_match_oracle() {
        let cases: [(&str, &str, SdpElement); 8] = [
            ("E002", "00ff", SdpElement::Nil),
            ("E009", "11fffe", SdpElement::Int16(-2)),
            ("E011", "13ffffffffffffffff", SdpElement::Int64(-1)),
            ("E017", "2800", SdpElement::Boolean(0)),
            ("E019", "2600026162", SdpElement::String(b"ab")),
            ("E020", "27000000016162", SdpElement::String(b"a")),
            ("E027", "3500", SdpElement::Sequence(b"\x35\x00")),
            ("E028", "460000", SdpElement::Url(b"")),
        ];
        for (id, input, expected) in cases {
            let bytes = hex(input);
            let mut data = SENTINEL;
            assert_eq!(
                bluetooth_sdp_get_element_data(Some(&bytes), Some(&mut data)),
                0,
                "{id}"
            );
            assert_eq!(data, expected, "{id}");
        }
    }

    #[test]
    fn read_element_rejects_empty_input() {
        assert_eq!(read_element(&[]).unwrap_err(), ERROR_INVALID_PARAMETER);
    }

    #[test]
    fn container_missing_arguments_leave_state_untouched() {
        let bytes = hex("35010801");
        let mut handle = None;
        let mut data = SENTINEL;
        assert_eq!(
            bluetooth_sdp_get_container_element_data(None, Some(&mut handle), Some(&mut data)),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!(
            bluetooth_sdp_get_container_element_data(Some(&bytes), None, Some(&mut data)),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!(
            bluetooth_sdp_get_container_element_data(Some(&bytes), Some(&mut handle), None),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!((handle, data), (None, SENTINEL));
    }

    #[test]
    fn container_empty_stream_is_invalid_even_with_a_cursor() {
        let mut data = SENTINEL;
        let mut handle = None;
        assert_eq!(
            bluetooth_sdp_get_container_element_data(Some(&[]), Some(&mut handle), Some(&mut data)),
            ERROR_INVALID_PARAMETER
        );
        let mut handle = Some(0);
        assert_eq!(
            bluetooth_sdp_get_container_element_data(Some(&[]), Some(&mut handle), Some(&mut data)),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!((handle, data), (Some(0), SENTINEL));
    }

    /// Null-cursor header failures leave `data` and the handle as they were (`C056`, `C057`, `C058`).
    #[test]
    fn container_header_failures_leave_data_untouched() {
        for input in ["0801", "30010801", "35", "3600", "3f000001"] {
            let bytes = hex(input);
            let mut handle = None;
            let mut data = SENTINEL;
            assert_eq!(
                bluetooth_sdp_get_container_element_data(
                    Some(&bytes),
                    Some(&mut handle),
                    Some(&mut data)
                ),
                ERROR_INVALID_PARAMETER,
                "{input}"
            );
            assert_eq!((handle, data), (None, SENTINEL), "{input}");
        }
    }

    #[test]
    fn container_first_child_element_error_sets_nil() {
        let mut handle = None;
        let mut data = SENTINEL;
        let bytes = hex("35020901");
        assert_eq!(
            bluetooth_sdp_get_container_element_data(
                Some(&bytes),
                Some(&mut handle),
                Some(&mut data)
            ),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!((handle, data), (None, SdpElement::Nil));
    }

    #[test]
    fn container_empty_containers_have_no_children() {
        for input in ["3500", "3d00"] {
            let bytes = hex(input);
            let mut handle = None;
            let mut data = SENTINEL;
            assert_eq!(
                bluetooth_sdp_get_container_element_data(
                    Some(&bytes),
                    Some(&mut handle),
                    Some(&mut data)
                ),
                ERROR_NO_MORE_ITEMS,
                "{input}"
            );
            assert_eq!((handle, data), (None, SENTINEL), "{input}");
        }
    }

    /// Wine's cursor branch returns the bytes after an empty `35 00` header as its first child (`X04`). This port returns `ERROR_NO_MORE_ITEMS`.
    #[test]
    fn container_does_not_read_past_an_empty_header() {
        let bytes = hex("35000801");
        let mut handle = None;
        let mut data = SENTINEL;
        assert_eq!(
            bluetooth_sdp_get_container_element_data(
                Some(&bytes),
                Some(&mut handle),
                Some(&mut data)
            ),
            ERROR_NO_MORE_ITEMS
        );
        assert_eq!((handle, data), (None, SENTINEL));
    }

    #[test]
    fn container_cursor_bounds() {
        let bytes = hex("35010801");
        let mut data = SENTINEL;
        let mut handle = Some(9);
        assert_eq!(
            bluetooth_sdp_get_container_element_data(
                Some(&bytes),
                Some(&mut handle),
                Some(&mut data)
            ),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!((handle, data), (Some(9), SENTINEL));

        let mut handle = Some(4);
        assert_eq!(
            bluetooth_sdp_get_container_element_data(
                Some(&bytes),
                Some(&mut handle),
                Some(&mut data)
            ),
            ERROR_NO_MORE_ITEMS
        );
        assert_eq!((handle, data), (Some(4), SENTINEL));
    }

    /// Cursor element errors set `Nil` and keep the cursor (`C066`).
    #[test]
    fn container_cursor_element_error_sets_nil() {
        let bytes = hex("35020809");
        let mut handle = Some(1);
        let mut data = SENTINEL;
        assert_eq!(
            bluetooth_sdp_get_container_element_data(
                Some(&bytes),
                Some(&mut handle),
                Some(&mut data)
            ),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!((handle, data), (Some(1), SdpElement::Nil));
    }

    /// Advances a cursor through every child, from the first one to `ERROR_NO_MORE_ITEMS`.
    #[test]
    fn container_walks_children_then_reports_end() {
        let bytes = hex("3506080108020803");
        let mut handle = None;
        let mut values = Vec::new();
        loop {
            let mut data = SENTINEL;
            let code = bluetooth_sdp_get_container_element_data(
                Some(&bytes),
                Some(&mut handle),
                Some(&mut data),
            );
            if code != ERROR_SUCCESS {
                assert_eq!(code, ERROR_NO_MORE_ITEMS);
                break;
            }
            values.push(data);
        }
        assert_eq!(
            values,
            [
                SdpElement::Uint8(1),
                SdpElement::Uint8(2),
                SdpElement::Uint8(3)
            ]
        );
        assert_eq!(handle, Some(bytes.len()));
    }

    /// Oracle `X01`, `X02` and `X08`: alternative headers and a 4-byte size field. Cursor 6 is a bad element (`X08`).
    #[test]
    fn container_alternative_and_wide_size_fields() {
        let cases: [ContainerRow; 3] = [
            ("X01", "3d01082a", None, 0, Some(4), SdpElement::Uint8(0x2a)),
            (
                "X02",
                "3f00000001082a",
                None,
                0,
                Some(7),
                SdpElement::Uint8(0x2a),
            ),
            (
                "X08",
                "3f00000001082a",
                Some(6),
                ERROR_INVALID_PARAMETER,
                Some(6),
                SdpElement::Nil,
            ),
        ];
        for (id, input, cursor, ret, next, expected) in cases {
            let bytes = hex(input);
            let mut handle = cursor;
            let mut data = SENTINEL;
            assert_eq!(
                bluetooth_sdp_get_container_element_data(
                    Some(&bytes),
                    Some(&mut handle),
                    Some(&mut data)
                ),
                ret,
                "{id}"
            );
            assert_eq!(handle, next, "{id}");
            assert_eq!(data, expected, "{id}");
        }
    }

    #[test]
    fn enum_missing_arguments_are_invalid_parameter() {
        let bytes = hex("3500");
        let mut record = |_: u16, _: &[u8]| true;
        assert_eq!(
            bluetooth_sdp_enum_attributes(None, Some(&mut record)),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(
            bluetooth_sdp_enum_attributes(Some(&bytes), None),
            Err(ERROR_INVALID_PARAMETER)
        );
    }

    #[test]
    fn enum_rejects_non_containers_and_truncated_lists() {
        for input in ["", "090100", "350508"] {
            assert_eq!(
                enum_entries(&hex(input)).0,
                Err(ERROR_INVALID_DATA),
                "{input}"
            );
        }
    }

    #[test]
    fn enum_ids_must_be_uint16() {
        for input in ["350408010805", "3600010801"] {
            assert_eq!(
                enum_entries(&hex(input)).0,
                Err(ERROR_INVALID_DATA),
                "{input}"
            );
        }
    }

    /// `N077`: the first pair is reported before the second pair fails.
    #[test]
    fn enum_reports_entries_before_a_missing_value() {
        let (result, seen) = enum_entries(&hex("35080901000805090200"));
        assert_eq!(result, Err(ERROR_INVALID_DATA));
        assert_eq!(seen, vec![(0x0100, hex("0805"))]);
    }

    #[test]
    fn enum_value_truncated_inside_the_list_is_invalid_data() {
        let (result, seen) = enum_entries(&hex("35060901000c0000"));
        assert_eq!(result, Err(ERROR_INVALID_DATA));
        assert!(seen.is_empty());
    }

    #[test]
    fn enum_callback_false_stops_before_a_malformed_tail() {
        let bytes = hex("350609000008010c");
        let mut calls = 0;
        let mut stop = |_: u16, _: &[u8]| {
            calls += 1;
            false
        };
        assert_eq!(
            bluetooth_sdp_enum_attributes(Some(&bytes), Some(&mut stop)),
            Ok(())
        );
        assert_eq!(calls, 1);
    }

    /// Empty lists succeed with no callbacks (`X03`, `X05`: Wine reads past the header).
    #[test]
    fn enum_empty_lists_succeed() {
        for input in ["3500", "3d00"] {
            assert_eq!(enum_entries(&hex(input)), (Ok(()), Vec::new()), "{input}");
        }
    }

    /// Oracle `X06`: an alternative list is enumerated like a sequence.
    #[test]
    fn enum_alternative_lists_entries() {
        let (result, seen) = enum_entries(&hex("3d050900010805"));
        assert_eq!(result, Ok(()));
        assert_eq!(seen, vec![(0x0001, hex("0805"))]);
    }

    #[test]
    fn attribute_missing_arguments_are_invalid_parameter() {
        let mut data = SENTINEL;
        assert_eq!(
            bluetooth_sdp_get_attribute_value(None, 0, Some(&mut data)),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!(
            bluetooth_sdp_get_attribute_value(Some(&hex(RECORD)), 0, None),
            ERROR_INVALID_PARAMETER
        );
        assert_eq!(data, SENTINEL);
    }

    /// Enumeration failures map to `ERROR_INVALID_PARAMETER` (`A083`).
    #[test]
    fn attribute_enumeration_errors_are_invalid_parameter() {
        for input in ["", "090100"] {
            let bytes = hex(input);
            let mut data = SENTINEL;
            assert_eq!(
                bluetooth_sdp_get_attribute_value(Some(&bytes), 0, Some(&mut data)),
                ERROR_INVALID_PARAMETER,
                "{input}"
            );
            assert_eq!(data, SENTINEL, "{input}");
        }
    }

    /// Oracle `A070`, `A071`, `A080` and `A081`: values are decoded, and the first match wins (`A071` has two id 4 entries).
    #[test]
    fn attribute_values_and_first_match() {
        let cases: [(&str, &str, u16, SdpElement); 4] = [
            ("A070", RECORD, 1, SdpElement::String(b"abc")),
            (
                "A071",
                RECORD,
                4,
                SdpElement::Sequence(b"\x35\x04\x08\x01\x08\x02"),
            ),
            ("A080", "35050901000805", 256, SdpElement::Uint8(5)),
            ("A081", "350709010008050801", 256, SdpElement::Uint8(5)),
        ];
        for (id, input, attr, expected) in cases {
            let bytes = hex(input);
            let mut data = SENTINEL;
            assert_eq!(
                bluetooth_sdp_get_attribute_value(Some(&bytes), attr, Some(&mut data)),
                0,
                "{id}"
            );
            assert_eq!(data, expected, "{id}");
        }
    }

    /// Oracle `A082`, `A073`: an absent id reports `ERROR_FILE_NOT_FOUND`.
    #[test]
    fn attribute_absent_ids_are_not_found() {
        for (input, attr) in [("35050901000805", 512), (RECORD, 7)] {
            let bytes = hex(input);
            let mut data = SENTINEL;
            assert_eq!(
                bluetooth_sdp_get_attribute_value(Some(&bytes), attr, Some(&mut data)),
                ERROR_FILE_NOT_FOUND
            );
            assert_eq!(data, SENTINEL);
        }
    }

    /// The lookup stops at the first match, so a malformed entry after it is never read.
    #[test]
    fn attribute_lookup_stops_at_first_match() {
        let bytes = hex("350609000008010c");
        let mut data = SENTINEL;
        assert_eq!(
            bluetooth_sdp_get_attribute_value(Some(&bytes), 0, Some(&mut data)),
            0
        );
        assert_eq!(data, SdpElement::Uint8(1));
    }
}
