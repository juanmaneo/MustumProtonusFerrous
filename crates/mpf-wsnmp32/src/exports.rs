/// How `dlls/wsnmp32/wsnmp32.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `stdcall`: a C function in `wsnmp32.c`, ported as a free function in [`crate::calls`].
    Stdcall,
    /// `stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

use ExportKind::{Stdcall, Stub};

/// The `wsnmp32.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 48] = [
    ("SnmpGetTranslateMode", Stub),
    ("SnmpSetTranslateMode", Stdcall),
    ("SnmpGetRetransmitMode", Stub),
    ("SnmpSetRetransmitMode", Stdcall),
    ("SnmpGetTimeout", Stub),
    ("SnmpSetTimeout", Stub),
    ("SnmpSetRetry", Stub),
    ("SnmpGetRetry", Stub),
    ("_SnmpConveyAgentAddress@4", Stub),
    ("_SnmpSetAgentAddress@4", Stub),
    ("SnmpGetVendorInfo", Stub),
    ("SnmpStartup", Stdcall),
    ("SnmpCleanup", Stdcall),
    ("SnmpOpen", Stdcall),
    ("SnmpClose", Stub),
    ("SnmpSendMsg", Stub),
    ("SnmpRecvMsg", Stub),
    ("SnmpRegister", Stub),
    ("SnmpCreateSession", Stub),
    ("SnmpListen", Stub),
    ("SnmpCancelMsg", Stub),
    ("SnmpStrToEntity", Stub),
    ("SnmpEntityToStr", Stub),
    ("SnmpFreeEntity", Stub),
    ("SnmpSetPort", Stub),
    ("SnmpStrToContext", Stub),
    ("SnmpContextToStr", Stub),
    ("SnmpFreeContext", Stub),
    ("SnmpCreatePdu", Stub),
    ("SnmpGetPduData", Stub),
    ("SnmpSetPduData", Stub),
    ("SnmpDuplicatePdu", Stub),
    ("SnmpFreePdu", Stub),
    ("SnmpCreateVbl", Stub),
    ("SnmpDuplicateVbl", Stub),
    ("SnmpFreeVbl", Stub),
    ("SnmpCountVbl", Stub),
    ("SnmpGetVb", Stub),
    ("SnmpSetVb", Stub),
    ("SnmpDeleteVb", Stub),
    ("SnmpFreeDescriptor", Stub),
    ("SnmpEncodeMsg", Stub),
    ("SnmpDecodeMsg", Stub),
    ("SnmpStrToOid", Stub),
    ("SnmpOidToStr", Stub),
    ("SnmpOidCopy", Stub),
    ("SnmpOidCompare", Stub),
    ("SnmpGetLastError", Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
