//! Display-name parsing for `CreateAssemblyNameObject`, from `dlls/fusion/asmname.c`.
//!
//! The C copies the wide string, edits the copy in place and frees the partial result on failure.
//! The port reads the same string, cut at its first NUL, splits it into slices, and returns the
//! fields only on success. Key and architecture comparisons fold ASCII letters only. Windows
//! `lstrcmpiW` folds more than that, but every name in this format is ASCII.

use crate::token::BYTES_PER_TOKEN;
use crate::{Hresult, E_INVALIDARG, FUSION_E_INVALID_NAME, FUSION_E_PRIVATE_ASM_DISALLOWED};

pub const CANOF_PARSE_DISPLAY_NAME: u32 = 0x1;

const CHARS_PER_PUBKEY: usize = BYTES_PER_TOKEN * 2;

/// `wcstol` stops growing its value here. That is past the 32-bit range it is clamped to.
const OVERFLOW_CAP: i64 = 1 << 40;

const DOT: u16 = b'.' as u16;
const COMMA: u16 = b',' as u16;
const EQUALS: u16 = b'=' as u16;
const QUOTE: u16 = b'"' as u16;
const SPACE: u16 = b' ' as u16;
const MINUS: u16 = b'-' as u16;
const PLUS: u16 = b'+' as u16;
const ZERO: u16 = b'0' as u16;
const NINE: u16 = b'9' as u16;

/// `PEKIND` from `fusion.idl`, the value of `processorArchitecture`.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PeKind {
    #[default]
    NoKind = 0x0000_0000,
    Msil = 0x0000_0001,
    I386 = 0x0000_0002,
    Ia64 = 0x0000_0003,
    Amd64 = 0x0000_0004,
}

/// The fields of `IAssemblyNameImpl`. `None` marks a field the C leaves NULL.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AssemblyName {
    pub display_name: Option<Vec<u16>>,
    pub name: Option<Vec<u16>>,
    pub culture: Option<Vec<u16>>,
    pub procarch: Option<Vec<u16>>,
    pub version: [u16; 4],
    pub versize: u32,
    pub pubkey: Option<[u8; BYTES_PER_TOKEN]>,
    pub pekind: PeKind,
}

/// `CreateAssemblyNameObject` without the object. With `CANOF_PARSE_DISPLAY_NAME`, a NULL or empty
/// name is `E_INVALIDARG`. Otherwise the name is parsed, and a NULL name gives an empty result.
pub fn create_assembly_name(
    display_name: Option<&[u16]>,
    flags: u32,
) -> Result<AssemblyName, Hresult> {
    if (flags & CANOF_PARSE_DISPLAY_NAME) != 0
        && display_name.is_none_or(|wide| c_str(wide).is_empty())
    {
        return Err(E_INVALIDARG);
    }
    parse_display_name(display_name)
}

fn parse_display_name(display_name: Option<&[u16]>) -> Result<AssemblyName, Hresult> {
    let mut name = AssemblyName::default();
    let Some(wide) = display_name else {
        return Ok(name);
    };
    let text = c_str(wide);
    name.display_name = Some(text.to_vec());
    parse_fields(&mut name, text)?;
    Ok(name)
}

/// The loop of `parse_display_name`: the name before the first comma, then `key=value` pairs
/// separated by commas. An unknown key still needs a well-formed value, and is then ignored.
fn parse_fields(name: &mut AssemblyName, text: &[u16]) -> Result<(), Hresult> {
    let Some(comma) = text.iter().position(|&c| c == COMMA) else {
        if text.contains(&SPACE) {
            return Err(FUSION_E_INVALID_NAME);
        }
        name.name = Some(text.to_vec());
        return Ok(());
    };
    name.name = Some(text[..comma].to_vec());

    let mut rest = &text[comma + 1..];
    loop {
        let eq = rest
            .iter()
            .position(|&c| c == EQUALS)
            .ok_or(FUSION_E_INVALID_NAME)?;
        let key = &rest[..eq];
        let after = &rest[eq + 1..];
        if after.is_empty() {
            return Err(FUSION_E_INVALID_NAME);
        }
        let (raw_value, next) = match after.iter().position(|&c| c == COMMA) {
            Some(end) => (&after[..end], Some(&after[end + 1..])),
            None => (after, None),
        };
        let value = parse_value(raw_value).ok_or(FUSION_E_INVALID_NAME)?;

        let key = skip_leading(key, |c| c == SPACE);
        if eq_ignore_case(key, "Version") {
            parse_version(name, &value);
        } else if eq_ignore_case(key, "Culture") {
            name.culture = Some(parse_culture(&value));
        } else if eq_ignore_case(key, "PublicKeyToken") {
            name.pubkey = Some(parse_pubkey(&value)?);
        } else if eq_ignore_case(key, "processorArchitecture") {
            name.pekind = parse_procarch(&value)?;
            name.procarch = Some(value);
        }

        match next {
            Some(next) => rest = next,
            None => return Ok(()),
        }
    }
}

/// `parse_value`: a value, optionally in double quotes that must come as a pair. Text after the
/// closing quote is ignored. An unquoted value cannot contain a quote.
fn parse_value(raw: &[u16]) -> Option<Vec<u16>> {
    let (quoted, body) = match raw.split_first() {
        Some((&QUOTE, rest)) => (true, rest),
        _ => (false, raw),
    };
    let end = body.iter().position(|&c| c == QUOTE).unwrap_or(body.len());
    let closed = end < body.len();
    if closed == quoted {
        Some(body[..end].to_vec())
    } else {
        None
    }
}

/// `parse_version`: at most four dot-separated parts, each read with `wcstol`. The part count
/// adds up across repeated `Version` keys, as `versize` does in the C.
fn parse_version(name: &mut AssemblyName, version: &[u16]) {
    let mut rest = version;
    for slot in &mut name.version {
        if rest.is_empty() {
            return;
        }
        let (part, next) = match rest.iter().position(|&c| c == DOT) {
            Some(dot) => (&rest[..dot], Some(&rest[dot + 1..])),
            None => (rest, None),
        };
        *slot = wcstol10(part) as u16;
        name.versize += 1;
        match next {
            Some(next) => rest = next,
            None => return,
        }
    }
}

/// `parse_culture`: a two-character culture is kept, and any other length becomes empty.
fn parse_culture(value: &[u16]) -> Vec<u16> {
    if value.len() == 2 {
        value.to_vec()
    } else {
        Vec::new()
    }
}

/// `parse_pubkey`: `null` is disallowed. Otherwise the first 16 characters must be hex digits,
/// read as 8 bytes. Characters after them are ignored.
fn parse_pubkey(value: &[u16]) -> Result<[u8; BYTES_PER_TOKEN], Hresult> {
    if eq_ignore_case(value, "null") {
        return Err(FUSION_E_PRIVATE_ASM_DISALLOWED);
    }
    let chars = value.get(..CHARS_PER_PUBKEY).ok_or(FUSION_E_INVALID_NAME)?;
    let mut token = [0; BYTES_PER_TOKEN];
    let (pairs, _) = chars.as_chunks::<2>();
    for (byte, pair) in token.iter_mut().zip(pairs) {
        let high = hex_nibble(pair[0]).ok_or(FUSION_E_INVALID_NAME)?;
        let low = hex_nibble(pair[1]).ok_or(FUSION_E_INVALID_NAME)?;
        *byte = (high << 4) | low;
    }
    Ok(token)
}

/// `parse_procarch`: the four architectures the C accepts, matched ignoring ASCII case.
fn parse_procarch(value: &[u16]) -> Result<PeKind, Hresult> {
    const ARCHES: [(&str, PeKind); 4] = [
        ("msil", PeKind::Msil),
        ("x86", PeKind::I386),
        ("ia64", PeKind::Ia64),
        ("amd64", PeKind::Amd64),
    ];
    ARCHES
        .iter()
        .find(|(arch, _)| eq_ignore_case(value, arch))
        .map(|&(_, kind)| kind)
        .ok_or(FUSION_E_INVALID_NAME)
}

fn hex_nibble(c: u16) -> Option<u8> {
    match c {
        0x30..=0x39 => Some((c - 0x30) as u8),
        0x41..=0x46 => Some((c - 0x41 + 10) as u8),
        0x61..=0x66 => Some((c - 0x61 + 10) as u8),
        _ => None,
    }
}

/// `wcstol(text, NULL, 10)`: leading C whitespace, an optional sign and then decimal digits. The
/// digits end at the first non-digit. The result is clamped to the 32-bit `LONG` range.
fn wcstol10(text: &[u16]) -> i32 {
    let text = skip_leading(text, is_c_space);
    let (negative, digits) = match text.first() {
        Some(&MINUS) => (true, &text[1..]),
        Some(&PLUS) => (false, &text[1..]),
        _ => (false, text),
    };
    let mut value: i64 = 0;
    for &c in digits {
        if !(ZERO..=NINE).contains(&c) {
            break;
        }
        value = (value * 10 + i64::from(c - ZERO)).min(OVERFLOW_CAP);
    }
    let signed = if negative { -value } else { value };
    signed.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

/// The C `iswspace` in the C locale: space, and `\t` through `\r`.
fn is_c_space(c: u16) -> bool {
    matches!(c, 0x20 | 0x09..=0x0D)
}

/// The wide string a C pointer names: the slice before its first NUL.
fn c_str(wide: &[u16]) -> &[u16] {
    let end = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    &wide[..end]
}

/// Drops the leading characters that `skip` accepts.
fn skip_leading(text: &[u16], skip: impl Fn(u16) -> bool) -> &[u16] {
    let start = text.iter().position(|&c| !skip(c)).unwrap_or(text.len());
    &text[start..]
}

/// Case-insensitive equality with an ASCII name, folding ASCII letters only.
fn eq_ignore_case(wide: &[u16], ascii: &str) -> bool {
    wide.len() == ascii.len()
        && wide
            .iter()
            .zip(ascii.bytes())
            .all(|(&w, b)| w < 0x80 && (w as u8).eq_ignore_ascii_case(&b))
}
