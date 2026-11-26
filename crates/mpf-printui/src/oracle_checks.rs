//! Oracle-derived test vectors for the ported printui logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `parse_rundll` and
//! `PrintUIEntryW` (`dlls/printui/printui.c`) and of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/printui/`, and running the harness. The harness printed 23 explicit rows (21 parse
//! rows and 2 `PrintUIEntryW` rows) and 21 stub rows. ASan and UBSan flagged none of them, so none
//! are excluded.

use super::*;
use std::cell::Cell;

/// One explicit row of the `parse_rundll` oracle. `command` and `subcommand` are `0` when absent.
struct ParseRow {
    argv: &'static [&'static str],
    ok: bool,
    command: u8,
    subcommand: u8,
    options: &'static [(usize, &'static str)],
    flags: &'static str,
    next_arg: usize,
    pending: Option<&'static str>,
    /// Whether `PrintUIEntryW` reports the command as not implemented, rather than failing.
    unimpl: bool,
}

impl ParseRow {
    /// The state `parse_rundll` must leave behind for this row.
    fn expected(&self) -> RunDll {
        RunDll {
            ok: self.ok,
            command: u16::from(self.command),
            subcommand: u16::from(self.subcommand),
            options: options_of(self.options),
            flags: flags_of(self.flags),
            next_arg: self.next_arg,
            pending: self.pending.map(wide),
        }
    }
}

/// Converts test text to UTF-16. The vectors are ASCII, which is how the harness copies them.
pub(super) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// A `CommandLineToArgvW` stand-in that returns `argv` and adds one to `calls` per call.
pub(super) fn splitter<'a>(
    argv: &'a [Vec<u16>],
    calls: &'a Cell<u32>,
) -> impl FnOnce(&[u16]) -> Vec<Vec<u16>> + 'a {
    move |_| {
        calls.set(calls.get() + 1);
        argv.to_vec()
    }
}

fn options_of(pairs: &[(usize, &str)]) -> [Option<Vec<u16>>; OPT_MAX] {
    let mut options: [Option<Vec<u16>>; OPT_MAX] = Default::default();
    for &(index, value) in pairs {
        options[index] = Some(wide(value));
    }
    options
}

fn flags_of(letters: &str) -> [bool; FLAG_MAX] {
    std::array::from_fn(|i| letters.contains(char::from(b"qwyzZ"[i])))
}

const PARSE_ROWS: &[ParseRow] = &[
    // row 01 argv=["/?"] ret=1 cmd='?' sub=- opts=[] flags=- next=1 pending="" split=1 dispatch=unimpl
    ParseRow {
        argv: &["/?"],
        ok: true,
        command: b'?',
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 1,
        pending: Some(""),
        unimpl: true,
    },
    // row 02 argv=["/q"] ret=1 cmd=- sub=- opts=[] flags=q next=1 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["/q"],
        ok: true,
        command: 0,
        subcommand: 0,
        options: &[],
        flags: "q",
        next_arg: 1,
        pending: None,
        unimpl: false,
    },
    // row 03 argv=["/zZ"] ret=1 cmd=- sub=- opts=[] flags=zZ next=1 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["/zZ"],
        ok: true,
        command: 0,
        subcommand: 0,
        options: &[],
        flags: "zZ",
        next_arg: 1,
        pending: None,
        unimpl: false,
    },
    // row 04 argv=["/cfile.txt","/?"] ret=1 cmd='?' sub=- opts=[2:"file.txt"] flags=- next=2 pending="" split=1 dispatch=unimpl
    ParseRow {
        argv: &["/cfile.txt", "/?"],
        ok: true,
        command: b'?',
        subcommand: 0,
        options: &[(2, "file.txt")],
        flags: "",
        next_arg: 2,
        pending: Some(""),
        unimpl: true,
    },
    // row 05 argv=["/n","Printer A"] ret=1 cmd=- sub=- opts=[8:"Printer A"] flags=- next=2 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["/n", "Printer A"],
        ok: true,
        command: 0,
        subcommand: 0,
        options: &[(8, "Printer A")],
        flags: "",
        next_arg: 2,
        pending: None,
        unimpl: false,
    },
    // row 06 argv=["/n"] ret=1 cmd=- sub=- opts=[] flags=- next=1 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["/n"],
        ok: true,
        command: 0,
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 1,
        pending: None,
        unimpl: false,
    },
    // row 07 argv=["/n",""] ret=1 cmd=- sub=- opts=[8:""] flags=- next=2 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["/n", ""],
        ok: true,
        command: 0,
        subcommand: 0,
        options: &[(8, "")],
        flags: "",
        next_arg: 2,
        pending: None,
        unimpl: false,
    },
    // row 08 argv=["/rfoo","/tbar"] ret=1 cmd=- sub=- opts=[9:"bar",10:"foo"] flags=- next=2 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["/rfoo", "/tbar"],
        ok: true,
        command: 0,
        subcommand: 0,
        options: &[(9, "bar"), (10, "foo")],
        flags: "",
        next_arg: 2,
        pending: None,
        unimpl: false,
    },
    // row 09 argv=["/dx"] ret=1 cmd='d' sub='x' opts=[] flags=- next=1 pending=NULL split=1 dispatch=unimpl
    ParseRow {
        argv: &["/dx"],
        ok: true,
        command: b'd',
        subcommand: b'x',
        options: &[],
        flags: "",
        next_arg: 1,
        pending: None,
        unimpl: true,
    },
    // row 10 argv=["/dsomething"] ret=1 cmd='o' sub=- opts=[7:"ething"] flags=- next=1 pending=NULL split=1 dispatch=unimpl
    ParseRow {
        argv: &["/dsomething"],
        ok: true,
        command: b'o',
        subcommand: 0,
        options: &[(7, "ething")],
        flags: "",
        next_arg: 1,
        pending: None,
        unimpl: true,
    },
    // row 11 argv=["/Sx","extra"] ret=1 cmd='S' sub='x' opts=[] flags=- next=2 pending="extra" split=1 dispatch=unimpl
    ParseRow {
        argv: &["/Sx", "extra"],
        ok: true,
        command: b'S',
        subcommand: b'x',
        options: &[],
        flags: "",
        next_arg: 2,
        pending: Some("extra"),
        unimpl: true,
    },
    // row 12 argv=["/?/q"] ret=1 cmd='?' sub=- opts=[] flags=- next=1 pending="/q" split=1 dispatch=unimpl
    ParseRow {
        argv: &["/?/q"],
        ok: true,
        command: b'?',
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 1,
        pending: Some("/q"),
        unimpl: true,
    },
    // row 13 argv=["@file.rsp"] ret=0 cmd=- sub=- opts=[] flags=- next=1 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["@file.rsp"],
        ok: false,
        command: 0,
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 1,
        pending: None,
        unimpl: false,
    },
    // row 14 argv=["@","next"] ret=0 cmd=- sub=- opts=[] flags=- next=2 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["@", "next"],
        ok: false,
        command: 0,
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 2,
        pending: None,
        unimpl: false,
    },
    // row 15 argv=["foo"] ret=0 cmd=- sub=- opts=[] flags=- next=1 pending="oo" split=1 dispatch=failed
    ParseRow {
        argv: &["foo"],
        ok: false,
        command: 0,
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 1,
        pending: Some("oo"),
        unimpl: false,
    },
    // row 16 argv=["\t /?"] ret=1 cmd='?' sub=- opts=[] flags=- next=1 pending="" split=1 dispatch=unimpl
    ParseRow {
        argv: &["\t /?"],
        ok: true,
        command: b'?',
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 1,
        pending: Some(""),
        unimpl: true,
    },
    // row 17 argv=["","/?"] ret=1 cmd=- sub=- opts=[] flags=- next=1 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["", "/?"],
        ok: true,
        command: 0,
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 1,
        pending: None,
        unimpl: false,
    },
    // row 18 argv=["/x","","/?"] ret=1 cmd='x' sub=- opts=[] flags=- next=2 pending=NULL split=1 dispatch=unimpl
    ParseRow {
        argv: &["/x", "", "/?"],
        ok: true,
        command: b'x',
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 2,
        pending: None,
        unimpl: true,
    },
    // row 19 argv=["/"] ret=1 cmd=- sub=- opts=[] flags=- next=1 pending=NULL split=1 dispatch=failed
    ParseRow {
        argv: &["/"],
        ok: true,
        command: 0,
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 1,
        pending: None,
        unimpl: false,
    },
    // row 20 argv=["/d","x"] ret=0 cmd='d' sub=- opts=[] flags=- next=2 pending="" split=1 dispatch=failed
    ParseRow {
        argv: &["/d", "x"],
        ok: false,
        command: b'd',
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 2,
        pending: Some(""),
        unimpl: false,
    },
    // row 21 argv=["/","/?"] ret=1 cmd='?' sub=- opts=[] flags=- next=2 pending="" split=1 dispatch=unimpl
    ParseRow {
        argv: &["/", "/?"],
        ok: true,
        command: b'?',
        subcommand: 0,
        options: &[],
        flags: "",
        next_arg: 2,
        pending: Some(""),
        unimpl: true,
    },
];

/// An export's name and the helper that models it.
type StubRow = (&'static str, fn() -> StubException);

/// The `@ stub` oracle: each export and its helper, with the printed row.
const STUB_ROWS: [StubRow; 21] = [
    // stub 01 name=ConnectToPrinterDlg code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="ConnectToPrinterDlg"
    ("ConnectToPrinterDlg", connect_to_printer_dlg),
    // stub 02 name=ConnectToPrinterPropertyPage code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="ConnectToPrinterPropertyPage"
    (
        "ConnectToPrinterPropertyPage",
        connect_to_printer_property_page,
    ),
    // stub 03 name=ConstructPrinterFriendlyName code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="ConstructPrinterFriendlyName"
    (
        "ConstructPrinterFriendlyName",
        construct_printer_friendly_name,
    ),
    // stub 04 name=DllGetClassObject code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="DllGetClassObject"
    ("DllGetClassObject", dll_get_class_object),
    // stub 05 name=DocumentPropertiesWrap code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="DocumentPropertiesWrap"
    ("DocumentPropertiesWrap", document_properties_wrap),
    // stub 06 name=PnPInterface code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="PnPInterface"
    ("PnPInterface", pnp_interface),
    // stub 07 name=PrintNotifyTray_Exit code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="PrintNotifyTray_Exit"
    ("PrintNotifyTray_Exit", print_notify_tray_exit),
    // stub 08 name=PrintNotifyTray_Init code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="PrintNotifyTray_Init"
    ("PrintNotifyTray_Init", print_notify_tray_init),
    // stub 09 name=PrinterPropPageProvider code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="PrinterPropPageProvider"
    ("PrinterPropPageProvider", printer_prop_page_provider),
    // stub 10 name=RegisterPrintNotify code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="RegisterPrintNotify"
    ("RegisterPrintNotify", register_print_notify),
    // stub 11 name=ShowErrorMessageHR code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="ShowErrorMessageHR"
    ("ShowErrorMessageHR", show_error_message_hr),
    // stub 12 name=ShowErrorMessageSC code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="ShowErrorMessageSC"
    ("ShowErrorMessageSC", show_error_message_sc),
    // stub 13 name=UnregisterPrintNotify code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="UnregisterPrintNotify"
    ("UnregisterPrintNotify", unregister_print_notify),
    // stub 14 name=bFolderEnumPrinters code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="bFolderEnumPrinters"
    ("bFolderEnumPrinters", b_folder_enum_printers),
    // stub 15 name=bFolderGetPrinter code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="bFolderGetPrinter"
    ("bFolderGetPrinter", b_folder_get_printer),
    // stub 16 name=bFolderRefresh code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="bFolderRefresh"
    ("bFolderRefresh", b_folder_refresh),
    // stub 17 name=bPrinterSetup code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="bPrinterSetup"
    ("bPrinterSetup", b_printer_setup),
    // stub 18 name=vDocumentDefaults code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="vDocumentDefaults"
    ("vDocumentDefaults", v_document_defaults),
    // stub 19 name=vPrinterPropPages code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="vPrinterPropPages"
    ("vPrinterPropPages", v_printer_prop_pages),
    // stub 20 name=vQueueCreate code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="vQueueCreate"
    ("vQueueCreate", v_queue_create),
    // stub 21 name=vServerPropPages code=0x80000100 flags=0x1 nparams=2 raises=1 module="printui.dll" function="vServerPropPages"
    ("vServerPropPages", v_server_prop_pages),
];

#[test]
fn oracle_parse_rundll_rows() {
    for (index, row) in PARSE_ROWS.iter().enumerate() {
        let argv: Vec<Vec<u16>> = row.argv.iter().copied().map(wide).collect();
        assert_eq!(parse_rundll(&argv), row.expected(), "row {:02}", index + 1);

        let calls = Cell::new(0);
        let line = wide("rundll-command");
        let dispatch = print_ui_entry_w(Some(line.as_slice()), splitter(&argv, &calls));
        let expected = if row.unimpl {
            Dispatch::NotImplemented {
                command: u16::from(row.command),
                subcommand: u16::from(row.subcommand),
            }
        } else {
            Dispatch::Failed
        };
        assert_eq!(dispatch, expected, "row {:02}", index + 1);
        assert_eq!(calls.get(), 1, "row {:02} split calls", index + 1);
    }
}

#[test]
fn oracle_print_ui_entry_w_rows() {
    let calls = Cell::new(0);

    // row 22 cmdline=NULL split=0 dispatch=failed
    assert_eq!(
        print_ui_entry_w(None, splitter(&[], &calls)),
        Dispatch::Failed
    );

    // row 23 cmdline="" split=0 dispatch=failed
    let empty = wide("");
    assert_eq!(
        print_ui_entry_w(Some(empty.as_slice()), splitter(&[], &calls)),
        Dispatch::Failed
    );
    assert_eq!(calls.get(), 0);
}

#[test]
fn oracle_stub_rows() {
    for (name, stub) in STUB_ROWS {
        assert_eq!(
            stub(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "printui.dll",
                function: name,
            },
            "{name}"
        );
    }
}
