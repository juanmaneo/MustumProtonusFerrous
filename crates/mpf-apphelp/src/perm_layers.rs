use crate::StubResult;

/// `SetPermLayerState`: TRUE with `ERROR_CALL_NOT_IMPLEMENTED`. Its arguments only reach `FIXME`.
pub fn set_perm_layer_state() -> StubResult<bool> {
    StubResult::not_implemented(true)
}

/// `SdbSetPermLayerKeys`: TRUE with `ERROR_CALL_NOT_IMPLEMENTED`. Its arguments only reach `FIXME`.
pub fn sdb_set_perm_layer_keys() -> StubResult<bool> {
    StubResult::not_implemented(true)
}

/// `SdbGetPermLayerKeys`: TRUE with `ERROR_CALL_NOT_IMPLEMENTED`. It leaves the `layers` buffer
/// and the `size` out-pointer untouched.
pub fn sdb_get_perm_layer_keys() -> StubResult<bool> {
    StubResult::not_implemented(true)
}
