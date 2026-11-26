/// `BOOL` from `include/windef.h`: a 32-bit signed integer.
pub type Bool = i32;

/// `TRUE` from `include/windef.h`.
pub const TRUE: Bool = 1;

/// `FALSE` from `include/windef.h`.
pub const FALSE: Bool = 0;

/// A window handle. `HWND` is a pointer in C; the bodies never read it, so the port treats it as
/// an opaque integer.
pub type Hwnd = usize;

/// `WINNLS32GetEnableStatus` from `dlls/winnls32/winnls.c`: always `FALSE`. The window is never
/// read.
pub fn winnls32_get_enable_status(_hwnd: Hwnd) -> Bool {
    FALSE
}

/// `WINNLS32EnableIME` from `dlls/winnls32/winnls.c`: returns `!fEnable`, so `TRUE` only when
/// `enable` is `FALSE`. The C comment calls this a fake return of the previous status. The window
/// is never read.
pub fn winnls32_enable_ime(_hwnd: Hwnd, enable: Bool) -> Bool {
    Bool::from(enable == FALSE)
}
