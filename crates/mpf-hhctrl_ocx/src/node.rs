/// Returns the element name of a tag, as get_node_name in stream.c does: the bytes after `<` up to
/// the first whitespace byte or `>`. A tag without either ends at the end of `node`.
pub fn node_name(node: &[u8]) -> &[u8] {
    let body = node.get(1..).unwrap_or_default();
    let end = body
        .iter()
        .position(|&c| c == b'>' || is_c_space(c))
        .unwrap_or(body.len());
    &body[..end]
}

/// The bytes that isspace accepts in the C locale.
fn is_c_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r')
}
