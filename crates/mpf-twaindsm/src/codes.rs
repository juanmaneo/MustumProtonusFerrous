//! TWAIN constants from Wine's `include/twain.h`, as used by `dlls/twaindsm`.

pub const DG_CONTROL: u32 = 0x0001;

pub const DAT_NULL: u16 = 0x0000;
pub const DAT_EVENT: u16 = 0x0002;
pub const DAT_IDENTITY: u16 = 0x0003;
pub const DAT_PARENT: u16 = 0x0004;
pub const DAT_STATUS: u16 = 0x0008;
pub const DAT_USERINTERFACE: u16 = 0x0009;
pub const DAT_CALLBACK: u16 = 0x0010;
pub const DAT_CALLBACK2: u16 = 0x0012;
pub const DAT_ENTRYPOINT: u16 = 0x0403;

pub const MSG_NULL: u16 = 0x0000;
pub const MSG_GET: u16 = 0x0001;
pub const MSG_GETDEFAULT: u16 = 0x0003;
pub const MSG_GETFIRST: u16 = 0x0004;
pub const MSG_GETNEXT: u16 = 0x0005;
pub const MSG_SET: u16 = 0x0006;
pub const MSG_XFERREADY: u16 = 0x0101;
pub const MSG_CLOSEDSREQ: u16 = 0x0102;
pub const MSG_DEVICEEVENT: u16 = 0x0104;
pub const MSG_OPENDSM: u16 = 0x0301;
pub const MSG_CLOSEDSM: u16 = 0x0302;
pub const MSG_OPENDS: u16 = 0x0401;
pub const MSG_CLOSEDS: u16 = 0x0402;
pub const MSG_USERSELECT: u16 = 0x0403;
pub const MSG_ENABLEDS: u16 = 0x0502;
pub const MSG_ENABLEDSUIONLY: u16 = 0x0503;
pub const MSG_PROCESSEVENT: u16 = 0x0601;
pub const MSG_REGISTER_CALLBACK: u16 = 0x0902;

pub const TWRC_SUCCESS: u16 = 0;
pub const TWRC_FAILURE: u16 = 1;
pub const TWRC_CANCEL: u16 = 3;
pub const TWRC_DSEVENT: u16 = 4;
pub const TWRC_NOTDSEVENT: u16 = 5;
pub const TWRC_ENDOFLIST: u16 = 7;

pub const TWCC_SUCCESS: u16 = 0;
pub const TWCC_NODS: u16 = 3;
pub const TWCC_OPERATIONERROR: u16 = 5;
pub const TWCC_BADPROTOCOL: u16 = 9;
pub const TWCC_BADVALUE: u16 = 10;
pub const TWCC_SEQERROR: u16 = 11;
pub const TWCC_BADDEST: u16 = 12;

pub const DF_DSM2: u32 = 0x1000_0000;
pub const DF_APP2: u32 = 0x2000_0000;
pub const DF_DS2: u32 = 0x4000_0000;

/// `sizeof(TW_ENTRYPOINT)` on LP64 (`#pragma pack(2)`: a 4-byte `Size` followed by five
/// 8-byte function pointers). `TWAIN_GetEntrypoint` rejects smaller `Size` values.
pub const TW_ENTRYPOINT_SIZE: u32 = 44;

/// Messages that `TWAIN_ControlNull` queues for the application (`DG_CONTROL/DAT_NULL`).
pub fn is_control_null_msg(msg: u16) -> bool {
    matches!(msg, MSG_CLOSEDSREQ | MSG_DEVICEEVENT | MSG_XFERREADY)
}
