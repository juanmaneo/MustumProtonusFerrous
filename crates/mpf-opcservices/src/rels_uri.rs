/// `"/_rels"` as UTF-16 units.
const RELS_DIR: [u16; 6] = [0x2f, 0x5f, 0x72, 0x65, 0x6c, 0x73];
/// `".rels"` as UTF-16 units.
const RELS_EXT: [u16; 5] = [0x2e, 0x72, 0x65, 0x6c, 0x73];
const SLASH: u16 = 0x2f;

/// `opc_part_uri_get_rels_uri`, string core: the relationships part path for the
/// part path `path` (UTF-16 units, no terminating NUL), or `None` when `path`
/// already is a relationships part.
///
/// Bug-compatible: whenever the last `/` is at index 6 or later, the C takes the
/// insertion point six units before that slash without checking for `"/_rels"`.
/// So `/words/x` gives `/_rels/x.rels` and `/abcdef/x` gives `//_rels/x.rels`.
///
/// A path with no `/` also returns `None`. The C copies from a NULL-derived
/// size there, which is undefined; ASan reports `negative-size-param`. Valid part
/// paths start with `/` (inferred from `factory.c`, which combines against `/`),
/// so this input should not reach the function through the public API.
pub fn rels_part_path(path: &[u16]) -> Option<Vec<u16>> {
    let end = path.iter().rposition(|&unit| unit == SLASH)?;
    let start = end.checked_sub(RELS_DIR.len()).unwrap_or(end);
    // The C compares `len > sizeof(".rels")`, which counts the terminating NUL.
    if path.len() > RELS_EXT.len() + 1
        && path.ends_with(&RELS_EXT)
        && path[start..].starts_with(&RELS_DIR)
    {
        return None;
    }
    let mut rels = Vec::with_capacity(path.len() + RELS_DIR.len() + RELS_EXT.len());
    rels.extend_from_slice(&path[..start]);
    rels.extend_from_slice(&RELS_DIR);
    rels.extend_from_slice(&path[end..]);
    rels.extend_from_slice(&RELS_EXT);
    Some(rels)
}
