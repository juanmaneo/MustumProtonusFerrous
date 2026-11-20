//! The header helpers of `mimeole.c`: the blank-line search of `copy_headers_to_buf`, the field
//! split of `read_prop` and `read_value`, `unfold_header`, `unquote_string`, `split_params`, and the
//! Content-Type and Content-Transfer-Encoding readers.

/// `copy_headers_to_buf`: the length of the header block at the start of `data`. The block runs
/// through the first blank line. A block with no blank line runs to the end of `data`.
pub fn header_block_len(data: &[u8]) -> usize {
    let mut line_start = 0;
    while let Some(pos) = find_crlf(&data[line_start..]) {
        let line_end = line_start + pos + 2;
        if pos == 0 {
            return line_end;
        }
        line_start = line_end;
    }
    data.len()
}

/// One header field, as `read_prop` and `read_value` give it.
#[derive(Debug, PartialEq, Eq)]
pub struct Field<'a> {
    /// The text before the first colon, not trimmed. It spans lines when a line has no colon.
    pub name: &'a [u8],
    /// The unfolded value. For Content-Type and Content-Disposition, it stops before the first
    /// parameter.
    pub value: Vec<u8>,
    /// The parameters of Content-Type and Content-Disposition, in order.
    pub params: Vec<Param>,
    /// The text after the value, where the next field starts.
    pub rest: &'a [u8],
}

/// A `name=value` parameter. The name is not trimmed, and the value is unquoted.
#[derive(Debug, PartialEq, Eq)]
pub struct Param {
    pub name: Vec<u8>,
    pub value: Vec<u8>,
}

/// Reads the first field of `block`. Returns `None` when `block` has no colon.
pub fn read_field(block: &[u8]) -> Option<Field<'_>> {
    let colon = block.iter().position(|&c| c == b':')?;
    let name = &block[..colon];
    let body = &block[colon + 1..];
    let end = value_end(body);
    let unfolded = unfold(&body[..end]);
    let (value, params) = if has_params(name) {
        let (main, params) = split_params(&unfolded);
        (main.to_vec(), params)
    } else {
        (unfolded, Vec::new())
    };
    Some(Field {
        name,
        value,
        params,
        rest: &body[end..],
    })
}

/// The headers that `default_props` flags with `MPF_HASPARAMS`.
fn has_params(name: &[u8]) -> bool {
    name.eq_ignore_ascii_case(b"Content-Type") || name.eq_ignore_ascii_case(b"Content-Disposition")
}

/// The value runs through each CRLF that a blank follows, and ends after the first CRLF that no
/// blank follows. Without a CRLF, the value is the rest of `body`.
fn value_end(body: &[u8]) -> usize {
    let mut end = 0;
    while let Some(pos) = find_crlf(&body[end..]) {
        end += pos + 2;
        if !body.get(end).is_some_and(|&c| is_blank(c)) {
            return end;
        }
    }
    body.len()
}

/// `unfold_header`: a CRLF followed by a blank becomes one space, and the blanks after it are
/// dropped. The text after the first CRLF that no blank follows is dropped, as in the C code.
pub fn unfold(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len());
    let mut i = skip_blanks(raw, 0);
    while let Some(pos) = find_crlf(&raw[i..]) {
        out.extend_from_slice(&raw[i..i + pos]);
        i += pos + 2;
        if !raw.get(i).is_some_and(|&c| is_blank(c)) {
            return out;
        }
        out.push(b' ');
        i = skip_blanks(raw, i);
    }
    out.extend_from_slice(&raw[i..]);
    out
}

/// `unquote_string`: skips leading blanks and one opening quote, then copies the text. A backslash
/// keeps the byte after it. A closing quote ends a quoted value. An unquoted value keeps its quotes.
pub fn unquote(raw: &[u8]) -> Vec<u8> {
    let mut i = skip_blanks(raw, 0);
    let quoted = raw.get(i) == Some(&b'"');
    if quoted {
        i += 1;
    }
    let mut out = Vec::with_capacity(raw.len());
    while let Some(&c) = raw.get(i) {
        match c {
            b'\\' => {
                if let Some(&next) = raw.get(i + 1) {
                    out.push(next);
                }
                i += 2;
            }
            b'"' if quoted => break,
            _ => {
                out.push(c);
                i += 1;
            }
        }
    }
    out
}

/// `split_params`: the main value is the text before the first `;` outside quotes. Each later
/// segment is a parameter, and a segment with no `=` is dropped.
pub fn split_params(value: &[u8]) -> (&[u8], Vec<Param>) {
    let mut parts = segments(value).into_iter();
    let main = parts.next().unwrap_or_default();
    (main, parts.filter_map(param).collect())
}

/// Splits `value` at each `;` outside quotes. There is always at least one segment.
fn segments(value: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    for (i, &c) in value.iter().enumerate() {
        match c {
            b'"' => in_quotes = !in_quotes,
            b';' if !in_quotes => {
                out.push(&value[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&value[start..]);
    out
}

/// `add_param`: `name=value`. The name keeps its trailing blanks, and the value is unquoted.
fn param(segment: &[u8]) -> Option<Param> {
    let eq = segment.iter().position(|&c| c == b'=')?;
    let start = skip_blanks(&segment[..eq], 0);
    Some(Param {
        name: segment[start..eq].to_vec(),
        value: unquote(&segment[eq + 1..]),
    })
}

/// `init_content_type`: the primary and sub types, split at the first `/`. The sub type keeps any
/// later `/`. Returns `None` when there is no `/`.
pub fn content_type(value: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let slash = value.iter().position(|&c| c == b'/')?;
    Some((value[..slash].to_vec(), value[slash + 1..].to_vec()))
}

/// The transfer encodings that `init_content_encoding` recognizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentEncoding {
    Base64,
    QuotedPrintable,
    SevenBit,
    EightBit,
}

const ENCODINGS: [(&[u8], ContentEncoding); 4] = [
    (b"base64", ContentEncoding::Base64),
    (b"quoted-printable", ContentEncoding::QuotedPrintable),
    (b"7bit", ContentEncoding::SevenBit),
    (b"8bit", ContentEncoding::EightBit),
];

/// `init_content_encoding`: matches `value` ignoring ASCII case, without trimming. Returns `None`
/// for any other value, which leaves the encoding unchanged.
pub fn content_encoding(value: &[u8]) -> Option<ContentEncoding> {
    ENCODINGS
        .iter()
        .find(|(name, _)| value.eq_ignore_ascii_case(name))
        .map(|&(_, encoding)| encoding)
}

fn find_crlf(data: &[u8]) -> Option<usize> {
    data.windows(2).position(|w| w == b"\r\n")
}

fn is_blank(c: u8) -> bool {
    matches!(c, b' ' | b'\t')
}

fn skip_blanks(s: &[u8], mut i: usize) -> usize {
    while i < s.len() && is_blank(s[i]) {
        i += 1;
    }
    i
}
