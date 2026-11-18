//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/hidclass.sys/harness.c`, built with ASan and UBSan). Each row
//! is `(name, case, expected output)`. A case starts with `Q` (report ring),
//! `F` (report-ID lookup) or `S` (device string lookup), as in the harness.
//!
//! Excluded: `S` with a device id that has no backslash. ASan reports a SEGV
//! there because C reads `NULL + 1`. The port returns `None` instead, and
//! `edge_checks.rs` covers that.

use super::*;

fn run_case(line: &str) -> String {
    let mut tokens = line.split_whitespace();
    let op = tokens.next().unwrap();
    if op == "Q" {
        run_queue(tokens)
    } else if op == "F" {
        run_find(tokens)
    } else {
        run_string(tokens)
    }
}

fn run_queue<'a>(ops: impl Iterator<Item = &'a str>) -> String {
    let mut queue = ReportQueue::<HidReport>::new();
    let mut out = Vec::new();
    for op in ops {
        let arg = &op[1..];
        let result = match &op[..1] {
            "p" => {
                let byte: u8 = arg.parse().unwrap();
                queue.push(HidReport::new(&[byte], 1).unwrap());
                "ok".to_string()
            }
            "o" => match queue.pop() {
                Some(report) => report.buffer()[0].to_string(),
                None => "none".to_string(),
            },
            _ => match queue.resize(arg.parse().unwrap()) {
                Ok(()) => "ok".to_string(),
                Err(status) => format!("st{status:x}"),
            },
        };
        out.push(format!(
            "{result}@{}/{}/{}",
            queue.read_index(),
            queue.write_index(),
            queue.length()
        ));
    }
    out.join(" ")
}

fn run_find<'a>(mut tokens: impl Iterator<Item = &'a str>) -> String {
    let mut number = || -> i64 { tokens.next().unwrap().parse().unwrap() };
    let count = number();
    let reports: Vec<ReportIds> = (0..count)
        .map(|_| ReportIds {
            report_id: number() as u8,
            collection_number: number() as u8,
            input_length: number() as u16,
            output_length: number() as u16,
            feature_length: number() as u16,
        })
        .collect();
    let report_type = match number() {
        0 => ReportType::Input,
        1 => ReportType::Output,
        _ => ReportType::Feature,
    };
    let collection = number() as u8;
    let id = number() as u8;
    let any_id = number() != 0;

    match find_report_with_type_and_id(&reports, collection, report_type, id, any_id) {
        Some(index) => index.to_string(),
        None => "none".to_string(),
    }
}

fn run_string<'a>(mut tokens: impl Iterator<Item = &'a str>) -> String {
    let index: u32 = tokens.next().unwrap().parse().unwrap();
    let device_id: Vec<u16> = tokens.next().unwrap().encode_utf16().collect();

    match find_device_string(&device_id, index) {
        Some(product) => product.to_string(),
        None => "none".to_string(),
    }
}
#[rustfmt::skip]
const ROWS: &[(&str, &str, &str)] = &[
    ("q_fifo", "Q p1 p2 p3 o o o", "ok@0/1/32 ok@0/2/32 ok@0/3/32 1@1/3/32 2@2/3/32 3@3/3/32"),
    ("q_pop_empty", "Q o p7 o o", "none@0/0/32 ok@0/1/32 7@1/1/32 none@1/1/32"),
    ("q_resize_invalid", "Q r1 r513 r0 r2", "stc000000d@0/0/32 stc000000d@0/0/32 stc000000d@0/0/32 ok@0/0/2"),
    ("q_resize_same_then_reset", "Q p5 p6 r32 r16 p9 r16 o", "ok@0/1/32 ok@0/2/32 ok@0/2/32 ok@0/0/16 ok@0/1/16 ok@0/1/16 9@1/1/16"),
    ("q_wrap_length2", "Q r2 p1 p2 p3 o o o p4 o", "ok@0/0/2 ok@0/1/2 ok@1/0/2 ok@0/1/2 3@1/1/2 2@1/1/2 none@1/1/2 ok@1/0/2 4@0/0/2"),
    ("q_overwrite_40", "Q p0 p1 p2 p3 p4 p5 p6 p7 p8 p9 p10 p11 p12 p13 p14 p15 p16 p17 p18 p19 p20 p21 p22 p23 p24 p25 p26 p27 p28 p29 p30 p31 p32 p33 p34 p35 p36 p37 p38 p39 o o o o o o o o o o", "ok@0/1/32 ok@0/2/32 ok@0/3/32 ok@0/4/32 ok@0/5/32 ok@0/6/32 ok@0/7/32 ok@0/8/32 ok@0/9/32 ok@0/10/32 ok@0/11/32 ok@0/12/32 ok@0/13/32 ok@0/14/32 ok@0/15/32 ok@0/16/32 ok@0/17/32 ok@0/18/32 ok@0/19/32 ok@0/20/32 ok@0/21/32 ok@0/22/32 ok@0/23/32 ok@0/24/32 ok@0/25/32 ok@0/26/32 ok@0/27/32 ok@0/28/32 ok@0/29/32 ok@0/30/32 ok@0/31/32 ok@1/0/32 ok@2/1/32 ok@3/2/32 ok@4/3/32 ok@5/4/32 ok@6/5/32 ok@7/6/32 ok@8/7/32 ok@9/8/32 9@10/8/32 10@11/8/32 11@12/8/32 12@13/8/32 13@14/8/32 14@15/8/32 15@16/8/32 16@17/8/32 17@18/8/32 18@19/8/32"),
    ("q_interleaved", "Q p1 o p2 p3 o p4 p5 p6 o o o o", "ok@0/1/32 1@1/1/32 ok@1/2/32 ok@1/3/32 2@2/3/32 ok@2/4/32 ok@2/5/32 ok@2/6/32 3@3/6/32 4@4/6/32 5@5/6/32 6@6/6/32"),
    ("q_resize_max_drain", "Q r512 p1 p2 o o", "ok@0/0/512 ok@0/1/512 ok@0/2/512 1@1/2/512 2@2/2/512"),
    ("f_first_match", "F 2 1 1 8 0 0 2 1 8 0 0 0 0 2 0", "1"),
    ("f_id_zero_wildcard", "F 1 0 1 8 0 0 0 0 5 0", "0"),
    ("f_collection_filter", "F 2 0 1 8 0 0 0 2 8 0 0 0 2 0 0", "1"),
    ("f_any_id", "F 1 1 1 8 0 0 0 0 9 1", "0"),
    ("f_type_output_zero_length", "F 2 1 1 8 0 0 1 1 0 4 0 1 0 1 0", "1"),
    ("f_feature_zero_none", "F 1 1 1 8 0 0 2 0 1 0", "none"),
    ("f_collection_zero_any", "F 1 1 3 8 0 0 0 0 1 0", "0"),
    ("s_xbox_exact", "S 15 HID\\VID_045E&PID_028E", "Controller (XBOX 360 For Windows)"),
    ("s_sony_lowercase", "S 15 hid\\vid_054c&pid_05c4", "Wireless Controller"),
    ("s_last_entry", "S 15 HID\\VID_054C&PID_0DF2", "Wireless Controller"),
    ("s_tail_suffix", "S 15 HID\\VID_045E&PID_0B13&IG_00", "Controller (Xbox One For Windows)"),
    ("s_short_id", "S 15 HID\\VID_045E&PID_028", "none"),
    ("s_wrong_index", "S 14 HID\\VID_045E&PID_028E", "none"),
    ("s_unknown_pid", "S 15 HID\\VID_045E&PID_9999", "none"),
];
#[test]
fn rows_match_recorded_c_output() {
    for (name, case, expected) in ROWS {
        assert_eq!(run_case(case), *expected, "row {name}");
    }
}
