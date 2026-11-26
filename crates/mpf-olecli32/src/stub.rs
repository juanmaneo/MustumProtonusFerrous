/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = olecli32.dll` in `dlls/olecli32/Makefile.in`.
const OLECLI32_DLL_NAME: &str = "olecli32.dll";

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

/// `WEP`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn wep() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "WEP")
}

/// `OleDelete`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_delete() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleDelete")
}

/// `OleClone`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_clone() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleClone")
}

/// `OleCopyFromLink`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_copy_from_link() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleCopyFromLink")
}

/// `OleEqual`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_equal() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleEqual")
}

/// `OleCopyToClipboard`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_copy_to_clipboard() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleCopyToClipboard")
}

/// `OleSetTargetDevice`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_set_target_device() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleSetTargetDevice")
}

/// `OleSetBounds`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_set_bounds() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleSetBounds")
}

/// `OleQueryBounds`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_bounds() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQueryBounds")
}

/// `OleDraw`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_draw() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleDraw")
}

/// `OleQueryOpen`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_open() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQueryOpen")
}

/// `OleActivate`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_activate() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleActivate")
}

/// `OleUpdate`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_update() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleUpdate")
}

/// `OleReconnect`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_reconnect() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleReconnect")
}

/// `OleGetLinkUpdateOptions`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_get_link_update_options() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleGetLinkUpdateOptions")
}

/// `OleSetLinkUpdateOptions`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_set_link_update_options() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleSetLinkUpdateOptions")
}

/// `OleEnumFormats`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_enum_formats() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleEnumFormats")
}

/// `OleClose`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_close() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleClose")
}

/// `OleGetData`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_get_data() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleGetData")
}

/// `OleSetData`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_set_data() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleSetData")
}

/// `OleQueryProtocol`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_protocol() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQueryProtocol")
}

/// `OleQueryOutOfDate`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_out_of_date() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQueryOutOfDate")
}

/// `OleObjectConvert`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_object_convert() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleObjectConvert")
}

/// `OleCreateFromTemplate`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_create_from_template() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleCreateFromTemplate")
}

/// `OleQueryReleaseStatus`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_release_status() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQueryReleaseStatus")
}

/// `OleQueryReleaseError`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_release_error() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQueryReleaseError")
}

/// `OleQueryReleaseMethod`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_release_method() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQueryReleaseMethod")
}

/// `OleCreateLinkFromFile`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_create_link_from_file() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleCreateLinkFromFile")
}

/// `OleRelease`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_release() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleRelease")
}

/// `OleRevertClientDoc`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_revert_client_doc() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleRevertClientDoc")
}

/// `OleRename`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_rename() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleRename")
}

/// `OleEnumObjects`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_enum_objects() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleEnumObjects")
}

/// `OleQueryName`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_name() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQueryName")
}

/// `OleSetColorScheme`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_set_color_scheme() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleSetColorScheme")
}

/// `OleRequestData`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_request_data() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleRequestData")
}

/// `OleLockServer`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_lock_server() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleLockServer")
}

/// `OleUnlockServer`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_unlock_server() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleUnlockServer")
}

/// `OleQuerySize`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_size() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQuerySize")
}

/// `OleExecute`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_execute() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleExecute")
}

/// `OleCreateInvisible`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_create_invisible() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleCreateInvisible")
}

/// `OleQueryClientVersion`, exported as `@ stub` in `dlls/olecli32/olecli32.spec`.
pub fn ole_query_client_version() -> StubException {
    wine_spec_unimplemented_stub(OLECLI32_DLL_NAME, "OleQueryClientVersion")
}
