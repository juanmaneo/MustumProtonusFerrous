//! `apiset_hash` from `tools/winebuild/parser.c`, with `apiset_hash_factor` from `tools/winebuild/build.h`.

/// `apiset_hash_factor`.
pub(crate) const HASH_FACTOR: u32 = 31;

/// `apiset_hash` and `apiset_hash_len`: hashes the part of `name` before its last `-`, and returns the hash
/// with the length of that part. Returns `None` when `name` has no `-`, because Wine's `strrchr(..) - str`
/// then reads past the name.
pub fn apiset_hash(name: &[u8]) -> Option<(u32, usize)> {
    let hash_len = name.iter().rposition(|&byte| byte == b'-')?;
    let hash = name[..hash_len].iter().fold(0u32, |ret, &byte| {
        // `char` is signed on the reference platforms, so each byte is sign-extended before the addition.
        ret.wrapping_mul(HASH_FACTOR)
            .wrapping_add(byte as i8 as i32 as u32)
    });
    Some((hash, hash_len))
}
