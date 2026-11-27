/// `_toupper` from `dlls/msvcrt/ctype.c`. The C body subtracts 0x20 from any `int` and does not
/// test for a lowercase letter; the source marks this `/* sic */`. The subtraction wraps, as the
/// x86 `sub` does, where C leaves the overflow undefined.
pub fn toupper(c: i32) -> i32 {
    c.wrapping_sub(0x20)
}

/// `_tolower` from `dlls/msvcrt/ctype.c`. Like [`toupper`], it adds 0x20 to any `int` (`/* sic */`),
/// and the addition wraps.
pub fn tolower(c: i32) -> i32 {
    c.wrapping_add(0x20)
}
