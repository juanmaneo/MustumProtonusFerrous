use crate::fill::{fits, Fill};

const INDENT: &str = "     ";

/// Formats an already-decoded policy statement qualifier.
///
/// The Wine routine first ASN.1-decodes this value; that OS/library decode step
/// is outside this deterministic port. Capacities include the terminating NUL.
pub fn format_cps(value: &str, multi_line: bool, capacity: Option<usize>) -> Fill {
    let separator = if multi_line { "\r\n" } else { ", " };
    let value_units = value.encode_utf16().count();
    let mut chars_needed = 1;
    let mut text = String::new();

    if multi_line {
        chars_needed += INDENT.encode_utf16().count() * 3;
        if capacity.is_some() && fits(capacity, chars_needed) {
            text.push_str(INDENT);
            text.push_str(INDENT);
            text.push_str(INDENT);
        }
    }
    chars_needed += value_units + 1;
    if capacity.is_some() && fits(capacity, chars_needed) {
        text.push_str(value);
    }
    chars_needed += separator.encode_utf16().count();
    if capacity.is_some() && fits(capacity, chars_needed) {
        text.push_str(separator);
    }

    Fill {
        success: fits(capacity, chars_needed),
        chars_needed,
        text: capacity.map(|_| text),
    }
}
