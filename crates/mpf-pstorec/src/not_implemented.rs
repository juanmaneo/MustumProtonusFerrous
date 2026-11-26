/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: u32 = 0x8000_4001;

// Each function below is the `PStore_fn*` method of the same name in `dlls/pstorec/pstorec.c`:
// a FIXME trace, then `E_NOTIMPL`. The `This` pointer and the arguments are never read, so the
// port takes none.

/// `PStore_fnGetInfo`.
pub fn get_info() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnGetProvParam`.
pub fn get_prov_param() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnSetProvParam`.
pub fn set_prov_param() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnCreateType`.
pub fn create_type() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnGetTypeInfo`.
pub fn get_type_info() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnDeleteType`.
pub fn delete_type() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnCreateSubtype`.
pub fn create_subtype() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnGetSubtypeInfo`.
pub fn get_subtype_info() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnDeleteSubtype`.
pub fn delete_subtype() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnReadAccessRuleset`.
pub fn read_access_ruleset() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnWriteAccessRuleset`.
pub fn write_access_ruleset() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnEnumTypes`.
pub fn enum_types() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnEnumSubtypes`.
pub fn enum_subtypes() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnDeleteItem`.
pub fn delete_item() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnReadItem`.
pub fn read_item() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnWriteItem`.
pub fn write_item() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnOpenItem`.
pub fn open_item() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnCloseItem`.
pub fn close_item() -> u32 {
    E_NOTIMPL
}

/// `PStore_fnEnumItems`.
pub fn enum_items() -> u32 {
    E_NOTIMPL
}
