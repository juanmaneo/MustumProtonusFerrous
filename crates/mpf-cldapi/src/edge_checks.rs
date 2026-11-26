//! Edge-case tests for the ported cldapi logic.
//!
//! These cover the empty and absent out-slots, the `CF_PLATFORM_INFO` layout, and
//! the `@ stub` export table. Expectations that the oracle does not print are
//! marked with a comment saying so.

use super::*;

#[test]
fn placeholder_info_accepts_empty_buffer() {
    // Not oracle-derived: the harness never passes a zero-length slice. The C body never
    // reads buf, so this follows from C03 (non-null buf with len 0).
    let mut buf: [u8; 0] = [];
    let mut retlen = 5u32;
    assert_eq!(
        cf_get_placeholder_info(Some(&mut buf[..]), Some(&mut retlen)),
        E_NOTIMPL
    );
    assert_eq!(retlen, 5);
}

#[test]
fn placeholder_info_with_no_out_slots() {
    // Oracle-derived: C01 and C10 pass a null buf and a null retlen.
    assert_eq!(cf_get_placeholder_info(None, None), E_NOTIMPL);
}

#[test]
fn platform_info_with_null_pointer() {
    // Oracle-derived: P01 passes a null info.
    assert_eq!(cf_get_platform_info(None), E_NOTIMPL);
}

#[test]
fn platform_info_layout_is_three_dwords() {
    // Not oracle-derived: the harness does not print sizeof. From include/cfapi.h,
    // CF_PLATFORM_INFO is three DWORDs with 4-byte alignment.
    assert_eq!(std::mem::size_of::<CfPlatformInfo>(), 12);
    assert_eq!(std::mem::align_of::<CfPlatformInfo>(), 4);
}

#[test]
fn stub_raise_keeps_the_strings_it_is_given() {
    // Not oracle-derived: the harness passes string literals. The C body stores the
    // pointers it receives, so the port keeps the same references.
    let module = String::from("cldapi.dll");
    let function = String::from("CfExecute");
    let raise = wine_spec_unimplemented_stub(&module, &function);
    assert!(std::ptr::eq(raise.module, module.as_str()));
    assert!(std::ptr::eq(raise.function, function.as_str()));
}

#[test]
fn stub_raise_with_empty_function_name() {
    // Not oracle-derived: no harness row passes an empty name. The C body copies the
    // pointer as it is, so the empty name passes through unchanged.
    let raise = wine_spec_unimplemented_stub(MODULE_NAME, "");
    assert_eq!(raise.code, EXCEPTION_WINE_STUB);
    assert_eq!(raise.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(raise.function, "");
}

#[test]
fn stub_exports_are_unique_and_exclude_implemented_exports() {
    // Not oracle-derived: checks the table against cldapi.spec. The two stdcall exports
    // are implemented above and must not appear as stubs.
    for (i, name) in STUB_EXPORTS.iter().enumerate() {
        assert!(name.starts_with("Cf"));
        assert_ne!(*name, "CfGetPlaceholderInfo");
        assert_ne!(*name, "CfGetPlatformInfo");
        for other in &STUB_EXPORTS[i + 1..] {
            assert_ne!(name, other);
        }
    }
}
