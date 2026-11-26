//! Interface identifiers that `main.c` and `weakref.c` compare against.

/// The IIDs the port distinguishes. `Other` stands for any IID that no interface here matches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iid {
    IUnknown,
    IInspectable,
    IAgileObject,
    IGeolocator,
    IWeakReference,
    IWeakReferenceSource,
    IActivationFactory,
    Other,
}

/// The base IIDs that `geolocator_QueryInterface` and `factory_QueryInterface` both accept
/// alongside their own interface.
pub fn inspectable_base(iid: Iid) -> bool {
    matches!(iid, Iid::IUnknown | Iid::IInspectable | Iid::IAgileObject)
}
