//! `ConvertGuidToStringA` and `ConvertGuidToStringW`.

use crate::Error;

/// `CHARS_IN_GUID`: the length of `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` with its NUL.
const CHARS_IN_GUID: usize = 39;

/// A GUID, as the `GUID` structure lays it out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// Formats `guid` as `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` in upper-case hex.
///
/// `len` is the buffer length in characters, including the NUL.
pub fn guid_to_string(guid: &Guid, len: usize) -> Result<String, Error> {
    if len < CHARS_IN_GUID {
        return Err(Error::InsufficientBuffer);
    }
    let d = &guid.data4;
    Ok(format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    ))
}
