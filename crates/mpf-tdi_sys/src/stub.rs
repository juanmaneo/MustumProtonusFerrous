/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = tdi.sys` in `dlls/tdi.sys/Makefile.in`.
const TDI_SYS_DLL_NAME: &str = "tdi.sys";

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

/// `CTEAllocateString`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_allocate_string() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEAllocateString")
}

/// `CTEBlock`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_block() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEBlock")
}

/// `CTEBlockWithTracker`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_block_with_tracker() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEBlockWithTracker")
}

/// `CTEInitEvent`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_init_event() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEInitEvent")
}

/// `CTEInitString`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_init_string() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEInitString")
}

/// `CTEInitTimer`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_init_timer() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEInitTimer")
}

/// `CTEInitialize`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_initialize() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEInitialize")
}

/// `CTEInsertBlockTracker`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_insert_block_tracker() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEInsertBlockTracker")
}

/// `CTELogEvent`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_log_event() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTELogEvent")
}

/// `CTERemoveBlockTracker`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_remove_block_tracker() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTERemoveBlockTracker")
}

/// `CTEScheduleCriticalEvent`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_schedule_critical_event() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEScheduleCriticalEvent")
}

/// `CTEScheduleDelayedEvent`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_schedule_delayed_event() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEScheduleDelayedEvent")
}

/// `CTEScheduleEvent`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_schedule_event() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEScheduleEvent")
}

/// `CTESignal`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_signal() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTESignal")
}

/// `CTEStartTimer`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_start_timer() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTEStartTimer")
}

/// `CTESystemUpTime`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn cte_system_up_time() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "CTESystemUpTime")
}

/// `DllInitialize`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn dll_initialize() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "DllInitialize")
}

/// `DllUnload`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn dll_unload() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "DllUnload")
}

/// `TdiBuildNetbiosAddress`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_build_netbios_address() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiBuildNetbiosAddress")
}

/// `TdiBuildNetbiosAddressEa`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_build_netbios_address_ea() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiBuildNetbiosAddressEa")
}

/// `TdiCopyBufferToMdl`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_copy_buffer_to_mdl() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiCopyBufferToMdl")
}

/// `TdiCopyBufferToMdlWithReservedMappingAtDpcLevel`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_copy_buffer_to_mdl_with_reserved_mapping_at_dpc_level() -> StubException {
    wine_spec_unimplemented_stub(
        TDI_SYS_DLL_NAME,
        "TdiCopyBufferToMdlWithReservedMappingAtDpcLevel",
    )
}

/// `TdiCopyMdlChainToMdlChain`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_copy_mdl_chain_to_mdl_chain() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiCopyMdlChainToMdlChain")
}

/// `TdiCopyMdlToBuffer`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_copy_mdl_to_buffer() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiCopyMdlToBuffer")
}

/// `TdiDefaultChainedRcvDatagramHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_chained_rcv_datagram_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultChainedRcvDatagramHandler")
}

/// `TdiDefaultChainedRcvExpeditedHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_chained_rcv_expedited_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultChainedRcvExpeditedHandler")
}

/// `TdiDefaultChainedReceiveHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_chained_receive_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultChainedReceiveHandler")
}

/// `TdiDefaultConnectHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_connect_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultConnectHandler")
}

/// `TdiDefaultDisconnectHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_disconnect_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultDisconnectHandler")
}

/// `TdiDefaultErrorHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_error_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultErrorHandler")
}

/// `TdiDefaultRcvDatagramHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_rcv_datagram_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultRcvDatagramHandler")
}

/// `TdiDefaultRcvExpeditedHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_rcv_expedited_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultRcvExpeditedHandler")
}

/// `TdiDefaultReceiveHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_receive_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultReceiveHandler")
}

/// `TdiDefaultSendPossibleHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_default_send_possible_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDefaultSendPossibleHandler")
}

/// `TdiDeregisterAddressChangeHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_deregister_address_change_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDeregisterAddressChangeHandler")
}

/// `TdiDeregisterDeviceObject`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_deregister_device_object() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDeregisterDeviceObject")
}

/// `TdiDeregisterNetAddress`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_deregister_net_address() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDeregisterNetAddress")
}

/// `TdiDeregisterNotificationHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_deregister_notification_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDeregisterNotificationHandler")
}

/// `TdiDeregisterPnPHandlers`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_deregister_pnp_handlers() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDeregisterPnPHandlers")
}

/// `TdiDeregisterProvider`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_deregister_provider() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiDeregisterProvider")
}

/// `TdiEnumerateAddresses`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_enumerate_addresses() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiEnumerateAddresses")
}

/// `TdiGet9FTriageBlock`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_get9f_triage_block() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiGet9FTriageBlock")
}

/// `TdiInitialize`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_initialize() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiInitialize")
}

/// `TdiMapUserRequest`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_map_user_request() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiMapUserRequest")
}

/// `TdiMatchPdoWithChainedReceiveContext`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_match_pdo_with_chained_receive_context() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiMatchPdoWithChainedReceiveContext")
}

/// `TdiOpenNetbiosAddress`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_open_netbios_address() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiOpenNetbiosAddress")
}

/// `TdiPnPPowerComplete`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_pnp_power_complete() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiPnPPowerComplete")
}

/// `TdiPnPPowerRequest`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_pnp_power_request() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiPnPPowerRequest")
}

/// `TdiProviderReady`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_provider_ready() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiProviderReady")
}

/// `TdiRegisterAddressChangeHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_register_address_change_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiRegisterAddressChangeHandler")
}

/// `TdiRegisterDeviceObject`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_register_device_object() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiRegisterDeviceObject")
}

/// `TdiRegisterNetAddress`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_register_net_address() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiRegisterNetAddress")
}

/// `TdiRegisterNotificationHandler`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_register_notification_handler() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiRegisterNotificationHandler")
}

/// `TdiRegisterPnPHandlers`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_register_pnp_handlers() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiRegisterPnPHandlers")
}

/// `TdiRegisterProvider`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_register_provider() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiRegisterProvider")
}

/// `TdiReturnChainedReceives`, exported as `@ stub` in `dlls/tdi.sys/tdi.sys.spec`.
pub fn tdi_return_chained_receives() -> StubException {
    wine_spec_unimplemented_stub(TDI_SYS_DLL_NAME, "TdiReturnChainedReceives")
}
