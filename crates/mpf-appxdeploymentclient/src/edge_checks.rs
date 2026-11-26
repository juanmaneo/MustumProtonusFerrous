//! Edge-case tests for the ported appxdeploymentclient logic.
//!
//! These cover the IID tables, class-id matching boundaries, reference counts that wrap below zero,
//! and the `SUCCEEDED` boundaries. Expectations that are not oracle rows say so in a comment.

use super::*;
use crate::query::{factory_accepts, package_manager_view};

fn class_id(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// `(DllGetActivationFactory result, static factory refcount)`.
fn activation_factory(id: Option<&[u16]>) -> (HRESULT, i32) {
    let mut factory = ActivationFactory::default();
    let hr = dll_get_activation_factory(&mut factory, id);
    (hr, factory.ref_count())
}

fn activated() -> PackageManager {
    ActivationFactory::default()
        .activate_instance(true)
        .expect("allocation succeeds")
}

#[test]
fn package_manager_view_table() {
    // Not oracle rows: Inspectable and IPackageManager. The oracle covers Unknown, AgileObject,
    // PackageManager2, ActivationFactory and Other.
    assert_eq!(
        package_manager_view(Iid::Unknown),
        Some(View::PackageManager)
    );
    assert_eq!(
        package_manager_view(Iid::Inspectable),
        Some(View::PackageManager)
    );
    assert_eq!(
        package_manager_view(Iid::AgileObject),
        Some(View::PackageManager)
    );
    assert_eq!(
        package_manager_view(Iid::PackageManager),
        Some(View::PackageManager)
    );
    assert_eq!(
        package_manager_view(Iid::PackageManager2),
        Some(View::PackageManager2)
    );
    assert_eq!(package_manager_view(Iid::ActivationFactory), None);
    assert_eq!(package_manager_view(Iid::Other), None);
}

#[test]
fn factory_accepts_table() {
    // Not oracle rows: Unknown, Inspectable, PackageManager2 and Other on the factory. The oracle covers
    // ActivationFactory (accepted), and PackageManager and AgileObject (rejected).
    assert!(factory_accepts(Iid::Unknown));
    assert!(factory_accepts(Iid::Inspectable));
    assert!(factory_accepts(Iid::ActivationFactory));
    assert!(!factory_accepts(Iid::AgileObject));
    assert!(!factory_accepts(Iid::PackageManager));
    assert!(!factory_accepts(Iid::PackageManager2));
    assert!(!factory_accepts(Iid::Other));
}

#[test]
fn factory_query_interface_accepted_iids_add_refs() {
    // Not oracle rows: the oracle queries the factory once with IActivationFactory.
    let mut factory = ActivationFactory::default();
    assert_eq!(factory.query_interface(Iid::Unknown), Ok(()));
    assert_eq!(factory.query_interface(Iid::Inspectable), Ok(()));
    assert_eq!(factory.ref_count(), 3);
}

#[test]
fn package_manager_views_share_one_count() {
    // Not oracle rows: the oracle queries each fresh object once.
    let mut pm = activated();
    assert_eq!(
        pm.query_interface(Iid::PackageManager2),
        Ok(View::PackageManager2)
    );
    assert_eq!(
        pm.query_interface(Iid::Inspectable),
        Ok(View::PackageManager)
    );
    assert_eq!(
        pm.query_interface(Iid::PackageManager),
        Ok(View::PackageManager)
    );
    assert_eq!(pm.ref_count(), 4);
    assert_eq!(pm.release(), 3);
}

#[test]
fn class_id_edges() {
    // Not oracle rows: none of these class ids is an oracle input.
    let empty: Vec<u16> = Vec::new();
    assert_eq!(
        activation_factory(Some(empty.as_slice())),
        (CLASS_E_CLASSNOTAVAILABLE, 1)
    );

    let leading_nul = class_id("\0Windows.Management.Deployment.PackageManager");
    assert_eq!(
        activation_factory(Some(leading_nul.as_slice())),
        (CLASS_E_CLASSNOTAVAILABLE, 1)
    );

    let trailing_space = class_id("Windows.Management.Deployment.PackageManager ");
    assert_eq!(
        activation_factory(Some(trailing_space.as_slice())),
        (CLASS_E_CLASSNOTAVAILABLE, 1)
    );

    let double_nul = class_id("Windows.Management.Deployment.PackageManager\0\0junk");
    assert_eq!(activation_factory(Some(double_nul.as_slice())), (S_OK, 2));
}

#[test]
fn factory_count_wraps_below_zero_and_back() {
    // Not oracle rows: the oracle stops at -1 (L12).
    let mut factory = ActivationFactory::default();
    assert_eq!(factory.release(), 0);
    assert_eq!(factory.release(), 0xFFFF_FFFF);
    assert_eq!(factory.release(), 0xFFFF_FFFE);
    assert_eq!(factory.add_ref(), 0xFFFF_FFFF);
    assert_eq!(factory.add_ref(), 0);
    assert_eq!(factory.ref_count(), 0);
}

#[test]
fn repeated_activation_factory_accumulates_refs() {
    // Not oracle rows: the oracle calls DllGetActivationFactory once per reset.
    let mut factory = ActivationFactory::default();
    let id = class_id(PACKAGE_MANAGER_CLASS);
    for _ in 0..3 {
        assert_eq!(
            dll_get_activation_factory(&mut factory, Some(id.as_slice())),
            S_OK
        );
    }
    assert_eq!(factory.ref_count(), 4);
}

#[test]
fn activated_objects_have_independent_counts() {
    // Not oracle rows: the oracle activates one object per check.
    let factory = ActivationFactory::default();
    let mut first = factory
        .activate_instance(true)
        .expect("allocation succeeds");
    let second = factory
        .activate_instance(true)
        .expect("allocation succeeds");
    assert_eq!(first.add_ref(), 2);
    assert_eq!(second.ref_count(), 1);
    assert_eq!(factory.ref_count(), 1);
}

#[test]
fn failed_activation_leaves_factory_count_unchanged() {
    // Not oracle rows: the oracle checks the allocation failure's result, not the factory count.
    let factory = ActivationFactory::default();
    assert_eq!(factory.activate_instance(false).err(), Some(E_OUTOFMEMORY));
    assert_eq!(factory.ref_count(), 1);
}

#[test]
fn find_packages_both_null() {
    // Not oracle rows: L22 and L23 each pass one NULL argument.
    assert_eq!(find_packages_by_name_publisher(None, None), E_INVALIDARG);
}

#[test]
fn succeeded_boundaries() {
    // Not oracle rows: the oracle only observes the HRESULTs the harness produced.
    assert!(succeeded(0));
    assert!(succeeded(i32::MAX));
    assert!(!succeeded(-1));
    assert!(!succeeded(i32::MIN));
    assert!(!succeeded(E_NOTIMPL));
}
