use crate::StubResult;

/// `HANDLE`, the type behind `PDB` and `HSDB`.
pub type Handle = usize;

/// `TAGID`, a `DWORD`.
pub type TagId = u32;

/// `NULL` as a `HANDLE`.
pub const NULL_HANDLE: Handle = 0;

/// `TAGID_NULL` from `apphelp.c`.
pub const TAGID_NULL: TagId = 0;

/// `SdbCreateDatabase`: NULL with `ERROR_CALL_NOT_IMPLEMENTED`. Its path and type only reach `FIXME`.
pub fn sdb_create_database() -> StubResult<Handle> {
    StubResult::not_implemented(NULL_HANDLE)
}

/// `SdbInitDatabase`: NULL with `ERROR_CALL_NOT_IMPLEMENTED`. Its flags and path only reach `FIXME`.
pub fn sdb_init_database() -> StubResult<Handle> {
    StubResult::not_implemented(NULL_HANDLE)
}

/// `SdbOpenDatabase`: NULL with `ERROR_CALL_NOT_IMPLEMENTED`. Its path and type only reach `FIXME`.
pub fn sdb_open_database() -> StubResult<Handle> {
    StubResult::not_implemented(NULL_HANDLE)
}

/// `SdbGetFirstChild`: always `TAGID_NULL`, and it sets no error code.
pub fn sdb_get_first_child() -> TagId {
    TAGID_NULL
}

/// `SdbCloseDatabase`: does nothing.
pub fn sdb_close_database() {}

/// `SdbGetAppPatchDir`: when `size` is non-zero and `path` is non-NULL, writes NUL to `path[0]`.
/// The `hsdb` argument only reaches `FIXME`.
pub fn sdb_get_app_patch_dir(path: Option<&mut u16>, size: u32) {
    if size != 0 {
        if let Some(first) = path {
            *first = 0;
        }
    }
}
