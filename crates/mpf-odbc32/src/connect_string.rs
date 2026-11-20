//! Connection-string parsing, lookup and rebuilding (`parse_connect_string`, `get_attribute` and `build_connect_string`).

use crate::wide::{before_nul, eq_ignore_case};

const EQUALS: u16 = b'=' as u16;
const SEMICOLON: u16 = b';' as u16;
const OPEN_BRACE: u16 = b'{' as u16;
const CLOSE_BRACE: u16 = b'}' as u16;

/// One `name=value` pair of a connection string, both as UTF-16 code units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: Vec<u16>,
    pub value: Vec<u16>,
}

/// Failure of [`parse_connect_string`]. The C code returns `SQL_ERROR` in this case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// A value opened with `{` has no closing `}`.
    UnterminatedBrace,
}

/// Splits a connection string as `parse_connect_string` does.
///
/// The input is read up to its first NUL. A name runs to the first `=`, even across `;`. A value is either
/// `{...}`, which may contain `;`, or runs to the next `;`. The unit after a closing `}` is skipped without
/// being checked, as in the C code. A closing `}` at the very end of the input ends the parse. The C code
/// reads past the NUL there, which is undefined behaviour. Parsing stops quietly at a segment with no `=`
/// or an empty name and keeps the pairs found so far.
pub fn parse_connect_string(input: &[u16]) -> Result<Vec<Attribute>, ParseError> {
    let s = before_nul(input);
    let mut attrs = Vec::new();
    let mut p = 0;
    while p < s.len() {
        let Some(eq) = s[p..].iter().position(|&unit| unit == EQUALS) else {
            break;
        };
        let q = p + eq;
        if q == p {
            break;
        }
        let name = &s[p..q];
        let mut v = q + 1;
        let (value, next) = if s.get(v) == Some(&OPEN_BRACE) {
            v += 1;
            let Some(close) = s[v..].iter().position(|&unit| unit == CLOSE_BRACE) else {
                return Err(ParseError::UnterminatedBrace);
            };
            (&s[v..v + close], v + close + 2)
        } else {
            match s[v..].iter().position(|&unit| unit == SEMICOLON) {
                Some(semi) => (&s[v..v + semi], v + semi + 1),
                None => (&s[v..], s.len()),
            }
        };
        attrs.push(Attribute {
            name: name.to_vec(),
            value: value.to_vec(),
        });
        p = next;
    }
    Ok(attrs)
}

/// Returns the value of the first attribute whose name matches `name` without regard to ASCII case.
pub fn get_attribute<'a>(attrs: &'a [Attribute], name: &[u16]) -> Option<&'a [u16]> {
    attrs
        .iter()
        .find(|attr| eq_ignore_case(&attr.name, name))
        .map(|attr| attr.value.as_slice())
}

/// Joins attributes into `name=value;name=value` as `build_connect_string` does. Values are not escaped.
/// The result has no NUL terminator.
pub fn build_connect_string(attrs: &[Attribute]) -> Vec<u16> {
    let mut out = Vec::new();
    for (i, attr) in attrs.iter().enumerate() {
        if i > 0 {
            out.push(SEMICOLON);
        }
        out.extend_from_slice(&attr.name);
        out.push(EQUALS);
        out.extend_from_slice(&attr.value);
    }
    out
}
