/// `DWRITE_CONTAINER_TYPE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerType {
    Unknown,
    Woff,
    Woff2,
}

/// `opentype_analyze_container_type`: the container that the first four bytes of `data` name.
/// Data shorter than four bytes is `Unknown`, and bytes after the signature are not looked at.
pub fn container_type(data: &[u8]) -> ContainerType {
    match data.get(..4) {
        Some(b"wOFF") => ContainerType::Woff,
        Some(b"wOF2") => ContainerType::Woff2,
        _ => ContainerType::Unknown,
    }
}
