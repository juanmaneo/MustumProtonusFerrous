//! Edge-case tests for the ported dsuiext logic.
//!
//! These cover NULL arguments, the rule for when `obj` is written, reference-count wrap, the
//! vtable entries the oracle did not call, and the derived `Debug` output. Each expectation that
//! is not oracle-derived is marked `source-read`: it comes from `dsuiext.c`, `dsclient.h`, or the
//! `winecrt0` stub code.

use super::*;
use crate::dll::CLASS_INFO;

/// The untouched-`obj` marker, the same as the one in `oracle_checks`.
fn sentinel() -> Option<Interface> {
    Some(Interface::IClassFactory)
}

#[test]
fn guid_is_equal_compares_every_field() {
    // source-read: IsEqualGUID compares all 16 bytes of the GUID.
    assert!(IID_IUNKNOWN.is_equal(&IID_IUNKNOWN));
    let mut data1 = IID_IUNKNOWN;
    data1.data1 = 1;
    let mut data2 = IID_IUNKNOWN;
    data2.data2 = 1;
    let mut data3 = IID_IUNKNOWN;
    data3.data3 = 1;
    let mut data4 = IID_IUNKNOWN;
    data4.data4[7] = 0x47;
    for other in [data1, data2, data3, data4] {
        assert!(!IID_IUNKNOWN.is_equal(&other));
    }
}

#[test]
fn ids_iid_is_the_clsid() {
    // source-read: dsclient.h defines IID_IDsDisplaySpecifier as CLSID_DsDisplaySpecifier.
    assert!(IID_IDS_DISPLAY_SPECIFIER.is_equal(&CLSID_DS_DISPLAY_SPECIFIER));
    assert!(!IID_IDS_DISPLAY_SPECIFIER.is_equal(&IID_ICLASSFACTORY));
}

#[test]
fn refcount_wraps_like_interlocked() {
    // source-read: InterlockedIncrement and InterlockedDecrement wrap on LONG overflow.
    assert_eq!(RefCount(i32::MAX).add_ref(), 0x8000_0000);
    assert_eq!(RefCount(0).release(), u32::MAX);
}

#[test]
fn dispspec_query_idsdisplayspecifier_succeeds_and_adds_a_reference() {
    // source-read: dispspec_QueryInterface accepts IID_IDsDisplaySpecifier as well as IID_IUnknown.
    let mut spec = DisplaySpec::new();
    let mut out = sentinel();
    let hr = spec.query_interface(Some(&IID_IDS_DISPLAY_SPECIFIER), Some(&mut out));
    assert_eq!(hr, S_OK);
    assert_eq!(out, Some(Interface::IDsDisplaySpecifier));
    assert_eq!(spec.release(), 1);
    assert_eq!(spec.release(), 0);
}

#[test]
fn dispspec_query_with_null_obj_is_invalid_and_takes_no_reference() {
    // source-read: dispspec_QueryInterface checks `!iid || !obj` before AddRef.
    let mut spec = DisplaySpec::new();
    assert_eq!(
        spec.query_interface(Some(&IID_IUNKNOWN), None),
        E_INVALIDARG
    );
    assert_eq!(spec.release(), 0);
}

#[test]
fn dispspec_vtable_stubs_return_fixed_values() {
    // source-read: each dispspec_* entry only logs with FIXME and returns a constant.
    assert_eq!(DisplaySpec::set_server(), E_NOTIMPL);
    assert_eq!(DisplaySpec::set_language_id(), E_NOTIMPL);
    assert_eq!(DisplaySpec::get_display_specifier(), E_NOTIMPL);
    assert_eq!(DisplaySpec::get_icon_location(), E_NOTIMPL);
    assert_eq!(DisplaySpec::get_icon(), 0);
    assert_eq!(DisplaySpec::get_friendly_class_name(), E_NOTIMPL);
    assert_eq!(DisplaySpec::get_friendly_attribute_name(), E_NOTIMPL);
    assert!(!DisplaySpec::is_class_container());
    assert_eq!(DisplaySpec::get_class_creation_info(), E_NOTIMPL);
    assert_eq!(DisplaySpec::enum_class_attributes(), E_NOTIMPL);
    assert_eq!(DisplaySpec::get_attribute_ads_type(), ADSTYPE_INVALID);
}

#[test]
fn factory_query_classfactory_succeeds_and_adds_a_reference() {
    // source-read: factory_QueryInterface accepts IID_IClassFactory as well as IID_IUnknown.
    let mut factory = ClassFactory::new(&CLASS_INFO[0]);
    let mut out = sentinel();
    let hr = factory.query_interface(Some(&IID_ICLASSFACTORY), Some(&mut out));
    assert_eq!(hr, S_OK);
    assert_eq!(out, Some(Interface::IClassFactory));
    assert_eq!(factory.release(), 1);
    assert_eq!(factory.release(), 0);
}

#[test]
fn factory_query_failure_overwrites_obj_with_none() {
    // source-read: factory_QueryInterface sets *obj = NULL before it returns E_NOINTERFACE.
    let mut factory = ClassFactory::new(&CLASS_INFO[0]);
    let mut out = Some(Interface::IDsDisplaySpecifier);
    let hr = factory.query_interface(Some(&IID_IDS_DISPLAY_SPECIFIER), Some(&mut out));
    assert_eq!(hr, E_NOINTERFACE);
    assert_eq!(out, None);
}

#[test]
fn factory_query_with_null_arguments_is_invalid_and_takes_no_reference() {
    // source-read: factory_QueryInterface checks `!iid || !obj` before AddRef.
    let mut factory = ClassFactory::new(&CLASS_INFO[0]);
    assert_eq!(
        factory.query_interface(Some(&IID_IUNKNOWN), None),
        E_INVALIDARG
    );
    assert_eq!(factory.release(), 0);
}

#[test]
fn create_instance_with_null_arguments_is_invalid_and_keeps_obj() {
    // source-read: factory_CreateInstance returns E_INVALIDARG before `*obj = NULL`.
    let factory = ClassFactory::new(&CLASS_INFO[0]);
    let mut out = sentinel();
    assert_eq!(
        factory.create_instance(false, None, Some(&mut out)),
        E_INVALIDARG
    );
    assert_eq!(out, sentinel());
    assert_eq!(
        factory.create_instance(false, Some(&IID_IDS_DISPLAY_SPECIFIER), None),
        E_INVALIDARG
    );
}

#[test]
fn lock_server_is_a_no_op_success() {
    // source-read: factory_LockServer only logs FIXME and returns S_OK.
    assert_eq!(ClassFactory::lock_server(true), S_OK);
    assert_eq!(ClassFactory::lock_server(false), S_OK);
}

#[test]
fn class_info_has_one_row_for_dsdisplayspecifier() {
    // source-read: class_info[] holds { &CLSID_DsDisplaySpecifier, DsDisplaySpecifier_create }.
    assert_eq!(CLASS_INFO.len(), 1);
    assert!(CLASS_INFO[0].clsid.is_equal(&CLSID_DS_DISPLAY_SPECIFIER));
    let mut out = sentinel();
    assert_eq!(
        (CLASS_INFO[0].constructor)(&IID_IDS_DISPLAY_SPECIFIER, &mut out),
        S_OK
    );
    assert_eq!(out, Some(Interface::IDsDisplaySpecifier));
}

#[test]
fn dll_get_class_object_null_iid_or_obj_is_invalid_and_keeps_obj() {
    // source-read: DllGetClassObject checks `!clsid || !iid || !obj` before `*obj = NULL`.
    let mut out = sentinel();
    assert_eq!(
        dll_get_class_object(Some(&CLSID_DS_DISPLAY_SPECIFIER), None, Some(&mut out)),
        E_INVALIDARG
    );
    assert_eq!(out, sentinel());
    assert_eq!(
        dll_get_class_object(
            Some(&CLSID_DS_DISPLAY_SPECIFIER),
            Some(&IID_ICLASSFACTORY),
            None
        ),
        E_INVALIDARG
    );
}

#[test]
fn dll_get_class_object_accepts_iunknown() {
    // source-read: the factory accepts IID_IUnknown, so DllGetClassObject(..., IID_IUnknown) succeeds.
    let mut out = sentinel();
    let hr = dll_get_class_object(
        Some(&CLSID_DS_DISPLAY_SPECIFIER),
        Some(&IID_IUNKNOWN),
        Some(&mut out),
    );
    assert_eq!(hr, S_OK);
    assert_eq!(out, Some(Interface::IClassFactory));
}

#[test]
fn unimplemented_stub_passes_module_and_function_through() {
    // source-read: __wine_spec_unimplemented_stub records both names as the two raised arguments.
    assert_eq!(
        unimplemented_stub("other", "Fn"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            module: "other",
            function: "Fn",
        }
    );
}

#[test]
fn derived_debug_output_names_the_types() {
    // Not oracle-derived: exercises the derived Debug impls that failing assertions print.
    assert!(format!("{:?}", IID_IUNKNOWN).starts_with("Guid {"));
    assert!(format!("{:?}", Interface::IClassFactory).contains("IClassFactory"));
    assert!(format!("{:?}", RefCount(3)).contains('3'));
    assert!(format!("{:?}", unimplemented_stub("dsuiext", "DllInstall")).contains("DllInstall"));
}
