//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/hidparse.sys/harness.c`, built with ASan and UBSan). Each row
//! is `(name, op, descriptor hex, expected text)`: `p` parses one collection,
//! `t` lists the top-level collection bounds, and `c` describes the device.

use super::*;

/// Rows recorded from the verbatim Wine C code.
#[rustfmt::skip]
const ROWS: &[(&str, char, &str, &str)] = &[
    // mouse
    ("mouse", 'p', "05010902A1010901A100050919012903150025019503750181029501750581010501093009311581257F7508950281069102B102C0C0",
     "u=1:2 nodes=[1:2/0/1/1/0/1 1:1/0/0/0/0/0] in=4/4[U9:1-3,r0,sb1,sx0,bs1,rc3,tb3,eb2,bf2,lc1,L1:1,fl1c,st0-0,ds0-0,di0-2,nv0,lg0..0,ph0..0,un0,ue0 U0:0-0,r0,sb1,sx3,bs5,rc1,tb5,eb2,bf1,lc1,L1:1,fle,st0-0,ds0-0,di3-2,nv0,lg1..0,ph0..0,un0,ue0 U1:31-31,r0,sb3,sx0,bs8,rc1,tb8,eb4,bf6,lc1,L1:1,fl0,st0-0,ds0-0,di3-3,nv0,lg-127..127,ph0..0,un0,ue0 U1:30-30,r0,sb2,sx0,bs8,rc1,tb8,eb3,bf6,lc1,L1:1,fl0,st0-0,ds0-0,di4-4,nv0,lg-127..127,ph0..0,un0,ue0] out=1/3[U0:0-0,r0,sb1,sx0,bs8,rc2,tb16,eb3,bf2,lc1,L1:1,fl8,st0-0,ds0-0,di0-65535,nv0,lg-127..127,ph0..0,un0,ue0] feat=1/3[U0:0-0,r0,sb1,sx0,bs8,rc2,tb16,eb3,bf2,lc1,L1:1,fl8,st0-0,ds0-0,di0-65535,nv0,lg-127..127,ph0..0,un0,ue0]"),
    // keyboard
    ("keyboard", 'p', "05010906A101050719E029E71500250175019508810295017508810105081901290595057501910295017503910195017508B101C0",
     "u=1:6 nodes=[1:6/0/1/0/0/0] in=2/3[U7:e0-e7,r0,sb1,sx0,bs1,rc8,tb8,eb2,bf2,lc0,L1:6,fl1c,st0-0,ds0-0,di0-7,nv0,lg0..0,ph0..0,un0,ue0 U0:0-0,r0,sb2,sx0,bs8,rc1,tb8,eb3,bf1,lc0,L1:6,fle,st0-0,ds0-0,di8-7,nv0,lg1..0,ph0..0,un0,ue0] out=2/2[U8:1-5,r0,sb1,sx0,bs1,rc5,tb5,eb2,bf2,lc0,L1:6,fl1c,st0-0,ds0-0,di0-4,nv0,lg0..0,ph0..0,un0,ue0 U0:0-0,r0,sb1,sx5,bs3,rc1,tb3,eb2,bf1,lc0,L1:6,fle,st0-0,ds0-0,di5-4,nv0,lg1..0,ph0..0,un0,ue0] feat=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf1,lc0,L1:6,fle,st0-0,ds0-0,di0-65535,nv0,lg1..0,ph0..0,un0,ue0]"),
    // array
    ("array", 'p', "05010906A1010509190129051501250575089501810095017508910295017504B102C0",
     "u=1:6 nodes=[1:6/0/1/0/0/0] in=1/2[U9:1-5,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf0,lc0,L1:6,fl1c,st0-0,ds0-0,di0-4,nv1,lg5..0,ph0..0,un0,ue0] out=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg1..5,ph0..0,un0,ue0] feat=1/2[U0:0-0,r0,sb1,sx0,bs4,rc1,tb4,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg1..5,ph0..0,un0,ue0]"),
    // report_ids
    ("report_ids", 'p', "05010906A1018501050719E029E7150025017501950881028502950175089102850395027504B102C0",
     "u=1:6 nodes=[1:6/0/1/0/0/0] in=1/2[U7:e0-e7,r1,sb1,sx0,bs1,rc8,tb8,eb2,bf2,lc0,L1:6,fl1c,st0-0,ds0-0,di0-7,nv0,lg0..0,ph0..0,un0,ue0] out=1/2[U0:0-0,r2,sb1,sx0,bs8,rc1,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..1,ph0..0,un0,ue0] feat=1/2[U0:0-0,r3,sb1,sx0,bs4,rc2,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..1,ph0..0,un0,ue0]"),
    // usages_31_ok
    ("usages_31_ok", 'p', "05010906A1010901090109010901090109010901090109010901090109010901090109010901090109010901090109010901090109010901090109010901090109010901951F7501810295017508910295017504B102C0",
     "u=1:6 nodes=[1:6/0/1/0/0/0] in=31/5[U1:1-1,r0,sb4,sx6,bs1,rc1,tb1,eb5,bf2,lc0,L1:6,flc,st0-0,ds0-0,di0-0,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb4,sx5,bs1,rc1,tb1,eb5,bf2,lc0,L1:6,flc,st0-0,ds0-0,di1-1,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb4,sx4,bs1,rc1,tb1,eb5,bf2,lc0,L1:6,flc,st0-0,ds0-0,di2-2,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb4,sx3,bs1,rc1,tb1,eb5,bf2,lc0,L1:6,flc,st0-0,ds0-0,di3-3,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb4,sx2,bs1,rc1,tb1,eb5,bf2,lc0,L1:6,flc,st0-0,ds0-0,di4-4,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb4,sx1,bs1,rc1,tb1,eb5,bf2,lc0,L1:6,flc,st0-0,ds0-0,di5-5,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb4,sx0,bs1,rc1,tb1,eb5,bf2,lc0,L1:6,flc,st0-0,ds0-0,di6-6,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb3,sx7,bs1,rc1,tb1,eb4,bf2,lc0,L1:6,flc,st0-0,ds0-0,di7-7,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb3,sx6,bs1,rc1,tb1,eb4,bf2,lc0,L1:6,flc,st0-0,ds0-0,di8-8,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb3,sx5,bs1,rc1,tb1,eb4,bf2,lc0,L1:6,flc,st0-0,ds0-0,di9-9,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb3,sx4,bs1,rc1,tb1,eb4,bf2,lc0,L1:6,flc,st0-0,ds0-0,di10-10,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb3,sx3,bs1,rc1,tb1,eb4,bf2,lc0,L1:6,flc,st0-0,ds0-0,di11-11,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb3,sx2,bs1,rc1,tb1,eb4,bf2,lc0,L1:6,flc,st0-0,ds0-0,di12-12,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb3,sx1,bs1,rc1,tb1,eb4,bf2,lc0,L1:6,flc,st0-0,ds0-0,di13-13,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb3,sx0,bs1,rc1,tb1,eb4,bf2,lc0,L1:6,flc,st0-0,ds0-0,di14-14,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb2,sx7,bs1,rc1,tb1,eb3,bf2,lc0,L1:6,flc,st0-0,ds0-0,di15-15,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb2,sx6,bs1,rc1,tb1,eb3,bf2,lc0,L1:6,flc,st0-0,ds0-0,di16-16,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb2,sx5,bs1,rc1,tb1,eb3,bf2,lc0,L1:6,flc,st0-0,ds0-0,di17-17,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb2,sx4,bs1,rc1,tb1,eb3,bf2,lc0,L1:6,flc,st0-0,ds0-0,di18-18,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb2,sx3,bs1,rc1,tb1,eb3,bf2,lc0,L1:6,flc,st0-0,ds0-0,di19-19,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb2,sx2,bs1,rc1,tb1,eb3,bf2,lc0,L1:6,flc,st0-0,ds0-0,di20-20,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb2,sx1,bs1,rc1,tb1,eb3,bf2,lc0,L1:6,flc,st0-0,ds0-0,di21-21,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb2,sx0,bs1,rc1,tb1,eb3,bf2,lc0,L1:6,flc,st0-0,ds0-0,di22-22,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb1,sx7,bs1,rc1,tb1,eb2,bf2,lc0,L1:6,flc,st0-0,ds0-0,di23-23,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb1,sx6,bs1,rc1,tb1,eb2,bf2,lc0,L1:6,flc,st0-0,ds0-0,di24-24,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb1,sx5,bs1,rc1,tb1,eb2,bf2,lc0,L1:6,flc,st0-0,ds0-0,di25-25,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb1,sx4,bs1,rc1,tb1,eb2,bf2,lc0,L1:6,flc,st0-0,ds0-0,di26-26,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb1,sx3,bs1,rc1,tb1,eb2,bf2,lc0,L1:6,flc,st0-0,ds0-0,di27-27,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb1,sx2,bs1,rc1,tb1,eb2,bf2,lc0,L1:6,flc,st0-0,ds0-0,di28-28,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb1,sx1,bs1,rc1,tb1,eb2,bf2,lc0,L1:6,flc,st0-0,ds0-0,di29-29,nv0,lg0..0,ph0..0,un0,ue0 U1:1-1,r0,sb1,sx0,bs1,rc1,tb1,eb2,bf2,lc0,L1:6,flc,st0-0,ds0-0,di30-30,nv0,lg0..0,ph0..0,un0,ue0] out=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0] feat=1/2[U0:0-0,r0,sb1,sx0,bs4,rc1,tb4,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0]"),
    // usages_32_fail
    ("usages_32_fail", 'p', "05010906A1010901090109010901090109010901090109010901090109010901090109010901090109010901090109010901090109010901090109010901090109010901090195207501810295017508910295017504B102C0",
     "none"),
    // delimiter
    ("delimiter", 'p', "05010906A101A801810295017508910295017504B102C0",
     "none"),
    // pop_underflow
    ("pop_underflow", 'p', "05010906A101B4810295017508910295017504B102C0",
     "none"),
    // reserved_items
    ("reserved_items", 'p', "05010906A101AD07FC95017508810295017508910295017504B102C0",
     "u=1:6 nodes=[1:6/0/1/0/0/0] in=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0] out=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0] feat=1/2[U0:0-0,r0,sb1,sx0,bs4,rc1,tb4,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0]"),
    // truncated_item
    ("truncated_item", 'p', "05010906A101950175088102C092",
     "none"),
    // flag_bits
    ("flag_bits", 'p', "05010906A1019502750881079501750881429106B103C0",
     "u=1:6 nodes=[1:6/0/1/0/0/0] in=2/4[U0:0-0,r0,sb1,sx0,bs8,rc2,tb16,eb3,bf7,lc0,L1:6,fl2,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0 U0:0-0,r0,sb3,sx0,bs8,rc1,tb8,eb4,bf42,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv1,lg0..0,ph0..0,un0,ue0] out=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf6,lc0,L1:6,fl0,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0] feat=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf3,lc0,L1:6,fla,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0]"),
    // min_gt_max
    ("min_gt_max", 'p', "05010906A10105091905290295047501810295017508910295017504B102C0",
     "u=1:6 nodes=[1:6/0/1/0/0/0] in=1/2[U9:5-2,r0,sb1,sx0,bs1,rc4,tb4,eb2,bf2,lc0,L1:6,fl1c,st0-0,ds0-0,di0-65533,nv0,lg0..0,ph0..0,un0,ue0] out=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0] feat=1/2[U0:0-0,r0,sb1,sx0,bs4,rc1,tb4,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0]"),
    // empty_report_count
    ("empty_report_count", 'p', "05010906A1019500810275089502810295019102B102C0",
     "u=1:6 nodes=[1:6/0/1/0/0/0] in=2/3[U0:0-0,r0,sb1,sx0,bs8,rc2,tb16,eb3,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0] out=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0] feat=1/2[U0:0-0,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0]"),
    // usages_gt_count
    ("usages_gt_count", 'p', "05010906A10109300931093295017508810295039102B102C0",
     "u=1:6 nodes=[1:6/0/1/0/0/0] in=3/2[U1:32-32,r0,sb3,sx0,bs8,rc65535,tb65528,eb8194,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-0,nv0,lg0..0,ph0..0,un0,ue0 U1:31-31,r0,sb2,sx0,bs8,rc1,tb8,eb3,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di1-1,nv0,lg0..0,ph0..0,un0,ue0 U1:30-30,r0,sb1,sx0,bs8,rc1,tb8,eb2,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di2-2,nv0,lg0..0,ph0..0,un0,ue0] out=1/4[U0:0-0,r0,sb1,sx0,bs8,rc3,tb24,eb4,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0] feat=1/4[U0:0-0,r0,sb1,sx0,bs8,rc3,tb24,eb4,bf2,lc0,L1:6,fl8,st0-0,ds0-0,di0-65535,nv0,lg0..0,ph0..0,un0,ue0]"),
    // tlc_two
    ("tlc_two", 't', "05010902A1010901C0050C0901A1010902C0",
     "bounds=0,9,18"),
    // tlc_unbalanced_end
    ("tlc_unbalanced_end", 't', "C0A101C00501A101C0",
     "bounds=0"),
    // tlc_truncated_tail
    ("tlc_truncated_tail", 't', "A101C092",
     "bounds=0,3"),
    // device_two_tlc
    ("device_two_tlc", 'c', "05010902A10195027501810275089102B102C005010906A10195027501810275089102B102C0",
     "ok tlc=2 [1:2/1/2/3/3 1:6/2/2/3/3] rid=[R0/c2/2/3/3]"),
    // device_tlc_delimiter
    ("device_tlc_delimiter", 'c', "05010902A10195027501810275089102B102C005010906A101A801C0",
     "ok tlc=1 [1:2/1/2/3/3] rid=[R0/c1/2/3/3]"),
    // device_report_ids
    ("device_report_ids", 'c', "05010902A101850195027501810275089102B102C005010906A10185029501750881029102B102C0",
     "ok tlc=2 [1:2/1/2/3/3 1:6/2/2/2/2] rid=[R1/c1/2/3/3 R2/c2/2/2/2]"),
    // device_no_end
    ("device_no_end", 'c', "05010902A1018102",
     "ok tlc=0 [] rid=[]"),
];

fn unhex(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn caps_text(c: &ValueCaps) -> String {
    format!(
        "U{:x}:{:x}-{:x},r{},sb{},sx{},bs{},rc{},tb{},eb{},bf{:x},lc{},L{:x}:{:x},fl{:x},st{}-{},ds{}-{},di{}-{},nv{},lg{}..{},ph{}..{},un{},ue{}",
        c.usage_page,
        c.usage_min,
        c.usage_max,
        c.report_id,
        c.start_byte,
        c.start_bit,
        c.bit_size,
        c.report_count,
        c.total_bits,
        c.end_byte,
        c.bit_field,
        c.link_collection,
        c.link_usage_page,
        c.link_usage,
        c.flags,
        c.string_min,
        c.string_max,
        c.designator_min,
        c.designator_max,
        c.data_index_min,
        c.data_index_max,
        c.null_value,
        c.logical_min,
        c.logical_max,
        c.physical_min,
        c.physical_max,
        c.units,
        c.units_exp,
    )
}

fn caps_list(values: &[ValueCaps]) -> String {
    let items: Vec<String> = values.iter().map(caps_text).collect();
    format!("[{}]", items.join(" "))
}

fn node_text(node: &CollectionNode) -> String {
    format!(
        "{:x}:{:x}/{}/{:x}/{}/{}/{}",
        node.usage_page,
        node.usage,
        node.parent,
        node.collection_type,
        node.number_of_children,
        node.next_sibling,
        node.first_child,
    )
}

fn dump(data: &PreparsedData) -> String {
    let nodes: Vec<String> = data.collections.iter().map(node_text).collect();
    format!(
        "u={:x}:{:x} nodes=[{}] in={}/{}{} out={}/{}{} feat={}/{}{}",
        data.usage_page,
        data.usage,
        nodes.join(" "),
        data.input.count,
        data.input.byte_length,
        caps_list(&data.input.values),
        data.output.count,
        data.output.byte_length,
        caps_list(&data.output.values),
        data.feature.count,
        data.feature.byte_length,
        caps_list(&data.feature.values),
    )
}

fn collection_text(c: &CollectionDesc) -> String {
    format!(
        "{:x}:{:x}/{}/{}/{}/{}",
        c.usage_page,
        c.usage,
        c.collection_number,
        c.input_length,
        c.output_length,
        c.feature_length
    )
}

fn report_id_text(r: &ReportIdDesc) -> String {
    format!(
        "R{}/c{}/{}/{}/{}",
        r.report_id, r.collection_number, r.input_length, r.output_length, r.feature_length
    )
}

fn describe(bytes: &[u8]) -> String {
    match get_collection_description(bytes) {
        Err(_) => "err".to_string(),
        Ok(device) => {
            let collections: Vec<String> = device.collections.iter().map(collection_text).collect();
            let report_ids: Vec<String> = device.report_ids.iter().map(report_id_text).collect();
            format!(
                "ok tlc={} [{}] rid=[{}]",
                device.collections.len(),
                collections.join(" "),
                report_ids.join(" ")
            )
        }
    }
}

fn replay(op: char, hex: &str) -> String {
    let bytes = unhex(hex);
    match op {
        'p' => match parse_descriptor(&bytes) {
            None => "none".to_string(),
            Some(data) => dump(&data),
        },
        't' => {
            let bounds: Vec<String> = top_level_collections(&bytes)
                .iter()
                .map(usize::to_string)
                .collect();
            format!("bounds={}", bounds.join(","))
        }
        'c' => describe(&bytes),
        _ => panic!("unknown oracle op {op:?}"),
    }
}

#[test]
fn oracle_rows_match_wine() {
    for (name, op, hex, expected) in ROWS {
        assert_eq!(replay(*op, hex), *expected, "row {name}");
    }
}

#[test]
fn oracle_failure_text() {
    assert_eq!(replay('p', "92"), "none");
    assert_eq!(replay('c', "A101A8C0"), "err");
}

#[test]
#[should_panic(expected = "unknown oracle op")]
fn unknown_oracle_op_panics() {
    replay('x', "");
}
