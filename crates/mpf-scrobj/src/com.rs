//! `HRESULT` values from `include/winerror.h` and the COM methods in `dlls/scrobj/scrobj.c`
//! whose body is a fixed `return`. The C bodies only trace their arguments, so the ports
//! ignore them.

/// `HRESULT` from `include/winnt.h`.
pub type HRESULT = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: HRESULT = 0;

/// `E_FAIL` from `include/winerror.h`.
pub const E_FAIL: HRESULT = 0x8000_4005_u32 as HRESULT;

/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: HRESULT = 0x8000_4001_u32 as HRESULT;

/// `global_GetTypeInfoCount`, in `global_vtbl`.
pub fn global_get_type_info_count() -> HRESULT {
    E_NOTIMPL
}

/// `global_GetTypeInfo`, in `global_vtbl`.
pub fn global_get_type_info() -> HRESULT {
    E_NOTIMPL
}

/// `global_GetDispID`, in `global_vtbl`.
pub fn global_get_disp_id() -> HRESULT {
    E_NOTIMPL
}

/// `global_InvokeEx`, in `global_vtbl`.
pub fn global_invoke_ex() -> HRESULT {
    E_NOTIMPL
}

/// `global_DeleteMemberByName`, in `global_vtbl`.
pub fn global_delete_member_by_name() -> HRESULT {
    E_NOTIMPL
}

/// `global_DeleteMemberByDispID`, in `global_vtbl`.
pub fn global_delete_member_by_disp_id() -> HRESULT {
    E_NOTIMPL
}

/// `global_GetMemberProperties`, in `global_vtbl`.
pub fn global_get_member_properties() -> HRESULT {
    E_NOTIMPL
}

/// `global_GetMemberName`, in `global_vtbl`.
pub fn global_get_member_name() -> HRESULT {
    E_NOTIMPL
}

/// `global_GetNextDispID`, in `global_vtbl`.
pub fn global_get_next_disp_id() -> HRESULT {
    E_NOTIMPL
}

/// `global_GetNameSpaceParent`, in `global_vtbl`.
pub fn global_get_name_space_parent() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_GetTypeInfoCount`, in `DispatchExVtbl`.
pub fn scriptlet_get_type_info_count() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_GetTypeInfo`, in `DispatchExVtbl`.
pub fn scriptlet_get_type_info() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_DeleteMemberByName`, in `DispatchExVtbl`.
pub fn scriptlet_delete_member_by_name() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_DeleteMemberByDispID`, in `DispatchExVtbl`.
pub fn scriptlet_delete_member_by_disp_id() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_GetMemberProperties`, in `DispatchExVtbl`.
pub fn scriptlet_get_member_properties() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_GetMemberName`, in `DispatchExVtbl`.
pub fn scriptlet_get_member_name() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_GetNextDispID`, in `DispatchExVtbl`.
pub fn scriptlet_get_next_disp_id() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_GetNameSpaceParent`, in `DispatchExVtbl`.
pub fn scriptlet_get_name_space_parent() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_AddURL`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_add_url() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_put_Path`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_put_path() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_get_Path`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_get_path() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_put_Doc`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_put_doc() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_get_Doc`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_get_doc() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_put_Name`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_put_name() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_get_Name`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_get_name() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_put_MajorVersion`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_put_major_version() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_get_MajorVersion`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_get_major_version() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_put_MinorVersion`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_put_minor_version() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_get_MinorVersion`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_get_minor_version() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_Write`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_write() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_Reset`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_reset() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_typelib_put_GUID`, in `scriptlet_typelib_vtbl`.
pub fn scriptlet_typelib_put_guid() -> HRESULT {
    E_NOTIMPL
}

/// `scriptlet_factory_LockServer`, in `scriptlet_factory_vtbl`. The lock flag is traced and ignored.
pub fn scriptlet_factory_lock_server(_lock: bool) -> HRESULT {
    S_OK
}
