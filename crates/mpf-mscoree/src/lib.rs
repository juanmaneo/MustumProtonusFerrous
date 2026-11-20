//! Pure runtime-version parsing from Wine's `dlls/mscoree/metahost.c`.
//!
//! Wine's `parse_runtime_version` reads a NUL-terminated UTF-16 string with a
//! lowercase or uppercase `v` prefix and one to three unsigned decimal
//! components separated by `.`. This port returns `None` where Wine returns
//! `FALSE`. A string without a NUL terminator is rejected instead of being
//! read past its end. Component accumulation wraps at 32 bits, as Wine's
//! `DWORD` arithmetic does.
//!
//! The rest of `metahost.c` (runtime lookup, install roots, Mono loading and
//! the COM objects) depends on the registry, the file system or process state,
//! so it is not ported.

/// The major, minor and build components of a runtime version. Omitted minor
/// and build components are zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeVersion {
    /// The major component.
    pub major: u32,
    /// The minor component.
    pub minor: u32,
    /// The build component.
    pub build: u32,
}

fn is_digit(character: u16) -> bool {
    (u16::from(b'0')..=u16::from(b'9')).contains(&character)
}

/// Parse a runtime version the way Wine's `parse_runtime_version` does.
///
/// Accepted forms are `v<major>`, `v<major>.<minor>` and
/// `v<major>.<minor>.<build>`, with either `v` or `V`. Each component is one or
/// more ASCII decimal digits. The string ends at the first NUL, and any
/// characters after it are ignored, as they are in Wine's C-string parser.
pub fn parse_runtime_version(version: &[u16]) -> Option<RuntimeVersion> {
    let end = version.iter().position(|&character| character == 0)?;
    let input = &version[..end];
    let prefix = input.first().copied();
    if prefix != Some(u16::from(b'v')) && prefix != Some(u16::from(b'V')) {
        return None;
    }

    let mut index = 1;
    let major = parse_component(input, &mut index)?;
    if index == input.len() {
        return Some(RuntimeVersion {
            major,
            minor: 0,
            build: 0,
        });
    }
    expect_separator(input, &mut index)?;

    let minor = parse_component(input, &mut index)?;
    if index == input.len() {
        return Some(RuntimeVersion {
            major,
            minor,
            build: 0,
        });
    }
    expect_separator(input, &mut index)?;

    let build = parse_component(input, &mut index)?;
    if index == input.len() {
        Some(RuntimeVersion {
            major,
            minor,
            build,
        })
    } else {
        None
    }
}

fn expect_separator(input: &[u16], index: &mut usize) -> Option<()> {
    if input.get(*index) != Some(&u16::from(b'.')) {
        return None;
    }
    *index += 1;
    Some(())
}

fn parse_component(input: &[u16], index: &mut usize) -> Option<u32> {
    if *index == input.len() || !is_digit(input[*index]) {
        return None;
    }

    let mut value = 0_u32;
    while *index < input.len() && is_digit(input[*index]) {
        value = value
            .wrapping_mul(10)
            .wrapping_add(u32::from(input[*index] - u16::from(b'0')));
        *index += 1;
    }
    Some(value)
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
