/// `INVALID_TRACEID` from `include/rtutils.h`.
pub const INVALID_TRACEID: u32 = 0xFFFF_FFFF;

/// `ERROR_OUTOFMEMORY` from `include/winerror.h`.
pub const ERROR_OUTOFMEMORY: u32 = 14;

/// What `TraceRegisterExA` returns: the id, and the last error it set (`None` when it set none).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceRegistration {
    pub id: u32,
    pub last_error: Option<u32>,
}

/// `TraceRegisterExW` from `dlls/rtutils/tracing.c`. The C body is a stub: it traces its
/// arguments and returns `INVALID_TRACEID`.
pub fn trace_register_ex_w(_name: Option<&[u16]>, _flags: u32) -> u32 {
    INVALID_TRACEID
}

/// `TraceRegisterExA` from `dlls/rtutils/tracing.c`. It converts `name` to UTF-16 in a buffer of
/// the size `alloc` is asked for (`alloc` returns whether the allocation succeeded), calls
/// [`trace_register_ex_w`], and frees the buffer. When the allocation fails it sets
/// `ERROR_OUTOFMEMORY` and returns `INVALID_TRACEID`.
pub fn trace_register_ex_a(
    name: Option<&[u8]>,
    flags: u32,
    alloc: impl FnOnce(usize) -> bool,
) -> TraceRegistration {
    let name_w = ansi_to_wide(name);
    if !alloc(name_w.len() * std::mem::size_of::<u16>()) {
        return TraceRegistration {
            id: INVALID_TRACEID,
            last_error: Some(ERROR_OUTOFMEMORY),
        };
    }
    TraceRegistration {
        id: trace_register_ex_w(Some(name_w.as_slice()), flags),
        last_error: None,
    }
}

/// `MultiByteToWideChar(CP_ACP, 0, name, -1, ...)`. The conversion is modelled as one UTF-16 unit
/// per byte, up to the first NUL, plus the terminating NUL. A NULL `name` converts to nothing.
fn ansi_to_wide(name: Option<&[u8]>) -> Vec<u16> {
    match name {
        None => Vec::new(),
        Some(name) => name
            .iter()
            .take_while(|&&b| b != 0)
            .map(|&b| u16::from(b))
            .chain(std::iter::once(0))
            .collect(),
    }
}
