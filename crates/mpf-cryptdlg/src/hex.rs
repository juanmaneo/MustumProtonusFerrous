use crate::fill::{fits, Fill};

/// Formats bytes as lowercase hexadecimal pairs separated by spaces.
/// `capacity` is measured in UTF-16 code units and includes the trailing NUL;
/// `None` performs Wine's size-query call.
pub fn format_hex(bytes: &[u8], capacity: Option<usize>) -> Fill {
    let chars_needed = if bytes.is_empty() {
        1
    } else {
        bytes.len().saturating_mul(3)
    };
    let mut text = String::new();
    if capacity.is_some() && fits(capacity, chars_needed) {
        for (index, byte) in bytes.iter().enumerate() {
            if index != 0 {
                text.push(' ');
            }
            use std::fmt::Write;
            write!(&mut text, "{byte:02x}").expect("writing to a String cannot fail");
        }
    }
    Fill {
        success: fits(capacity, chars_needed),
        chars_needed,
        text: capacity.map(|_| text),
    }
}
