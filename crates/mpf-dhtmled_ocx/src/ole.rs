//! Fixed return paths of the OLE, view, connection point and service provider exports in `edit.c`.

use crate::hresult::{HResult, CONNECT_E_NOCONNECTION, E_NOINTERFACE, E_NOTIMPL, E_POINTER, S_OK};
use crate::qi::Iid;

pub const OLEIVERB_INPLACEACTIVATE: i32 = -5;
pub const VIEWSTATUS_OPAQUE: u32 = 1;
pub const VIEWSTATUS_SOLIDBKGND: u32 = 2;

/// `OleObject_DoVerb`. INPLACEACTIVATE also calls `OnShowWindow(TRUE)` on the client site; that
/// call is COM and is not modelled here.
pub fn ole_object_do_verb(verb: i32) -> HResult {
    if verb == OLEIVERB_INPLACEACTIVATE {
        S_OK
    } else {
        E_NOTIMPL
    }
}

/// `ViewObjectEx_GetViewStatus`.
pub fn view_object_ex_get_view_status() -> u32 {
    VIEWSTATUS_OPAQUE | VIEWSTATUS_SOLIDBKGND
}

/// `ConnectionPointContainer_FindConnectionPoint`. `point_is_null` is true when the caller passed
/// a NULL out pointer. The AddRef on the returned point is not modelled.
pub fn connection_point_container_find_connection_point(point_is_null: bool, riid: Iid) -> HResult {
    if point_is_null {
        return E_POINTER;
    }
    if riid == Iid::DHTMLEditEvents {
        S_OK
    } else {
        CONNECT_E_NOCONNECTION
    }
}

/// `ConnectionPoint_QueryInterface`.
pub fn connection_point_query_interface(iid: Iid) -> HResult {
    match iid {
        Iid::IUnknown | Iid::IConnectionPoint => S_OK,
        _ => E_NOINTERFACE,
    }
}

/// `ServiceProvider_QueryService`. No service is provided.
pub fn service_provider_query_service() -> HResult {
    E_NOINTERFACE
}
