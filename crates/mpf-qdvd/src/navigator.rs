//! The DVD navigator's `QueryInterface` hook and pin table from Wine's `dlls/qdvd/navigator.c`.
//! The filter object itself is built on `strmbase`, which is not ported.

use std::convert::Infallible;

use crate::guid::{is_equal_guid, Guid, IID_IDVDCONTROL2, IID_IDVDINFO2};
use crate::hresult::{Hresult, E_NOINTERFACE};

/// The interfaces `navigator_query_interface` hands out.
#[derive(PartialEq, Eq)]
pub enum NavigatorInterface {
    DvdControl2,
    DvdInfo2,
}

/// `navigator_query_interface`, the `filter_query_interface` hook. `strmbase` calls the hook
/// before its own checks and answers `IUnknown` itself after the hook fails. So `IUnknown` is
/// refused here.
pub fn navigator_query_interface(iid: &Guid) -> Result<NavigatorInterface, Hresult> {
    if is_equal_guid(iid, &IID_IDVDCONTROL2) {
        Ok(NavigatorInterface::DvdControl2)
    } else if is_equal_guid(iid, &IID_IDVDINFO2) {
        Ok(NavigatorInterface::DvdInfo2)
    } else {
        Err(E_NOINTERFACE)
    }
}

/// `navigator_get_pin`, which returns NULL for every index. `Infallible` means `None` is the only
/// value the type can hold.
pub fn navigator_get_pin(_index: u32) -> Option<Infallible> {
    None
}
