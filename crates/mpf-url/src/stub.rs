/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = url.dll` in `dlls/url/Makefile.in`.
const URL_DLL_NAME: &str = "url.dll";

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

/// `AutodialHookCallback`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn autodial_hook_callback() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "AutodialHookCallback")
}

/// `DummyEntryPoint`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn dummy_entry_point() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "DummyEntryPoint")
}

/// `DummyEntryPointA`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn dummy_entry_point_a() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "DummyEntryPointA")
}

/// `MIMEAssociationDialogA`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn mime_association_dialog_a() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "MIMEAssociationDialogA")
}

/// `MIMEAssociationDialogW`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn mime_association_dialog_w() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "MIMEAssociationDialogW")
}

/// `MailToProtocolHandler`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn mail_to_protocol_handler() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "MailToProtocolHandler")
}

/// `MailToProtocolHandlerA`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn mail_to_protocol_handler_a() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "MailToProtocolHandlerA")
}

/// `NewsProtocolHandler`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn news_protocol_handler() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "NewsProtocolHandler")
}

/// `NewsProtocolHandlerA`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn news_protocol_handler_a() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "NewsProtocolHandlerA")
}

/// `TranslateURLA`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn translate_url_a() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "TranslateURLA")
}

/// `TranslateURLW`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn translate_url_w() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "TranslateURLW")
}

/// `URLAssociationDialogA`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn url_association_dialog_a() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "URLAssociationDialogA")
}

/// `URLAssociationDialogW`, exported as `@ stub` in `dlls/url/url.spec`.
pub fn url_association_dialog_w() -> StubException {
    wine_spec_unimplemented_stub(URL_DLL_NAME, "URLAssociationDialogW")
}
