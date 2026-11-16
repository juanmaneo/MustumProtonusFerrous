//! RTL unicode strings, mirroring `dlls/ntdll/rtlstr.c` / `unicode.c`.

/// Win32 ABI view of a unicode string (`UNICODE_STRING` in winnt.h).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: *mut u16,
}

/// Safe, owned UTF-16 string backing the FFI view above.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WideString {
    inner: Vec<u16>,
}

impl WideString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_utf8(s: &str) -> Self {
        Self {
            inner: s.encode_utf16().collect(),
        }
    }

    pub fn from_slice(s: &[u16]) -> Self {
        Self { inner: s.to_vec() }
    }

    pub fn as_slice(&self) -> &[u16] {
        &self.inner
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Byte length, as stored in `UNICODE_STRING.Length` (excludes terminator).
    pub fn byte_len(&self) -> usize {
        self.inner.len() * 2
    }

    /// Mirrors `RtlInitUnicodeString`: Length excludes the NUL terminator.
    pub fn rtl_init(&self) -> UnicodeString {
        let len = self.byte_len() as u16;
        UnicodeString {
            length: len,
            maximum_length: len.saturating_add(2),
            buffer: self.inner.as_ptr() as *mut u16,
        }
    }

    /// Mutable ABI view, for APIs that write into the buffer.
    pub fn as_unicode_string_mut(&mut self) -> UnicodeString {
        let len = self.byte_len() as u16;
        UnicodeString {
            length: len,
            maximum_length: len.saturating_add(2),
            buffer: self.inner.as_mut_ptr(),
        }
    }

    /// Mirrors `RtlCompareUnicodeString`: compares the common prefix, then
    /// falls back to length comparison. Returns <0, 0, >0.
    pub fn compare(&self, other: &Self) -> i32 {
        let n = self.inner.len().min(other.inner.len());
        for i in 0..n {
            let (a, b) = (self.inner[i], other.inner[i]);
            if a != b {
                return a as i32 - b as i32;
            }
        }
        self.inner.len() as i32 - other.inner.len() as i32
    }

    /// Case-insensitive equality (ASCII + Latin-1 fold), like
    /// `RtlEqualUnicodeString` with `IgnoreCase = TRUE` for the BMP subset
    /// we currently support.
    pub fn equals_ignore_case(&self, other: &Self) -> bool {
        if self.inner.len() != other.inner.len() {
            return false;
        }
        self.inner
            .iter()
            .zip(other.inner.iter())
            .all(|(a, b)| fold(*a) == fold(*b))
    }

    pub fn to_string_lossy(&self) -> String {
        self.inner
            .iter()
            .filter_map(|&c| char::from_u32(c as u32))
            .collect()
    }
}

fn fold(c: u16) -> u16 {
    match c {
        0x41..=0x5A => c + 0x20,              // A-Z -> a-z
        0xC0..=0xDE if c != 0xD7 => c + 0x20, // Latin-1 upper (excl. multiplication sign)
        _ => c,
    }
}

pub fn rtl_compare_unicode_strings(a: &WideString, b: &WideString) -> i32 {
    a.compare(b)
}

pub fn rtl_equal_unicode_string_ignore_case(a: &WideString, b: &WideString) -> bool {
    a.equals_ignore_case(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rtl_init_lengths() {
        let s = WideString::from_utf8("abc");
        let u = s.rtl_init();
        assert_eq!(u.length, 6);
        assert_eq!(u.maximum_length, 8);
        assert!(!u.buffer.is_null());
    }

    #[test]
    fn compare_orders() {
        let a = WideString::from_utf8("abc");
        let b = WideString::from_utf8("abd");
        let c = WideString::from_utf8("ab");
        assert!(a.compare(&b) < 0);
        assert!(b.compare(&a) > 0);
        assert_eq!(a.compare(&a), 0);
        assert!(c.compare(&a) < 0);
    }

    #[test]
    fn ignore_case() {
        let a = WideString::from_utf8("Kernel32.DLL");
        let b = WideString::from_utf8("kernel32.dll");
        assert!(a.equals_ignore_case(&b));
        assert!(!a.equals_ignore_case(&WideString::from_utf8("kernel33.dll")));
    }

    #[test]
    fn utf8_roundtrip() {
        let s = WideString::from_utf8("héllo");
        assert_eq!(s.to_string_lossy(), "héllo");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn constructors_and_mutable_abi_view() {
        let empty = WideString::new();
        assert!(empty.is_empty());
        assert_eq!(empty.as_slice(), &[]);
        assert_eq!(empty.len(), 0);
        assert_eq!(empty.byte_len(), 0);

        let mut s = WideString::from_slice(&[b'A' as u16, 0xD800]);
        assert!(!s.is_empty());
        assert_eq!(s.as_slice(), &[b'A' as u16, 0xD800]);
        let view = s.as_unicode_string_mut();
        assert_eq!(view.length, 4);
        assert_eq!(view.maximum_length, 6);
        assert!(!view.buffer.is_null());
        assert_eq!(s.to_string_lossy(), "A");
    }

    #[test]
    fn comparisons_and_case_folding_cover_boundaries() {
        let empty = WideString::new();
        assert_eq!(empty.compare(&empty), 0);
        assert!(WideString::from_utf8("b").compare(&WideString::from_utf8("a")) > 0);
        assert!(WideString::from_utf8("é").equals_ignore_case(&WideString::from_utf8("É")));
        assert!(
            WideString::from_slice(&[0xD7]).equals_ignore_case(&WideString::from_slice(&[0xD7]))
        );
        assert!(
            !WideString::from_utf8("same").equals_ignore_case(&WideString::from_utf8("different"))
        );

        let a = WideString::from_utf8("same");
        let b = WideString::from_utf8("SAME");
        assert_eq!(rtl_compare_unicode_strings(&a, &a), 0);
        assert!(rtl_compare_unicode_strings(&a, &b) != 0);
        assert!(rtl_equal_unicode_string_ignore_case(&a, &b));
        assert!(!rtl_equal_unicode_string_ignore_case(
            &a,
            &WideString::new()
        ));
    }

    #[test]
    fn abi_maximum_length_saturates() {
        let s = WideString::from_slice(&vec![b'x' as u16; 32_767]);
        let view = s.rtl_init();
        assert_eq!(view.length, 65_534);
        assert_eq!(view.maximum_length, u16::MAX);
    }
}
