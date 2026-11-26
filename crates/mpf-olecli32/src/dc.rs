/// `OBJ_METADC` from `include/wingdi.h`.
pub const OBJ_METADC: u32 = 4;

/// `OleIsDcMeta`, given the `GetObjectType` result for the DC. The OS call stays outside the port.
/// A failed call returns 0, which is not `OBJ_METADC`.
pub fn ole_is_dc_meta(object_type: u32) -> bool {
    object_type == OBJ_METADC
}
