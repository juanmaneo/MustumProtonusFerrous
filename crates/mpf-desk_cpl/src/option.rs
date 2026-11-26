//! Port of the value decision in `get_option` and the `IS_OPTION_TRUE` test from
//! `dlls/desk.cpl/main.c`.

/// `IS_OPTION_TRUE`: the character is `y`, `Y`, `t`, `T` or `1`.
pub fn is_option_true(ch: u16) -> bool {
    b"yYtT1".iter().any(|&c| u16::from(c) == ch)
}

/// `get_option` once the registry read is done. `value` is `None` when the read failed, which
/// gives `default_value`. Otherwise only the first character decides, as `buffer[0]` does in C,
/// so an empty string gives `false`.
pub fn option_value(value: Option<&[u16]>, default_value: bool) -> bool {
    match value {
        Some(text) => is_option_true(text.first().copied().unwrap_or(0)),
        None => default_value,
    }
}
