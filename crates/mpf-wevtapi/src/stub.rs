/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = wevtapi.dll` in `dlls/wevtapi/Makefile.in`.
pub const WEVTAPI_DLL_NAME: &str = "wevtapi.dll";

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

/// `EvtArchiveExportedLog`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_archive_exported_log() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtArchiveExportedLog")
}

/// `EvtCancel`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_cancel() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtCancel")
}

/// `EvtClearLog`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_clear_log() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtClearLog")
}

/// `EvtFormatMessage`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_format_message() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtFormatMessage")
}

/// `EvtGetEventInfo`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_get_event_info() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtGetEventInfo")
}

/// `EvtGetEventMetadataProperty`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_get_event_metadata_property() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtGetEventMetadataProperty")
}

/// `EvtGetExtendedStatus`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_get_extended_status() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtGetExtendedStatus")
}

/// `EvtGetLogInfo`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_get_log_info() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtGetLogInfo")
}

/// `EvtGetObjectArrayProperty`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_get_object_array_property() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtGetObjectArrayProperty")
}

/// `EvtGetObjectArraySize`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_get_object_array_size() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtGetObjectArraySize")
}

/// `EvtGetPublisherMetadataProperty`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_get_publisher_metadata_property() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtGetPublisherMetadataProperty")
}

/// `EvtGetQueryInfo`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_get_query_info() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtGetQueryInfo")
}

/// `EvtIntAssertConfig`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_assert_config() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntAssertConfig")
}

/// `EvtIntCreateBinXMLFromCustomXML`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_create_bin_xml_from_custom_xml() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntCreateBinXMLFromCustomXML")
}

/// `EvtIntCreateLocalLogfile`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_create_local_logfile() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntCreateLocalLogfile")
}

/// `EvtIntGetClassicLogDisplayName`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_get_classic_log_display_name() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntGetClassicLogDisplayName")
}

/// `EvtIntRenderResourceEventTemplate`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_render_resource_event_template() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntRenderResourceEventTemplate")
}

/// `EvtIntReportAuthzEventAndSourceAsync`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_report_authz_event_and_source_async() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntReportAuthzEventAndSourceAsync")
}

/// `EvtIntReportEventAndSourceAsync`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_report_event_and_source_async() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntReportEventAndSourceAsync")
}

/// `EvtIntRetractConfig`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_retract_config() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntRetractConfig")
}

/// `EvtIntSysprepCleanup`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_sysprep_cleanup() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntSysprepCleanup")
}

/// `EvtIntWriteXmlEventToLocalLogfile`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_int_write_xml_event_to_local_logfile() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtIntWriteXmlEventToLocalLogfile")
}

/// `EvtNextEventMetadata`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_next_event_metadata() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtNextEventMetadata")
}

/// `EvtNextPublisherId`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_next_publisher_id() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtNextPublisherId")
}

/// `EvtOpenEventMetadataEnum`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_open_event_metadata_enum() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtOpenEventMetadataEnum")
}

/// `EvtOpenPublisherEnum`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_open_publisher_enum() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtOpenPublisherEnum")
}

/// `EvtOpenPublisherMetadata`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_open_publisher_metadata() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtOpenPublisherMetadata")
}

/// `EvtSeek`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_seek() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtSeek")
}

/// `EvtSetObjectArrayProperty`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_set_object_array_property() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtSetObjectArrayProperty")
}

/// `EvtUpdateBookmark`, exported as `@ stub` in `dlls/wevtapi/wevtapi.spec`.
pub fn evt_update_bookmark() -> StubException {
    wine_spec_unimplemented_stub(WEVTAPI_DLL_NAME, "EvtUpdateBookmark")
}
