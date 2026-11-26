//! IID dispatch of the `QueryInterface` implementations in `package.c`.

/// The IIDs that `QueryInterface` distinguishes. `Other` stands for any other GUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iid {
    Unknown,
    Inspectable,
    AgileObject,
    PackageManager,
    PackageManager2,
    ActivationFactory,
    Other,
}

/// The interface view of `struct package_manager` that a successful query returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    PackageManager,
    PackageManager2,
}

/// `package_manager_QueryInterface`: `IUnknown`, `IInspectable`, `IAgileObject` and
/// `IPackageManager` select `IPackageManager_iface`; `IPackageManager2` selects `IPackageManager2_iface`.
pub(crate) fn package_manager_view(iid: Iid) -> Option<View> {
    match iid {
        Iid::Unknown | Iid::Inspectable | Iid::AgileObject | Iid::PackageManager => {
            Some(View::PackageManager)
        }
        Iid::PackageManager2 => Some(View::PackageManager2),
        Iid::ActivationFactory | Iid::Other => None,
    }
}

/// `factory_QueryInterface`: `IUnknown`, `IInspectable` and `IActivationFactory` are accepted.
pub(crate) fn factory_accepts(iid: Iid) -> bool {
    matches!(
        iid,
        Iid::Unknown | Iid::Inspectable | Iid::ActivationFactory
    )
}
