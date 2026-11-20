/// Returns the value of attribute `name` in the tag `node`, as get_attr in stream.c does.
///
/// The name is matched without regard to ASCII case, as the substring `name="`, so it also
/// matches the end of a longer name such as `xname`. The value keeps its case and runs to the
/// next double quote. Returns `None` when the substring is not found or the value has no closing
/// quote.
pub fn attr_value<'a>(node: &'a [u8], name: &[u8]) -> Option<&'a [u8]> {
    let lower_node = node.to_ascii_lowercase();
    let mut needle = name.to_ascii_lowercase();
    needle.extend_from_slice(b"=\"");
    let found = lower_node
        .windows(needle.len())
        .position(|window| window == needle.as_slice())?;
    let start = found + needle.len();
    let len = lower_node[start..].iter().position(|&c| c == b'"')?;
    Some(&node[start..start + len])
}
