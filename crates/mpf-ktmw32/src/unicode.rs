/// The `UNICODE_STRING` that `RtlInitUnicodeString` leaves behind. `length` and `maximum_length`
/// are in bytes. `buffer` is the source string, or `None` for a NULL source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnicodeString<'a> {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: Option<&'a [u16]>,
}

/// `RtlInitUnicodeString` from `dlls/ntdll/rtlstr.c`, which `CreateTransaction` calls on its
/// description. The length stops at the first NUL, as `wcslen` does, and is capped at 0xfffc
/// bytes. C reads a source without a NUL past its end. The port stops at the end of the slice.
pub fn init_unicode_string(source: Option<&[u16]>) -> UnicodeString<'_> {
    match source {
        Some(buffer) => {
            let chars = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
            let length = (chars * 2).min(0xfffc) as u16;
            UnicodeString {
                length,
                maximum_length: length + 2,
                buffer: Some(buffer),
            }
        }
        None => UnicodeString {
            length: 0,
            maximum_length: 0,
            buffer: None,
        },
    }
}
