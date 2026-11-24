//! OID syntax check from Wine's `is_valid_oid` in `dlls/cryptui/main.c`.

/// Accepts a dotted OID whose first arc is 0, 1 or 2 and that has at least one
/// further arc of ASCII digits. The input ends at the first NUL byte, as the C
/// string does.
pub fn is_valid_oid(oid: &[u8]) -> bool {
    let oid = match oid.iter().position(|&byte| byte == 0) {
        Some(end) => &oid[..end],
        None => oid,
    };
    match oid {
        [b'0' | b'1' | b'2', b'.', arcs @ ..] => arcs
            .split(|&byte| byte == b'.')
            .all(|arc| !arc.is_empty() && arc.iter().all(u8::is_ascii_digit)),
        _ => false,
    }
}
