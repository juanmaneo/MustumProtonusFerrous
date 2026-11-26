/// `MSVCRT20__getmainargs` from `dlls/msvcrt20/msvcrt20.c`. The CRT's `__getmainargs` receives
/// `argc`, `argv` and `envp` unchanged, and `new_mode` by address of a local copy. Writes the CRT
/// makes through that address are dropped when the wrapper returns.
pub fn getmainargs<A, V, E>(
    argc: A,
    argv: V,
    envp: E,
    expand_wildcards: i32,
    new_mode: i32,
    crt_getmainargs: impl FnOnce(A, V, E, i32, &mut i32),
) {
    let mut new_mode = new_mode;
    crt_getmainargs(argc, argv, envp, expand_wildcards, &mut new_mode);
}

/// `MSVCRT20__wgetmainargs` from `dlls/msvcrt20/msvcrt20.c`. It forwards its arguments the same
/// way as [`getmainargs`], to the CRT's `__wgetmainargs`.
pub fn wgetmainargs<A, V, E>(
    argc: A,
    argv: V,
    envp: E,
    expand_wildcards: i32,
    new_mode: i32,
    crt_wgetmainargs: impl FnOnce(A, V, E, i32, &mut i32),
) {
    let mut new_mode = new_mode;
    crt_wgetmainargs(argc, argv, envp, expand_wildcards, &mut new_mode);
}
