//! `GUID` values and `GUIDFromString` from `dlls/mfplat/main.c`.

/// A COM `GUID`, laid out as in the Windows header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// The value of an ASCII hex digit, in either case, as `is_valid_hex` and `guid_conv_table` give it.
fn hex_digit(c: u16) -> Option<u32> {
    match c {
        0x30..=0x39 => Some(u32::from(c - 0x30)),
        0x41..=0x46 => Some(u32::from(c - 0x41 + 10)),
        0x61..=0x66 => Some(u32::from(c - 0x61 + 10)),
        _ => None,
    }
}

/// `GUIDFromString`: parses `XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX`, with hex digits in either case.
///
/// The C reads its input as a NUL-terminated string. A position past the end of `s` reads as NUL,
/// so a slice of exactly 36 units is complete, and anything after position 36 is ignored.
pub fn parse_guid(s: &[u16]) -> Option<Guid> {
    let at = |i: usize| s.get(i).copied().unwrap_or(0);
    let hex = |start: usize, end: usize| {
        (start..end).try_fold(0u32, |value, i| Some((value << 4) | hex_digit(at(i))?))
    };

    let data1 = hex(0, 8)?;
    let data2 = hex(9, 13)?;
    let data3 = hex(14, 18)?;
    let mut data4 = [0u8; 8];
    for (byte, start) in data4.iter_mut().zip([19, 21, 24, 26, 28, 30, 32, 34]) {
        *byte = hex(start, start + 2)? as u8;
    }

    if [8, 13, 18, 23].iter().any(|&i| at(i) != u16::from(b'-')) || at(36) != 0 {
        return None;
    }
    Some(Guid {
        data1,
        data2: data2 as u16,
        data3: data3 as u16,
        data4,
    })
}
