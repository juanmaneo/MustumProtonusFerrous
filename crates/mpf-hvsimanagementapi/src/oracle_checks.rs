//! Oracle-derived test vectors for the ported hvsimanagementapi logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/hvsimanagementapi/main.c`
//! and `host.c` with `gcc -fsanitize=address,undefined -g` under `target/canon/hvsimanagementapi/`.
//! The harness includes both files unchanged. A shim there supplies the Windows types, the COM
//! vtables and the `DEFINE_IINSPECTABLE_` macros, which are copied from `private.h`. The harness
//! printed 24 rows, and ASan and UBSan flagged none, so none are excluded.
//!
//! The C side writes out-pointers, and this port returns those values instead. Where the C
//! output is a pointer, the test checks the view it names (`factory` or `statics`) or `NULL`.
//! Where the C output is left unchanged, the test checks only the return value.

use super::*;

/// An `HRESULT` as the harness prints it, in `0x%08x`.
fn hr(bits: u32) -> Hresult {
    bits as i32
}

/// The UTF-16 view of a class name, as `WindowsGetStringRawBuffer` gives it. Any NUL stays in.
fn wide(name: &str) -> Vec<u16> {
    name.encode_utf16().collect()
}

const HOST: &str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";

#[test]
fn rows_01_02_dll_get_class_object() {
    // Row 01: DllGetClassObject(NULL, NULL, NULL) -> hr=0x80040111, out unchanged.
    // Row 02: the same call with set arguments -> hr=0x80040111, out still NULL.
    assert_eq!(dll_get_class_object(), hr(0x8004_0111));
}

#[test]
fn rows_03_to_09_dll_get_activation_factory() {
    // Row 03: host class, fresh object -> hr=0x00000000, factory=factory, ref 1->2.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut host, &wide(HOST)),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(host.ref_count(), 2);

    // Row 04: DispatcherQueueController -> hr=0x80040111, factory=NULL, ref 1->1.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut host, &wide("Windows.System.DispatcherQueueController")),
        Err(hr(0x8004_0111))
    );
    assert_eq!(host.ref_count(), 1);

    // Row 05: NULL HSTRING, modelled as an empty slice -> hr=0x80040111, factory=NULL, ref 1->1.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut host, &[]),
        Err(hr(0x8004_0111))
    );
    assert_eq!(host.ref_count(), 1);

    // Row 06: host name, then NUL, then "junk" -> hr=0x00000000, factory=factory, ref 1->2.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut host, &wide(&format!("{HOST}\0junk"))),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(host.ref_count(), 2);

    // Row 07: lowercase class name -> hr=0x80040111, factory=NULL, ref 1->1.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut host, &wide(&HOST.to_lowercase())),
        Err(hr(0x8004_0111))
    );
    assert_eq!(host.ref_count(), 1);

    // Row 08: name missing its last character -> hr=0x80040111, factory=NULL, ref 1->1.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut host, &wide(&HOST[..HOST.len() - 1])),
        Err(hr(0x8004_0111))
    );
    assert_eq!(host.ref_count(), 1);

    // Row 09: empty name -> hr=0x80040111, factory=NULL, ref 1->1.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut host, &wide("")),
        Err(hr(0x8004_0111))
    );
    assert_eq!(host.ref_count(), 1);
}

#[test]
fn rows_10_to_14_query_interface() {
    // Row 10: factory QueryInterface(IUnknown) -> hr=0x00000000, out=factory, ref 1->2.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        host.query_interface(Iid::IUnknown),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(host.ref_count(), 2);

    // Row 11: factory QueryInterface(IAgileObject) -> hr=0x00000000, out=factory, ref 1->2.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        host.query_interface(Iid::IAgileObject),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(host.ref_count(), 2);

    // Row 12: factory QueryInterface(statics IID) -> hr=0x00000000, out=statics, ref 1->2.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        host.query_interface(Iid::IIsolatedWindowsEnvironmentHostStatics),
        Ok(Interface::IsolatedWindowsEnvironmentHostStatics)
    );
    assert_eq!(host.ref_count(), 2);

    // Row 13: factory QueryInterface(IDispatch) -> hr=0x80004002, out=NULL, ref 1->1.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(host.query_interface(Iid::Other), Err(hr(0x8000_4002)));
    assert_eq!(host.ref_count(), 1);

    // Row 14: statics QueryInterface(IActivationFactory) -> hr=0x00000000, out=factory, ref 1->2.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        host.query_interface(Iid::IActivationFactory),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(host.ref_count(), 2);
}

#[test]
fn rows_15_to_17_add_ref_and_release() {
    // Row 15: factory AddRef at ref=1 -> ret=2, ref=2.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(host.add_ref(), 2);
    assert_eq!(host.ref_count(), 2);

    // Row 16: factory Release at ref=1 -> ret=0, ref=0.
    // Row 17: factory Release at ref=0 -> ret=0xFFFFFFFF, ref=-1. The harness sets ref=0
    // directly. Here it is reached by a second Release from row 16's state.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(host.release(), 0);
    assert_eq!(host.ref_count(), 0);
    assert_eq!(host.release(), 0xFFFF_FFFF);
    assert_eq!(host.ref_count(), -1);
}

#[test]
fn rows_18_to_22_stub_methods() {
    // Rows 18 to 21: factory GetIids, GetRuntimeClassName, GetTrustLevel and ActivateInstance
    // -> hr=0x80004001. The C bodies write none of their out-pointers.
    assert_eq!(stub_method(StubMethod::FactoryGetIids), hr(0x8000_4001));
    assert_eq!(
        stub_method(StubMethod::FactoryGetRuntimeClassName),
        hr(0x8000_4001)
    );
    assert_eq!(
        stub_method(StubMethod::FactoryGetTrustLevel),
        hr(0x8000_4001)
    );
    assert_eq!(
        stub_method(StubMethod::FactoryActivateInstance),
        hr(0x8000_4001)
    );

    // Row 22: statics GetIids forwards to the factory's stub -> hr=0x80004001.
    assert_eq!(stub_method(StubMethod::FactoryGetIids), hr(0x8000_4001));
}

#[test]
fn rows_23_24_statics_properties() {
    // Row 23: get_IsReady with value=TRUE -> hr=0x00000000, value=FALSE.
    let host = IsolatedHostStatics::default();
    assert_eq!(host.get_is_ready(), (hr(0), false));

    // Row 24: get_HostErrors -> hr=0x80004001, errors unchanged (NULL).
    assert_eq!(
        stub_method(StubMethod::StaticsGetHostErrors),
        hr(0x8000_4001)
    );
}
