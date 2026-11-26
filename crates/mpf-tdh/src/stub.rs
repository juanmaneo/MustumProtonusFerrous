/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = tdh.dll` in `dlls/tdh/Makefile.in`.
const TDH_DLL_NAME: &str = "tdh.dll";

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

/// `TdhAggregatePayloadFilters`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_aggregate_payload_filters() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhAggregatePayloadFilters")
}

/// `TdhApplyPayloadFilter`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_apply_payload_filter() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhApplyPayloadFilter")
}

/// `TdhCleanupPayloadEventFilterDescriptor`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_cleanup_payload_event_filter_descriptor() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhCleanupPayloadEventFilterDescriptor")
}

/// `TdhCloseDecodingHandle`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_close_decoding_handle() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhCloseDecodingHandle")
}

/// `TdhCreatePayloadFilter`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_create_payload_filter() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhCreatePayloadFilter")
}

/// `TdhDeletePayloadFilter`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_delete_payload_filter() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhDeletePayloadFilter")
}

/// `TdhEnumerateManifestProviderEvents`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_enumerate_manifest_provider_events() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhEnumerateManifestProviderEvents")
}

/// `TdhEnumerateProviderFieldInformation`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_enumerate_provider_field_information() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhEnumerateProviderFieldInformation")
}

/// `TdhEnumerateProviderFilters`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_enumerate_provider_filters() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhEnumerateProviderFilters")
}

/// `TdhEnumerateRemoteWBEMProviderFieldInformation`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_enumerate_remote_wbem_provider_field_information() -> StubException {
    wine_spec_unimplemented_stub(
        TDH_DLL_NAME,
        "TdhEnumerateRemoteWBEMProviderFieldInformation",
    )
}

/// `TdhEnumerateRemoteWBEMProviders`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_enumerate_remote_wbem_providers() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhEnumerateRemoteWBEMProviders")
}

/// `TdhFormatProperty`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_format_property() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhFormatProperty")
}

/// `TdhGetAllEventsInformation`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_all_events_information() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetAllEventsInformation")
}

/// `TdhGetDecodingParameter`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_decoding_parameter() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetDecodingParameter")
}

/// `TdhGetEventInformation`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_event_information() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetEventInformation")
}

/// `TdhGetEventMapInformation`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_event_map_information() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetEventMapInformation")
}

/// `TdhGetManifestEventInformation`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_manifest_event_information() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetManifestEventInformation")
}

/// `TdhGetProperty`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_property() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetProperty")
}

/// `TdhGetPropertyOffsetAndSize`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_property_offset_and_size() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetPropertyOffsetAndSize")
}

/// `TdhGetPropertySize`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_property_size() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetPropertySize")
}

/// `TdhGetWppMessage`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_wpp_message() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetWppMessage")
}

/// `TdhGetWppProperty`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_get_wpp_property() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhGetWppProperty")
}

/// `TdhOpenDecodingHandle`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_open_decoding_handle() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhOpenDecodingHandle")
}

/// `TdhQueryProviderFieldInformation`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_query_provider_field_information() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhQueryProviderFieldInformation")
}

/// `TdhQueryRemoteWBEMProviderFieldInformation`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_query_remote_wbem_provider_field_information() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhQueryRemoteWBEMProviderFieldInformation")
}

/// `TdhSetDecodingParameter`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_set_decoding_parameter() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhSetDecodingParameter")
}

/// `TdhUnloadManifest`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_unload_manifest() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhUnloadManifest")
}

/// `TdhValidatePayloadFilter`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdh_validate_payload_filter() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhValidatePayloadFilter")
}

/// `TdhpFindMatchClassFromWBEM`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdhp_find_match_class_from_wbem() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhpFindMatchClassFromWBEM")
}

/// `TdhpGetBestTraceEventInfoWBEM`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdhp_get_best_trace_event_info_wbem() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhpGetBestTraceEventInfoWBEM")
}

/// `TdhpGetEventMapInfoWBEM`, exported as `@ stub` in `dlls/tdh/tdh.spec`.
pub fn tdhp_get_event_map_info_wbem() -> StubException {
    wine_spec_unimplemented_stub(TDH_DLL_NAME, "TdhpGetEventMapInfoWBEM")
}
