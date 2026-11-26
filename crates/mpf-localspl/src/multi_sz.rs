/// `multi_sz_lenW`: byte length of a `REG_MULTI_SZ` buffer including its final terminator.
///
/// `None` (C `NULL`) gives 0. Returns `None` when the buffer has no double NUL, where the
/// C loop would read past the end. The C loop ends at the first NUL that follows another NUL,
/// so the first double NUL marks the end of the list.
pub fn multi_sz_len_bytes(s: Option<&[u16]>) -> Option<usize> {
    let Some(s) = s else {
        return Some(0);
    };
    s.windows(2)
        .position(|w| *w == [0, 0])
        .map(|i| (i + 2) * std::mem::size_of::<u16>())
}
