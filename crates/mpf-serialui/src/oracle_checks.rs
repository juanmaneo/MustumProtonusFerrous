//! Checks the port against the rows of the Wine `dlls/serialui` oracle.
//!
//! The rows come from `target/canon/serialui/out.txt`, which compiles the verbatim
//! `confdlg.c` functions of commit 9f8c4ae4d35c7ff4153267aba9618c026f5f5583 under ASan
//! and UBSan, driven by the inputs in `vectors.txt`. Each test is one output row. The
//! oracle prints a failed lookup as `-`, which is `None` here.

use super::*;

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// Builds a DCB from the vector order `BAUD|STOP|PARITY|BYTESIZE|RTS|CTS|DTR|DSR|OUTX|INX`.
fn dcb_from_row(row: [u32; 10]) -> Dcb {
    let [baud_rate, stop_bits, parity, byte_size, rts, cts, dtr, dsr, outx, inx] = row;
    Dcb {
        baud_rate,
        stop_bits: stop_bits as u8,
        parity: parity as u8,
        byte_size: byte_size as u8,
        f_rts_control: rts,
        f_outx_cts_flow: cts != 0,
        f_dtr_control: dtr,
        f_outx_dsr_flow: dsr != 0,
        f_out_x: outx != 0,
        f_in_x: inx != 0,
        ..Dcb::default()
    }
}

/// Reads the five combo box values the way the oracle reads them from the window text.
fn dialog(baud: &[u8], stop: &[u8], parity: &[u8], data: &[u8], flow: &[u8]) -> DialogValues {
    DialogValues {
        baud: value_from_text(BAUD_TABLE, baud).expect("baud text"),
        stop_bits: value_from_text(STOP_TABLE, stop).expect("stop text"),
        parity: value_from_text(PARITY_TABLE, parity).expect("parity text"),
        byte_size: value_from_text(DATA_TABLE, data).expect("data text"),
        flow: value_from_text(FLOW_TABLE, flow).expect("flow text"),
    }
}

#[test]
fn oracle_baud_dword_0xff16() {
    assert_eq!(baud_to_rate(0xFF16), Some(9600));
}

#[test]
fn oracle_baud_enum_9600() {
    assert_eq!(rate_to_baud(9600), Some(0xFF16));
}

#[test]
fn oracle_baud_enum_12345_is_unchanged() {
    assert_eq!(rate_to_baud(12345), None);
}

#[test]
fn oracle_get_conf_baud_9600() {
    assert_eq!(value_from_text(BAUD_TABLE, b"9600"), Some(9600));
}

#[test]
fn oracle_get_conf_flow_truncates_to_nineteen_bytes() {
    assert_eq!(
        value_from_text(FLOW_TABLE, b"Software (XON/XOFF)XYZ"),
        Some(FLOW_SOFTWARE)
    );
}

#[test]
fn oracle_get_conf_flow_none_lowercase_fails() {
    assert_eq!(value_from_text(FLOW_TABLE, b"none"), None);
}

#[test]
fn oracle_add_conf_baud_9600_selects_index_six() {
    assert_eq!(BAUD_TABLE.len(), 15);
    assert_eq!(combo_selection(BAUD_TABLE, 9600), Some(6));
}

#[test]
fn oracle_dcb_to_dialog_0xff21_hardware() {
    let dcb = dcb_from_row([0xFF21, 1, 1, 7, 2, 0, 0, 0, 0, 0]);
    let (info, selections) = dcb_to_dialog(&dcb);
    assert_eq!(
        info,
        DialogInfo {
            convert: true,
            flow: FLOW_HARDWARE
        }
    );
    assert_eq!(
        selections,
        Selections {
            baud: Some(12),
            stop: Some(1),
            parity: Some(1),
            data: Some(2),
            flow: Some(1),
        }
    );
}

#[test]
fn oracle_dcb_to_dialog_unknown_baud_software() {
    let dcb = dcb_from_row([12345, 0, 0, 8, 0, 0, 0, 0, 1, 0]);
    let (info, selections) = dcb_to_dialog(&dcb);
    assert_eq!(
        info,
        DialogInfo {
            convert: false,
            flow: FLOW_SOFTWARE
        }
    );
    assert_eq!(
        selections,
        Selections {
            baud: None,
            stop: Some(0),
            parity: Some(0),
            data: Some(3),
            flow: Some(2),
        }
    );
}

#[test]
fn oracle_dcb_to_dialog_0xff16_none() {
    let dcb = dcb_from_row([0xFF16, 2, 4, 5, 0, 0, 0, 0, 0, 0]);
    let (info, selections) = dcb_to_dialog(&dcb);
    assert_eq!(
        info,
        DialogInfo {
            convert: true,
            flow: FLOW_NONE
        }
    );
    assert_eq!(
        selections,
        Selections {
            baud: Some(6),
            stop: Some(2),
            parity: Some(4),
            data: Some(0),
            flow: Some(0),
        }
    );
}

#[test]
fn oracle_dialog_to_dcb_hardware_flow_and_code() {
    let mut dcb = dcb_from_row([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let info = DialogInfo {
        convert: true,
        flow: FLOW_NONE,
    };
    let values = dialog(b"9600", b"1", b"None", b"8", b"Hardware (RTS/CTS)");
    dialog_to_dcb(&mut dcb, info, values);
    assert_eq!(dcb, dcb_from_row([65302, 0, 0, 8, 2, 1, 0, 0, 0, 0]));
}

#[test]
fn oracle_dialog_to_dcb_software_flow_changed() {
    let mut dcb = dcb_from_row([0, 0, 0, 0, 2, 1, 0, 0, 0, 0]);
    let info = DialogInfo {
        convert: false,
        flow: FLOW_HARDWARE,
    };
    let values = dialog(b"1200", b"1.5", b"Odd", b"7", b"Software (XON/XOFF)");
    dialog_to_dcb(&mut dcb, info, values);
    assert_eq!(dcb, dcb_from_row([1200, 1, 1, 7, 0, 0, 0, 0, 1, 1]));
}

#[test]
fn oracle_dialog_to_dcb_unchanged_flow_keeps_dtr_dsr() {
    let mut dcb = dcb_from_row([0, 0, 0, 0, 0, 0, 1, 1, 0, 1]);
    let info = DialogInfo {
        convert: false,
        flow: FLOW_SOFTWARE,
    };
    let values = dialog(b"1200", b"1", b"Even", b"7", b"Software (XON/XOFF)");
    dialog_to_dcb(&mut dcb, info, values);
    assert_eq!(dcb, dcb_from_row([1200, 0, 2, 7, 0, 0, 1, 1, 0, 1]));
}

#[test]
fn oracle_comm_dialog_null_config_is_invalid_parameter() {
    assert_eq!(
        check_comm_config_dialog(Some(&wide("COM1")), None),
        Err(ERROR_INVALID_PARAMETER)
    );
}

#[test]
fn oracle_comm_dialog_small_config_is_insufficient_buffer() {
    assert_eq!(
        check_comm_config_dialog(Some(&wide("COM1")), Some(51)),
        Err(ERROR_INSUFFICIENT_BUFFER)
    );
}

#[test]
fn oracle_comm_dialog_empty_name_is_badkey() {
    assert_eq!(
        check_comm_config_dialog(Some(&wide("")), Some(52)),
        Err(ERROR_BADKEY)
    );
}

#[test]
fn oracle_comm_dialog_valid_arguments_reach_the_dialog() {
    assert_eq!(
        check_comm_config_dialog(Some(&wide("COM1")), Some(52)),
        Ok(())
    );
}

#[test]
fn oracle_set_default_without_config_fails() {
    assert!(!check_set_default(false, 52));
}

#[test]
fn oracle_set_default_small_size_fails() {
    assert!(!check_set_default(true, 51));
}

#[test]
fn oracle_set_default_valid_arguments_reach_the_registry() {
    assert!(check_set_default(true, 52));
}

#[test]
fn oracle_set_default_connect_failure_is_after_the_argument_check() {
    assert!(check_set_default(true, 52));
}

#[test]
fn oracle_get_default_null_device_is_invalid_parameter() {
    let mut size = 52;
    assert_eq!(
        check_get_default(None, true, Some(&mut size)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(size, 52);
}

#[test]
fn oracle_get_default_small_size_is_rewritten_and_insufficient() {
    let mut size = 51;
    assert_eq!(
        check_get_default(Some(&wide("COM1")), true, Some(&mut size)),
        Err(ERROR_INSUFFICIENT_BUFFER)
    );
    assert_eq!(size, 52);
}

#[test]
fn oracle_get_default_com5_fallback_fields() {
    let mut size = 60;
    assert_eq!(
        check_get_default(Some(&wide("cOm5")), true, Some(&mut size)),
        Ok(())
    );
    assert_eq!(size, 52);

    let config = default_comm_config();
    assert_eq!(config.dw_size, 52);
    assert_eq!(config.w_version, 1);
    assert_eq!(config.dw_provider_sub_type, 1);
    assert_eq!(config.dcb.dcb_length, 28);
    assert_eq!(config.dcb.baud_rate, 9600);
    assert!(config.dcb.f_binary);
    assert!(!config.dcb.f_parity);
    assert_eq!(config.dcb.byte_size, 8);
    assert_eq!(config.dcb.parity, 0);
    assert_eq!(config.dcb.stop_bits, 0);
}

#[test]
fn oracle_get_default_com10_is_badkey() {
    let mut size = 52;
    assert_eq!(
        check_get_default(Some(&wide("com10")), true, Some(&mut size)),
        Err(ERROR_BADKEY)
    );
    assert_eq!(size, 52);
}
