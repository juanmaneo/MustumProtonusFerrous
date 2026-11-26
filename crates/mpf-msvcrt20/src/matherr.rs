/// `MSVCRT20__matherr` from `dlls/msvcrt20/msvcrt20.c`, exported as `_matherr`. The C body ignores
/// its `struct _exception *` argument, which may be NULL, and returns 0.
pub fn matherr<E>(_e: *const E) -> i32 {
    0
}
