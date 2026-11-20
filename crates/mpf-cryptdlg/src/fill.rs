/// Result of a Wine formatter call, with capacities measured in UTF-16 code units.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fill {
    /// Whether the call succeeded. A size query succeeds without producing text.
    pub success: bool,
    /// Required capacity, including the terminating NUL.
    pub chars_needed: usize,
    /// Text copied to a supplied destination; short buffers may contain a prefix.
    pub text: Option<String>,
}

pub(crate) fn fits(capacity: Option<usize>, chars_needed: usize) -> bool {
    match capacity {
        Some(capacity) => capacity >= chars_needed,
        None => true,
    }
}
