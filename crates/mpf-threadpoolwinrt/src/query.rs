//! IID dispatch of the `QueryInterface` implementations in Wine's `dlls/threadpoolwinrt/main.c`.

/// The IIDs that `QueryInterface` distinguishes. `Other` stands for any other GUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iid {
    Unknown,
    Inspectable,
    AgileObject,
    ActivationFactory,
    ThreadPoolStatics,
    AsyncAction,
    AsyncInfo,
    Other,
}

/// The interface view of the static factory that a successful query returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FactoryView {
    ActivationFactory,
    ThreadPoolStatics,
}

/// The interface view of an async action that a successful query returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionView {
    AsyncAction,
    AsyncInfo,
}

/// `threadpool_factory_QueryInterface`: `IUnknown`, `IInspectable`, `IAgileObject` and
/// `IActivationFactory` select the factory view, and `IThreadPoolStatics` selects the statics view.
/// `threadpool_statics_QueryInterface` forwards to the same table.
pub(crate) fn factory_view(iid: Iid) -> Option<FactoryView> {
    match iid {
        Iid::Unknown | Iid::Inspectable | Iid::AgileObject | Iid::ActivationFactory => {
            Some(FactoryView::ActivationFactory)
        }
        Iid::ThreadPoolStatics => Some(FactoryView::ThreadPoolStatics),
        Iid::AsyncAction | Iid::AsyncInfo | Iid::Other => None,
    }
}

/// `async_action_QueryInterface`: `IAsyncAction`, `IInspectable` and `IUnknown` select the action
/// view, and `IAsyncInfo` selects the info view. `IAgileObject` is not accepted here.
pub(crate) fn action_view(iid: Iid) -> Option<ActionView> {
    match iid {
        Iid::AsyncAction | Iid::Inspectable | Iid::Unknown => Some(ActionView::AsyncAction),
        Iid::AsyncInfo => Some(ActionView::AsyncInfo),
        Iid::AgileObject | Iid::ActivationFactory | Iid::ThreadPoolStatics | Iid::Other => None,
    }
}
