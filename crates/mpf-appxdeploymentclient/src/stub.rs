//! The `E_NOTIMPL` exports of `IPackageManager`, `IPackageManager2`, and `IActivationFactory` in Wine's
//! `dlls/appxdeploymentclient/package.c`. Each body only emits a FIXME and returns `E_NOTIMPL`.

use crate::hresult::{E_NOTIMPL, HRESULT};

/// One stub export. `FindPackagesByNamePublisher` is not listed because its NULL check is ported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StubExport {
    PackageManagerGetIids,
    PackageManagerGetRuntimeClassName,
    PackageManagerGetTrustLevel,
    PackageManagerAddPackageAsync,
    PackageManagerUpdatePackageAsync,
    PackageManagerRemovePackageAsync,
    PackageManagerStagePackageAsync,
    PackageManagerRegisterPackageAsync,
    PackageManagerFindPackages,
    PackageManagerFindPackagesByUserSecurityId,
    PackageManagerFindPackagesByUserSecurityIdNamePublisher,
    PackageManagerFindUsers,
    PackageManagerSetPackageState,
    PackageManagerFindPackageByPackageFullName,
    PackageManagerCleanupPackageForUserAsync,
    PackageManagerFindPackagesByPackageFamilyName,
    PackageManagerFindPackagesByUserSecurityIdPackageFamilyName,
    PackageManagerFindPackageByUserSecurityIdPackageFullName,
    PackageManager2GetIids,
    PackageManager2GetRuntimeClassName,
    PackageManager2GetTrustLevel,
    PackageManager2RemovePackageWithOptionsAsync,
    PackageManager2StagePackageWithOptionsAsync,
    PackageManager2RegisterPackageByFullNameAsync,
    PackageManager2FindPackagesWithPackageTypes,
    PackageManager2FindPackagesByUserSecurityIdWithPackageTypes,
    PackageManager2FindPackagesByNamePublisherWithPackageTypes,
    PackageManager2FindPackagesByUserSecurityIdNamePublisherWithPackageTypes,
    PackageManager2FindPackagesByPackageFamilyNameWithPackageTypes,
    PackageManager2FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes,
    PackageManager2StageUserDataAsync,
    ActivationFactoryGetIids,
    ActivationFactoryGetRuntimeClassName,
    ActivationFactoryGetTrustLevel,
}

impl StubExport {
    /// The value the C body returns.
    pub fn result(self) -> HRESULT {
        E_NOTIMPL
    }
}
