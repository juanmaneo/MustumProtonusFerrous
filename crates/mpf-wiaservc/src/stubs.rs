/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = wiaservc.dll` in `dlls/wiaservc/Makefile.in`.
pub const WIASERVC_DLL_NAME: &str = "wiaservc.dll";

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

macro_rules! stub_exports {
    ($($helper:ident => $export:literal,)*) => {
        $(
            #[doc = concat!("`", $export, "`, exported as `@ stub` in `dlls/wiaservc/wiaservc.spec`.")]
            pub fn $helper() -> StubException {
                wine_spec_unimplemented_stub(WIASERVC_DLL_NAME, $export)
            }
        )*

        /// The `@ stub` exports of `wiaservc.spec`, in spec order, with their helpers.
        pub const STUB_EXPORTS: &[(&str, fn() -> StubException)] = &[$(($export, $helper)),*];
    };
}

stub_exports! {
    wias_create_child_app_item => "wiasCreateChildAppItem",
    wias_create_drv_item => "wiasCreateDrvItem",
    wias_create_log_instance => "wiasCreateLogInstance",
    wias_create_prop_context => "wiasCreatePropContext",
    wias_debug_error => "wiasDebugError",
    wias_debug_trace => "wiasDebugTrace",
    wias_down_sample_buffer => "wiasDownSampleBuffer",
    wias_format_args => "wiasFormatArgs",
    wias_free_prop_context => "wiasFreePropContext",
    wias_get_changed_value_float => "wiasGetChangedValueFloat",
    wias_get_changed_value_guid => "wiasGetChangedValueGuid",
    wias_get_changed_value_long => "wiasGetChangedValueLong",
    wias_get_changed_value_str => "wiasGetChangedValueStr",
    wias_get_children_contexts => "wiasGetChildrenContexts",
    wias_get_context_from_name => "wiasGetContextFromName",
    wias_get_drv_item => "wiasGetDrvItem",
    wias_get_image_information => "wiasGetImageInformation",
    wias_get_item_type => "wiasGetItemType",
    wias_get_property_attributes => "wiasGetPropertyAttributes",
    wias_get_root_item => "wiasGetRootItem",
    wias_is_prop_changed => "wiasIsPropChanged",
    wias_parse_endorser_string => "wiasParseEndorserString",
    wias_print_debug_hresult => "wiasPrintDebugHResult",
    wias_queue_event => "wiasQueueEvent",
    wias_read_multiple => "wiasReadMultiple",
    wias_read_prop_bin => "wiasReadPropBin",
    wias_read_prop_float => "wiasReadPropFloat",
    wias_read_prop_guid => "wiasReadPropGuid",
    wias_read_prop_long => "wiasReadPropLong",
    wias_read_prop_str => "wiasReadPropStr",
    wias_send_end_of_page => "wiasSendEndOfPage",
    wias_set_item_prop_attribs => "wiasSetItemPropAttribs",
    wias_set_item_prop_names => "wiasSetItemPropNames",
    wias_set_prop_changed => "wiasSetPropChanged",
    wias_set_property_attributes => "wiasSetPropertyAttributes",
    wias_set_valid_flag => "wiasSetValidFlag",
    wias_set_valid_list_float => "wiasSetValidListFloat",
    wias_set_valid_list_guid => "wiasSetValidListGuid",
    wias_set_valid_list_long => "wiasSetValidListLong",
    wias_set_valid_list_str => "wiasSetValidListStr",
    wias_set_valid_range_float => "wiasSetValidRangeFloat",
    wias_set_valid_range_long => "wiasSetValidRangeLong",
    wias_update_scan_rect => "wiasUpdateScanRect",
    wias_update_valid_format => "wiasUpdateValidFormat",
    wias_validate_item_properties => "wiasValidateItemProperties",
    wias_write_buf_to_file => "wiasWriteBufToFile",
    wias_write_multiple => "wiasWriteMultiple",
    wias_write_page_buf_to_file => "wiasWritePageBufToFile",
    wias_write_page_buf_to_stream => "wiasWritePageBufToStream",
    wias_write_prop_bin => "wiasWritePropBin",
    wias_write_prop_float => "wiasWritePropFloat",
    wias_write_prop_guid => "wiasWritePropGuid",
    wias_write_prop_long => "wiasWritePropLong",
    wias_write_prop_str => "wiasWritePropStr",
}
