use crate::object::{OleStatus, OLE_OK};

/// `LHCLIENTDOC`, a `LONG` in `dlls/olecli32/olecli.h`.
pub type LhClientDoc = i32;

/// The `OLE_current_handle` static behind `OleRegisterClientDoc`, as explicit state: the last
/// handle handed out.
#[derive(Debug, Default)]
pub struct ClientDocRegistry {
    pub last_handle: LhClientDoc,
}

/// `OleRegisterClientDoc`. Hands out the next handle and stores it in `hret`. The C `++` on a
/// `LONG` overflows at `LONG_MAX`, which is undefined behaviour in C. The port wraps, which is what
/// the oracle build produced.
pub fn ole_register_client_doc(
    registry: &mut ClientDocRegistry,
    hret: &mut LhClientDoc,
) -> OleStatus {
    registry.last_handle = registry.last_handle.wrapping_add(1);
    *hret = registry.last_handle;
    OLE_OK
}

/// `OleRenameClientDoc`, which returns `OLE_OK` without reading the document or the new name.
pub fn ole_rename_client_doc() -> OleStatus {
    OLE_OK
}

/// `OleRevokeClientDoc`, which returns `OLE_OK` without reading the document.
pub fn ole_revoke_client_doc() -> OleStatus {
    OLE_OK
}

/// `OleSavedClientDoc`, which returns `OLE_OK` without reading the document.
pub fn ole_saved_client_doc() -> OleStatus {
    OLE_OK
}
