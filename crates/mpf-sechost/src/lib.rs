//! Deterministic SID string conversion logic from Wine's `dlls/sechost/security.c`.
//!
//! This crate ports the numeric and two-character well-known SID cases from
//! `parse_sid`. It does not allocate Windows memory or call Win32 APIs.

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

/// Parses a numeric or supported two-character SDDL SID string.
///
/// Numeric SIDs use the `S-revision-authority-subauthority...` spelling. The
/// parser accepts decimal tokens and `0x` hexadecimal tokens, matching Wine's
/// token conversion, and rejects values outside the SID structure's 15-RID
/// limit. Supported aliases include common principals, builtin groups, and
/// mandatory integrity labels.
pub fn parse_string_sid(input: &str) -> Option<Sid> {
    if input.len() == 2 {
        if let Some(sid) = well_known_sid(input) {
            return Some(sid);
        }
        if let Some(rid) = well_known_rid(input) {
            return Some(Sid {
                revision: 1,
                identifier_authority: 5,
                sub_authorities: vec![21, 0, 0, 0, rid],
            });
        }
    }

    let bytes = input.as_bytes();
    if bytes.len() < 2 || !matches!(bytes[0], b'S' | b's') || bytes[1] != b'-' || !input.is_ascii()
    {
        return None;
    }

    let mut cursor = 2;
    let mut tokens = Vec::new();
    while let Some((value, end)) = parse_token(bytes, cursor) {
        tokens.push(value);
        cursor = end;
    }

    if cursor != bytes.len() || tokens.len() < 3 || tokens.len() > 17 {
        return None;
    }
    let revision = u8::try_from(tokens[0]).ok()?;
    if revision != 1 {
        return None;
    }
    let sub_authorities = &tokens[2..];
    Some(Sid {
        revision,
        identifier_authority: u64::from(tokens[1]),
        sub_authorities: sub_authorities.to_vec(),
    })
}

fn parse_token(bytes: &[u8], start: usize) -> Option<(u32, usize)> {
    let first = *bytes.get(start)?;
    let (mut cursor, radix) = if first == b'0' && matches!(bytes.get(start + 1), Some(b'X' | b'x'))
    {
        (start + 2, 16)
    } else if first.is_ascii_digit() || first == b'-' {
        (start, 10)
    } else {
        return None;
    };

    let negative = bytes.get(cursor) == Some(&b'-');
    if negative || bytes.get(cursor) == Some(&b'+') {
        cursor += 1;
    }
    let digits_start = cursor;
    let mut value = 0u64;
    while let Some(digit) = bytes.get(cursor).and_then(|byte| digit_value(*byte)) {
        if digit >= radix {
            break;
        }
        value = value
            .checked_mul(radix)
            .and_then(|current| current.checked_add(digit))
            .unwrap_or(u64::MAX);
        cursor += 1;
    }
    if cursor == digits_start {
        return None;
    }

    if bytes.get(cursor) == Some(&b'-') {
        cursor += 1;
    }
    let value = if negative {
        0u64.wrapping_sub(value)
    } else {
        value
    };
    Some((value as u32, cursor))
}

fn digit_value(byte: u8) -> Option<u64> {
    match byte {
        b'0'..=b'9' => Some(u64::from(byte - b'0')),
        b'a'..=b'f' => Some(u64::from(byte - b'a' + 10)),
        b'A'..=b'F' => Some(u64::from(byte - b'A' + 10)),
        _ => None,
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
