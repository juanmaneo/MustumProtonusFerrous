/// `OLESTATUS` from `dlls/olecli32/olecli.h`.
pub type OleStatus = u32;

/// `OLE_OK`, the first value of `OLESTATUS`.
pub const OLE_OK: OleStatus = 0;

/// The literal `0x10` that `OleQueryType` returns for a NULL object. `olecli.h` numbers 16 as
/// `OLE_ERROR_ADDRESS`, but the C code uses the literal, so the port returns the literal too.
pub const OLE_NULL_OBJECT_STATUS: OleStatus = 0x10;

/// The `OLEOBJECTVTBL` slot that `OleQueryType` calls through the object's `lpvtbl`.
pub trait OleObject {
    /// `OLEOBJECTVTBL.QueryType`: stores the type in `xlong` and returns its status.
    fn query_type(&self, xlong: &mut i32) -> OleStatus;
}

/// `OleQueryType`. A NULL object returns [`OLE_NULL_OBJECT_STATUS`] without touching `xlong`.
/// Otherwise the result is the object's own `QueryType`.
pub fn ole_query_type(oleob: Option<&dyn OleObject>, xlong: &mut i32) -> OleStatus {
    match oleob {
        None => OLE_NULL_OBJECT_STATUS,
        Some(object) => object.query_type(xlong),
    }
}

/// `OleSetHostNames`, which returns `OLE_OK` without reading the object or the host names.
pub fn ole_set_host_names() -> OleStatus {
    OLE_OK
}
