/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = wsnmp32.dll` in `dlls/wsnmp32/Makefile.in`.
pub const WSNMP32_DLL_NAME: &str = "wsnmp32.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// Defines one helper per `@ stub` export, and the table of them in spec order.
macro_rules! stub_exports {
    ($($helper:ident => $export:literal,)*) => {
        $(
            #[doc = concat!("`", $export, "`, exported as `stub` in `dlls/wsnmp32/wsnmp32.spec`.")]
            pub fn $helper() -> StubException {
                wine_spec_unimplemented_stub(WSNMP32_DLL_NAME, $export)
            }
        )*

        /// The `stub` exports of `wsnmp32.spec`, in spec order, with their helpers.
        pub const STUB_EXPORTS: &[(&str, fn() -> StubException)] = &[$(($export, $helper)),*];
    };
}

stub_exports! {
    snmp_get_translate_mode => "SnmpGetTranslateMode",
    snmp_get_retransmit_mode => "SnmpGetRetransmitMode",
    snmp_get_timeout => "SnmpGetTimeout",
    snmp_set_timeout => "SnmpSetTimeout",
    snmp_set_retry => "SnmpSetRetry",
    snmp_get_retry => "SnmpGetRetry",
    snmp_convey_agent_address => "_SnmpConveyAgentAddress@4",
    snmp_set_agent_address => "_SnmpSetAgentAddress@4",
    snmp_get_vendor_info => "SnmpGetVendorInfo",
    snmp_close => "SnmpClose",
    snmp_send_msg => "SnmpSendMsg",
    snmp_recv_msg => "SnmpRecvMsg",
    snmp_register => "SnmpRegister",
    snmp_create_session => "SnmpCreateSession",
    snmp_listen => "SnmpListen",
    snmp_cancel_msg => "SnmpCancelMsg",
    snmp_str_to_entity => "SnmpStrToEntity",
    snmp_entity_to_str => "SnmpEntityToStr",
    snmp_free_entity => "SnmpFreeEntity",
    snmp_set_port => "SnmpSetPort",
    snmp_str_to_context => "SnmpStrToContext",
    snmp_context_to_str => "SnmpContextToStr",
    snmp_free_context => "SnmpFreeContext",
    snmp_create_pdu => "SnmpCreatePdu",
    snmp_get_pdu_data => "SnmpGetPduData",
    snmp_set_pdu_data => "SnmpSetPduData",
    snmp_duplicate_pdu => "SnmpDuplicatePdu",
    snmp_free_pdu => "SnmpFreePdu",
    snmp_create_vbl => "SnmpCreateVbl",
    snmp_duplicate_vbl => "SnmpDuplicateVbl",
    snmp_free_vbl => "SnmpFreeVbl",
    snmp_count_vbl => "SnmpCountVbl",
    snmp_get_vb => "SnmpGetVb",
    snmp_set_vb => "SnmpSetVb",
    snmp_delete_vb => "SnmpDeleteVb",
    snmp_free_descriptor => "SnmpFreeDescriptor",
    snmp_encode_msg => "SnmpEncodeMsg",
    snmp_decode_msg => "SnmpDecodeMsg",
    snmp_str_to_oid => "SnmpStrToOid",
    snmp_oid_to_str => "SnmpOidToStr",
    snmp_oid_copy => "SnmpOidCopy",
    snmp_oid_compare => "SnmpOidCompare",
    snmp_get_last_error => "SnmpGetLastError",
}
