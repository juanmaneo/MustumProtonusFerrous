/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: u32 = 0x8000_4001;

/// `CreateHrtfApo` from `dlls/hrtfapo/main.c`. The C body only emits a `FIXME` and returns
/// `E_NOTIMPL`. It never reads `init` or writes through `xapo`, so the port accepts any pointer
/// shape for them and ignores both.
pub fn create_hrtf_apo<Init, Slot>(_init: Option<&Init>, _xapo: Option<&mut Slot>) -> u32 {
    E_NOTIMPL
}
