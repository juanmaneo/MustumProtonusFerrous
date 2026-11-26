//! The IIDs that `QueryInterface` compares. `dlls/dataexchange` does not contain the GUID
//! bytes, so each interface is named symbolically.

/// An interface identifier. [`Iid::Other`] is any IID the DLL does not implement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iid {
    IUnknown,
    IInspectable,
    IAgileObject,
    IActivationFactory,
    ICoreDragDropManagerStatics,
    IDragDropManagerInterop,
    ICoreDragDropManager,
    Other,
}

/// The interface that `dataexchange_QueryInterface` hands out. The factory, statics and
/// interop pointers all refer to one object with one reference count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FactoryInterface {
    ActivationFactory,
    CoreDragDropManagerStatics,
    DragDropManagerInterop,
}

/// The `IsEqualGUID` chain in `dataexchange_QueryInterface`. `None` is `E_NOINTERFACE`.
pub fn factory_interface(iid: Iid) -> Option<FactoryInterface> {
    match iid {
        Iid::IUnknown | Iid::IInspectable | Iid::IAgileObject | Iid::IActivationFactory => {
            Some(FactoryInterface::ActivationFactory)
        }
        Iid::ICoreDragDropManagerStatics => Some(FactoryInterface::CoreDragDropManagerStatics),
        Iid::IDragDropManagerInterop => Some(FactoryInterface::DragDropManagerInterop),
        Iid::ICoreDragDropManager | Iid::Other => None,
    }
}
