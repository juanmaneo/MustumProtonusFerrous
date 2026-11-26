use crate::guid::{
    is_equal_guid, Guid, IID_CLASS_FACTORY, IID_UNKNOWN, IID_WINDOWS_PARENTAL_CONTROLS,
    IID_WINDOWS_PARENTAL_CONTROLS_CORE,
};

/// The interfaces that `dlls/wpc/wpc.c` compares IIDs against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    Unknown,
    ClassFactory,
    WindowsParentalControlsCore,
    WindowsParentalControls,
}

/// The interface that `riid` names, or `None` for any other IID.
pub fn interface_for_iid(riid: &Guid) -> Option<Interface> {
    if is_equal_guid(riid, &IID_UNKNOWN) {
        Some(Interface::Unknown)
    } else if is_equal_guid(riid, &IID_CLASS_FACTORY) {
        Some(Interface::ClassFactory)
    } else if is_equal_guid(riid, &IID_WINDOWS_PARENTAL_CONTROLS_CORE) {
        Some(Interface::WindowsParentalControlsCore)
    } else if is_equal_guid(riid, &IID_WINDOWS_PARENTAL_CONTROLS) {
        Some(Interface::WindowsParentalControls)
    } else {
        None
    }
}
