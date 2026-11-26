//! Edge-case tests for the ported wmilib.sys logic.
//!
//! These cover the constants pinned from the Wine headers and the stub record's pass-through of
//! its arguments. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h, include/winnt.h, include/ntstatus.h
    // and the MODULE line of dlls/wmilib.sys/Makefile.in.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(STATUS_SUCCESS, 0);
    assert_eq!(MODULE_NAME, "wmilib.sys");
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the 5 export names of wmilib.sys.spec. The
    // code, flags and parameter count do not depend on the names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn stub_helpers_use_the_module_constant() {
    // Not oracle-derived: every helper passes MODULE_NAME, so the module field follows it.
    assert_eq!(dll_initialize().module, MODULE_NAME);
    assert_eq!(dll_unload().module, MODULE_NAME);
    assert_eq!(wmi_complete_request().module, MODULE_NAME);
    assert_eq!(wmi_fire_event().module, MODULE_NAME);
    assert_eq!(wmi_system_control().module, MODULE_NAME);
}
