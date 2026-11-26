use crate::codes::*;

/// The `DG_CONTROL` operations that `TWAIN_SourceManagerHandler` routes to the Source Manager
/// itself (no destination source).
#[derive(Debug, PartialEq)]
pub enum SourceManagerOp {
    CloseDs,
    GetDefault,
    GetFirst,
    GetNext,
    OpenDs,
    UserSelect,
    CloseDsm,
    OpenDsm,
    GetEntrypoint,
    GetDsmStatus,
}

/// Maps `DAT`/`MSG` to a Source Manager operation. Any other pair is `TWCC_BADPROTOCOL`.
pub fn source_manager_op(dat: u16, msg: u16) -> Option<SourceManagerOp> {
    match (dat, msg) {
        (DAT_IDENTITY, MSG_CLOSEDS) => Some(SourceManagerOp::CloseDs),
        (DAT_IDENTITY, MSG_GETDEFAULT) => Some(SourceManagerOp::GetDefault),
        (DAT_IDENTITY, MSG_GETFIRST) => Some(SourceManagerOp::GetFirst),
        (DAT_IDENTITY, MSG_GETNEXT) => Some(SourceManagerOp::GetNext),
        (DAT_IDENTITY, MSG_OPENDS) => Some(SourceManagerOp::OpenDs),
        (DAT_IDENTITY, MSG_USERSELECT) => Some(SourceManagerOp::UserSelect),
        (DAT_PARENT, MSG_CLOSEDSM) => Some(SourceManagerOp::CloseDsm),
        (DAT_PARENT, MSG_OPENDSM) => Some(SourceManagerOp::OpenDsm),
        (DAT_ENTRYPOINT, MSG_GET) => Some(SourceManagerOp::GetEntrypoint),
        (DAT_STATUS, MSG_GET) => Some(SourceManagerOp::GetDsmStatus),
        _ => None,
    }
}
