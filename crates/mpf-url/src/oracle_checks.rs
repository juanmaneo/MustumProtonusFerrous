//! Oracle-derived test vectors for the ported url logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `AddMIMEFileTypesPS`,
//! `InetIsOffline`, `FileProtocolHandlerA`, `OpenURLA`, `TelnetProtocolHandlerA` (from
//! `dlls/url/url_main.c`) and `__wine_spec_unimplemented_stub` (from `dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/url/`, and running the harness.
//! The harness printed 25 rows. None was flagged by ASan or UBSan, so none is excluded. The
//! `url.spec` aliases are gcc aliases of the same bodies, so rows 08, 10 and 12 repeat the
//! results of rows 05, 09 and 11 through the alias name.

use super::*;

/// One `@ stub` export helper, such as [`autodial_hook_callback`].
type StubFn = fn() -> StubException;

/// Rows 05 to 08 pass the URL `file:///C:/a.txt` and the path `C:\a.txt`. The port takes
/// the `PathCreateFromUrlA` result and the path it produced as inputs. The `show` argument
/// is not ported.
const URL: &str = "file:///C:/a.txt";

#[test]
fn oracle_rows_explicit_functions() {
    // row 01 AddMIMEFileTypesPS(NULL, NULL) -> ret=0
    assert_eq!(add_mime_file_types_ps(), 0);

    // row 02 InetIsOffline(0x0) -> ret=0
    assert!(!inet_is_offline());
    // row 03 InetIsOffline(0x1) -> ret=0
    assert!(!inet_is_offline());
    // row 04 InetIsOffline(0xffffffff) -> ret=0
    assert!(!inet_is_offline());

    // row 05 FPH S_OK hr=0x00000000 -> file=C:\a.txt
    assert_eq!(
        file_protocol_handler_a_target(S_OK, "C:\\a.txt", URL),
        "C:\\a.txt"
    );
    // row 06 FPH E_INVALIDARG hr=0x80070057 -> file=file:///C:/a.txt
    assert_eq!(
        file_protocol_handler_a_target(0x8007_0057, "unused", URL),
        URL
    );
    // row 07 FPH S_FALSE hr=0x00000001 -> file=file:///C:/a.txt
    assert_eq!(file_protocol_handler_a_target(1, "unused", URL), URL);
    // row 08 FileProtocolHandler(alias) S_OK -> file=C:\a.txt
    assert_eq!(
        file_protocol_handler_a_target(S_OK, "C:\\a.txt", URL),
        "C:\\a.txt"
    );

    // row 09 OpenURLA(NULL, NULL, http://x, 1) -> void, shell_calls=0
    open_url_a();
    // row 10 OpenURL(alias) -> void, shell_calls=0
    open_url_a();

    // row 11 TelnetProtocolHandlerA(NULL, telnet://host) -> ret=0x80004001
    assert_eq!(telnet_protocol_handler_a(), 0x8000_4001);
    // row 12 TelnetProtocolHandler(alias) -> ret=0x80004001
    assert_eq!(telnet_protocol_handler_a(), 0x8000_4001);
}

#[test]
fn oracle_rows_stub_exports() {
    // rows 13 to 25: the record the harness saw when each `@ stub` export was called. The
    // module is `url.dll`, the stub raised 0x80000100 with flags 0x01 and two parameters.
    let rows: [(StubFn, &str); 13] = [
        (autodial_hook_callback, "AutodialHookCallback"),
        (dummy_entry_point, "DummyEntryPoint"),
        (dummy_entry_point_a, "DummyEntryPointA"),
        (mime_association_dialog_a, "MIMEAssociationDialogA"),
        (mime_association_dialog_w, "MIMEAssociationDialogW"),
        (mail_to_protocol_handler, "MailToProtocolHandler"),
        (mail_to_protocol_handler_a, "MailToProtocolHandlerA"),
        (news_protocol_handler, "NewsProtocolHandler"),
        (news_protocol_handler_a, "NewsProtocolHandlerA"),
        (translate_url_a, "TranslateURLA"),
        (translate_url_w, "TranslateURLW"),
        (url_association_dialog_a, "URLAssociationDialogA"),
        (url_association_dialog_w, "URLAssociationDialogW"),
    ];
    for (row, (stub, function)) in rows.into_iter().enumerate() {
        assert_eq!(
            stub(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "url.dll",
                function,
            },
            "row {}",
            row + 13
        );
    }
}
