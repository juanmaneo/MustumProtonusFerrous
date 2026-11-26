/// `__telemetry_main_invoke_trigger` from `dlls/vcruntime140/misc.c`. The C body only traces with `FIXME`,
/// so there is nothing to port. `hinst` is the `HINSTANCE` handle value.
pub fn __telemetry_main_invoke_trigger(_hinst: usize) {}

/// `__telemetry_main_return_trigger` from `dlls/vcruntime140/misc.c`. The C body only traces with `FIXME`,
/// so there is nothing to port. `hinst` is the `HINSTANCE` handle value.
pub fn __telemetry_main_return_trigger(_hinst: usize) {}

/// `__uncaught_exceptions` from `dlls/vcruntime140/misc.c`. The C code returns the `int` behind
/// `__processing_throw()`, which the caller passes in as `processing_throw`.
pub fn __uncaught_exceptions(processing_throw: &i32) -> i32 {
    *processing_throw
}
