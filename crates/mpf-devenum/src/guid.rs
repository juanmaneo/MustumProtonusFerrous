//! GUID text from Wine's `combase`: `guid_from_string`, which `CLSIDFromString` uses for the
//! braced form, and `StringFromGUID2`.

/// `CHARS_IN_GUID`: a braced GUID with its terminating NUL.
pub const CHARS_IN_GUID: usize = 39;

/// Length of the braced GUID text, `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}`, without the NUL.
const GUID_TEXT_LEN: usize = CHARS_IN_GUID - 1;

/// A 128-bit GUID, laid out like the Windows `GUID` structure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// Port of `guid_from_string`. `text` must be exactly the braced GUID text. Upper and lower case
/// hex digits are both accepted.
pub fn parse_guid(text: &[u16]) -> Option<Guid> {
    if text.len() != GUID_TEXT_LEN || text[0] != u16::from(b'{') || text[37] != u16::from(b'}') {
        return None;
    }
    let data1 = hex_field(text, 1, 8)?;
    expect_dash(text, 9)?;
    let data2 = hex_field(text, 10, 4)? as u16;
    expect_dash(text, 14)?;
    let data3 = hex_field(text, 15, 4)? as u16;
    expect_dash(text, 19)?;
    let mut data4 = [0u8; 8];
    for (index, byte) in data4.iter_mut().enumerate() {
        if index == 2 {
            expect_dash(text, 24)?;
        }
        let start = if index < 2 {
            20 + 2 * index
        } else {
            21 + 2 * index
        };
        *byte = hex_field(text, start, 2)? as u8;
    }
    Some(Guid {
        data1,
        data2,
        data3,
        data4,
    })
}

/// Parses the GUID at the start of `name` the way `lstrcpynW(buffer, name, CHARS_IN_GUID)`
/// followed by `CLSIDFromString` does: at most 38 units, cut at the first NUL.
pub fn guid_prefix(name: &[u16]) -> Option<Guid> {
    let window = name.get(..GUID_TEXT_LEN).unwrap_or(name);
    let end = window
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(window.len());
    parse_guid(&window[..end])
}

/// Port of `StringFromGUID2`: upper-case braced text, as UTF-16 without the NUL.
pub fn format_guid(guid: &Guid) -> Vec<u16> {
    let tail: String = guid.data4[2..]
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect();
    let text = format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{tail}}}",
        guid.data1, guid.data2, guid.data3, guid.data4[0], guid.data4[1]
    );
    text.encode_utf16().collect()
}

fn hex_digit(unit: u16) -> Option<u8> {
    match unit {
        0x30..=0x39 => Some((unit - 0x30) as u8),
        0x41..=0x46 => Some((unit - 0x41 + 10) as u8),
        0x61..=0x66 => Some((unit - 0x61 + 10) as u8),
        _ => None,
    }
}

fn hex_field(text: &[u16], start: usize, digits: usize) -> Option<u32> {
    text[start..start + digits]
        .iter()
        .try_fold(0u32, |value, &unit| {
            Some((value << 4) | u32::from(hex_digit(unit)?))
        })
}

fn expect_dash(text: &[u16], index: usize) -> Option<()> {
    (text[index] == u16::from(b'-')).then_some(())
}
