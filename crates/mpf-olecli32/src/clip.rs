use crate::object::{OleStatus, OLE_OK};

/// `OleQueryLinkFromClip`, which returns `OLE_OK` without reading the clipboard.
pub fn ole_query_link_from_clip() -> OleStatus {
    OLE_OK
}

/// `OleQueryCreateFromClip`, which returns `OLE_OK` without reading the clipboard.
pub fn ole_query_create_from_clip() -> OleStatus {
    OLE_OK
}

/// `OleCreateLinkFromClip`, which returns `OLE_OK` without creating an object.
pub fn ole_create_link_from_clip() -> OleStatus {
    OLE_OK
}

/// `OleCreateFromClip`, which returns `OLE_OK` without creating an object.
pub fn ole_create_from_clip() -> OleStatus {
    OLE_OK
}
