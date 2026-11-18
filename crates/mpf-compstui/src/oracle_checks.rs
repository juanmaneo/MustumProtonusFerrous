//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/compstui/harness.c`, built with ASan and UBSan, no reports).
//! Each row is `(ops, replies)`: the ops run in order on one fresh handle table.

use crate::handles::{HandleTable, Kind};
use crate::page::{
    page_copy_layout, strip_page_info, PROPSHEETPAGEA_V1_SIZE, PROPSHEETPAGEW_V1_SIZE, PSPINFO_SIZE,
};
use crate::{build_sheet, sheet_result, SheetHeader, SheetIcon, SheetStrings};

/// Rows recorded from the verbatim Wine C code: `(ops, replies)`.
#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    ("sizes", "info=32 pspa_v1=72 pspw_v1=72"),
    ("alloc 1|alloc 2|alloc 3|alloc 0|alloc 4", "0x43440001|0x43440002|0x43440003|0|0"),
    ("alloc 1|alloc 2|free 0x43440001|alloc 3|get 0x43440001|get 0x43440002|get 0x43440003", "0x43440001|0x43440002|ok|0x43440001|3|2|0"),
    ("alloc 1|free 0x43440001|free 0x43440001|free 0|get 0x43440001", "0x43440001|ok|ok|ok|0"),
    ("get 0|get 0x43440fff|get 0x43441000|get 0x4343ffff", "0|0|0|0"),
    ("fill 1|fill 3|alloc 2", "n=4096|n=0|0"),
    ("alloc 1|alloc 2|alloc 1|free 0x43440002|free 0x43440001|alloc 2|alloc 2|alloc 3", "0x43440001|0x43440002|0x43440003|ok|ok|0x43440001|0x43440002|0x43440004"),
    ("alloc 2|free 0x43440001|fill 1", "0x43440001|ok|n=4096"),
    ("sheet 0 0 - 0 1", "rc=-8"),
    ("sheet 0x10 0 Hello~World 1 2", "flags=0 icon=none title=Hello World Default pages=1 rc=2"),
    ("sheet 0x2 0x2000 - 1 3", "flags=84 icon=id:8192 title=Options pages=1 rc=3"),
    ("sheet 0 0 = 1 -1", "flags=0 icon=none title= pages=1 rc=-1"),
    ("sheet 0x8 0x1234 - 2 1", "flags=2 icon=h:0x1234 title=Options pages=2 rc=1"),
    ("sheet 0x8 0 Title 1 0", "flags=2 icon=h:0x0 title=Title pages=1 rc=1"),
    ("sheet 0 64111 - 1 0|sheet 0 64112 - 1 0|sheet 0 63999 - 1 0", "flags=0 icon=none title=Options pages=1 rc=1|flags=4 icon=id:64112 title=Options pages=1 rc=1|flags=4 icon=id:63999 title=Options pages=1 rc=1"),
    ("sheet 0x30 0 - 1 2", "flags=0 icon=none title=Options Default pages=1 rc=2"),
    ("sheet 0x34 0 Mine~Printer 1 0", "flags=0 icon=none title=Mine Printer pages=1 rc=1"),
    ("sheet 0x14 0 - 1 0", "flags=0 icon=none title=Options Default Properties pages=1 rc=1"),
    ("sheet 0x10 0 rep:255 1 0", "flags=0 icon=none title=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx pages=1 rc=1"),
    ("sheet 0x10 0 rep:250 1 0", "flags=0 icon=none title=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx Defa pages=1 rc=1"),
    ("sheet 0x10 0 rep:256 1 0|sheet 0x34 0 rep:300 1 0", "flags=0 icon=none title= Default pages=1 rc=1|flags=0 icon=none title= pages=1 rc=1"),
    ("sheet 0x1 0 - 1 5|sheet 0 0 - 1 3|sheet 0 0 - 1 2", "flags=0 icon=none title=Options pages=1 rc=1|flags=0 icon=none title=Options pages=1 rc=3|flags=0 icon=none title=Options pages=1 rc=2"),
    ("addpage A 72|addpage W 72|addpage A 71|addpage W 0", "ok copy=104|ok copy=104|none|none"),
    ("addpage A 0x100|addpage W 200", "ok copy=288|ok copy=232"),
    ("initdlg A 104|initdlg W 104|initdlg A 80|initdlg W 72", "dwSize=72 ret=1 dlg=1 setwl=1|dwSize=72 ret=1 dlg=1 setwl=1|dwSize=48 ret=1 dlg=1 setwl=1|dwSize=40 ret=1 dlg=1 setwl=1"),
];

#[test]
fn oracle_rows_match_wine() {
    for (ops, expected) in ROWS {
        assert_eq!(replay(ops), *expected, "ops: {ops}");
    }
}

#[test]
fn unknown_op_is_bad() {
    assert_eq!(run_op(&mut HandleTable::default(), "bogus"), "bad-op");
}

#[test]
fn page_layout_edges() {
    assert_eq!(
        page_copy_layout(71, PROPSHEETPAGEA_V1_SIZE, PSPINFO_SIZE),
        None
    );
    let copy = page_copy_layout(72, PROPSHEETPAGEW_V1_SIZE, PSPINFO_SIZE).unwrap();
    assert_eq!((copy.size, copy.info_offset), (104, 72));
    assert_eq!(strip_page_info(10, PSPINFO_SIZE), None);
    assert_eq!(strip_page_info(104, PSPINFO_SIZE), Some(72));
}

#[test]
fn sheet_without_pages_is_rejected() {
    let [options, default, properties] = STRINGS.map(wide);
    let strings = SheetStrings {
        options: &options,
        default: &default,
        properties: &properties,
    };
    let header = SheetHeader {
        flags: 0,
        title: None,
        icon: 0,
    };
    assert_eq!(build_sheet(&header, &strings, 0).unwrap_err(), -8);
}

fn replay(ops: &str) -> String {
    let mut table = HandleTable::default();
    ops.split('|')
        .map(|op| run_op(&mut table, op))
        .collect::<Vec<_>>()
        .join("|")
}

fn run_op(table: &mut HandleTable<()>, op: &str) -> String {
    let t: Vec<&str> = op.split(' ').collect();
    match t[0] {
        "sizes" => format!(
            "info={PSPINFO_SIZE} pspa_v1={PROPSHEETPAGEA_V1_SIZE} pspw_v1={PROPSHEETPAGEW_V1_SIZE}"
        ),
        "alloc" => Kind::from_raw(num(t[1]) as i32)
            .and_then(|kind| table.alloc(kind, ()))
            .map_or_else(|| "0".to_string(), |handle| format!("0x{handle:x}")),
        "free" => {
            table.free(num(t[1]) as u64);
            "ok".to_string()
        }
        "get" => table
            .get(num(t[1]) as u64)
            .map_or(0, |(kind, _)| kind as i32)
            .to_string(),
        "fill" => format!("n={}", fill(table, num(t[1]) as i32)),
        "sheet" => sheet_reply(&t),
        "addpage" => addpage_reply(t[1], num(t[2]) as u32),
        "initdlg" => initdlg_reply(num(t[2]) as u32),
        _ => "bad-op".to_string(),
    }
}

fn fill(table: &mut HandleTable<()>, raw: i32) -> usize {
    Kind::from_raw(raw).map_or(0, |kind| {
        std::iter::from_fn(|| table.alloc(kind, ())).count()
    })
}

fn sheet_reply(t: &[&str]) -> String {
    let pages = num(t[4]) as usize;
    let title = parse_title(t[3]);
    let icon = num(t[2]) as u64;
    let header = SheetHeader {
        flags: num(t[1]) as u16,
        title: title.as_deref(),
        icon,
    };
    let [options, default, properties] = STRINGS.map(wide);
    let strings = SheetStrings {
        options: &options,
        default: &default,
        properties: &properties,
    };
    match build_sheet(&header, &strings, pages) {
        Err(code) => format!("rc={code}"),
        Ok(request) => format!(
            "flags={:x} icon={} title={} pages={pages} rc={}",
            request.flags,
            icon_text(request.icon),
            ascii(&request.title),
            sheet_result(num(t[5]) as isize)
        ),
    }
}

fn addpage_reply(wide: &str, dw_size: u32) -> String {
    let v1 = if wide == "W" {
        PROPSHEETPAGEW_V1_SIZE
    } else {
        PROPSHEETPAGEA_V1_SIZE
    };
    page_copy_layout(dw_size, v1, PSPINFO_SIZE).map_or_else(
        || "none".to_string(),
        |copy| format!("ok copy={}", copy.size),
    )
}

/// The dialog stubs of the harness always return 1 and are always called.
fn initdlg_reply(copy_size: u32) -> String {
    let dw_size = strip_page_info(copy_size, PSPINFO_SIZE).unwrap();
    format!("dwSize={dw_size} ret=1 dlg=1 setwl=1")
}

/// The resource strings that the harness's `LoadStringW` stub returns: options, default, properties.
const STRINGS: [&str; 3] = ["Options", "Default", "Properties"];

fn icon_text(icon: SheetIcon) -> String {
    match icon {
        SheetIcon::None => "none".to_string(),
        SheetIcon::Handle(handle) => format!("h:0x{handle:x}"),
        SheetIcon::Id(id) => format!("id:{id}"),
    }
}

/// The harness prints non-ASCII units as `?`.
fn ascii(text: &[u16]) -> String {
    text.iter()
        .map(|&unit| {
            u8::try_from(unit)
                .ok()
                .filter(u8::is_ascii)
                .map_or('?', char::from)
        })
        .collect()
}

/// The `title` argument of the harness: `-` is NULL, `=` is empty, `rep:N` is N `x`s, and `~` is a space.
fn parse_title(text: &str) -> Option<Vec<u16>> {
    match text {
        "-" => None,
        "=" => Some(Vec::new()),
        _ => Some(match text.strip_prefix("rep:") {
            Some(count) => vec![u16::from(b'x'); num(count) as usize],
            None => wide(&text.replace('~', " ")),
        }),
    }
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// `strtol`/`strtoul` with base 0: an optional minus sign, then `0x` hex or decimal.
fn num(text: &str) -> i64 {
    let (negative, body) = text
        .strip_prefix('-')
        .map_or((false, text), |rest| (true, rest));
    let magnitude = body.strip_prefix("0x").map_or_else(
        || body.parse::<i64>().unwrap(),
        |hex| i64::from_str_radix(hex, 16).unwrap(),
    );
    if negative {
        -magnitude
    } else {
        magnitude
    }
}
