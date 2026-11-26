/// `TRUE` from `include/windef.h`.
pub const TRUE: i32 = 1;

/// `WinUsb_Free`, exported as `@ stdcall` in `dlls/winusb/winusb.spec`. The C body only traces
/// the handle, which is a `PVOID` in `winusb.h`, and returns `TRUE`. Nothing is freed.
pub fn winusb_free<H>(_handle: H) -> i32 {
    TRUE
}
