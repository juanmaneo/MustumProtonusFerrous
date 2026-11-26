/// `ERROR_SUCCESS` from `winerror.h`.
pub const ERROR_SUCCESS: u32 = 0;

/// `ERROR_OUTOFMEMORY` from `winerror.h`.
pub const ERROR_OUTOFMEMORY: u32 = 14;

/// `strdupAW` from `loadperf_main.c`. A NULL `s` gives `Ok(None)`. For any other `s`, `convert`
/// makes the wide copy, and a `None` from it (the `HeapAlloc` failing) gives
/// `Err(ERROR_OUTOFMEMORY)`.
pub fn strdup_aw(
    s: Option<&[u8]>,
    convert: impl FnOnce(&[u8]) -> Option<Vec<u16>>,
) -> Result<Option<Vec<u16>>, u32> {
    match s {
        None => Ok(None),
        Some(s) => convert(s).map(Some).ok_or(ERROR_OUTOFMEMORY),
    }
}

/// `InstallPerfDllW`: logs a `FIXME` and returns `ERROR_SUCCESS` without reading its arguments.
pub fn install_perf_dll_w(_computer: Option<&[u16]>, _ini: Option<&[u16]>, _flags: usize) -> u32 {
    ERROR_SUCCESS
}

/// `InstallPerfDllA`: converts `computer`, then `ini`. A failed conversion returns
/// `ERROR_OUTOFMEMORY` without calling `InstallPerfDllW`.
pub fn install_perf_dll_a(
    computer: Option<&[u8]>,
    ini: Option<&[u8]>,
    flags: usize,
    mut convert: impl FnMut(&[u8]) -> Option<Vec<u16>>,
) -> u32 {
    let computer_w = match strdup_aw(computer, &mut convert) {
        Ok(computer_w) => computer_w,
        Err(ret) => return ret,
    };
    let ini_w = match strdup_aw(ini, &mut convert) {
        Ok(ini_w) => ini_w,
        Err(ret) => return ret,
    };
    install_perf_dll_w(computer_w.as_deref(), ini_w.as_deref(), flags)
}

/// `LoadPerfCounterTextStringsW`: logs a `FIXME` and returns `ERROR_SUCCESS` without reading its
/// arguments.
pub fn load_perf_counter_text_strings_w(_cmdline: Option<&[u16]>, _quiet: bool) -> u32 {
    ERROR_SUCCESS
}

/// `LoadPerfCounterTextStringsA`: converts `cmdline` and calls
/// [`load_perf_counter_text_strings_w`]. A failed conversion returns `ERROR_OUTOFMEMORY`.
pub fn load_perf_counter_text_strings_a(
    cmdline: Option<&[u8]>,
    quiet: bool,
    convert: impl FnOnce(&[u8]) -> Option<Vec<u16>>,
) -> u32 {
    match strdup_aw(cmdline, convert) {
        Ok(cmdline_w) => load_perf_counter_text_strings_w(cmdline_w.as_deref(), quiet),
        Err(ret) => ret,
    }
}

/// `UnloadPerfCounterTextStringsW`: logs a `FIXME` and returns `ERROR_SUCCESS` without reading its
/// arguments.
pub fn unload_perf_counter_text_strings_w(_cmdline: Option<&[u16]>, _quiet: bool) -> u32 {
    ERROR_SUCCESS
}

/// `UnloadPerfCounterTextStringsA`: converts `cmdline` and calls
/// [`unload_perf_counter_text_strings_w`]. A failed conversion returns `ERROR_OUTOFMEMORY`.
pub fn unload_perf_counter_text_strings_a(
    cmdline: Option<&[u8]>,
    quiet: bool,
    convert: impl FnOnce(&[u8]) -> Option<Vec<u16>>,
) -> u32 {
    match strdup_aw(cmdline, convert) {
        Ok(cmdline_w) => unload_perf_counter_text_strings_w(cmdline_w.as_deref(), quiet),
        Err(ret) => ret,
    }
}
