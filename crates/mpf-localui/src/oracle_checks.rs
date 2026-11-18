//! Oracle vectors for `mpf-localui`, derived from Wine's `dlls/localui/localui.c`.
//!
//! Each row is named by its harness ID (T01 to C03). The expected values are the results that the harness recorded in `target/canon/localui/out.txt`. The harness compiles the Wine code verbatim, with `gcc -std=gnu11 -fshort-wchar -g -fsanitize=address,undefined`. The run had no sanitizer reports. The rows cover 8 port-type rows and 17 single-call rows, 25 in all. Rows are not generated from the Rust code.
//!
//! The file probe for port types is a boolean stand-in for the `CreateFileW` pair. Each table row gives the probe answer the harness used. The harness does not record each `CreateFileW` call, so this file does not check the probe's call count.

use super::*;

fn w(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn port_type(name: &str, probe: bool) -> u32 {
    port_type_from_name(&w(name), || probe)
}

#[test]
fn port_type_rows() {
    // (ID, name, file probe answer, expected port type)
    let rows: &[(&str, &str, bool, u32)] = &[
        ("T01", "LPT1", false, PORT_IS_LPT),
        ("T03", "FILE:", false, PORT_IS_FILE),
        ("T04", "F\u{131}LE:", false, PORT_IS_FILE),
        ("T05", "/dev/lp0", false, PORT_IS_UNIXNAME),
        ("T06", "CUPS:queue", false, PORT_IS_CUPS),
        ("T07", "cups:queue", false, PORT_IS_UNKNOWN),
        ("T08", "C:\\out.prn", true, PORT_IS_FILENAME),
        ("T09", "C:\\out.prn", false, PORT_IS_UNKNOWN),
    ];
    for &(id, name, probe, expected) in rows {
        assert_eq!(port_type(name, probe), expected, "{id}");
    }
}

#[test]
fn open_name_m01() {
    assert_eq!(
        open_name(",XcvPort ", Some(&w("LPT1:"))),
        w(",XcvPort LPT1:\0")
    );
}

#[test]
fn strip_port_colon_s01() {
    assert_eq!(strip_port_colon(&w("COM1:")), Some(w("COM1")));
}

#[test]
fn lpt_retry_l01_to_l03() {
    assert_eq!(lpt_retry_timeout_request(1, true), Some(w("1\0")));
    assert_eq!(lpt_retry_timeout_request(1_000_000, true), None);
    assert_eq!(lpt_retry_timeout_request(45, false), None);
}

#[test]
fn port_name_check_d01_to_d03() {
    assert_eq!(port_name_check(true, ERROR_SUCCESS), PortNameCheck::Valid);
    assert_eq!(
        port_name_check(true, ERROR_INVALID_NAME),
        PortNameCheck::InvalidName
    );
    assert_eq!(
        dialog_command(make_wparam(IDCANCEL, BN_CLICKED)),
        DialogCommand::Cancel
    );
}

#[test]
fn add_port_a01() {
    assert_eq!(
        add_port_outcome(None, true),
        AddPortOutcome {
            success: false,
            last_error: Some(1223),
            publish_name: false,
            already_exists_notice: false,
        }
    );
}

#[test]
fn add_port_a02() {
    assert_eq!(
        add_port_outcome(Some((true, ERROR_SUCCESS)), true),
        AddPortOutcome {
            success: true,
            last_error: None,
            publish_name: true,
            already_exists_notice: false,
        }
    );
}

#[test]
fn add_port_a03() {
    assert_eq!(
        add_port_outcome(Some((true, ERROR_ALREADY_EXISTS)), true),
        AddPortOutcome {
            success: true,
            last_error: None,
            publish_name: false,
            already_exists_notice: true,
        }
    );
}

#[test]
fn delete_port_x01_to_x03() {
    assert_eq!(delete_port_precheck(None), Err(87));
    assert_eq!(
        delete_port_outcome(Some((true, 50))),
        PortOutcome {
            success: false,
            last_error: Some(50),
        }
    );
    assert_eq!(
        delete_port_outcome(None),
        PortOutcome {
            success: false,
            last_error: Some(1796),
        }
    );
}

#[test]
fn configure_port_c01_to_c03() {
    let com = port_type("COM1:", false);
    assert_eq!(
        configure_port_outcome(com, false),
        PortOutcome {
            success: false,
            last_error: None,
        }
    );
    let lpt = port_type("LPT1:", false);
    assert_eq!(
        configure_port_outcome(lpt, false),
        PortOutcome {
            success: false,
            last_error: Some(1223),
        }
    );
    let file = port_type("FILE:", false);
    assert_eq!(
        configure_port_outcome(file, false),
        PortOutcome {
            success: false,
            last_error: Some(1223),
        }
    );
}
