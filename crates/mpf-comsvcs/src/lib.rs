//! Headless subset of Wine's `dlls/comsvcs` (COM+ Services).
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - [`guid_from_string`]: `guid_from_string` from `main.c`, the GUID text parser.
//! - [`new_moniker`]: the `new:` split of `new_moniker_parse_displayname` from `main.c`.
//!
//! Skipped: the dispenser manager, holders and resource pool (threads, events, critical
//! sections and driver calls), the `IMoniker` and `IROTData` methods (COM and streams), the
//! class factories and `group_manager` in `property.c` (COM objects and stubs), and
//! `CLSIDFromProgID`, which reads the registry. The port takes that lookup as a closure.

pub mod new_moniker;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A Windows `GUID`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// Parses GUID text the way `guid_from_string` does. Hex digits may be in either case.
///
/// Units past the end of the slice read as NUL, as they do past the terminator of the C
/// string, so a NUL fails the parse wherever it would be read. Text after a complete GUID is
/// ignored. Only a leading `{` requires a closing `}`.
pub fn guid_from_string(s: &[u16]) -> Option<Guid> {
    let braced = char_at(s, 0) == u16::from(b'{');
    let p = usize::from(braced);
    let data1 = hex_run(s, p, 8)? as u32;
    dash_at(s, p + 8)?;
    let data2 = hex_run(s, p + 9, 4)? as u16;
    dash_at(s, p + 13)?;
    let data3 = hex_run(s, p + 14, 4)? as u16;
    dash_at(s, p + 18)?;
    let mut data4 = [0u8; 8];
    for (k, byte) in data4.iter_mut().enumerate() {
        if k == 2 {
            dash_at(s, p + 23)?;
        }
        *byte = hex_run(s, p + 19 + 2 * k + usize::from(k >= 2), 2)? as u8;
    }
    if braced && char_at(s, p + 36) != u16::from(b'}') {
        return None;
    }
    Some(Guid {
        data1,
        data2,
        data3,
        data4,
    })
}

fn char_at(s: &[u16], index: usize) -> u16 {
    s.get(index).copied().unwrap_or(0)
}

/// `count` hex digits starting at `start`, most significant first.
fn hex_run(s: &[u16], start: usize, count: usize) -> Option<u64> {
    (start..start + count).try_fold(0, |acc, i| {
        let digit = char::from_u32(u32::from(char_at(s, i)))?.to_digit(16)?;
        Some((acc << 4) | u64::from(digit))
    })
}

fn dash_at(s: &[u16], index: usize) -> Option<()> {
    (char_at(s, index) == u16::from(b'-')).then_some(())
}
