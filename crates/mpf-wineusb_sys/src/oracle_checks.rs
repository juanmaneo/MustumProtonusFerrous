//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/wineusb.sys/harness.c`, built with ASan and UBSan and
//! `-fshort-wchar`). Each row is `(name, input, expected)`: `id` queries a
//! device ID string, `arr` reserves array capacity, and `usb` maps a libusb
//! transfer status. The input and expected text are the harness lines.

use super::*;

/// Rows recorded from the verbatim Wine C code.
#[rustfmt::skip]
const ROWS: &[(&str, &str, &str)] = &[
    ("dev_id_plain", "id 1133 49271 4608 512 0 0 0 1 3 0 -1 0",
     "ok USB\\VID_046D&PID_C077|"),
    ("dev_id_interface", "id 1133 49271 4608 512 3 1 1 1 3 1 0 0",
     "ok USB\\VID_046D&PID_C077&MI_00|"),
    ("hw_ids_plain", "id 1133 49271 4608 512 0 0 0 1 3 0 -1 1",
     "ok USB\\VID_046D&PID_C077&REV_1200|USB\\VID_046D&PID_C077||"),
    ("hw_ids_interface", "id 1133 49271 4608 512 3 1 1 1 3 1 2 1",
     "ok USB\\VID_046D&PID_C077&REV_1200&MI_02|USB\\VID_046D&PID_C077&MI_02||"),
    ("compat_dev_class", "id 1133 49271 4608 512 9 0 0 1 3 0 -1 2",
     "ok USB\\DevClass_09&SubClass_00&Prot_00|USB\\DevClass_09&SubClass_00|USB\\DevClass_09||"),
    ("compat_interface_class", "id 1133 49271 4608 512 3 1 1 1 3 1 2 2",
     "ok USB\\Class_03&SubClass_01&Prot_01|USB\\Class_03&SubClass_01|USB\\Class_03||"),
    ("compat_flag_clear_index", "id 1133 49271 4608 512 3 1 1 1 3 0 2 2",
     "ok USB\\Class_03&SubClass_01&Prot_01|USB\\Class_03&SubClass_01|USB\\Class_03||"),
    ("instance_id", "id 1133 49271 4608 512 9 0 0 1 3 0 -1 3",
     "ok 512&4608&1&3|"),
    ("instance_id_max", "id 65535 65535 65535 65535 255 255 255 255 255 1 127 3",
     "ok 65535&65535&255&255|"),
    ("dev_id_max_interface", "id 65535 65535 65535 65535 255 255 255 255 255 1 127 0",
     "ok USB\\VID_FFFF&PID_FFFF&MI_7F|"),
    ("serial_unhandled", "id 1133 49271 4608 512 0 0 0 1 3 0 -1 4",
     "unhandled"),
    ("arr_fits", "arr 8 5 8",
     "true 8"),
    ("arr_grow_from_empty", "arr 0 1 40",
     "true 4"),
    ("arr_double", "arr 4 5 8",
     "true 8"),
    ("arr_multi_double", "arr 4 33 8",
     "true 64"),
    ("arr_clamp_u32_max", "arr 0 2147483649 1",
     "true 4294967295"),
    ("arr_count_over_u32", "arr 0 4294967296 8",
     "false 0"),
    ("usb_completed", "usb 0",
     "00000000"),
    ("usb_error", "usb 1",
     "80000500"),
    ("usb_timed_out", "usb 2",
     "C0006000"),
    ("usb_cancelled", "usb 3",
     "C0010000"),
    ("usb_stall", "usb 4",
     "C0000030"),
    ("usb_no_device", "usb 5",
     "C0007000"),
    ("usb_overflow", "usb 6",
     "80000500"),
];

/// Runs one harness line and returns its output line.
fn run(line: &str) -> String {
    let fields: Vec<&str> = line.split_whitespace().collect();
    let num = |i: usize| fields[i].parse::<i64>().expect("number");
    match fields[0] {
        "id" => {
            let device = UsbDevice {
                vendor: num(1) as u16,
                product: num(2) as u16,
                revision: num(3) as u16,
                usbver: num(4) as u16,
                class: num(5) as u8,
                subclass: num(6) as u8,
                protocol: num(7) as u8,
                busnum: num(8) as u8,
                portnum: num(9) as u8,
                interface: num(10) != 0,
                interface_index: num(11) as i16,
            };
            match query_id(&device, num(12) as u32) {
                Some(ids) => format!("ok {}", text(&ids)),
                None => "unhandled".to_string(),
            }
        }
        "arr" => {
            let (capacity, count, size) = (num(1) as usize, num(2) as usize, num(3) as usize);
            match reserve_capacity(capacity, count, size) {
                Some(new_capacity) => format!("true {new_capacity}"),
                None => format!("false {capacity}"),
            }
        }
        _ => format!("{:08X}", usbd_status_from_libusb(num(1) as i32) as u32),
    }
}

/// Renders UTF-16 ID strings as the harness prints them: each NUL is `|`.
fn text(units: &[u16]) -> String {
    units
        .iter()
        .map(|&u| if u == 0 { '|' } else { u as u8 as char })
        .collect()
}

#[test]
fn rows_match_the_verbatim_c() {
    for (name, input, expected) in ROWS {
        assert_eq!(run(input), *expected, "row {name}: {input}");
    }
}
