//! Deterministic SID string conversion logic from Wine's `dlls/sechost/security.c`.
//!
//! This crate ports `parse_token`, `get_sid_size` and `parse_sid` for SDDL SID
//! strings. It does not allocate Windows memory or call Win32 APIs.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A parsed Windows security identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sid {
    /// SID revision, normally 1.
    pub revision: u8,
    /// The 48-bit identifier authority.
    pub identifier_authority: u64,
    /// Relative identifiers, in their original order.
    pub sub_authorities: Vec<u32>,
}

impl Sid {
    /// Encodes the SID in the byte layout used by Windows SID structures.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.sub_authorities.len() * 4);
        bytes.push(self.revision);
        bytes.push(self.sub_authorities.len() as u8);
        let authority = self.identifier_authority.to_be_bytes();
        bytes.extend_from_slice(&authority[2..]);
        for sub_authority in &self.sub_authorities {
            bytes.extend_from_slice(&sub_authority.to_le_bytes());
        }
        bytes
    }
}

/// Parses an SDDL SID string: `S-revision-authority-subauthority...` or a
/// two-character alias.
///
/// Leading spaces are skipped as in Wine's `parse_sid`. Numeric tokens follow
/// Wine's `parse_token`, which converts them with the C runtime `wcstoul`:
/// decimal, or hexadecimal with an optional `0x` prefix, a leading sign, and
/// saturation to `u32::MAX` on overflow (never negated). Each token may be
/// followed by one `-`. The revision is truncated to a byte before it is
/// checked against 1. Strings with more than 15 subauthorities are rejected,
/// because Wine's `parse_sid` writes past the fixed `SubAuthority[15]` array
/// for them.
pub fn parse_string_sid(input: &str) -> Option<Sid> {
    let input = input.trim_start_matches(' ');
    if !input.is_ascii() {
        return None;
    }
    let bytes = input.as_bytes();
    if bytes.len() < 2 {
        return None;
    }
    if matches!(bytes[0], b'S' | b's') && bytes[1] == b'-' {
        return parse_numeric(bytes);
    }
    if bytes.len() != 2 {
        return None;
    }
    if let Some(sid) = well_known_sid(input) {
        return Some(sid);
    }
    well_known_rid(input).map(domain_relative_sid)
}

fn parse_numeric(bytes: &[u8]) -> Option<Sid> {
    let mut cursor = 2;
    let mut tokens = Vec::new();
    while let Some((value, end)) = parse_token(bytes, cursor) {
        tokens.push(value);
        cursor = end;
    }
    if cursor != bytes.len() || tokens.len() < 3 || tokens.len() > 17 {
        return None;
    }
    let revision = tokens[0] as u8;
    if revision != 1 {
        return None;
    }
    Some(Sid {
        revision,
        identifier_authority: u64::from(tokens[1]),
        sub_authorities: tokens[2..].to_vec(),
    })
}

fn parse_token(bytes: &[u8], start: usize) -> Option<(u32, usize)> {
    let first = byte_at(bytes, start);
    let (value, end) = if first == b'0' && matches!(byte_at(bytes, start + 1), b'X' | b'x') {
        wcstoul(bytes, start + 2, 16)
    } else if first.is_ascii_digit() || first == b'-' {
        wcstoul(bytes, start, 10)
    } else {
        return None;
    };
    let end = if byte_at(bytes, end) == b'-' {
        end + 1
    } else {
        end
    };
    Some((value, end))
}

/// Converts the ASCII digits at `start` the way glibc's `wcstoul` does, which
/// is what Wine's oracle build uses. Returns the value and the index where
/// conversion stopped.
fn wcstoul(bytes: &[u8], start: usize, radix: u32) -> (u32, usize) {
    let mut cursor = start;
    while matches!(
        byte_at(bytes, cursor),
        b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r'
    ) {
        cursor += 1;
    }
    let negative = byte_at(bytes, cursor) == b'-';
    if matches!(byte_at(bytes, cursor), b'+' | b'-') {
        cursor += 1;
    }
    let prefixed = radix == 16
        && byte_at(bytes, cursor) == b'0'
        && byte_at(bytes, cursor + 1).eq_ignore_ascii_case(&b'x');
    if prefixed {
        cursor += 2;
    }

    let digits_start = cursor;
    let mut value = 0u64;
    loop {
        match digit_value(byte_at(bytes, cursor)) {
            Some(digit) if digit < radix => {
                value = value
                    .saturating_mul(u64::from(radix))
                    .saturating_add(u64::from(digit));
                cursor += 1;
            }
            _ => break,
        }
    }

    if cursor == digits_start {
        // Without digits glibc consumes only the "0" of a "0x" prefix; otherwise
        // nothing is consumed.
        return (0, if prefixed { cursor - 1 } else { start });
    }
    if value > u64::from(u32::MAX) {
        return (u32::MAX, cursor);
    }
    let value = value as u32;
    (
        if negative {
            value.wrapping_neg()
        } else {
            value
        },
        cursor,
    )
}

fn byte_at(bytes: &[u8], index: usize) -> u8 {
    bytes.get(index).copied().unwrap_or(0)
}

fn digit_value(byte: u8) -> Option<u32> {
    match byte {
        b'0'..=b'9' => Some(u32::from(byte - b'0')),
        b'a'..=b'f' => Some(u32::from(byte - b'a' + 10)),
        b'A'..=b'F' => Some(u32::from(byte - b'A' + 10)),
        _ => None,
    }
}

fn domain_relative_sid(rid: u32) -> Sid {
    Sid {
        revision: 1,
        identifier_authority: 5,
        sub_authorities: vec![21, 0, 0, 0, rid],
    }
}

fn well_known_sid(alias: &str) -> Option<Sid> {
    let (authority, sub_authorities) = if alias.eq_ignore_ascii_case("WD") {
        (1, vec![0])
    } else if alias.eq_ignore_ascii_case("CO") {
        (3, vec![0])
    } else if alias.eq_ignore_ascii_case("CG") {
        (3, vec![1])
    } else if alias.eq_ignore_ascii_case("OW") {
        (3, vec![4])
    } else if alias.eq_ignore_ascii_case("NU") {
        (5, vec![2])
    } else if alias.eq_ignore_ascii_case("IU") {
        (5, vec![4])
    } else if alias.eq_ignore_ascii_case("SU") {
        (5, vec![6])
    } else if alias.eq_ignore_ascii_case("AN") {
        (5, vec![7])
    } else if alias.eq_ignore_ascii_case("ED") {
        (5, vec![9])
    } else if alias.eq_ignore_ascii_case("PS") {
        (5, vec![10])
    } else if alias.eq_ignore_ascii_case("AU") {
        (5, vec![11])
    } else if alias.eq_ignore_ascii_case("RC") {
        (5, vec![12])
    } else if alias.eq_ignore_ascii_case("SY") {
        (5, vec![18])
    } else if alias.eq_ignore_ascii_case("LS") {
        (5, vec![19])
    } else if alias.eq_ignore_ascii_case("NS") {
        (5, vec![20])
    } else if alias.eq_ignore_ascii_case("BA") {
        (5, vec![32, 544])
    } else if alias.eq_ignore_ascii_case("BU") {
        (5, vec![32, 545])
    } else if alias.eq_ignore_ascii_case("BG") {
        (5, vec![32, 546])
    } else if alias.eq_ignore_ascii_case("PU") {
        (5, vec![32, 547])
    } else if alias.eq_ignore_ascii_case("AO") {
        (5, vec![32, 548])
    } else if alias.eq_ignore_ascii_case("SO") {
        (5, vec![32, 549])
    } else if alias.eq_ignore_ascii_case("PO") {
        (5, vec![32, 550])
    } else if alias.eq_ignore_ascii_case("BO") {
        (5, vec![32, 551])
    } else if alias.eq_ignore_ascii_case("RE") {
        (5, vec![32, 552])
    } else if alias.eq_ignore_ascii_case("RU") {
        (5, vec![32, 554])
    } else if alias.eq_ignore_ascii_case("RD") {
        (5, vec![32, 555])
    } else if alias.eq_ignore_ascii_case("NO") {
        (5, vec![32, 556])
    } else if alias.eq_ignore_ascii_case("LW") {
        (16, vec![4096])
    } else if alias.eq_ignore_ascii_case("ME") {
        (16, vec![8192])
    } else if alias.eq_ignore_ascii_case("HI") {
        (16, vec![12288])
    } else if alias.eq_ignore_ascii_case("SI") {
        (16, vec![16384])
    } else if alias.eq_ignore_ascii_case("AC") {
        (15, vec![2, 1])
    } else {
        return None;
    };
    Some(Sid {
        revision: 1,
        identifier_authority: authority,
        sub_authorities,
    })
}

fn well_known_rid(alias: &str) -> Option<u32> {
    if alias.eq_ignore_ascii_case("LA") {
        Some(500)
    } else if alias.eq_ignore_ascii_case("LG") {
        Some(501)
    } else if alias.eq_ignore_ascii_case("DA") {
        Some(512)
    } else if alias.eq_ignore_ascii_case("DU") {
        Some(513)
    } else if alias.eq_ignore_ascii_case("DG") {
        Some(514)
    } else if alias.eq_ignore_ascii_case("DC") {
        Some(515)
    } else if alias.eq_ignore_ascii_case("DD") {
        Some(516)
    } else if alias.eq_ignore_ascii_case("CA") {
        Some(517)
    } else if alias.eq_ignore_ascii_case("SA") {
        Some(518)
    } else if alias.eq_ignore_ascii_case("EA") {
        Some(519)
    } else if alias.eq_ignore_ascii_case("PA") {
        Some(520)
    } else if alias.eq_ignore_ascii_case("RS") {
        Some(553)
    } else {
        None
    }
}
