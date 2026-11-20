//! Deterministic parsing helpers from Wine's `dlls/http.sys`.
//!
//! These functions preserve the source driver's token and prefix-matching
//! behavior. They do not perform socket, queue, or IRP operations.

const VERBS: [&[u8]; 17] = [
    b"OPTIONS",
    b"GET",
    b"HEAD",
    b"POST",
    b"PUT",
    b"DELETE",
    b"TRACE",
    b"CONNECT",
    b"TRACK",
    b"MOVE",
    b"COPY",
    b"PROPFIND",
    b"PROPPATCH",
    b"MKCOL",
    b"LOCK",
    b"UNLOCK",
    b"SEARCH",
];

const REQUEST_HEADERS: [&[u8]; 41] = [
    b"Cache-Control",
    b"Connection",
    b"Date",
    b"Keep-Alive",
    b"Pragma",
    b"Trailer",
    b"Transfer-Encoding",
    b"Upgrade",
    b"Via",
    b"Warning",
    b"Allow",
    b"Content-Length",
    b"Content-Type",
    b"Content-Encoding",
    b"Content-Language",
    b"Content-Location",
    b"Content-MD5",
    b"Content-Range",
    b"Expires",
    b"Last-Modified",
    b"Accept",
    b"Accept-Charset",
    b"Accept-Encoding",
    b"Accept-Language",
    b"Authorization",
    b"Cookie",
    b"Expect",
    b"From",
    b"Host",
    b"If-Match",
    b"If-Modified-Since",
    b"If-None-Match",
    b"If-Range",
    b"If-Unmodified-Since",
    b"Max-Forwards",
    b"Proxy-Authorization",
    b"Referer",
    b"Range",
    b"TE",
    b"Translate",
    b"User-Agent",
];

/// Wine's `HttpVerbUnknown` value, one past the final recognized verb.
pub const HTTP_VERB_UNKNOWN: u8 = VERBS.len() as u8;
/// Wine's `HttpHeaderRequestMaximum` sentinel, one past the last known header.
pub const HTTP_HEADER_REQUEST_MAXIMUM: usize = REQUEST_HEADERS.len();

/// Find the verb using Wine's case-sensitive, length-limited `strncmp` lookup.
///
/// The result is the zero-based `HTTP_VERB` offset from `HttpVerbOPTIONS`, or
/// [`HTTP_VERB_UNKNOWN`] when no entry matches. Short prefixes deliberately
/// match the first verb beginning with that prefix, as in Wine.
pub fn parse_verb(verb: &[u8]) -> u8 {
    VERBS
        .iter()
        .position(|candidate| c_string_n_eq(verb, candidate, false))
        .map_or(HTTP_VERB_UNKNOWN, |index| index as u8)
}

/// Return the length of a token as defined by Wine's RFC 2616 helper.
pub fn parse_token(input: &[u8]) -> usize {
    const SEPARATORS: &[u8] = b"()<>@,;:\\\"/[]?={}";
    input
        .iter()
        .take_while(|byte| byte.is_ascii_graphic() && !SEPARATORS.contains(byte))
        .count()
}

/// Find a request header using Wine's case-insensitive prefix lookup.
///
/// Returns the zero-based header ID, or [`HTTP_HEADER_REQUEST_MAXIMUM`] for
/// an unknown name. A short prefix, including an empty name, follows Wine's
/// `strnicmp` behavior and may match an earlier table entry.
pub fn parse_header_name(header: &[u8]) -> usize {
    REQUEST_HEADERS
        .iter()
        .position(|candidate| c_string_n_eq(header, candidate, true))
        .unwrap_or(HTTP_HEADER_REQUEST_MAXIMUM)
}

/// A parsed header name and its raw value, excluding surrounding trailing LWS.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParsedHeader<'a> {
    /// Token bytes preceding optional whitespace and the colon.
    pub name: &'a [u8],
    /// Printable value bytes after leading whitespace and before trailing LWS.
    pub value: &'a [u8],
}

/// Parse a header line with Wine's token, colon-skip, and value-trimming rules.
pub fn parse_header(line: &[u8]) -> ParsedHeader<'_> {
    let name_len = parse_token(line);
    let mut cursor = name_len;
    while line
        .get(cursor)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t'))
    {
        cursor += 1;
    }
    cursor = cursor.saturating_add(1).min(line.len());
    while line
        .get(cursor)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t'))
    {
        cursor += 1;
    }
    let value_start = cursor;
    while line
        .get(cursor)
        .is_some_and(|byte| byte.is_ascii_graphic() || *byte == b' ' || *byte == b'\t')
    {
        cursor += 1;
    }
    while cursor > value_start && matches!(line[cursor - 1], b' ' | b'\t') {
        cursor -= 1;
    }
    ParsedHeader {
        name: &line[..name_len],
        value: &line[value_start..cursor],
    }
}

/// Compare an available prefix against `expected`.
///
/// Returns `1` for a complete match, `0` when more input is needed, and `-1`
/// at the first differing byte.
pub fn compare_exact(input: &[u8], expected: &[u8]) -> i32 {
    for (index, expected_byte) in expected.iter().enumerate() {
        let Some(input_byte) = input.get(index) else {
            return 0;
        };
        if input_byte != expected_byte {
            return -1;
        }
    }
    1
}

/// Parse leading ASCII decimal digits, returning the value and consumed bytes.
///
/// As in Wine's `int` implementation, callers should keep the value within
/// `i32`; signed overflow in the C source is undefined and is not oracle-tested.
pub fn parse_number(input: &[u8]) -> (i32, usize) {
    let mut number = 0_i32;
    let mut consumed = 0;
    while let Some(byte) = input.get(consumed).filter(|byte| byte.is_ascii_digit()) {
        number = number
            .wrapping_mul(10)
            .wrapping_add(i32::from(*byte - b'0'));
        consumed += 1;
    }
    (number, consumed)
}

/// Compare a registered URL path with a request path using Wine's matching rule.
///
/// Query strings and one trailing slash on either path are ignored. Empty paths
/// return no match rather than indexing before the start as the C helper would.
pub fn compare_paths(queue_path: &[u8], connection_path: &[u8]) -> u32 {
    let mut queue_len = queue_path.len();
    let mut connection_len = connection_path
        .iter()
        .position(|byte| *byte == b'?')
        .unwrap_or(connection_path.len());
    if queue_len == 0 || connection_len == 0 {
        return 0;
    }
    if queue_path[queue_len - 1] == b'/' {
        queue_len -= 1;
    }
    if connection_path[connection_len - 1] == b'/' {
        connection_len -= 1;
    }
    if connection_len < queue_len {
        return 0;
    }
    let mut slash_count = 1;
    for index in 0..queue_len {
        if queue_path[index] != connection_path[index] {
            return 0;
        }
        if queue_path[index] == b'/' {
            slash_count += 1;
        }
    }
    if queue_len == connection_len || connection_path[queue_len] == b'/' {
        slash_count
    } else {
        0
    }
}

fn c_string_n_eq(input: &[u8], candidate: &[u8], ignore_ascii_case: bool) -> bool {
    let count = input.len();
    for index in 0..count {
        let input_byte = input.get(index).copied().unwrap_or(0);
        let candidate_byte = candidate.get(index).copied().unwrap_or(0);
        let (input_byte, candidate_byte) = if ignore_ascii_case {
            (
                input_byte.to_ascii_lowercase(),
                candidate_byte.to_ascii_lowercase(),
            )
        } else {
            (input_byte, candidate_byte)
        };
        if input_byte != candidate_byte {
            return false;
        }
        if input_byte == 0 {
            return true;
        }
    }
    true
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
