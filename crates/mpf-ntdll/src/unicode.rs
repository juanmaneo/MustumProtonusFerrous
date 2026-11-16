//! RTL unicode strings, mirroring `dlls/ntdll/rtlstr.c` / `unicode.c`.

use crate::ntstatus::{self, NtStatus};

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

/// RtlEqualUnicodeString with IgnoreCase = FALSE.
pub fn rtl_equal_unicode_string(a: &WideString, b: &WideString) -> bool {
    a == b
}

/// RtlCopyUnicodeString: copies `src` into `dst` (truncating at dst's
/// current capacity, like copying into a fixed UNICODE_STRING buffer).
/// Returns STATUS_BUFFER_OVERFLOW when truncated, STATUS_SUCCESS otherwise.
pub fn rtl_copy_unicode_string(dst: &mut WideString, src: &WideString) -> NtStatus {
    let cap = dst.inner.capacity();
    let n = src.len().min(cap);
    dst.inner.clear();
    dst.inner.extend_from_slice(&src.inner[..n]);
    if n < src.len() {
        ntstatus::STATUS_BUFFER_OVERFLOW
    } else {
        ntstatus::STATUS_SUCCESS
    }
}

/// RtlAppendUnicodeStringToString: appends `src` to `dst`.
pub fn rtl_append_unicode_string_to_string(dst: &mut WideString, src: &WideString) -> NtStatus {
    dst.inner.extend_from_slice(src.as_slice());
    ntstatus::STATUS_SUCCESS
}

/// RtlAppendUnicodeToString: appends a single char.
pub fn rtl_append_unicode_to_string(dst: &mut WideString, c: u16) -> NtStatus {
    dst.inner.push(c);
    ntstatus::STATUS_SUCCESS
}

/// RtlDuplicateUnicodeString.
pub fn rtl_duplicate_unicode_string(src: &WideString) -> WideString {
    src.clone()
}

/// RtlUpcaseUnicodeChar (ASCII + Latin-1 subset).
pub fn rtl_upcase_unicode_char(c: u16) -> u16 {
    match c {
        0x61..=0x7A => c - 0x20,
        0xE0..=0xFE if c != 0xF7 => c - 0x20,
        _ => c,
    }
}

/// RtlDowncaseUnicodeChar (ASCII + Latin-1 subset).
pub fn rtl_downcase_unicode_char(c: u16) -> u16 {
    fold(c)
}

/// RtlUpcaseUnicodeString: returns an uppercased copy.
pub fn rtl_upcase_unicode_string(src: &WideString) -> WideString {
    WideString::from_slice(
        &src.inner
            .iter()
            .map(|&c| rtl_upcase_unicode_char(c))
            .collect::<Vec<_>>(),
    )
}

/// RtlUnicodeStringToAnsiString: maps each UTF-16 unit to one byte
/// (units above 0xFF become b'?'), like the ANSI code page truncation.
pub fn rtl_unicode_string_to_ansi_string(src: &WideString) -> Vec<u8> {
    src.inner
        .iter()
        .map(|&c| if c <= 0xFF { c as u8 } else { b'?' })
        .collect()
}

/// RtlAnsiStringToUnicodeString.
pub fn rtl_ansi_string_to_unicode_string(bytes: &[u8]) -> WideString {
    WideString::from_slice(&bytes.iter().map(|&b| b as u16).collect::<Vec<_>>())
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

    #[test]
    fn copy_append_duplicate() {
        let src = WideString::from_utf8("hello");
        let mut dst = WideString::from_utf8("XXXXXXXXXX"); // capacity >= 10
        assert_eq!(
            rtl_copy_unicode_string(&mut dst, &src),
            ntstatus::STATUS_SUCCESS
        );
        assert_eq!(dst, src);

        let mut small = WideString::new();
        small.inner.reserve_exact(2);
        assert_eq!(
            rtl_copy_unicode_string(&mut small, &src),
            ntstatus::STATUS_BUFFER_OVERFLOW
        );
        assert_eq!(small.len(), 2);

        let mut dst = WideString::from_utf8("ab");
        assert_eq!(
            rtl_append_unicode_string_to_string(&mut dst, &WideString::from_utf8("cd")),
            ntstatus::STATUS_SUCCESS
        );
        assert_eq!(dst.to_string_lossy(), "abcd");
        assert_eq!(
            rtl_append_unicode_to_string(&mut dst, b'e' as u16),
            ntstatus::STATUS_SUCCESS
        );
        assert_eq!(dst.to_string_lossy(), "abcde");
        assert_eq!(rtl_duplicate_unicode_string(&src), src);
    }

    #[test]
    fn case_conversion_and_ansi_roundtrip() {
        assert_eq!(rtl_upcase_unicode_char(b'a' as u16), b'A' as u16);
        assert_eq!(rtl_downcase_unicode_char(b'Z' as u16), b'z' as u16);
        assert_eq!(rtl_upcase_unicode_char(0xE9), 0xC9); // é -> É
        let s = WideString::from_utf8("KERNEL32.Dll");
        assert_eq!(
            rtl_upcase_unicode_string(&s).to_string_lossy(),
            "KERNEL32.DLL"
        );
        assert!(rtl_equal_unicode_string(
            &s,
            &WideString::from_utf8("KERNEL32.Dll")
        ));
        assert!(!rtl_equal_unicode_string(
            &s,
            &WideString::from_utf8("kernel32.dll")
        ));

        let wide = WideString::from_utf8("abc");
        let ansi = rtl_unicode_string_to_ansi_string(&wide);
        assert_eq!(ansi, b"abc");
        assert_eq!(rtl_ansi_string_to_unicode_string(&ansi), wide);
        // Non-Latin-1 unit truncates to '?'.
        let wide2 = WideString::from_slice(&[0x20AC]); // €
        assert_eq!(rtl_unicode_string_to_ansi_string(&wide2), b"?");
    }
}
