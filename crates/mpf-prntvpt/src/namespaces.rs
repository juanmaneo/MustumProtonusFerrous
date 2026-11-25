/// `set_SelectionNamespaces`: writes each `xmlns` and `xmlns:*` attribute of the root
/// element as `name="value" `, with a trailing space. `attrs` holds the (name, value)
/// pairs in document order. The C code also writes a NUL terminator, which is not part
/// of the returned string.
pub fn selection_namespaces(attrs: &[(&str, &str)]) -> String {
    let mut out = String::new();
    for &(name, value) in attrs {
        if name == "xmlns" || name.starts_with("xmlns:") {
            out.push_str(name);
            out.push_str("=\"");
            out.push_str(value);
            out.push_str("\" ");
        }
    }
    out
}
