//! `dhtml_edit_qi`: which interface pointer a requested IID resolves to.

/// The IIDs `dhtml_edit_qi` and the connection point code compare against.
/// `Other` stands for any IID not listed here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Iid {
    IUnknown,
    IDispatch,
    IDHTMLEdit,
    IOleObject,
    IProvideClassInfo,
    IProvideClassInfo2,
    IPersistStorage,
    IPersistStreamInit,
    IOleControl,
    IViewObject,
    IViewObject2,
    IViewObjectEx,
    IOleWindow,
    IOleInPlaceObject,
    IOleInPlaceObjectWindowless,
    IOleInPlaceActiveObject,
    IConnectionPointContainer,
    IConnectionPoint,
    IDataObject,
    IServiceProvider,
    IClassFactory,
    DHTMLEditEvents,
    Other,
}

/// The `DHTMLEditImpl` member whose address `dhtml_edit_qi` returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    DHTMLEdit,
    OleObject,
    ProvideClassInfo2,
    PersistStorage,
    PersistStreamInit,
    OleControl,
    ViewObjectEx,
    OleInPlaceObjectWindowless,
    OleInPlaceActiveObject,
    ConnectionPointContainer,
    DataObject,
    ServiceProvider,
}

/// `dhtml_edit_qi`. `None` is the `E_NOINTERFACE` path.
pub fn dhtml_edit_qi(iid: Iid) -> Option<Target> {
    use Iid::*;
    Some(match iid {
        IUnknown | IDispatch | IDHTMLEdit => Target::DHTMLEdit,
        IOleObject => Target::OleObject,
        IProvideClassInfo | IProvideClassInfo2 => Target::ProvideClassInfo2,
        IPersistStorage => Target::PersistStorage,
        IPersistStreamInit => Target::PersistStreamInit,
        IOleControl => Target::OleControl,
        IViewObject | IViewObject2 | IViewObjectEx => Target::ViewObjectEx,
        IOleWindow | IOleInPlaceObject | IOleInPlaceObjectWindowless => {
            Target::OleInPlaceObjectWindowless
        }
        IOleInPlaceActiveObject => Target::OleInPlaceActiveObject,
        IConnectionPointContainer => Target::ConnectionPointContainer,
        IDataObject => Target::DataObject,
        IServiceProvider => Target::ServiceProvider,
        IConnectionPoint | IClassFactory | DHTMLEditEvents | Other => return None,
    })
}
