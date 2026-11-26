use crate::error::Error;
use crate::text::eq_fold;

const PERCENT: u16 = b'%' as u16;

/// One `add_replacement` entry, with key and item already cut at their first NUL.
pub(crate) struct Replacement {
    pub(crate) key: Vec<u16>,
    pub(crate) item: Vec<u16>,
}

/// `do_preprocess`: expands `%NAME%` (case-insensitive) and `%%`. Expanded text is not rescanned.
pub(crate) fn preprocess(data: &[u16], replacements: &[Replacement]) -> Result<Vec<u16>, Error> {
    let mut out = Vec::with_capacity(data.len());
    let mut rest = data;
    while let Some(open) = rest.iter().position(|&c| c == PERCENT) {
        out.extend_from_slice(&rest[..open]);
        let after = &rest[open + 1..];
        let close = after
            .iter()
            .position(|&c| c == PERCENT)
            .ok_or(Error::Exception)?;
        let name = &after[..close];
        if name.is_empty() {
            out.push(PERCENT);
        } else {
            out.extend_from_slice(lookup(replacements, name).ok_or(Error::Exception)?);
        }
        rest = &after[close + 1..];
    }
    out.extend_from_slice(rest);
    Ok(out)
}

/// The newest replacement whose key matches `name`.
fn lookup<'a>(replacements: &'a [Replacement], name: &[u16]) -> Option<&'a [u16]> {
    replacements
        .iter()
        .rev()
        .find(|r| eq_fold(name, &r.key))
        .map(|r| r.item.as_slice())
}
