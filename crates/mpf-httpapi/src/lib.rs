//! Safe, deterministic builders for two Wine `dlls/httpapi` exports.
//!
//! Ported exports (2 of 37 in `httpapi.spec`):
//! - `HttpSendHttpResponse`: builds the payload Wine writes to its response
//!   IOCTL: status line, generated Date, known and unknown headers, blank line,
//!   and in-memory entity chunks.
//! - `HttpSendResponseEntityBody`: concatenates in-memory entity chunks.
//!
//! The returned bytes are only the payload. No socket or queue is touched, and
//! the Date comes from an injected [`SystemTime`] instead of the clock.
//!
//! Deliberate differences from Wine (C is undefined or unsafe there):
//! - Out-of-range month or weekday returns [`ERROR_INVALID_PARAMETER`]; Wine
//!   indexes its static name arrays with them.
//! - Wine sizes buffers from declared lengths and assumes a four-digit year;
//!   this port emits exactly the bytes it formats, with no over- or underrun.
//! - Content-Length is the true decimal length; Wine's `int` total can wrap.
//!
//! Skipped groups (not ported):
//! - 14 explicit `@ stub` exports (for example `HttpAddFragmentToCache`,
//!   `HttpControlService`): Wine has no implementation to port.
//! - Server-session and URL-group management (`HttpCreateServerSession`,
//!   `HttpAddUrlToUrlGroup`, ...): depends on the live global session and URL
//!   lists.
//! - Request-queue, receive, and URL operations (`HttpCreateRequestQueue`,
//!   `HttpReceiveHttpRequest`, `HttpAddUrl`, ...): depend on device handles,
//!   sockets, or the request queue.
//! - Service configuration, property setters, `HttpInitialize`, and
//!   `HttpTerminate`: depend on the SCM, registry, or process-global state.
//!
//! The C oracle harnesses and their vectors are under `target/canon/httpapi/`.
//! This crate has no external dependencies.

/// Win32 `ERROR_INVALID_PARAMETER`.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// Win32 `ERROR_CALL_NOT_IMPLEMENTED`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;
/// Win32 `ERROR_CONNECTION_INVALID`.
pub const ERROR_CONNECTION_INVALID: u32 = 1229;

const HEADER_NAMES: [&str; 30] = [
    "Cache-Control",
    "Connection",
    "Date",
    "Keep-Alive",
    "Pragma",
    "Trailer",
    "Transfer-Encoding",
    "Upgrade",
    "Via",
    "Warning",
    "Allow",
    "Content-Length",
    "Content-Type",
    "Content-Encoding",
    "Content-Language",
    "Content-Location",
    "Content-MD5",
    "Content-Range",
    "Expires",
    "Last-Modified",
    "Accept-Ranges",
    "Age",
    "ETag",
    "Location",
    "Proxy-Authenticate",
    "Retry-After",
    "Server",
    "Set-Cookie",
    "Vary",
    "WWW-Authenticate",
];
const DATE_HEADER: usize = 2;
const CONTENT_LENGTH_HEADER: usize = 11;
const MORE_DATA: u32 = 0x2;

/// Win32 `SYSTEMTIME` fields used to generate the response Date header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SystemTime {
    /// Calendar year, printed without padding (Wine's `%u`).
    pub year: u16,
    /// Month, 1 through 12.
    pub month: u16,
    /// Sunday is 0 through Saturday as 6.
    pub day_of_week: u16,
    /// Day of month, printed with a minimum width of two.
    pub day: u16,
    /// Hour, printed with a minimum width of two.
    pub hour: u16,
    /// Minute, printed with a minimum width of two.
    pub minute: u16,
    /// Second, printed with a minimum width of two.
    pub second: u16,
}

/// One arbitrary response header, emitted in input order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnknownHeader<'a> {
    /// Header name; emitted up to the first NUL, as with C's `%.*s`.
    pub name: &'a [u8],
    /// Header value; emitted up to the first NUL, as with C's `%.*s`.
    pub value: &'a [u8],
}

/// Entity chunk accepted by either exported builder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntityChunk<'a> {
    /// Bytes copied verbatim into the body.
    FromMemory(&'a [u8]),
    /// File-handle and fragment-cache chunks; Wine returns error 120 for these.
    Unsupported,
}

/// Input fields used by [`build_http_response`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HttpResponse<'a> {
    /// Numeric status code, printed in decimal.
    pub status_code: u16,
    /// Reason phrase; emitted up to the first NUL, as with C's `%.*s`.
    pub reason: &'a [u8],
    /// Pairs of known response-header index and raw value. Index 2 (Date) is
    /// always generated, so a supplied Date is ignored, as in Wine.
    pub known_headers: &'a [(usize, &'a [u8])],
    /// Unknown headers, emitted in the supplied order.
    pub unknown_headers: &'a [UnknownHeader<'a>],
    /// Entity chunks appended after the blank line.
    pub entity_chunks: &'a [EntityChunk<'a>],
}

/// Build the byte payload Wine sends for `HttpSendHttpResponse`.
///
/// Only the MORE_DATA bit of `flags` affects the output; Content-Length is
/// generated unless it is set and no explicit Content-Length was supplied.
/// A known header with an empty value counts as absent, and a value that is
/// only a NUL emits its name with an empty value.
pub fn build_http_response(
    response: &HttpResponse<'_>,
    flags: u32,
    time: SystemTime,
) -> Result<Vec<u8>, u32> {
    if time.day_of_week >= 7 || !(1..=12).contains(&time.month) {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let body = collect_body(response.entity_chunks)?;

    let mut output = Vec::new();
    output.extend_from_slice(b"HTTP/1.1 ");
    append_decimal(&mut output, u64::from(response.status_code));
    output.push(b' ');
    append_c_string(&mut output, response.reason);
    output.extend_from_slice(b"\r\n");

    for (index, name) in HEADER_NAMES.iter().enumerate() {
        if index == DATE_HEADER {
            append_date(&mut output, time);
        } else if let Some(value) = known_value(response.known_headers, index) {
            output.extend_from_slice(name.as_bytes());
            output.extend_from_slice(b": ");
            append_c_string(&mut output, value);
            output.extend_from_slice(b"\r\n");
        } else if index == CONTENT_LENGTH_HEADER && flags & MORE_DATA == 0 {
            output.extend_from_slice(b"Content-Length: ");
            append_decimal(&mut output, body.len() as u64);
            output.extend_from_slice(b"\r\n");
        }
    }

    for header in response.unknown_headers {
        append_c_string(&mut output, header.name);
        output.extend_from_slice(b": ");
        append_c_string(&mut output, header.value);
        output.extend_from_slice(b"\r\n");
    }
    output.extend_from_slice(b"\r\n");
    output.extend_from_slice(&body);
    Ok(output)
}

/// Concatenate memory chunks for `HttpSendResponseEntityBody`.
///
/// A zero request ID is rejected before the chunks are examined. Flags do not
/// change the payload, so they are not an input here.
pub fn build_entity_body(request_id: u64, chunks: &[EntityChunk<'_>]) -> Result<Vec<u8>, u32> {
    if request_id == 0 {
        return Err(ERROR_CONNECTION_INVALID);
    }
    collect_body(chunks)
}

fn collect_body(chunks: &[EntityChunk<'_>]) -> Result<Vec<u8>, u32> {
    let mut body = Vec::new();
    for chunk in chunks {
        match chunk {
            EntityChunk::FromMemory(bytes) => body.extend_from_slice(bytes),
            EntityChunk::Unsupported => return Err(ERROR_CALL_NOT_IMPLEMENTED),
        }
    }
    Ok(body)
}

fn known_value<'a>(headers: &'a [(usize, &'a [u8])], index: usize) -> Option<&'a [u8]> {
    headers
        .iter()
        .find(|(header_index, value)| *header_index == index && !value.is_empty())
        .map(|(_, value)| *value)
}

fn append_c_string(output: &mut Vec<u8>, bytes: &[u8]) {
    let length = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    output.extend_from_slice(&bytes[..length]);
}

fn append_decimal(output: &mut Vec<u8>, value: u64) {
    output.extend_from_slice(value.to_string().as_bytes());
}

fn append_date(output: &mut Vec<u8>, time: SystemTime) {
    const DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    output.extend_from_slice(b"Date: ");
    output.extend_from_slice(DAYS[usize::from(time.day_of_week)].as_bytes());
    output.extend_from_slice(b", ");
    append_two_digits(output, time.day);
    output.push(b' ');
    output.extend_from_slice(MONTHS[usize::from(time.month - 1)].as_bytes());
    output.push(b' ');
    append_decimal(output, u64::from(time.year));
    output.push(b' ');
    append_two_digits(output, time.hour);
    output.push(b':');
    append_two_digits(output, time.minute);
    output.push(b':');
    append_two_digits(output, time.second);
    output.extend_from_slice(b" GMT\r\n");
}

fn append_two_digits(output: &mut Vec<u8>, value: u16) {
    if value < 10 {
        output.push(b'0');
    }
    append_decimal(output, u64::from(value));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known<'a>(headers: &'a [(usize, &'a [u8])]) -> &'a [(usize, &'a [u8])] {
        headers
    }

    fn check(name: &str, actual: Result<Vec<u8>, u32>, expected: Result<&[u8], u32>) {
        assert_eq!(
            actual.as_deref().map_err(|error| *error),
            expected,
            "{name}"
        );
    }

    const T0: SystemTime = SystemTime {
        year: 2026,
        month: 11,
        day_of_week: 3,
        day: 18,
        hour: 17,
        minute: 39,
        second: 5,
    };

    // BEGIN GENERATED: target/canon/httpapi/gen.py
    // Oracle vectors: rows from target/canon/httpapi/out_*.txt, generated by gen.py.
    #[test]
    fn oracle_http_response_vectors() {
        check(
            "H01",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H02",
            build_http_response(
                &HttpResponse {
                    status_code: 204,
                    reason: &b""[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 204 \r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H03",
            build_http_response(
                &HttpResponse {
                    status_code: 65535,
                    reason: &b"Max"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 65535 Max\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H04",
            build_http_response(
                &HttpResponse {
                    status_code: 0,
                    reason: &b"Zero"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 0 Zero\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H05",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"hello"[..])],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 5\r\n\r\nhello"[..]),
        );
        check(
            "H06",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"ab"[..]), EntityChunk::FromMemory(&b""[..]), EntityChunk::FromMemory(&b"cd"[..])],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 4\r\n\r\nabcd"[..]),
        );
        check(
            "H07",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"x"[..])],
                },
                0x2,
                SystemTime {
                    year: 2026,
                    month: 11,
                    day_of_week: 3,
                    day: 18,
                    hour: 17,
                    minute: 39,
                    second: 5,
                },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\n\r\nx"[..]),
        );
        check(
            "H08",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"x"[..])],
                },
                0xfffffffd,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 1\r\n\r\nx"[..]),
        );
        check(
            "H09",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"x"[..])],
                },
                0xffffffff,
                SystemTime {
                    year: 2026,
                    month: 11,
                    day_of_week: 3,
                    day: 18,
                    hour: 17,
                    minute: 39,
                    second: 5,
                },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\n\r\nx"[..]),
        );
        check(
            "H10",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[(11, &b"42"[..])]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"hello"[..])],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 42\r\n\r\nhello"[..]),
        );
        check(
            "H11",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[(11, &b"42"[..])]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"hello"[..])],
                },
                0x2,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 42\r\n\r\nhello"[..]),
        );
        check(
            "H12",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[(2, &b"yesteryear"[..])]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H13",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[(0, &b"v0"[..]), (1, &b"v1"[..]), (2, &b"v2"[..]), (3, &b"v3"[..]), (4, &b"v4"[..]), (5, &b"v5"[..]), (6, &b"v6"[..]), (7, &b"v7"[..]), (8, &b"v8"[..]), (9, &b"v9"[..]), (10, &b"v10"[..]), (11, &b"v11"[..]), (12, &b"v12"[..]), (13, &b"v13"[..]), (14, &b"v14"[..]), (15, &b"v15"[..]), (16, &b"v16"[..]), (17, &b"v17"[..]), (18, &b"v18"[..]), (19, &b"v19"[..]), (20, &b"v20"[..]), (21, &b"v21"[..]), (22, &b"v22"[..]), (23, &b"v23"[..]), (24, &b"v24"[..]), (25, &b"v25"[..]), (26, &b"v26"[..]), (27, &b"v27"[..]), (28, &b"v28"[..]), (29, &b"v29"[..])]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"body"[..])],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nCache-Control: v0\r\nConnection: v1\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nKeep-Alive: v3\r\nPragma: v4\r\nTrailer: v5\r\nTransfer-Encoding: v6\r\nUpgrade: v7\r\nVia: v8\r\nWarning: v9\r\nAllow: v10\r\nContent-Length: v11\r\nContent-Type: v12\r\nContent-Encoding: v13\r\nContent-Language: v14\r\nContent-Location: v15\r\nContent-MD5: v16\r\nContent-Range: v17\r\nExpires: v18\r\nLast-Modified: v19\r\nAccept-Ranges: v20\r\nAge: v21\r\nETag: v22\r\nLocation: v23\r\nProxy-Authenticate: v24\r\nRetry-After: v25\r\nServer: v26\r\nSet-Cookie: v27\r\nVary: v28\r\nWWW-Authenticate: v29\r\n\r\nbody"[..]),
        );
        check(
            "H14",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[(0, &b""[..]), (11, &b""[..])]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"abc"[..])],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 3\r\n\r\nabc"[..]),
        );
        check(
            "H15",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK\x00ZZ"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H16",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[(1, &b"keep\x00alive"[..])]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nConnection: keep\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H17",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[(8, &b"\x00"[..])]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nVia: \r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H18",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[UnknownHeader { name: &b"X-A"[..], value: &b"1"[..] }, UnknownHeader { name: &b""[..], value: &b"empty-name"[..] }, UnknownHeader { name: &b"X-C"[..], value: &b""[..] }],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\nX-A: 1\r\n: empty-name\r\nX-C: \r\n\r\n"[..]),
        );
        check(
            "H19",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[UnknownHeader { name: &b"X\x00Y"[..], value: &b"v\x00w"[..] }],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\nX: v\r\n\r\n"[..]),
        );
        check(
            "H20",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[
                        EntityChunk::FromMemory(&b"ok"[..]),
                        EntityChunk::Unsupported,
                    ],
                },
                0x0,
                SystemTime {
                    year: 2026,
                    month: 11,
                    day_of_week: 3,
                    day: 18,
                    hour: 17,
                    minute: 39,
                    second: 5,
                },
            ),
            Err(120),
        );
        check(
            "H21",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 999, month: 12, day_of_week: 2, day: 31, hour: 23, minute: 59, second: 59 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Tue, 31 Dec 999 23:59:59 GMT\r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H22",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 2, day_of_week: 0, day: 1, hour: 0, minute: 0, second: 0 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Sun, 01 Feb 2026 00:00:00 GMT\r\nContent-Length: 0\r\n\r\n"[..]),
        );
        check(
            "H23",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[],
                    entity_chunks: &[EntityChunk::FromMemory(&b"\x00\x01\x7f\x80\xfe\r\n"[..])],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 7\r\n\r\n\x00\x01\x7f\x80\xfe\r\n"[..]),
        );
        check(
            "H24",
            build_http_response(
                &HttpResponse {
                    status_code: 200,
                    reason: &b"OK"[..],
                    known_headers: known(&[]),
                    unknown_headers: &[UnknownHeader { name: &b"X-\xff"[..], value: &b"\x80"[..] }],
                    entity_chunks: &[],
                },
                0x0,
                SystemTime { year: 2026, month: 11, day_of_week: 3, day: 18, hour: 17, minute: 39, second: 5 },
            ),
            Ok(&b"HTTP/1.1 200 OK\r\nDate: Wed, 18 Nov 2026 17:39:05 GMT\r\nContent-Length: 0\r\nX-\xff: \x80\r\n\r\n"[..]),
        );
    }

    #[test]
    fn oracle_entity_body_vectors() {
        check(
            "E01",
            build_entity_body(0x0, &[EntityChunk::FromMemory(&b"x"[..])]),
            Err(1229),
        );
        check(
            "E02",
            build_entity_body(0x0, &[EntityChunk::Unsupported]),
            Err(1229),
        );
        check("E03", build_entity_body(0x1, &[]), Ok(&b""[..]));
        check(
            "E04",
            build_entity_body(0x7, &[EntityChunk::FromMemory(&b"hello"[..])]),
            Ok(&b"hello"[..]),
        );
        check(
            "E05",
            build_entity_body(
                0x7,
                &[
                    EntityChunk::FromMemory(&b"ab"[..]),
                    EntityChunk::FromMemory(&b"cd"[..]),
                ],
            ),
            Ok(&b"abcd"[..]),
        );
        check(
            "E06",
            build_entity_body(0x7, &[EntityChunk::FromMemory(&b"a\x00b\x00"[..])]),
            Ok(&b"a\x00b\x00"[..]),
        );
        check(
            "E07",
            build_entity_body(0xffffffffffffffff, &[EntityChunk::FromMemory(&b"x"[..])]),
            Ok(&b"x"[..]),
        );
        check(
            "E08",
            build_entity_body(0x1, &[EntityChunk::FromMemory(&b""[..])]),
            Ok(&b""[..]),
        );
        check(
            "E09",
            build_entity_body(0x7, &[EntityChunk::Unsupported]),
            Err(120),
        );
        check(
            "E10",
            build_entity_body(
                0x7,
                &[
                    EntityChunk::FromMemory(&b"ok"[..]),
                    EntityChunk::Unsupported,
                ],
            ),
            Err(120),
        );
        check(
            "E11",
            build_entity_body(0x7, &[EntityChunk::Unsupported]),
            Err(120),
        );
        check(
            "E12",
            build_entity_body(0x7, &[EntityChunk::FromMemory(&b"more"[..])]),
            Ok(&b"more"[..]),
        );
        check(
            "E13",
            build_entity_body(0x7, &[EntityChunk::FromMemory(&b"z"[..])]),
            Ok(&b"z"[..]),
        );
        check(
            "E14",
            build_entity_body(0x7, &[EntityChunk::FromMemory(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\n\x0b\x0c\r\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f !\"#$%&'()*+,-./0123456789:;"[..]), EntityChunk::FromMemory(&b"<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abc"[..])]),
            Ok(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\n\x0b\x0c\r\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abc"[..]),
        );
        check(
            "E15",
            build_entity_body(
                0x7,
                &[
                    EntityChunk::FromMemory(&b"\x80\x81"[..]),
                    EntityChunk::FromMemory(&b""[..]),
                    EntityChunk::FromMemory(&b"\xfe\xff"[..]),
                    EntityChunk::FromMemory(&b"\x00"[..]),
                ],
            ),
            Ok(&b"\x80\x81\xfe\xff\x00"[..]),
        );
    }
    // END GENERATED

    fn response<'a>(
        reason: &'a [u8],
        known_headers: &'a [(usize, &'a [u8])],
        unknown_headers: &'a [UnknownHeader<'a>],
        entity_chunks: &'a [EntityChunk<'a>],
    ) -> HttpResponse<'a> {
        HttpResponse {
            status_code: 200,
            reason,
            known_headers,
            unknown_headers,
            entity_chunks,
        }
    }

    #[test]
    fn edge_http_response_time_validation() {
        let plain = response(b"OK", known(&[]), &[], &[]);
        assert_eq!(
            build_http_response(&plain, 0, SystemTime { month: 0, ..T0 }),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(
            build_http_response(&plain, 0, SystemTime { month: 13, ..T0 }),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(
            build_http_response(
                &plain,
                0,
                SystemTime {
                    day_of_week: 7,
                    ..T0
                }
            ),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert!(build_http_response(
            &plain,
            0,
            SystemTime {
                month: 1,
                day_of_week: 6,
                ..T0
            }
        )
        .is_ok());
    }

    #[test]
    fn edge_http_response_header_presence_and_nul_truncation() {
        let empty_value = [(0, b"".as_slice())];
        let bytes =
            build_http_response(&response(b"OK", known(&empty_value), &[], &[]), 0, T0).unwrap();
        assert!(!bytes
            .windows(b"Cache-Control".len())
            .any(|w| w == b"Cache-Control"));

        let nul_only = [(8, b"\0".as_slice())];
        let bytes =
            build_http_response(&response(b"OK", known(&nul_only), &[], &[]), 0, T0).unwrap();
        assert!(bytes.windows(b"Via: \r\n".len()).any(|w| w == b"Via: \r\n"));

        let unknown = [UnknownHeader {
            name: b"X\0",
            value: b"\0",
        }];
        let bytes = build_http_response(&response(b"", known(&[]), &unknown, &[]), 0, T0).unwrap();
        assert!(bytes.windows(b"X: \r\n".len()).any(|w| w == b"X: \r\n"));
    }

    #[test]
    fn edge_http_response_entity_errors_and_body_order() {
        let chunks = [EntityChunk::FromMemory(b"ab"), EntityChunk::Unsupported];
        assert_eq!(
            build_http_response(&response(b"OK", known(&[]), &[], &chunks), 0, T0),
            Err(ERROR_CALL_NOT_IMPLEMENTED)
        );

        let chunks = [
            EntityChunk::FromMemory(b"ab"),
            EntityChunk::FromMemory(b"cd"),
        ];
        let bytes = build_http_response(&response(b"OK", known(&[]), &[], &chunks), 0, T0).unwrap();
        assert!(bytes.ends_with(b"\r\n\r\nabcd"));
    }

    #[test]
    fn edge_entity_body_request_and_chunk_validation() {
        assert_eq!(
            build_entity_body(0, &[EntityChunk::Unsupported]),
            Err(ERROR_CONNECTION_INVALID)
        );
        assert_eq!(
            build_entity_body(1, &[EntityChunk::Unsupported]),
            Err(ERROR_CALL_NOT_IMPLEMENTED)
        );
        assert_eq!(build_entity_body(1, &[]), Ok(Vec::new()));
    }

    #[test]
    fn edge_decimal_formatting() {
        let mut out = Vec::new();
        append_decimal(&mut out, 0);
        append_decimal(&mut out, 65535);
        assert_eq!(out, b"065535");
    }
}
