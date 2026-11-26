/// `S_OK` from `include/winerror.h`.
pub const S_OK: i32 = 0;

/// `FALSE` from `include/windef.h`.
pub const FALSE: i32 = 0;

/// `WofIsExternalFile` from `dlls/wofutil/main.c`. The body never reads `path` or `ptr`. It writes
/// `FALSE` to `result`, 0 to `provider` and 0 to `length`, each only when that pointer is non-NULL,
/// and returns `S_OK`.
pub fn wof_is_external_file(
    _path: Option<&[u16]>,
    result: Option<&mut i32>,
    provider: Option<&mut u32>,
    _ptr: usize,
    length: Option<&mut u32>,
) -> i32 {
    if let Some(result) = result {
        *result = FALSE;
    }
    if let Some(provider) = provider {
        *provider = 0;
    }
    if let Some(length) = length {
        *length = 0;
    }
    S_OK
}

/// `WofShouldCompressBinaries` from `dlls/wofutil/main.c`. The body never reads `volume` or
/// writes `alg`, and returns `FALSE`.
pub fn wof_should_compress_binaries(_volume: Option<&[u16]>, _alg: Option<&mut u32>) -> i32 {
    FALSE
}
