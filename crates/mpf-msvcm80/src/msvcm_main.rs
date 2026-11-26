//! The `msvcm_main.c` functions behind the `@ cdecl` exports of `msvcm80.spec`.
//!
//! Each C body only writes a `FIXME` trace, so each helper returns without doing anything. The
//! `usize` parameters stand for the C pointers, which the bodies trace and never dereference.

/// `CrtImplementationDetails::DoDllLanguageSupportValidation` in `msvcm_main.c`.
pub fn crt_implementation_details_do_dll_language_support_validation() {}

/// `CrtImplementationDetails::RegisterModuleUninitializer` in `msvcm_main.c`.
pub fn crt_implementation_details_register_module_uninitializer(_handler: usize) {}

/// `__setusermatherr_m` in `msvcm_main.c`.
pub fn setusermatherr_m(_handler: usize) {}

/// `CrtImplementationDetails::ThrowModuleLoadException` in `msvcm_main.c`.
pub fn crt_implementation_details_throw_module_load_exception(_message: usize) {}

/// `CrtImplementationDetails::ThrowModuleLoadException` with an inner exception in `msvcm_main.c`.
pub fn crt_implementation_details_throw_module_load_exception_inner(
    _message: usize,
    _inner: usize,
) {
}

/// `CrtImplementationDetails::ThrowNestedModuleLoadException` in `msvcm_main.c`.
pub fn crt_implementation_details_throw_nested_module_load_exception(
    _inner: usize,
    _nested: usize,
) {
}
