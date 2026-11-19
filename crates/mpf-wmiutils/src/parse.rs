use crate::path::{Key, ObjectPath};
use crate::{
    BACKSLASH, COLON, COMMA, DOT, EQ, SLASH, WBEMPATH_INFO_CIM_COMPLIANT,
    WBEMPATH_INFO_PATH_HAD_SERVER, WBEMPATH_INFO_V2_COMPLIANT, WBEM_E_INVALID_PARAMETER,
};

/// Parses a non-empty, NUL-free path as parse_text in path.c does. The result has
/// no text; the caller stores it.
pub(crate) fn parse_text(text: &[u16]) -> Result<ObjectPath, i32> {
    let mut path = ObjectPath::default();
    let mut pos = 0;
    if text.starts_with(&[BACKSLASH, BACKSLASH]) || text.starts_with(&[SLASH, SLASH]) {
        let body = &text[2..];
        let end = body.iter().position(|&c| is_sep(c)).unwrap_or(body.len());
        path.server = Some(body[..end].to_vec());
        path.flags |= WBEMPATH_INFO_PATH_HAD_SERVER;
        pos = 2 + end;
    }

    let rest = &text[pos..];
    let mut q = pos;
    if rest.iter().any(|&c| is_sep(c)) {
        q = pos + rest.iter().position(|&c| c == COLON).unwrap_or(rest.len());
        path.namespaces = split_namespaces(&text[pos..q]);
    }
    if text.get(q) == Some(&COLON) {
        q += 1;
    }

    let tail = &text[q..];
    let class_len = tail
        .iter()
        .position(|&c| c == DOT || c == EQ)
        .unwrap_or(tail.len());
    path.class = Some(tail[..class_len].to_vec());
    let keys = &tail[class_len..];
    match keys.first() {
        Some(&DOT) => {
            for piece in keys[1..].split(|&c| c == COMMA) {
                path.keys.push(parse_key(piece)?);
            }
        }
        Some(&EQ) => {
            let end = keys.iter().position(|&c| c == COMMA).unwrap_or(keys.len());
            path.keys.push(parse_key(&keys[..end])?);
        }
        _ => {}
    }

    path.flags |= WBEMPATH_INFO_CIM_COMPLIANT | WBEMPATH_INFO_V2_COMPLIANT;
    Ok(path)
}

/// One key piece, `name=value`, without its comma. The name has no white space,
/// and the value does not start with white space.
fn parse_key(piece: &[u16]) -> Result<Key, i32> {
    let eq = piece
        .iter()
        .position(|&c| c == EQ)
        .ok_or(WBEM_E_INVALID_PARAMETER)?;
    let (name, value) = (&piece[..eq], &piece[eq + 1..]);
    let name_has_space = name.iter().any(|&c| is_space(c));
    let value_starts_ok = value.first().is_some_and(|&c| !is_space(c));
    if name_has_space || !value_starts_ok {
        return Err(WBEM_E_INVALID_PARAMETER);
    }
    Ok(Key {
        name: name.to_vec(),
        value: value.to_vec(),
    })
}

/// The namespaces of a region. A leading separator is not a namespace start, so
/// it is dropped; every later separator starts a new one, and an empty segment
/// is kept.
fn split_namespaces(region: &[u16]) -> Vec<Vec<u16>> {
    let body = match region.first() {
        None => return Vec::new(),
        Some(&c) if is_sep(c) => &region[1..],
        Some(_) => region,
    };
    body.split(|&c| is_sep(c)).map(<[u16]>::to_vec).collect()
}

fn is_sep(c: u16) -> bool {
    c == BACKSLASH || c == SLASH
}

fn is_space(c: u16) -> bool {
    matches!(c, 0x09..=0x0d | 0x20)
}
