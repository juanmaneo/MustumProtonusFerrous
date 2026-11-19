//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/hid/harness.c`, built with ASan and UBSan). Each row is
//! `(spec, expected)`: `replay` runs the spec through the port and prints the
//! result the way the harness does.

use super::*;

/// Rows recorded from the verbatim Wine C code: `(spec, harness output)`.
#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    // get_usage_value
    ("op=GetUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=48 r=007f00 il=3 ol=3 fl=3", "st=00110000 v=127"),
    // get_usage_value_not_found
    ("op=GetUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=49 r=007f00 il=3 ol=3 fl=3", "st=c0110004"),
    // get_usage_value_array
    ("op=GetUsageValueArray in=1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 pg=1 us=48 vb=0000 r=00abcd il=3 ol=3 fl=3", "st=00110000 vb=abcd"),
    // get_scaled_usage_value
    ("op=GetScaledUsageValue in=1:49:49:0:2:0:8:1:2:8:1:1:1:0:255:0:100 pg=1 co=1 us=49 r=000080 il=3 ol=3 fl=3", "st=00110000 v=50"),
    // set_usage_value
    ("op=SetUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=48 v=85 r=000000 il=3 ol=3 fl=3", "st=00110000 r=005500"),
    // set_usage_value_array
    ("op=SetUsageValueArray in=1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 pg=1 us=48 vb=abcd r=000000 il=3 ol=3 fl=3", "st=00110000 r=00abcd"),
    // set_scaled_usage_value
    ("op=SetScaledUsageValue in=1:49:49:0:2:0:8:1:2:8:1:1:1:0:255:0:100 pg=1 co=1 us=49 v=50 r=000000 il=3 ol=3 fl=3", "st=00110000 r=000080"),
    // get_usages
    ("op=GetUsages in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 r=000500 il=3 ol=3 fl=3", "st=00110000 u=1,3"),
    // get_usages_ex
    ("op=GetUsagesEx in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 r=000500 il=3 ol=3 fl=3", "st=00110000 u=7:1,7:3"),
    // set_usages
    ("op=SetUsages in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 ul=1,3 r=000000 il=3 ol=3 fl=3", "st=00110000 r=000500"),
    // unset_usages
    ("op=UnsetUsages in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 ul=1 r=000700 il=3 ol=3 fl=3", "st=00110000 r=000600"),
    // unset_usages_array_bug2
    ("op=UnsetUsages in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 pg=7 ul=7 r=0002030000 il=5 ol=5 fl=5", "st=00110000 r=0000030000"),
    // get_data
    ("op=GetData in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 r=000500 il=3 ol=3 fl=3", "st=00110000 d=0:1,2:1"),
    // get_data_value
    ("op=GetData in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 r=007f00 il=3 ol=3 fl=3", "st=00110000 d=0:127"),
    // set_data_press
    ("op=SetData in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 dl=0:1 r=000000 il=3 ol=3 fl=3", "st=00110000 r=000100"),
    // set_data_value_bug1
    ("op=SetData in=1:48:48:0:1:3:8:1:2:8:0:0:1:0:255:0:0 dl=0:55 r=000000 il=3 ol=3 fl=3", "st=00110000 r=000600"),
    // max_usage_list_length
    ("op=MaxUsageListLength in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7", "n=3"),
    // max_data_list_length
    ("op=MaxDataListLength in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0,7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0,7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0,1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0", "n=9"),
    // get_caps
    ("op=GetCaps in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0,7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0,7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0,1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 il=5 ol=0 fl=0 nc=2 ppg=7 pus=1", "caps=1:7:5:0:0:2:2:2:9:0:0:0:0:0:0"),
    // get_specific_value_caps
    ("op=GetSpecificValueCaps in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0,1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 pg=1", "st=00110000 caps=1:0:1:2:48:48,1:0:1:0:48:48"),
    // get_specific_button_caps
    ("op=GetSpecificButtonCaps in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7", "st=00110000 caps=7:0:1:2:1:3"),
    // initialize_report_for_id
    ("op=InitializeReportForID in=1:48:48:5:1:0:8:1:2:8:0:0:1:-127:127:0:0 rid=5 r=000000 il=3 ol=3 fl=3", "st=00110000 r=050000"),
    // copy_bits
    ("op=CopyBits dst=00000000 src=ff sh=-3 cnt=5", "dst=1f000000"),
    // get_usages_array
    ("op=GetUsages in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 pg=7 r=0002000000 il=5 ol=5 fl=5", "st=00110000 u=5"),
    // get_usages_cap_zero
    ("op=GetUsages in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 r=000500 cap=0 il=3 ol=3 fl=3", "st=c0110007"),
];

/// One spec: the `key=value` pairs that the harness reads.
#[derive(Default)]
struct Spec {
    op: String,
    rt: u8,
    page: u16,
    collection: u16,
    usage: u16,
    value: i64,
    value_bytes: Vec<u8>,
    report: Vec<u8>,
    usages: Vec<u16>,
    data: Vec<HidpData>,
    report_id: u8,
    shift: i64,
    count: usize,
    dst: Vec<u8>,
    src: Vec<u8>,
    cap: Option<usize>,
    input: Vec<ValueCaps>,
    output: Vec<ValueCaps>,
    feature: Vec<ValueCaps>,
    input_len: u16,
    output_len: u16,
    feature_len: u16,
    link_nodes: u16,
    device_page: u16,
    device_usage: u16,
}

impl Spec {
    fn parse(text: &str) -> Spec {
        let mut spec = Spec::default();
        for pair in text.split(' ') {
            let (key, val) = pair.split_once('=').unwrap();
            match key {
                "op" => spec.op = val.to_string(),
                "rt" => spec.rt = val.parse().unwrap(),
                "pg" => spec.page = val.parse().unwrap(),
                "co" => spec.collection = val.parse().unwrap(),
                "us" => spec.usage = val.parse().unwrap(),
                "v" => spec.value = val.parse().unwrap(),
                "vb" => spec.value_bytes = unhex(val),
                "r" => spec.report = unhex(val),
                "ul" => spec.usages = items(val).map(|s| s.parse().unwrap()).collect(),
                "dl" => spec.data = items(val).map(data_from).collect(),
                "cap" => spec.cap = Some(val.parse().unwrap()),
                "rid" => spec.report_id = val.parse().unwrap(),
                "sh" => spec.shift = val.parse().unwrap(),
                "cnt" => spec.count = val.parse().unwrap(),
                "dst" => spec.dst = unhex(val),
                "src" => spec.src = unhex(val),
                "in" => spec.input = items(val).map(cap_from).collect(),
                "out" => spec.output = items(val).map(cap_from).collect(),
                "feat" => spec.feature = items(val).map(cap_from).collect(),
                "il" => spec.input_len = val.parse().unwrap(),
                "ol" => spec.output_len = val.parse().unwrap(),
                "fl" => spec.feature_len = val.parse().unwrap(),
                "nc" => spec.link_nodes = val.parse().unwrap(),
                "ppg" => spec.device_page = val.parse().unwrap(),
                "pus" => spec.device_usage = val.parse().unwrap(),
                _ => panic!("unknown oracle key {key:?}"),
            }
        }
        spec
    }

    fn preparsed(&self) -> PreparsedData {
        PreparsedData {
            usage_page: self.device_page,
            usage: self.device_usage,
            link_collection_nodes: self.link_nodes,
            input: ReportCaps {
                byte_length: self.input_len,
                values: self.input.clone(),
            },
            output: ReportCaps {
                byte_length: self.output_len,
                values: self.output.clone(),
            },
            feature: ReportCaps {
                byte_length: self.feature_len,
                values: self.feature.clone(),
            },
        }
    }
}

fn items(text: &str) -> impl Iterator<Item = &str> {
    text.split(',').filter(|s| !s.is_empty())
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn join(items: impl Iterator<Item = String>, sep: &str) -> String {
    items.collect::<Vec<_>>().join(sep)
}

fn cap_from(text: &str) -> ValueCaps {
    let f: Vec<i64> = text.split(':').map(|s| s.parse().unwrap()).collect();
    ValueCaps {
        usage_page: f[0] as u16,
        usage_min: f[1] as u16,
        usage_max: f[2] as u16,
        report_id: f[3] as u8,
        start_byte: f[4] as u16,
        start_bit: f[5] as u8,
        bit_size: f[6] as u16,
        report_count: f[7] as u16,
        bit_field: f[8] as u32,
        flags: f[9] as u32,
        data_index_min: f[10] as u16,
        data_index_max: f[11] as u16,
        link_collection: f[12] as u16,
        logical_min: f[13] as i32,
        logical_max: f[14] as i32,
        physical_min: f[15] as i32,
        physical_max: f[16] as i32,
    }
}

fn data_from(item: &str) -> HidpData {
    let (index, raw) = item.split_once(':').unwrap();
    HidpData {
        data_index: index.parse().unwrap(),
        raw_value: raw.parse().unwrap(),
    }
}

fn report_type(n: u8) -> ReportType {
    match n {
        0 => ReportType::Input,
        1 => ReportType::Output,
        2 => ReportType::Feature,
        _ => panic!("unknown oracle report type {n}"),
    }
}

fn status_line(result: Result<String, Status>) -> String {
    match result {
        Ok(suffix) => format!("st={:08x}{suffix}", HIDP_STATUS_SUCCESS),
        Err(st) => format!("st={st:08x}"),
    }
}

fn report_line(result: Result<(), Status>, report: &[u8]) -> String {
    let st = result.err().unwrap_or(HIDP_STATUS_SUCCESS);
    format!("st={st:08x} r={}", hex(report))
}

fn caps_text(c: &ValueCaps) -> String {
    let max = if c.is_range() {
        c.usage_max
    } else {
        c.usage_min
    };
    format!(
        "{}:{}:{}:{}:{}:{}",
        c.usage_page, c.report_id, c.link_collection, c.bit_field, c.usage_min, max
    )
}

fn usage_text(u: &UsageAndPage) -> String {
    format!("{}:{}", u.usage_page, u.usage)
}

fn data_text(d: &HidpData) -> String {
    format!("{}:{}", d.data_index, d.raw_value)
}

/// Runs one spec through the port and formats the result the way `harness` prints it.
pub(super) fn replay(text: &str) -> String {
    let s = Spec::parse(text);
    let p = s.preparsed();
    let rt = report_type(s.rt);
    let (page, co, usage) = (s.page, s.collection, s.usage);
    match s.op.as_str() {
        "GetUsageValue" => status_line(
            get_usage_value(&p, rt, page, co, usage, &s.report).map(|v| format!(" v={v}")),
        ),
        "GetUsageValueArray" => {
            let mut value = s.value_bytes.clone();
            let st = get_usage_value_array(&p, rt, page, co, usage, &mut value, &s.report);
            status_line(st.map(|()| format!(" vb={}", hex(&value))))
        }
        "GetScaledUsageValue" => status_line(
            get_scaled_usage_value(&p, rt, page, co, usage, &s.report).map(|v| format!(" v={v}")),
        ),
        "SetUsageValue" => {
            let mut report = s.report.clone();
            let st = set_usage_value(&p, rt, page, co, usage, s.value as u32, &mut report);
            report_line(st, &report)
        }
        "SetUsageValueArray" => {
            let mut report = s.report.clone();
            let st = set_usage_value_array(&p, rt, page, co, usage, &s.value_bytes, &mut report);
            report_line(st, &report)
        }
        "SetScaledUsageValue" => {
            let mut report = s.report.clone();
            let st = set_scaled_usage_value(&p, rt, page, co, usage, s.value as i32, &mut report);
            report_line(st, &report)
        }
        "GetUsages" => {
            let mut out = vec![0; s.cap.unwrap_or(16)];
            let n = get_usages(&p, rt, page, co, &s.report, &mut out);
            status_line(n.map(|n| format!(" u={}", join(out[..n].iter().map(u16::to_string), ","))))
        }
        "GetUsagesEx" => {
            let mut out = vec![UsageAndPage::default(); s.cap.unwrap_or(16)];
            let n = get_usages_ex(&p, rt, co, &s.report, &mut out);
            status_line(n.map(|n| format!(" u={}", join(out[..n].iter().map(usage_text), ","))))
        }
        "SetUsages" => {
            let mut report = s.report.clone();
            let st = set_usages(&p, rt, page, co, &s.usages, &mut report);
            report_line(st, &report)
        }
        "UnsetUsages" => {
            let mut report = s.report.clone();
            let st = unset_usages(&p, rt, page, co, &s.usages, &mut report);
            report_line(st, &report)
        }
        "GetData" => {
            let mut out = vec![HidpData::default(); s.cap.unwrap_or(16)];
            let n = get_data(&p, rt, &s.report, &mut out);
            status_line(n.map(|n| format!(" d={}", join(out[..n].iter().map(data_text), ","))))
        }
        "SetData" => {
            let mut report = s.report.clone();
            let st = set_data(&p, rt, &s.data, &mut report);
            report_line(st, &report)
        }
        "MaxUsageListLength" => format!("n={}", max_usage_list_length(&p, rt, page)),
        "MaxDataListLength" => format!("n={}", max_data_list_length(&p, rt)),
        "InitializeReportForID" => {
            let mut report = s.report.clone();
            let st = initialize_report_for_id(&p, rt, s.report_id, &mut report);
            report_line(st, &report)
        }
        "GetCaps" => {
            let c = get_caps(&p);
            let fields = [
                c.usage,
                c.usage_page,
                c.input_report_byte_length,
                c.output_report_byte_length,
                c.feature_report_byte_length,
                c.number_link_collection_nodes,
                c.number_input_button_caps,
                c.number_input_value_caps,
                c.number_input_data_indices,
                c.number_output_button_caps,
                c.number_output_value_caps,
                c.number_output_data_indices,
                c.number_feature_button_caps,
                c.number_feature_value_caps,
                c.number_feature_data_indices,
            ];
            format!("caps={}", join(fields.iter().map(u16::to_string), ":"))
        }
        "GetSpecificValueCaps" => {
            let mut out = vec![ValueCaps::default(); s.cap.unwrap_or(16)];
            let n = get_specific_value_caps(&p, rt, page, co, usage, &mut out);
            status_line(n.map(|n| format!(" caps={}", join(out[..n].iter().map(caps_text), ","))))
        }
        "GetSpecificButtonCaps" => {
            let mut out = vec![ValueCaps::default(); s.cap.unwrap_or(16)];
            let n = get_specific_button_caps(&p, rt, page, co, usage, &mut out);
            status_line(n.map(|n| format!(" caps={}", join(out[..n].iter().map(caps_text), ","))))
        }
        "CopyBits" => {
            let mut dst = s.dst.clone();
            let dst_bit = s.shift.max(0) as usize;
            let src_bit = (-s.shift).max(0) as usize;
            bits::copy_bits(&mut dst, dst_bit, &s.src, src_bit, s.count);
            format!("dst={}", hex(&dst))
        }
        other => panic!("unknown oracle op {other:?}"),
    }
}

#[test]
fn oracle_rows_match_wine() {
    for (spec, expected) in ROWS {
        assert_eq!(replay(spec), *expected, "row {spec}");
    }
}

#[test]
#[should_panic(expected = "unknown oracle op")]
fn unknown_oracle_op_panics() {
    replay("op=Bogus");
}

#[test]
#[should_panic(expected = "unknown oracle key")]
fn unknown_oracle_key_panics() {
    replay("op=GetCaps bogus=1");
}

#[test]
#[should_panic(expected = "unknown oracle report type 3")]
fn unknown_oracle_report_type_panics() {
    replay("op=GetCaps rt=3");
}
