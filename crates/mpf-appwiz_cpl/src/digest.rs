/// The comparison in `sha_check`: the lowercase hex of the SHA-256 `digest` must equal `expected`
/// exactly, so uppercase or truncated text never matches.
pub fn digest_matches(digest: &[u8; 32], expected: &str) -> bool {
    let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    hex == expected
}
