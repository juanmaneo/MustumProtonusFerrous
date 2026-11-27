//! The `strings` buffer of `struct apiset`, filled by `apiset_add_str` in `tools/winebuild/parser.c`.

/// The string pool. Strings are stored back to back with no separator, so a string can be found inside another one.
#[derive(Debug, Default)]
pub(crate) struct StringPool {
    bytes: Vec<u8>,
}

impl StringPool {
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// `apiset_add_str`: returns the offset of the first occurrence of `needle` in the pool. When there is none,
    /// appends the first `len` bytes of `needle` and returns their offset. Wine searches for the whole `needle`
    /// but appends only `len` bytes, so a `name:value` token that is not in the pool adds only its name.
    pub(crate) fn add(&mut self, needle: &[u8], len: usize) -> usize {
        let found = (0..=self.bytes.len().saturating_sub(needle.len()))
            .find(|&offset| self.bytes[offset..].starts_with(needle));
        if let Some(offset) = found {
            return offset;
        }
        let offset = self.bytes.len();
        self.bytes.extend_from_slice(&needle[..len]);
        offset
    }
}
