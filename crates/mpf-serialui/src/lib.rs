//! Port of the portable logic in Wine's `dlls/serialui/confdlg.c`.
//!
//! The DLL configures a COM port. It converts DCB fields to and from the
//! combo boxes of the configuration dialog, checks the arguments of the `drv*`
//! exports, and supplies a default `COMMCONFIG`. The window, the registry, the
//! common dialog and the heap belong to the caller, so this crate takes plain
//! values and returns plain values.
//!
//! `serialui.spec` exports 7 functions. Ported (argument checks and conversions):
//! - `drvCommConfigDialogW`: [`check_comm_config_dialog`]; showing the dialog is the caller's.
//! - `drvSetDefaultCommConfigW`: [`check_set_default`]; the registry write is the caller's.
//! - `drvGetDefaultCommConfigW`: [`check_get_default`] and [`default_comm_config`] (the branch taken when the port has no registry entry); the registry read is the caller's.
//! - The dialog conversions Wine runs inside `drvCommConfigDialogW`: [`dcb_to_dialog`] (`SERIALUI_DCBToDialogInfo`) and [`dialog_to_dcb`] (`SERIALUI_DialogInfoToDCB`), with [`combo_selection`] and [`value_from_text`] for the combo box lookups, and [`baud_to_rate`] and [`rate_to_baud`] for the `CBR_*` codes.
//!
//! Skipped:
//! - `EnumPropPages`: `FIXME` stub that returns FALSE.
//! - `drvCommConfigDialogA`, `drvSetDefaultCommConfigA`, `drvGetDefaultCommConfigA`: ANSI wrappers that convert through the heap and `CP_ACP`. The W functions hold the logic.
//! - `SERIALUI_ConfigDialogProc`: window messages, `EndDialog`, and a title built from window text.
//! - The registry key in `drvSetDefaultCommConfigW` and `drvGetDefaultCommConfigW`: Wine marks the key as wrong (`FIXME`), and it needs the registry.
//! - `DllMain`: process state only.
//!
//! Divergences from C:
//! - `SERIALUI_GetConfItems` leaves its output uninitialised when a lookup fails, which is undefined behaviour in C. [`dialog_to_dcb`] takes all five values, so the caller decides what a failed lookup means.
//! - The name comparison in `lstrcmpA` (Wine's `CompareStringA` with no flags) is modelled as exact byte equality. The dialog names are ASCII, where the two agree.
//! - `lstrcmpiW` in the device check is modelled with ASCII case folding. Wine folds through the upcase table, but only `c`, `o` and `m` in either case can fold to the device name's letters, so the result is the same.

/// `ERROR_INVALID_PARAMETER`.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// `ERROR_INSUFFICIENT_BUFFER`.
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
/// `ERROR_BADKEY`.
pub const ERROR_BADKEY: u32 = 1010;

/// `sizeof(COMMCONFIG)` on Wine's targets.
pub const COMMCONFIG_SIZE: u32 = 52;
/// `sizeof(DCB)` on Wine's targets.
pub const DCB_SIZE: u32 = 28;
/// `PST_RS232`.
const PST_RS232: u32 = 1;
/// `COMMCONFIG.wVersion` written by `drvGetDefaultCommConfigW`.
const COMMCONFIG_VERSION: u16 = 1;

/// `RTS_CONTROL_DISABLE`.
const RTS_CONTROL_DISABLE: u32 = 0;
/// `RTS_CONTROL_HANDSHAKE`.
const RTS_CONTROL_HANDSHAKE: u32 = 2;
/// `DTR_CONTROL_DISABLE`.
const DTR_CONTROL_DISABLE: u32 = 0;

/// `NOPARITY`.
pub const NOPARITY: u32 = 0;
/// `ODDPARITY`.
pub const ODDPARITY: u32 = 1;
/// `EVENPARITY`.
pub const EVENPARITY: u32 = 2;
/// `MARKPARITY`.
pub const MARKPARITY: u32 = 3;
/// `SPACEPARITY`.
pub const SPACEPARITY: u32 = 4;
/// `ONESTOPBIT`.
pub const ONESTOPBIT: u32 = 0;
/// `ONE5STOPBITS`.
pub const ONE5STOPBITS: u32 = 1;
/// `TWOSTOPBITS`.
pub const TWOSTOPBITS: u32 = 2;

/// Flow control value for no handshake, as the dialog's flow combo box numbers it.
pub const FLOW_NONE: u32 = 0;
/// Flow control value for RTS/CTS.
pub const FLOW_HARDWARE: u32 = 1;
/// Flow control value for XON/XOFF.
pub const FLOW_SOFTWARE: u32 = 2;

/// Bytes of window text the dialog reads: `CHAR lpEntry[20]`, one byte of it for the NUL.
const LOOKUP_BUFFER_LEN: usize = 20;

/// The `SERIALUI_BaudConvertTable` pairs: each `CBR_*` code with its integer rate.
const BAUD_CONVERT: [(u32, u32); 15] = [
    (0xFF10, 110),
    (0xFF11, 300),
    (0xFF12, 600),
    (0xFF13, 1200),
    (0xFF14, 2400),
    (0xFF15, 4800),
    (0xFF16, 9600),
    (0xFF17, 14400),
    (0xFF18, 19200),
    (0xFF1B, 38400),
    (0xFF1F, 56000),
    (0xFF20, 57600),
    (0xFF21, 115200),
    (0xFF23, 128000),
    (0xFF27, 256000),
];

/// `SERIALUI_Baud2StrData`: the baud rates the dialog offers, with the text shown for each.
pub const BAUD_TABLE: &[(u32, &str)] = &[
    (110, "110"),
    (300, "300"),
    (600, "600"),
    (1200, "1200"),
    (2400, "2400"),
    (4800, "4800"),
    (9600, "9600"),
    (14400, "14400"),
    (19200, "19200"),
    (38400, "38400"),
    (56000, "56000"),
    (57600, "57600"),
    (115200, "115200"),
    (128000, "128000"),
    (256000, "256000"),
];

/// `SERIALUI_Parity2StrData`.
pub const PARITY_TABLE: &[(u32, &str)] = &[
    (NOPARITY, "None"),
    (ODDPARITY, "Odd"),
    (EVENPARITY, "Even"),
    (MARKPARITY, "Mark"),
    (SPACEPARITY, "Space"),
];

/// `SERIALUI_Stop2StrData`.
pub const STOP_TABLE: &[(u32, &str)] =
    &[(ONESTOPBIT, "1"), (ONE5STOPBITS, "1.5"), (TWOSTOPBITS, "2")];

/// `SERIALUI_Data2StrData`.
pub const DATA_TABLE: &[(u32, &str)] = &[(5, "5"), (6, "6"), (7, "7"), (8, "8"), (16, "16")];

/// `SERIALUI_Flow2StrData`.
pub const FLOW_TABLE: &[(u32, &str)] = &[
    (FLOW_NONE, "None"),
    (FLOW_HARDWARE, "Hardware (RTS/CTS)"),
    (FLOW_SOFTWARE, "Software (XON/XOFF)"),
];

/// The fields of `DCB` that the dialog reads or writes. The rest of the structure
/// is left at its default, which is what the zeroed `memset` in Wine gives.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dcb {
    pub dcb_length: u32,
    pub baud_rate: u32,
    pub f_binary: bool,
    pub f_parity: bool,
    pub f_outx_cts_flow: bool,
    pub f_outx_dsr_flow: bool,
    pub f_dtr_control: u32,
    pub f_out_x: bool,
    pub f_in_x: bool,
    pub f_rts_control: u32,
    pub byte_size: u8,
    pub parity: u8,
    pub stop_bits: u8,
}

/// The fields of `COMMCONFIG` that `drvGetDefaultCommConfigW` sets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CommConfig {
    pub dw_size: u32,
    pub w_version: u16,
    pub dw_provider_sub_type: u32,
    pub dcb: Dcb,
}

/// What `SERIALUI_DCBToDialogInfo` keeps between reading the DCB and writing it back:
/// `bConvert` and `dwFlowControl`. The device name and the `COMMCONFIG` pointer stay with the caller.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DialogInfo {
    pub convert: bool,
    pub flow: u32,
}

/// The index each combo box highlights after `SERIALUI_DCBToDialogInfo`. `None` means no entry matched.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Selections {
    pub baud: Option<usize>,
    pub stop: Option<usize>,
    pub parity: Option<usize>,
    pub data: Option<usize>,
    pub flow: Option<usize>,
}

/// The five values `SERIALUI_DialogInfoToDCB` reads back from the combo boxes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DialogValues {
    pub baud: u32,
    pub stop_bits: u32,
    pub parity: u32,
    pub byte_size: u32,
    pub flow: u32,
}

/// The code units of a NUL-terminated UTF-16 string, without the terminator.
fn wstr(text: &[u16]) -> &[u16] {
    match text.iter().position(|&c| c == 0) {
        Some(end) => &text[..end],
        None => text,
    }
}

/// ASCII upper-to-lower folding. Code units above 0x7F are left alone, so `0x0163`
/// does not fold to `c` the way a truncation to a byte would.
fn ascii_lower(c: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
        c + 32
    } else {
        c
    }
}

/// Ports `SERIALUI_MakeBaudDword`: a `CBR_*` code becomes its integer rate. Any other value is not changed (`None`).
pub fn baud_to_rate(baud: u32) -> Option<u32> {
    BAUD_CONVERT
        .iter()
        .find(|&&(cbr, _)| cbr == baud)
        .map(|&(_, rate)| rate)
}

/// Ports `SERIALUI_MakeBaudEnum`: an integer rate becomes its `CBR_*` code. Any other value is not changed (`None`).
pub fn rate_to_baud(rate: u32) -> Option<u32> {
    BAUD_CONVERT
        .iter()
        .find(|&&(_, value)| value == rate)
        .map(|&(cbr, _)| cbr)
}

/// Ports the selection in `SERIALUI_AddConfItems`. Wine selects every entry whose value matches, so the last match wins.
pub fn combo_selection(table: &[(u32, &str)], value: u32) -> Option<usize> {
    table.iter().rposition(|&(entry, _)| entry == value)
}

/// Ports the lookup in `SERIALUI_GetConfItems`.
///
/// The window text is read into a 20-byte buffer, so only its first 19 bytes are compared, and the text ends at its first NUL. Empty text fails, as `GetWindowTextA` returns 0 for it.
pub fn value_from_text(table: &[(u32, &str)], text: &[u8]) -> Option<u32> {
    let text = match text.iter().position(|&c| c == 0) {
        Some(end) => &text[..end],
        None => text,
    };
    let shown = &text[..text.len().min(LOOKUP_BUFFER_LEN - 1)];
    if shown.is_empty() {
        return None;
    }
    table
        .iter()
        .find(|&&(_, name)| name.as_bytes() == shown)
        .map(|&(value, _)| value)
}

/// The dialog's flow control value for a DCB: the first part of `SERIALUI_DCBToDialogInfo`.
pub fn flow_from_dcb(dcb: &Dcb) -> u32 {
    if dcb.f_rts_control == RTS_CONTROL_HANDSHAKE || dcb.f_outx_cts_flow {
        FLOW_HARDWARE
    } else if dcb.f_out_x || dcb.f_in_x {
        FLOW_SOFTWARE
    } else {
        FLOW_NONE
    }
}

/// Sets the flow control fields of `dcb` for a dialog value: the switch in `SERIALUI_DialogInfoToDCB`.
/// A value other than the three flow values leaves the DCB unchanged, as the switch has no default case.
pub fn apply_flow(dcb: &mut Dcb, flow: u32) {
    let (cts, xon, rts) = match flow {
        FLOW_NONE => (false, false, RTS_CONTROL_DISABLE),
        FLOW_HARDWARE => (true, false, RTS_CONTROL_HANDSHAKE),
        FLOW_SOFTWARE => (false, true, RTS_CONTROL_DISABLE),
        _ => return,
    };
    dcb.f_outx_cts_flow = cts;
    dcb.f_outx_dsr_flow = false;
    dcb.f_dtr_control = DTR_CONTROL_DISABLE;
    dcb.f_out_x = xon;
    dcb.f_in_x = xon;
    dcb.f_rts_control = rts;
}

/// Ports `SERIALUI_DCBToDialogInfo`: the dialog state and the combo box selections for a DCB.
pub fn dcb_to_dialog(dcb: &Dcb) -> (DialogInfo, Selections) {
    let (rate, convert) = match baud_to_rate(dcb.baud_rate) {
        Some(rate) => (rate, true),
        None => (dcb.baud_rate, false),
    };
    let flow = flow_from_dcb(dcb);
    let selections = Selections {
        baud: combo_selection(BAUD_TABLE, rate),
        stop: combo_selection(STOP_TABLE, u32::from(dcb.stop_bits)),
        parity: combo_selection(PARITY_TABLE, u32::from(dcb.parity)),
        data: combo_selection(DATA_TABLE, u32::from(dcb.byte_size)),
        flow: combo_selection(FLOW_TABLE, flow),
    };
    (DialogInfo { convert, flow }, selections)
}

/// Ports `SERIALUI_DialogInfoToDCB`: writes the dialog values into `dcb`.
///
/// Flow control is rewritten only when the user changed it. A converted baud rate goes back to its `CBR_*` code, when it has one.
pub fn dialog_to_dcb(dcb: &mut Dcb, info: DialogInfo, values: DialogValues) {
    dcb.baud_rate = values.baud;
    dcb.stop_bits = values.stop_bits as u8;
    dcb.parity = values.parity as u8;
    dcb.byte_size = values.byte_size as u8;
    if info.flow != values.flow {
        apply_flow(dcb, values.flow);
    }
    if info.convert {
        dcb.baud_rate = rate_to_baud(dcb.baud_rate).unwrap_or(dcb.baud_rate);
    }
}

/// Ports the argument checks of `drvCommConfigDialogW`, in their order.
///
/// `device` is the port name, or `None` for a NULL pointer. `config_dw_size` is `lpCommConfig->dwSize`, or `None` when the config pointer is NULL.
pub fn check_comm_config_dialog(
    device: Option<&[u16]>,
    config_dw_size: Option<u32>,
) -> Result<(), u32> {
    let (Some(device), Some(size)) = (device, config_dw_size) else {
        return Err(ERROR_INVALID_PARAMETER);
    };
    if size < COMMCONFIG_SIZE {
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    if wstr(device).is_empty() {
        return Err(ERROR_BADKEY);
    }
    Ok(())
}

/// Ports the argument check of `drvSetDefaultCommConfigW`: a config pointer, and a size argument of at least `sizeof(COMMCONFIG)`.
pub fn check_set_default(config_present: bool, dw_size: u32) -> bool {
    config_present && dw_size >= COMMCONFIG_SIZE
}

/// Ports the argument checks of `drvGetDefaultCommConfigW`.
///
/// `size` is `lpdwSize`. A size below `sizeof(COMMCONFIG)` is replaced by that size, and so is a size that passes. A device name that is not `com1` to `com9` leaves the size alone.
pub fn check_get_default(
    device: Option<&[u16]>,
    config_present: bool,
    size: Option<&mut u32>,
) -> Result<(), u32> {
    let (Some(device), true, Some(size)) = (device, config_present, size) else {
        return Err(ERROR_INVALID_PARAMETER);
    };
    if *size < COMMCONFIG_SIZE {
        *size = COMMCONFIG_SIZE;
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    if !is_comm_port_name(device) {
        return Err(ERROR_BADKEY);
    }
    *size = COMMCONFIG_SIZE;
    Ok(())
}

/// The `lpszDevice` test in `drvGetDefaultCommConfigW`: the name must be exactly `com1` to `com9`, in any letter case.
pub fn is_comm_port_name(device: &[u16]) -> bool {
    match wstr(device) {
        [c, o, m, digit] => {
            ascii_lower(*c) == u16::from(b'c')
                && ascii_lower(*o) == u16::from(b'o')
                && ascii_lower(*m) == u16::from(b'm')
                && (u16::from(b'1')..=u16::from(b'9')).contains(digit)
        }
        _ => false,
    }
}

/// The `COMMCONFIG` that `drvGetDefaultCommConfigW` returns when the port has no registry entry: 9600 baud, 8 data bits, no parity, one stop bit, binary mode.
///
/// Wine marks these values as a placeholder (`FIXME`), so they are a behaviour to match, not a considered default.
pub fn default_comm_config() -> CommConfig {
    CommConfig {
        dw_size: COMMCONFIG_SIZE,
        w_version: COMMCONFIG_VERSION,
        dw_provider_sub_type: PST_RS232,
        dcb: Dcb {
            dcb_length: DCB_SIZE,
            baud_rate: 9600,
            f_binary: true,
            f_parity: false,
            byte_size: 8,
            parity: NOPARITY as u8,
            stop_bits: ONESTOPBIT as u8,
            ..Dcb::default()
        },
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod tests {
    use super::*;

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    #[test]
    fn baud_conversion_round_trips_every_pair() {
        for (cbr, rate) in BAUD_CONVERT {
            assert_eq!(baud_to_rate(cbr), Some(rate));
            assert_eq!(rate_to_baud(rate), Some(cbr));
        }
    }

    #[test]
    fn baud_table_rates_match_the_conversion_table() {
        let rates: Vec<u32> = BAUD_TABLE.iter().map(|&(rate, _)| rate).collect();
        let converted: Vec<u32> = BAUD_CONVERT.iter().map(|&(_, rate)| rate).collect();
        assert_eq!(rates, converted);
        for &(rate, name) in BAUD_TABLE {
            assert_eq!(name, rate.to_string());
        }
    }

    #[test]
    fn values_that_are_not_codes_or_rates_are_unchanged() {
        assert_eq!(baud_to_rate(9600), None);
        assert_eq!(baud_to_rate(0), None);
        assert_eq!(rate_to_baud(0xFF16), None);
        assert_eq!(rate_to_baud(12345), None);
    }

    #[test]
    fn combo_selection_takes_the_last_match() {
        let table: &[(u32, &str)] = &[(1, "a"), (2, "b"), (1, "c")];
        assert_eq!(combo_selection(table, 1), Some(2));
        assert_eq!(combo_selection(table, 2), Some(1));
        assert_eq!(combo_selection(table, 3), None);
        assert_eq!(combo_selection(&[], 3), None);
    }

    #[test]
    fn text_lookup_reads_nineteen_bytes_and_stops_at_nul() {
        assert_eq!(value_from_text(BAUD_TABLE, b"9600"), Some(9600));
        assert_eq!(value_from_text(BAUD_TABLE, b"9600\0junk"), Some(9600));
        assert_eq!(value_from_text(BAUD_TABLE, b"\0"), None);
        assert_eq!(value_from_text(BAUD_TABLE, b""), None);
        assert_eq!(value_from_text(BAUD_TABLE, b"96000"), None);
        assert_eq!(
            value_from_text(FLOW_TABLE, b"Software (XON/XOFF)"),
            Some(FLOW_SOFTWARE)
        );
        assert_eq!(
            value_from_text(FLOW_TABLE, b"Software (XON/XOFF)tail"),
            Some(FLOW_SOFTWARE)
        );
        assert_eq!(
            value_from_text(FLOW_TABLE, b"Hardware (RTS/CTS)"),
            Some(FLOW_HARDWARE)
        );
        assert_eq!(value_from_text(FLOW_TABLE, b"hardware (rts/cts)"), None);
    }

    #[test]
    fn flow_is_read_from_the_handshake_bits() {
        let plain = Dcb::default();
        assert_eq!(flow_from_dcb(&plain), FLOW_NONE);
        let rts = Dcb {
            f_rts_control: RTS_CONTROL_HANDSHAKE,
            ..plain
        };
        assert_eq!(flow_from_dcb(&rts), FLOW_HARDWARE);
        let cts = Dcb {
            f_outx_cts_flow: true,
            ..plain
        };
        assert_eq!(flow_from_dcb(&cts), FLOW_HARDWARE);
        let out_x = Dcb {
            f_out_x: true,
            ..plain
        };
        assert_eq!(flow_from_dcb(&out_x), FLOW_SOFTWARE);
        let in_x = Dcb {
            f_in_x: true,
            ..plain
        };
        assert_eq!(flow_from_dcb(&in_x), FLOW_SOFTWARE);
        let rts_disable = Dcb {
            f_rts_control: RTS_CONTROL_DISABLE,
            ..plain
        };
        assert_eq!(flow_from_dcb(&rts_disable), FLOW_NONE);
    }

    #[test]
    fn apply_flow_sets_the_fields_of_each_flow_value() {
        let mut dcb = Dcb {
            f_outx_dsr_flow: true,
            f_dtr_control: 1,
            ..Dcb::default()
        };
        apply_flow(&mut dcb, FLOW_HARDWARE);
        assert!(dcb.f_outx_cts_flow && !dcb.f_out_x && !dcb.f_in_x);
        assert_eq!(dcb.f_rts_control, RTS_CONTROL_HANDSHAKE);
        assert!(!dcb.f_outx_dsr_flow);
        assert_eq!(dcb.f_dtr_control, DTR_CONTROL_DISABLE);

        apply_flow(&mut dcb, FLOW_SOFTWARE);
        assert!(!dcb.f_outx_cts_flow && dcb.f_out_x && dcb.f_in_x);
        assert_eq!(dcb.f_rts_control, RTS_CONTROL_DISABLE);

        apply_flow(&mut dcb, FLOW_NONE);
        assert!(!dcb.f_outx_cts_flow && !dcb.f_out_x && !dcb.f_in_x);
    }

    #[test]
    fn apply_flow_ignores_values_outside_the_table() {
        let before = Dcb {
            f_out_x: true,
            f_rts_control: RTS_CONTROL_HANDSHAKE,
            ..Dcb::default()
        };
        let mut dcb = before;
        apply_flow(&mut dcb, 3);
        apply_flow(&mut dcb, u32::MAX);
        assert_eq!(dcb, before);
    }

    #[test]
    fn dcb_with_unknown_values_selects_nothing() {
        let dcb = Dcb {
            baud_rate: 12345,
            stop_bits: 9,
            parity: 9,
            byte_size: 9,
            ..Dcb::default()
        };
        let (info, selections) = dcb_to_dialog(&dcb);
        assert_eq!(
            info,
            DialogInfo {
                convert: false,
                flow: FLOW_NONE
            }
        );
        assert_eq!(
            selections,
            Selections {
                baud: None,
                stop: None,
                parity: None,
                data: None,
                flow: Some(0),
            }
        );
    }

    #[test]
    fn dialog_keeps_an_unconverted_baud_rate_when_it_has_no_code() {
        let mut dcb = Dcb::default();
        let info = DialogInfo {
            convert: true,
            flow: FLOW_NONE,
        };
        let values = DialogValues {
            baud: 12345,
            ..DialogValues::default()
        };
        dialog_to_dcb(&mut dcb, info, values);
        assert_eq!(dcb.baud_rate, 12345);
    }

    #[test]
    fn dialog_without_conversion_writes_the_rate_as_it_is() {
        let mut dcb = Dcb::default();
        let info = DialogInfo {
            convert: false,
            flow: FLOW_NONE,
        };
        let values = DialogValues {
            baud: 9600,
            stop_bits: 2,
            parity: 4,
            byte_size: 16,
            flow: FLOW_NONE,
        };
        dialog_to_dcb(&mut dcb, info, values);
        assert_eq!(dcb.baud_rate, 9600);
        assert_eq!((dcb.stop_bits, dcb.parity, dcb.byte_size), (2, 4, 16));
    }

    #[test]
    fn comm_config_dialog_checks_in_wine_order() {
        let name = wide("COM1");
        assert_eq!(
            check_comm_config_dialog(None, Some(52)),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(
            check_comm_config_dialog(Some(&name), None),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(
            check_comm_config_dialog(None, None),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(
            check_comm_config_dialog(Some(&name), Some(51)),
            Err(ERROR_INSUFFICIENT_BUFFER)
        );
        assert_eq!(
            check_comm_config_dialog(Some(&[0]), Some(52)),
            Err(ERROR_BADKEY)
        );
        assert_eq!(
            check_comm_config_dialog(Some(&[]), Some(52)),
            Err(ERROR_BADKEY)
        );
        assert_eq!(check_comm_config_dialog(Some(&name), Some(52)), Ok(()));
    }

    #[test]
    fn set_default_needs_a_config_and_a_full_size() {
        assert!(!check_set_default(false, 52));
        assert!(!check_set_default(true, 51));
        assert!(check_set_default(true, 52));
        assert!(check_set_default(true, 60));
    }

    #[test]
    fn get_default_rewrites_the_size_before_the_name_check() {
        let name = wide("COM2");
        let mut size = 60;
        assert_eq!(
            check_get_default(Some(&name), true, Some(&mut size)),
            Ok(())
        );
        assert_eq!(size, COMMCONFIG_SIZE);

        let mut small = 4;
        assert_eq!(
            check_get_default(Some(&name), true, Some(&mut small)),
            Err(ERROR_INSUFFICIENT_BUFFER)
        );
        assert_eq!(small, COMMCONFIG_SIZE);

        let bad = wide("COM0");
        let mut kept = 60;
        assert_eq!(
            check_get_default(Some(&bad), true, Some(&mut kept)),
            Err(ERROR_BADKEY)
        );
        assert_eq!(kept, 60);

        let mut any = 60;
        assert_eq!(
            check_get_default(None, true, Some(&mut any)),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(
            check_get_default(Some(&name), false, Some(&mut any)),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(
            check_get_default(Some(&name), true, None),
            Err(ERROR_INVALID_PARAMETER)
        );
        assert_eq!(any, 60);
    }

    #[test]
    fn comm_port_names_are_com1_to_com9_in_any_case() {
        for name in ["com1", "COM9", "Com5", "cOm3"] {
            assert!(is_comm_port_name(&wide(name)), "{name}");
        }
        for name in [
            "com0", "com:", "com10", "com", "", "xom1", "cxm1", "cox1", "co", "lpt1", "com1 ",
        ] {
            assert!(!is_comm_port_name(&wide(name)), "{name}");
        }
        assert!(is_comm_port_name(&[
            u16::from(b'c'),
            u16::from(b'o'),
            u16::from(b'm'),
            u16::from(b'1'),
            0,
            0x78
        ]));
        assert!(!is_comm_port_name(&[
            0x0163,
            u16::from(b'o'),
            u16::from(b'm'),
            u16::from(b'1')
        ]));
        assert!(!is_comm_port_name(&[0]));
    }

    #[test]
    fn default_config_matches_the_fallback_values() {
        let config = default_comm_config();
        assert_eq!(config.dw_size, COMMCONFIG_SIZE);
        assert_eq!(config.w_version, 1);
        assert_eq!(config.dw_provider_sub_type, PST_RS232);
        assert_eq!(config.dcb.dcb_length, DCB_SIZE);
        assert_eq!(config.dcb.baud_rate, 9600);
        assert!(config.dcb.f_binary && !config.dcb.f_parity);
        assert_eq!(
            (
                config.dcb.byte_size,
                config.dcb.parity,
                config.dcb.stop_bits
            ),
            (8, 0, 0)
        );
        assert_eq!(config.dcb.f_rts_control, 0);
    }
}
