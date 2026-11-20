//! Port of the portable SSML `<prosody>` value logic in Wine's `dlls/sapi/xml.c`.
//!
//! `xml.c` parses `rate`, `volume` and `pitch` attribute values with `wcstod`
//! and maps them onto SAPI's `RateAdj`, `Volume` and `PitchAdj.MiddleAdj`
//! fields. This crate ports that conversion over UTF-16 attribute text. It does
//! not parse XML and does not touch SAPI, COM or audio.
//!
//! Numeric prefixes follow `wcstod` for ASCII decimal notation (optional sign,
//! digits, optional fraction, optional exponent). Hexadecimal, `inf` and `nan`
//! forms, which `wcstod` also accepts, are not ported and are rejected.
//!
//! Ported (3 attributes, 1 pure function):
//! - `<prosody>` `rate`, `volume`, `pitch`: [`apply_prosody`].
//!
//! Skipped: the XML tokenizer, the `<speak>`/`<p>`/`<s>` element walk, text
//! fragment allocation, and `FIXME`/`ERR` logging. These need XML, heap
//! allocation of `SPVTEXTFRAG`, or the debug channel.

/// The SSML `<prosody>` attribute whose value is converted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProsodyAttribute {
    /// `rate`: SAPI `RateAdj`, a logarithmic scale with base 3.
    Rate,
    /// `volume`: SAPI `Volume`, clamped to 0 through 100.
    Volume,
    /// `pitch`: SAPI `PitchAdj.MiddleAdj`, a logarithmic scale with base 2.
    Pitch,
}

/// The attribute value is not in a form Wine's `<prosody>` parser accepts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsupportedFormat;

const RATE_NAMES: &[(&str, i32)] = &[
    ("x-slow", -9),
    ("slow", -4),
    ("medium", 0),
    ("fast", 4),
    ("x-fast", 9),
];

const VOLUME_NAMES: &[(&str, i32)] = &[
    ("silent", 0),
    ("x-soft", 20),
    ("soft", 40),
    ("medium", 60),
    ("loud", 80),
    ("x-loud", 100),
];

const PITCH_NAMES: &[(&str, i32)] = &[
    ("x-low", -9),
    ("low", -4),
    ("medium", 0),
    ("high", 4),
    ("x-high", 9),
];

/// Converts one `<prosody>` attribute value into the new SAPI state value.
///
/// `value` is the attribute text without its quotes. `current` is the state
/// value before the attribute applies. Named keywords take precedence over
/// numbers. For `rate` and `volume`, a `%` suffix makes the number relative,
/// and a leading `+` or `-` on `volume` makes it an offset. `pitch` accepts
/// only `%`, and an `Hz` value leaves `current` unchanged.
pub fn apply_prosody(
    attribute: ProsodyAttribute,
    value: &[u16],
    current: i32,
) -> Result<i32, UnsupportedFormat> {
    if value.is_empty() {
        return Err(UnsupportedFormat);
    }
    let names = match attribute {
        ProsodyAttribute::Rate => RATE_NAMES,
        ProsodyAttribute::Volume => VOLUME_NAMES,
        ProsodyAttribute::Pitch => PITCH_NAMES,
    };
    if let Some(named) = named_value(value, names) {
        return Ok(named);
    }

    let (number, read_len) = parse_decimal_prefix(value);
    match attribute {
        ProsodyAttribute::Rate => rate_value(value, number, read_len),
        ProsodyAttribute::Volume => volume_value(value, number, read_len, current),
        ProsodyAttribute::Pitch => pitch_value(value, number, read_len, current),
    }
}

fn named_value(value: &[u16], names: &[(&str, i32)]) -> Option<i32> {
    for &(name, result) in names {
        if value.iter().copied().eq(name.bytes().map(u16::from)) {
            return Some(result);
        }
    }
    None
}

fn rate_value(value: &[u16], number: f64, read_len: usize) -> Result<i32, UnsupportedFormat> {
    check_optional_percent(value, read_len)?;
    let rate = if ends_with_percent(value) {
        1.0 + number / 100.0
    } else {
        number
    };
    Ok(if rate < 0.0 {
        0
    } else if rate <= 0.01 {
        -10
    } else {
        (rate.ln() * (10.0 / 3.0_f64.ln())).round() as i32
    })
}

fn volume_value(
    value: &[u16],
    number: f64,
    read_len: usize,
    current: i32,
) -> Result<i32, UnsupportedFormat> {
    check_optional_percent(value, read_len)?;
    let volume = if ends_with_percent(value) {
        f64::from(current) * (1.0 + number / 100.0)
    } else if is_sign(value.first().copied()) {
        f64::from(current) + number
    } else {
        number
    };
    Ok(volume.round().clamp(0.0, 100.0) as i32)
}

fn pitch_value(
    value: &[u16],
    number: f64,
    read_len: usize,
    current: i32,
) -> Result<i32, UnsupportedFormat> {
    if value.len() > 2
        && read_len == value.len() - 2
        && is_ascii(value.get(read_len).copied(), b'H')
        && is_ascii(value.get(read_len + 1).copied(), b'z')
    {
        return Ok(current);
    }
    if read_len != value.len() - 1 || !ends_with_percent(value) {
        return Err(UnsupportedFormat);
    }
    Ok(if number > -99.0 {
        current + ((1.0 + number / 100.0).log2() * 24.0).round() as i32
    } else {
        current - 10
    })
}

/// Rejects a parse that stopped inside the value. A value with a trailing unit
/// is accepted only when that unit is the single `%` character.
fn check_optional_percent(value: &[u16], read_len: usize) -> Result<(), UnsupportedFormat> {
    if read_len == value.len() || (read_len + 1 == value.len() && ends_with_percent(value)) {
        Ok(())
    } else {
        Err(UnsupportedFormat)
    }
}

fn ends_with_percent(value: &[u16]) -> bool {
    is_ascii(value.last().copied(), b'%')
}

fn is_sign(unit: Option<u16>) -> bool {
    is_ascii(unit, b'+') || is_ascii(unit, b'-')
}

fn is_ascii(unit: Option<u16>, byte: u8) -> bool {
    unit == Some(u16::from(byte))
}

fn is_c_space(unit: u16) -> bool {
    matches!(unit, 0x09..=0x0d | 0x20)
}

fn scan_digits(value: &[u16], mut end: usize) -> usize {
    while value
        .get(end)
        .is_some_and(|&unit| (b'0' as u16..=b'9' as u16).contains(&unit))
    {
        end += 1;
    }
    end
}

/// Returns the value `wcstod` would produce and the index just past the parsed
/// text, measured from the start of `value`. With no digits, it returns
/// `(0.0, 0)`, the no-conversion result.
fn parse_decimal_prefix(value: &[u16]) -> (f64, usize) {
    let mut start = 0;
    while value.get(start).is_some_and(|&unit| is_c_space(unit)) {
        start += 1;
    }

    let mut end = start;
    if is_sign(value.get(end).copied()) {
        end += 1;
    }
    let sign_end = end;
    end = scan_digits(value, end);
    let mut digits = end - sign_end;

    if is_ascii(value.get(end).copied(), b'.') {
        let fraction_end = scan_digits(value, end + 1);
        digits += fraction_end - end - 1;
        end = fraction_end;
    }
    if digits == 0 {
        return (0.0, 0);
    }

    if is_ascii(value.get(end).copied(), b'e') || is_ascii(value.get(end).copied(), b'E') {
        let mut exponent = end + 1;
        if is_sign(value.get(exponent).copied()) {
            exponent += 1;
        }
        let exponent_end = scan_digits(value, exponent);
        if exponent_end > exponent {
            end = exponent_end;
        }
    }

    let mut text = String::new();
    for &unit in &value[start..end] {
        text.push(char::from(unit as u8));
    }
    (text.parse::<f64>().unwrap_or(0.0), end)
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
