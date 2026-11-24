//! SSDP M-SEARCH reply parsing (`port.c`, `parse_search_response`).

use crate::decimal_int;

/// The `LOCATION:` URL of an SSDP reply whose status line reads `HTTP/... 200`, as UTF-16.
///
/// The reply is split with `strtok` rules on `\n`, then on spaces and `\r`, and it is cut at its first
/// NUL like the C string. The URL must fit in `location_size` bytes, counting its NUL terminator, so the
/// URL length in units must be below `location_size / 2`. Wine's `MultiByteToWideChar` with a zero-length
/// buffer only returns the needed size and writes nothing. This port returns `None` for it because there
/// is no buffer to fill. Invalid UTF-8 is converted lossily, which Wine's CP_UTF8 conversion does not
/// guarantee.
pub fn search_location(response: &[u8], location_size: usize) -> Option<Vec<u16>> {
    let end = response
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(response.len());
    let mut rest = &response[..end];
    let mut status_line = next_token(&mut rest, b"\n")?;
    next_token(&mut status_line, b" ")?;
    let status = next_token(&mut status_line, b" ")?;
    if decimal_int(status.iter().map(|&byte| u32::from(byte))) != 200 {
        return None;
    }
    while let Some(mut line) = next_token(&mut rest, b"\n") {
        let Some(key) = next_token(&mut line, b" ") else {
            continue;
        };
        if key.eq_ignore_ascii_case(b"LOCATION:") {
            let value = next_token(&mut line, b" \r")?;
            let units: Vec<u16> = String::from_utf8_lossy(value).encode_utf16().collect();
            return (units.len() < location_size / 2).then_some(units);
        }
    }
    None
}

/// One `strtok` step: the next non-empty run of bytes outside `delims`. `rest` moves past the delimiter
/// that ended the run.
fn next_token<'a>(rest: &mut &'a [u8], delims: &[u8]) -> Option<&'a [u8]> {
    let input: &'a [u8] = rest;
    let start = input.iter().position(|byte| !delims.contains(byte))?;
    let input = &input[start..];
    let end = input
        .iter()
        .position(|byte| delims.contains(byte))
        .unwrap_or(input.len());
    *rest = input.get(end + 1..).unwrap_or_default();
    Some(&input[..end])
}
