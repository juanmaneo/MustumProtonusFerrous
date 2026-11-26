//! Oracle-derived test vectors for the ported appxdeploymentclient logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`,
//! `DllGetActivationFactory`, and the `package.c` functions with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/appxdeploymentclient/` and running the harness. The harness printed 24 logic rows
//! (L01-L24) and 34 stub rows (S01-S34). ASan and UBSan flagged none of them, so 0 vectors are excluded.
//! Expected values are written as the harness printed them: `0` for `S_OK` or a non-error count, and
//! `0x8000_4002`-style codes for `HRESULT`s.

use super::*;

/// `HSTRING` contents as UTF-16 code units, without a terminating NUL.
fn hstring(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// `0` for success, the error code otherwise, as the harness prints it.
fn hr_of<T>(result: Result<T, HRESULT>) -> u32 {
    match result {
        Ok(_) => 0,
        Err(hr) => hr as u32,
    }
}

/// `(DllGetActivationFactory result, static factory refcount)`.
fn activation_factory(class_id: Option<&[u16]>) -> (u32, i32) {
    let mut factory = ActivationFactory::default();
    let hr = dll_get_activation_factory(&mut factory, class_id);
    (hr as u32, factory.ref_count())
}

/// `(factory QueryInterface result, static factory refcount)`.
fn factory_qi(iid: Iid) -> (u32, i32) {
    let mut factory = ActivationFactory::default();
    let result = factory.query_interface(iid);
    (hr_of(result), factory.ref_count())
}

/// `(package manager QueryInterface result, view, refcount)` on a freshly activated object.
fn pm_qi(iid: Iid) -> (u32, Option<View>, i32) {
    let mut pm = activated();
    let result = pm.query_interface(iid);
    (hr_of(result), result.ok(), pm.ref_count())
}

/// `(ActivateInstance result, refcount of the new object)`.
fn activate(alloc_succeeds: bool) -> (u32, Option<i32>) {
    match ActivationFactory::default().activate_instance(alloc_succeeds) {
        Ok(pm) => (0, Some(pm.ref_count())),
        Err(hr) => (hr as u32, None),
    }
}

fn activated() -> PackageManager {
    ActivationFactory::default()
        .activate_instance(true)
        .expect("allocation succeeds")
}

#[test]
fn oracle_l01_dll_get_class_object() {
    assert_eq!(dll_get_class_object() as u32, 0x8004_0111);
}

#[test]
fn oracle_l02_activation_factory_package_manager() {
    let class_id = hstring("Windows.Management.Deployment.PackageManager");
    assert_eq!(
        activation_factory(Some(class_id.as_slice())),
        (0x0000_0000, 2)
    );
}

#[test]
fn oracle_l03_activation_factory_other_class() {
    let class_id = hstring("Windows.Management.Deployment.Other");
    assert_eq!(
        activation_factory(Some(class_id.as_slice())),
        (0x8004_0111, 1)
    );
}

#[test]
fn oracle_l04_activation_factory_null_class() {
    assert_eq!(activation_factory(None), (0x8004_0111, 1));
}

#[test]
fn oracle_l05_activation_factory_nul_terminates_class() {
    let class_id = hstring("Windows.Management.Deployment.PackageManager\0junk");
    assert_eq!(
        activation_factory(Some(class_id.as_slice())),
        (0x0000_0000, 2)
    );
}

#[test]
fn oracle_l06_activation_factory_lowercase_class() {
    let class_id = hstring("windows.management.deployment.packagemanager");
    assert_eq!(
        activation_factory(Some(class_id.as_slice())),
        (0x8004_0111, 1)
    );
}

#[test]
fn oracle_l07_activation_factory_prefix_class() {
    let class_id = hstring("Windows.Management.Deployment.PackageManage");
    assert_eq!(
        activation_factory(Some(class_id.as_slice())),
        (0x8004_0111, 1)
    );
}

#[test]
fn oracle_l08_factory_qi_activation_factory() {
    assert_eq!(factory_qi(Iid::ActivationFactory), (0x0000_0000, 2));
}

#[test]
fn oracle_l09_factory_qi_package_manager() {
    assert_eq!(factory_qi(Iid::PackageManager), (0x8000_4002, 1));
}

#[test]
fn oracle_l10_factory_qi_agile_object() {
    assert_eq!(factory_qi(Iid::AgileObject), (0x8000_4002, 1));
}

#[test]
fn oracle_l11_factory_release_from_one() {
    let mut factory = ActivationFactory::default();
    assert_eq!(factory.release(), 0);
    assert_eq!(factory.ref_count(), 0);
}

#[test]
fn oracle_l12_factory_release_from_zero() {
    let mut factory = ActivationFactory::default();
    factory.release();
    assert_eq!(factory.release(), 0xFFFF_FFFF);
    assert_eq!(factory.ref_count(), -1);
}

#[test]
fn oracle_l13_package_manager_qi_iunknown() {
    assert_eq!(
        pm_qi(Iid::Unknown),
        (0x0000_0000, Some(View::PackageManager), 2)
    );
}

#[test]
fn oracle_l14_package_manager_qi_package_manager2() {
    assert_eq!(
        pm_qi(Iid::PackageManager2),
        (0x0000_0000, Some(View::PackageManager2), 2)
    );
}

#[test]
fn oracle_l15_package_manager_qi_agile_object() {
    assert_eq!(
        pm_qi(Iid::AgileObject),
        (0x0000_0000, Some(View::PackageManager), 2)
    );
}

#[test]
fn oracle_l16_package_manager_qi_activation_factory() {
    assert_eq!(pm_qi(Iid::ActivationFactory), (0x8000_4002, None, 1));
}

#[test]
fn oracle_l17_package_manager_qi_other() {
    assert_eq!(pm_qi(Iid::Other), (0x8000_4002, None, 1));
}

#[test]
fn oracle_l18_package_manager_release_from_one() {
    assert_eq!(activated().release(), 0);
}

#[test]
fn oracle_l19_package_manager_add_ref_from_one() {
    let mut pm = activated();
    assert_eq!(pm.add_ref(), 2);
    assert_eq!(pm.ref_count(), 2);
}

#[test]
fn oracle_l20_activate_instance_allocation_ok() {
    assert_eq!(activate(true), (0x0000_0000, Some(1)));
}

#[test]
fn oracle_l21_activate_instance_allocation_fails() {
    assert_eq!(activate(false), (0x8007_000E, None));
}

#[test]
fn oracle_l22_find_packages_null_name() {
    let publisher = hstring("y");
    assert_eq!(
        find_packages_by_name_publisher(None, Some(publisher.as_slice())) as u32,
        0x8007_0057
    );
}

#[test]
fn oracle_l23_find_packages_null_publisher() {
    let name = hstring("x");
    assert_eq!(
        find_packages_by_name_publisher(Some(name.as_slice()), None) as u32,
        0x8007_0057
    );
}

#[test]
fn oracle_l24_find_packages_not_implemented() {
    let name = hstring("x");
    let publisher = hstring("y");
    assert_eq!(
        find_packages_by_name_publisher(Some(name.as_slice()), Some(publisher.as_slice())) as u32,
        0x8000_4001
    );
}

#[test]
fn oracle_stub_rows_return_e_notimpl() {
    let rows = [
        ("S01", StubExport::PackageManagerGetIids),
        ("S02", StubExport::PackageManagerGetRuntimeClassName),
        ("S03", StubExport::PackageManagerGetTrustLevel),
        ("S04", StubExport::PackageManagerAddPackageAsync),
        ("S05", StubExport::PackageManagerUpdatePackageAsync),
        ("S06", StubExport::PackageManagerRemovePackageAsync),
        ("S07", StubExport::PackageManagerStagePackageAsync),
        ("S08", StubExport::PackageManagerRegisterPackageAsync),
        ("S09", StubExport::PackageManagerFindPackages),
        ("S10", StubExport::PackageManagerFindPackagesByUserSecurityId),
        ("S11", StubExport::PackageManagerFindPackagesByUserSecurityIdNamePublisher),
        ("S12", StubExport::PackageManagerFindUsers),
        ("S13", StubExport::PackageManagerSetPackageState),
        ("S14", StubExport::PackageManagerFindPackageByPackageFullName),
        ("S15", StubExport::PackageManagerCleanupPackageForUserAsync),
        ("S16", StubExport::PackageManagerFindPackagesByPackageFamilyName),
        ("S17", StubExport::PackageManagerFindPackagesByUserSecurityIdPackageFamilyName),
        ("S18", StubExport::PackageManagerFindPackageByUserSecurityIdPackageFullName),
        ("S19", StubExport::PackageManager2GetIids),
        ("S20", StubExport::PackageManager2GetRuntimeClassName),
        ("S21", StubExport::PackageManager2GetTrustLevel),
        ("S22", StubExport::PackageManager2RemovePackageWithOptionsAsync),
        ("S23", StubExport::PackageManager2StagePackageWithOptionsAsync),
        ("S24", StubExport::PackageManager2RegisterPackageByFullNameAsync),
        ("S25", StubExport::PackageManager2FindPackagesWithPackageTypes),
        ("S26", StubExport::PackageManager2FindPackagesByUserSecurityIdWithPackageTypes),
        ("S27", StubExport::PackageManager2FindPackagesByNamePublisherWithPackageTypes),
        ("S28", StubExport::PackageManager2FindPackagesByUserSecurityIdNamePublisherWithPackageTypes),
        ("S29", StubExport::PackageManager2FindPackagesByPackageFamilyNameWithPackageTypes),
        ("S30", StubExport::PackageManager2FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes),
        ("S31", StubExport::PackageManager2StageUserDataAsync),
        ("S32", StubExport::ActivationFactoryGetIids),
        ("S33", StubExport::ActivationFactoryGetRuntimeClassName),
        ("S34", StubExport::ActivationFactoryGetTrustLevel),
    ];
    for (id, stub) in rows {
        assert_eq!(stub.result() as u32, 0x8000_4001, "{id}");
    }
}
