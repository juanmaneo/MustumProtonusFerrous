//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/usbd.sys/harness.c`, built with ASan and UBSan). Each row is
//! `(name, input, expected)`: `parse`, `pcex` and `pcd` give a descriptor offset,
//! `gil` a length, `val` a status and `ver` the version pair. The descriptor
//! bytes are hex. The input and expected text are the harness lines.

use super::*;

/// Rows recorded from the verbatim Wine C code.
#[rustfmt::skip]
const ROWS: &[(&str, &str, &str)] = &[
    ("parse_first_interface", "parse 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 4",
     "9"),
    ("parse_after_first_interface", "parse 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 10 4",
     "25"),
    ("parse_endpoint_from_start", "parse 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 5",
     "18"),
    ("parse_config_self", "parse 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 2",
     "0"),
    ("parse_type_absent", "parse 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 10",
     "none"),
    ("parse_start_past_end", "parse 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 64 4",
     "none"),
    ("pcex_first_interface", "pcex 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 1 -1 -1 -1 -1",
     "25"),
    ("pcex_alt_setting", "pcex 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 -1 1 -1 -1 -1",
     "48"),
    ("pcex_start_skips_match", "pcex 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 26 1 -1 -1 -1 -1",
     "48"),
    ("pcex_class_subclass_protocol", "pcex 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 -1 -1 8 6 80",
     "25"),
    ("pcex_subclass_mismatch", "pcex 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 -1 -1 -1 7 -1",
     "none"),
    ("pcex_protocol_first", "pcex 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 -1 -1 -1 -1 1",
     "9"),
    ("pcex_absent_interface", "pcex 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 2 -1 -1 -1 -1",
     "none"),
    ("pcd_second_alt", "pcd 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 1 1",
     "48"),
    ("pcd_absent_alt", "pcd 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 0 1",
     "none"),
    ("gil_first_interface", "gil 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 9 64",
     "16"),
    ("gil_stops_at_next_interface", "gil 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 25 64",
     "23"),
    ("gil_truncated_buffer_end", "gil 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 25 40",
     "16"),
    ("gil_end_before_endpoint", "gil 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 9 10",
     "9"),
    ("val_full_buffer", "val 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 64 0 0",
     "00000000"),
    ("val_short_length", "val 09024000030100803209040000010301010007058103400000090401000208065000070582024000000705020240000009040101010806500007058301400000 8 0 0",
     "80000000"),
    ("val_bad_b_length", "val 080240000301008032 9 0 0",
     "80000000"),
    ("val_total_too_short", "val 090211000201008032 9 0 0",
     "80000000"),
    ("val_total_exact", "val 090212000201008032 9 0 0",
     "00000000"),
    ("usbdi_version", "ver",
     "768 512"),
];

/// Decodes a hex string into descriptor bytes.
fn hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex"))
        .collect()
}

/// Runs one harness line and returns its output line.
fn run(line: &str) -> String {
    let fields: Vec<&str> = line.split_whitespace().collect();
    let num = |i: usize| fields[i].parse::<i64>().expect("number");
    let found = |offset: Option<usize>| offset.map_or("none".to_string(), |o| o.to_string());
    match fields[0] {
        "parse" => {
            let buf = hex(fields[1]);
            found(parse_descriptors(
                &buf,
                buf.len(),
                num(2) as usize,
                num(3) as i32,
            ))
        }
        "pcex" => {
            let buf = hex(fields[1]);
            found(parse_configuration_descriptor_ex(
                &buf,
                num(2) as usize,
                num(3) as i32,
                num(4) as i32,
                num(5) as i32,
                num(6) as i32,
                num(7) as i32,
            ))
        }
        "pcd" => {
            let buf = hex(fields[1]);
            found(parse_configuration_descriptor(
                &buf,
                num(2) as u8,
                num(3) as u8,
            ))
        }
        "gil" => {
            let buf = hex(fields[1]);
            get_interface_length(&buf, num(2) as usize, num(3) as usize).to_string()
        }
        "val" => {
            let buf = hex(fields[1]);
            let status = validate_configuration_descriptor(
                &buf[..num(2) as usize],
                num(3) as u16,
                num(4) as u32,
            );
            format!("{:08X}", status as u32)
        }
        _ => {
            let version = get_usbd_i_version();
            format!(
                "{} {}",
                version.usbdi_version, version.supported_usb_version
            )
        }
    }
}

#[test]
fn rows_match_the_verbatim_c() {
    for (name, input, expected) in ROWS {
        assert_eq!(run(input), *expected, "row {name}: {input}");
    }
}
