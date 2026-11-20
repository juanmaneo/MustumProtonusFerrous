//! Replays the commands of the Wine oracle (`target/canon/propsys`) against this crate.
//! The vectors are the verbatim output of Wine's `propsys_main.c` and `propvar.c` under
//! ASan/UBSan, printed by `harness.c`. Each row is a harness command line and its stdout.

use super::*;

#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("key_from_string {12345678-9abc-def0-1234-56789abcdef0}%2025", "hr=00000000 key={12345678-9ABC-DEF0-1234-56789ABCDEF0}/25"),
    ("key_from_string {12345678-zzzz-def0-1234-56789abcdef0}%2025", "hr=80070057 key={12345678-0000-0000-0000-000000000000}/0"),
    ("key_to_string 12345678abcdabcd1234567890abcdef 25 41", "hr=8007007A buf=000000310032003300340035003600370038002D0041004200430044002D0041004200430044002D0031003200330034002D003500360037003800390030004100420043004400450046007D002000000032"),
    ("key_to_string 12345678abcdabcd1234567890abcdef 4294967295 45", "hr=8007007A buf=000000310032003300340035003600370038002D0041004200430044002D0041004200430044002D0031003200330034002D003500360037003800390030004100420043004400450046007D0020000000390032003700360039"),
    ("key_to_string ! 25 41", "hr=8007007A buf=0000CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC"),
    ("guid_from_propvar LPWSTR:{12345678-9abc-def0-1234-56789abcdefg}", "hr=00000000 guid={12345678-9ABC-DEF0-1234-56789ABCDEFF}"),
    ("guid_from_propvar CLSID:12345678abcdabcd1234567890abcdef", "hr=00000000 guid={12345678-ABCD-ABCD-1234-567890ABCDEF}"),
    ("to_i16 I4:40000", "hr=80070216 v=0"),
    ("to_i32 UI4:4294967295", "hr=80070216 v=0"),
    ("to_i64 NULL", "hr=80004001 v=0"),
    ("to_i64 LPSTR:%20%20010", "hr=00000000 v=8"),
    ("to_i64 LPWSTR:4294967296", "hr=00000000 v=2147483647"),
    ("to_i64 I2:-5", "hr=00000000 v=-5"),
    ("to_u16 I1:-1", "hr=80070216 v=0"),
    ("to_u16 UI2:65535", "hr=00000000 v=65535"),
    ("to_u32 EMPTY", "hr=00000000 v=0"),
    ("to_u64 I8:-5", "hr=80070216 v=0"),
    ("to_u64 UI8:18446744073709551615", "hr=00000000 v=18446744073709551615"),
    ("double R8:1.5", "hr=00000000 bits=3FF8000000000000"),
    ("double R4:2.5", "hr=00000000 bits=4004000000000000"),
    ("bool BOOL:-1", "hr=00000000 v=1"),
    ("buffer VEC_UI1:01ff10 4", "hr=80004005 buf=CCCCCCCC"),
    ("u32def UI1:7 9", "v=7"),
    ("str_default BSTR:ab%20x !", "s=0061006200200078"),
    ("str_default I4:1 !", "s=!"),
];

/// Runs one harness command and returns the line the harness prints for it.
fn replay(command: &str) -> String {
    let words: Vec<&str> = command.split(' ').collect();
    match words.as_slice() {
        ["key_from_string", text] => {
            let text = wide_arg(text);
            let mut key = PropertyKey::default();
            let hr = ps_property_key_from_string(text.as_deref(), Some(&mut key));
            format!("hr={hr:08X} key={}/{}", guid_text(&key.fmtid), key.pid)
        }
        ["key_to_string", guid, pid, cch] => {
            let key = (*guid != "!").then(|| PropertyKey {
                fmtid: guid_arg(guid),
                pid: pid.parse().unwrap(),
            });
            let mut buf = vec![0xCCCC_u16; cch.parse().unwrap()];
            let hr = ps_string_from_property_key(key.as_ref(), Some(&mut buf[..]));
            format!("hr={hr:08X} buf={}", hex_units(&buf))
        }
        ["guid_from_propvar", spec] => {
            let mut guid = Guid::default();
            let hr = prop_variant_to_guid(&arg(spec), &mut guid);
            format!("hr={hr:08X} guid={}", guid_text(&guid))
        }
        ["to_i16", spec] => {
            let mut v = 0;
            let hr = prop_variant_to_int16(&arg(spec), &mut v);
            format!("hr={hr:08X} v={v}")
        }
        ["to_i32", spec] => {
            let mut v = 0;
            let hr = prop_variant_to_int32(&arg(spec), &mut v);
            format!("hr={hr:08X} v={v}")
        }
        ["to_i64", spec] => {
            let mut v = 0;
            let hr = prop_variant_to_int64(&arg(spec), &mut v);
            format!("hr={hr:08X} v={v}")
        }
        ["to_u16", spec] => {
            let mut v = 0;
            let hr = prop_variant_to_uint16(&arg(spec), &mut v);
            format!("hr={hr:08X} v={v}")
        }
        ["to_u32", spec] => {
            let mut v = 0;
            let hr = prop_variant_to_uint32(&arg(spec), &mut v);
            format!("hr={hr:08X} v={v}")
        }
        ["to_u64", spec] => {
            let mut v = 0;
            let hr = prop_variant_to_uint64(&arg(spec), &mut v);
            format!("hr={hr:08X} v={v}")
        }
        ["double", spec] => {
            let mut d = 0.0;
            let hr = prop_variant_to_double(&arg(spec), &mut d);
            format!("hr={hr:08X} bits={:016X}", d.to_bits())
        }
        ["bool", spec] => {
            let mut b = false;
            let hr = prop_variant_to_boolean(&arg(spec), &mut b);
            format!("hr={hr:08X} v={}", i32::from(b))
        }
        ["buffer", spec, cb] => {
            let mut dst = [0xCC_u8; 256];
            let cb: usize = cb.parse().unwrap();
            let hr = prop_variant_to_buffer(&arg(spec), &mut dst[..cb]);
            let bytes: String = dst[..cb].iter().map(|b| format!("{b:02X}")).collect();
            format!("hr={hr:08X} buf={bytes}")
        }
        ["u32def", spec, default] => {
            let v = prop_variant_to_uint32_with_default(&arg(spec), default.parse().unwrap());
            format!("v={v}")
        }
        ["str_default", spec, default] => {
            let default = wide_arg(default);
            match prop_variant_to_string_with_default(&arg(spec), default.as_deref()) {
                None => "s=!".to_string(),
                Some(s) => format!("s={}", hex_units(s)),
            }
        }
        _ => panic!("unknown command {command}"),
    }
}

/// Builds a `PropValue` from `TYPE[:payload]`, as the harness's `value_arg` does.
fn arg(spec: &str) -> PropValue {
    let (name, payload) = spec.split_once(':').unwrap_or((spec, ""));
    match name {
        "EMPTY" => PropValue::Empty,
        "NULL" => PropValue::Null,
        "I1" => PropValue::I1(num(payload) as i8),
        "UI1" => PropValue::Ui1(num(payload) as u8),
        "I2" => PropValue::I2(num(payload) as i16),
        "UI2" => PropValue::Ui2(num(payload) as u16),
        "I4" => PropValue::I4(num(payload) as i32),
        "UI4" => PropValue::Ui4(num(payload) as u32),
        "I8" => PropValue::I8(num(payload) as i64),
        "UI8" => PropValue::Ui8(num(payload) as u64),
        "BOOL" => PropValue::Bool(num(payload) as i16),
        "R4" => PropValue::R4(payload.parse().unwrap()),
        "R8" => PropValue::R8(payload.parse().unwrap()),
        "LPSTR" => PropValue::LpStr((payload != "!").then(|| decode(payload))),
        "LPWSTR" => PropValue::LpWStr(wide_arg(payload)),
        "BSTR" => PropValue::Bstr(wide_arg(payload)),
        "CLSID" => PropValue::Clsid((payload != "!").then(|| guid_arg(payload))),
        "VEC_UI1" => PropValue::VectorUi1(hex_bytes(payload)),
        _ => panic!("unknown type {name}"),
    }
}

fn num(text: &str) -> i128 {
    text.parse().unwrap()
}

/// Percent-decodes a harness argument. The table only contains well-formed `%XX` escapes.
fn decode(text: &str) -> Vec<u8> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            out.push(u8::from_str_radix(&text[i + 1..i + 3], 16).unwrap());
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

/// The harness's `wide_arg`: `!` is a NULL pointer, anything else is one unit per byte.
fn wide_arg(text: &str) -> Option<Vec<u16>> {
    (text != "!").then(|| decode(text).into_iter().map(u16::from).collect())
}

fn hex_bytes(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

/// Parses 32 hex digits as `Data1` (8), `Data2` (4), `Data3` (4) and `Data4` (16).
fn guid_arg(text: &str) -> Guid {
    let b = hex_bytes(text);
    Guid {
        data1: u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
        data2: u16::from_be_bytes([b[4], b[5]]),
        data3: u16::from_be_bytes([b[6], b[7]]),
        data4: b[8..16].try_into().unwrap(),
    }
}

fn guid_text(g: &Guid) -> String {
    let tail: String = g.data4.iter().map(|b| format!("{b:02X}")).collect();
    format!(
        "{{{:08X}-{:04X}-{:04X}-{}-{}}}",
        g.data1,
        g.data2,
        g.data3,
        &tail[..4],
        &tail[4..]
    )
}

fn hex_units(units: &[u16]) -> String {
    units.iter().map(|u| format!("{u:04X}")).collect()
}

#[test]
fn oracle_rows_match_wine() {
    for &(command, expected) in VECTORS {
        assert_eq!(replay(command), expected, "row `{command}`");
    }
}

#[test]
#[should_panic(expected = "unknown command")]
fn unknown_command_panics() {
    replay("to_i128 I4:1");
}

#[test]
#[should_panic(expected = "unknown type")]
fn unknown_type_panics() {
    arg("I128:1");
}
