/// `OLESTATUS` from `dlls/olesvr32/olesvr_main.c`. This DLL only returns `OLE_OK`.
pub type OleStatus = i32;

/// `OLE_OK`, the first member of `OLESTATUS`.
pub const OLE_OK: OleStatus = 0;

/// `FALSE` from `include/windef.h`, the value `OleUnblockServer` stores in `*block`.
const FALSE: i32 = 0;

/// `static LONG OLE_current_handle` in `olesvr_main.c`, shared by `OleRegisterServer` and
/// `OleRegisterServerDoc`. It starts at 0.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HandleCounter {
    pub current_handle: i32,
}

impl HandleCounter {
    /// `++OLE_current_handle`. C leaves signed overflow undefined. The oracle build wraps to
    /// `i32::MIN`, and the port does the same.
    pub fn next_handle(&mut self) -> i32 {
        self.current_handle = self.current_handle.wrapping_add(1);
        self.current_handle
    }
}

/// `OleBlockServer`: traces its handle and returns `OLE_OK`.
pub fn ole_block_server() -> OleStatus {
    OLE_OK
}

/// `OleUnblockServer`: stores `FALSE` in `*block` and returns `OLE_OK`.
pub fn ole_unblock_server(block: &mut i32) -> OleStatus {
    *block = FALSE;
    OLE_OK
}

/// `OleRegisterServer`: stores the next handle in `*hRet` and returns `OLE_OK`.
pub fn ole_register_server(counter: &mut HandleCounter, h_ret: &mut i32) -> OleStatus {
    *h_ret = counter.next_handle();
    OLE_OK
}

/// `OleRegisterServerDoc`: stores the next handle in `*hRet` and returns `OLE_OK`.
pub fn ole_register_server_doc(counter: &mut HandleCounter, h_ret: &mut i32) -> OleStatus {
    *h_ret = counter.next_handle();
    OLE_OK
}

/// `OleRevokeServer`: traces its handle and returns `OLE_OK`.
pub fn ole_revoke_server() -> OleStatus {
    OLE_OK
}

/// `OleRevokeServerDoc`: traces its handle and returns `OLE_OK`.
pub fn ole_revoke_server_doc() -> OleStatus {
    OLE_OK
}

/// `OleRenameServerDoc`: traces its arguments and returns `OLE_OK`.
pub fn ole_rename_server_doc() -> OleStatus {
    OLE_OK
}

/// `OleRevertServerDoc`: traces its handle and returns `OLE_OK`.
pub fn ole_revert_server_doc() -> OleStatus {
    OLE_OK
}

/// `OleSavedServerDoc`: traces its handle and returns `OLE_OK`.
pub fn ole_saved_server_doc() -> OleStatus {
    OLE_OK
}
