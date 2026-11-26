/// `iswspace` in the C locale: space and `\t` through `\r`.
pub(crate) fn is_space(c: u16) -> bool {
    c == 0x20 || (0x09..=0x0d).contains(&c)
}

/// ASCII-only lowercase, matching the `lstrcmpiW` and `wcsnicmp` shims of the oracle.
pub(crate) fn fold(c: u16) -> u16 {
    match c {
        0x41..=0x5a => c + 0x20,
        _ => c,
    }
}

pub(crate) fn eq_fold(a: &[u16], b: &[u16]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(&x, &y)| fold(x) == fold(y))
}

/// `eq_fold` against an ASCII literal.
pub(crate) fn eq_fold_str(word: &[u16], s: &str) -> bool {
    word.len() == s.len()
        && word
            .iter()
            .zip(s.bytes())
            .all(|(&x, y)| fold(x) == fold(u16::from(y)))
}

/// The part of `s` before its first NUL, as `lstrlenW` sees it.
pub(crate) fn until_nul(s: &[u16]) -> &[u16] {
    let len = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..len]
}
