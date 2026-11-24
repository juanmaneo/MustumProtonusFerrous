use crate::{c_str, HResult, E_FAIL, STG_E_READFAULT, S_OK};

const SAVE_MAGIC: u32 = 2;
const MONIKER_PRESENT: u32 = 0x01;
const MONIKER_IS_ABSOLUTE: u32 = 0x02;
const LOCATION_PRESENT: u32 = 0x08;
const FRIENDLY_FIXME: u32 = 0x04;
const FRIENDLY_PRESENT: u32 = 0x10;
const TARGET_FRAME_PRESENT: u32 = 0x80;
const SAVE_ALL: u32 = MONIKER_PRESENT
    | MONIKER_IS_ABSOLUTE
    | LOCATION_PRESENT
    | FRIENDLY_FIXME
    | FRIENDLY_PRESENT
    | TARGET_FRAME_PRESENT;

const DWORD_SIZE: usize = 4;
const WCHAR_SIZE: usize = 2;
const HEADER_SIZE: u64 = 8;

/// Builds the flags DWORD that `IPersistStream::Save` writes after the magic. Wine sets
/// `FRIENDLY_FIXME` together with the friendly-name bit.
pub fn save_flags(
    moniker: bool,
    absolute: bool,
    location: bool,
    friendly: bool,
    target_frame: bool,
) -> u32 {
    let mut flags = 0;
    if moniker {
        flags |= MONIKER_PRESENT;
    }
    if absolute {
        flags |= MONIKER_IS_ABSOLUTE;
    }
    if location {
        flags |= LOCATION_PRESENT;
    }
    if friendly {
        flags |= FRIENDLY_PRESENT | FRIENDLY_FIXME;
    }
    if target_frame {
        flags |= TARGET_FRAME_PRESENT;
    }
    flags
}

/// The bits that the save format does not define. Wine's `parse_header` reports them.
pub fn unknown_flags(flags: u32) -> u32 {
    flags & !SAVE_ALL
}

/// Reads a little-endian DWORD and returns it with the bytes after it.
fn field(bytes: &[u8]) -> Option<(u32, &[u8])> {
    bytes
        .split_first_chunk::<DWORD_SIZE>()
        .map(|(word, rest)| (u32::from_le_bytes(*word), rest))
}

/// Reads the save header: the magic DWORD, then the flags DWORD.
pub fn parse_header(bytes: &[u8]) -> Result<u32, HResult> {
    match field(bytes) {
        Some((SAVE_MAGIC, rest)) => field(rest).map(|(flags, _)| flags).ok_or(E_FAIL),
        _ => Err(E_FAIL),
    }
}

/// Encodes a string as `IPersistStream` writes it: a DWORD count that includes the NUL,
/// then the units, then the NUL.
pub fn encode_string(text: &[u16]) -> Vec<u8> {
    let units = c_str(text);
    let count = u32::try_from(units.len() + 1).expect("hlink string longer than u32::MAX units");
    let mut out = count.to_le_bytes().to_vec();
    out.extend(units.iter().flat_map(|unit| unit.to_le_bytes()));
    out.extend_from_slice(&[0, 0]);
    out
}

/// Decodes a string record. Returns the units, NUL included, and the number of bytes
/// the record used. Bytes after the record are ignored.
pub fn decode_string(bytes: &[u8]) -> Result<(Vec<u16>, usize), HResult> {
    let Some((count, body)) = field(bytes) else {
        return Err(STG_E_READFAULT);
    };
    let byte_len = u64::from(count) * WCHAR_SIZE as u64;
    if (body.len() as u64) < byte_len {
        return Err(STG_E_READFAULT);
    }
    let byte_len = byte_len as usize;
    let (pairs, _) = body[..byte_len].as_chunks::<WCHAR_SIZE>();
    let units = pairs.iter().map(|pair| u16::from_le_bytes(*pair)).collect();
    Ok((units, DWORD_SIZE + byte_len))
}

/// The encoded length of a string: its count DWORD and its units, NUL included.
pub fn string_size(text: &[u16]) -> u64 {
    (DWORD_SIZE + (c_str(text).len() + 1) * WCHAR_SIZE) as u64
}

/// `GetSizeMax`: the total size of the save stream, and the result. The moniker counts
/// only when it is present, and then the result is `S_OK`. Without it the result is
/// `E_FAIL`, but the size is still reported.
pub fn size_max(
    target_frame: Option<&[u16]>,
    friendly: Option<&[u16]>,
    location: Option<&[u16]>,
    moniker: Option<u64>,
) -> (HResult, u64) {
    let strings: u64 = [target_frame, friendly, location]
        .into_iter()
        .flatten()
        .map(string_size)
        .fold(0, u64::wrapping_add);
    let result = if moniker.is_some() { S_OK } else { E_FAIL };
    let size = HEADER_SIZE
        .wrapping_add(strings)
        .wrapping_add(moniker.unwrap_or(0));
    (result, size)
}
