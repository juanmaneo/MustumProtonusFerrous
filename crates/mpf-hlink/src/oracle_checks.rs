//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/hlink/harness.c`, built with ASan and UBSan). Each row is
//! `(spec, output)`: `replay` runs the spec through the port and prints the result the
//! way the harness does.

use super::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

/// Rows recorded from the verbatim Wine C code: `(spec, harness output)`.
#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    // split_hash_target
    ("op=split tgt=a#b loc=-", "tgt=a loc=b"),
    // split_hash_location_given
    ("op=split tgt=a#b loc=c", "tgt=a loc=c"),
    // split_hash_at_start
    ("op=split tgt=#b loc=-", "tgt=- loc=b"),
    // split_empty_location_after_hash
    ("op=split tgt=a# loc=-", "tgt=a loc=~"),
    // split_second_hash_kept_in_location
    ("op=split tgt=a#b#c loc=-", "tgt=a loc=b#c"),
    // split_no_hash
    ("op=split tgt=abc loc=x", "tgt=abc loc=x"),
    // strip_file_slashes
    ("op=strip_file s=file:///C:/x", "eaten=8"),
    // strip_file_no_prefix
    ("op=strip_file s=fil:x", "eaten=0"),
    // shortcut_null
    ("op=shortcut s=-", "hr=80070057"),
    // shortcut_url_upper
    ("op=shortcut s=a.URL", "hr=00000000"),
    // shortcut_bare_ext
    ("op=shortcut s=.url", "hr=00000000"),
    // shortcut_too_short
    ("op=shortcut s=url", "hr=00000001"),
    // save_flags_all
    ("op=save_flags mon=1 abs=1 loc=1 fn=1 tf=1", "flags=0000009f"),
    // save_flags_absolute_only
    ("op=save_flags mon=0 abs=1 loc=0 fn=0 tf=0", "flags=00000002"),
    // parse_header_ok
    ("op=parse_header hex=0200000009000000", "hr=00000000 flags=00000009 unk=00000000"),
    // parse_header_bad_magic
    ("op=parse_header hex=0300000009000000", "hr=80004005"),
    // parse_header_unknown_bits
    ("op=parse_header hex=02000000c0000000", "hr=00000000 flags=000000c0 unk=00000040"),
    // parse_header_short
    ("op=parse_header hex=02000000", "hr=80004005"),
    // write_string_ab
    ("op=write_string s=ab", "03000000610062000000"),
    // read_string_ab
    ("op=read_string hex=03000000610062000000", "hr=00000000 s=ab\\z used=10"),
    // read_string_short
    ("op=read_string hex=0300000061006200", "hr=8003001e"),
    // size_max_no_moniker
    ("op=size_max tf=- fn=- loc=- mon=-", "size=8 hr=80004005"),
    // size_max_all
    ("op=size_max tf=abc fn=- loc=xy mon=10", "size=40 hr=00000000"),
    // headers_append_crlf
    ("op=headers s=a", "s=a\\r\\n"),
    // headers_keep_newline
    ("op=headers s=a\\n", "s=a\\n"),
];

type Fields<'a> = HashMap<&'a str, &'a str>;

/// Runs one spec through the port and prints the result the way the harness does.
pub(super) fn replay(spec: &str) -> String {
    let f: Fields = spec
        .split(' ')
        .map(|token| token.split_once('=').expect("spec token needs key=value"))
        .collect();
    match f["op"] {
        "split" => {
            let target = text(&f, "tgt");
            let location = text(&f, "loc");
            let (target, location) = split_target(target.as_deref(), location.as_deref());
            format!("tgt={} loc={}", shown(target), shown(location))
        }
        "strip_file" => {
            let name = text(&f, "s").expect("strip_file needs s");
            format!("eaten={}", name.len() - strip_file_scheme(&name).len())
        }
        "shortcut" => {
            let name = text(&f, "s");
            format!("hr={:08x}", is_shortcut(name.as_deref()))
        }
        "save_flags" => format!(
            "flags={:08x}",
            save_flags(
                flag(f["mon"]),
                flag(f["abs"]),
                flag(f["loc"]),
                flag(f["fn"]),
                flag(f["tf"])
            )
        ),
        "parse_header" => match parse_header(&hex(f["hex"])) {
            Ok(flags) => format!(
                "hr={S_OK:08x} flags={flags:08x} unk={:08x}",
                unknown_flags(flags)
            ),
            Err(hr) => format!("hr={hr:08x}"),
        },
        "write_string" => {
            let name = text(&f, "s").expect("write_string needs s");
            hex_text(&encode_string(&name))
        }
        "read_string" => match decode_string(&hex(f["hex"])) {
            Ok((decoded, used)) => format!("hr={S_OK:08x} s={} used={used}", render(&decoded)),
            Err(hr) => format!("hr={hr:08x}"),
        },
        "size_max" => {
            let target_frame = text(&f, "tf");
            let friendly = text(&f, "fn");
            let location = text(&f, "loc");
            let moniker = match f["mon"] {
                "-" => None,
                raw => Some(number::<u64>(raw)),
            };
            let (hr, size) = size_max(
                target_frame.as_deref(),
                friendly.as_deref(),
                location.as_deref(),
                moniker,
            );
            format!("size={size} hr={hr:08x}")
        }
        "headers" => {
            let headers = normalize_headers(text(&f, "s").as_deref());
            format!("s={}", shown(headers.as_deref()))
        }
        "special" => match special_value_name(number(f["ref"])) {
            Ok(name) => format!(
                "hr={S_OK:08x} name={}",
                render(&name.encode_utf16().collect::<Vec<u16>>())
            ),
            Err(hr) => format!("hr={hr:08x}"),
        },
        "check_get_ref" => format!(
            "hr={:08x}",
            hr_of(check_get_string_reference(number(f["which"])))
        ),
        "resolve_ref" => format!("ref={}", resolve_ref_type(number(f["which"]))),
        "set_moniker" => match set_moniker_reference_plan(number(f["flags"])) {
            Ok(plan) => format!(
                "hr={S_OK:08x} target={} location={}",
                u8::from(plan.target),
                u8::from(plan.location)
            ),
            Err(hr) => format!("hr={hr:08x}"),
        },
        "set_string" => {
            let target = text(&f, "tgt");
            let location = text(&f, "loc");
            match set_string_reference_plan(
                number(f["flags"]),
                target.as_deref(),
                location.as_deref(),
            ) {
                Ok(plan) => format!(
                    "hr={S_OK:08x} clear={} tgt={} repl={} loc={}",
                    u8::from(plan.clear_target),
                    shown(plan.target),
                    u8::from(plan.replace_location),
                    shown(plan.location)
                ),
                Err(hr) => format!("hr={hr:08x}"),
            }
        }
        "fallback" => {
            let target = text(&f, "tgt").expect("fallback needs tgt");
            let kind = if fallback_is_url(&target) {
                "url"
            } else {
                "file"
            };
            format!("kind={kind}")
        }
        "abs_name" => {
            let name = text(&f, "s");
            format!(
                "abs={}",
                u8::from(is_absolute_display_name(name.as_deref()))
            )
        }
        "update_stack" => format!(
            "hr={:08x}",
            hr_of(check_update_stack_item(flag(f["frame"]), flag(f["bc"])))
        ),
        "clone" => format!(
            "hr={:08x}",
            hr_of(check_clone(
                flag(f["hlink"]),
                flag(f["riid"]),
                flag(f["obj"])
            ))
        ),
        other => panic!("unknown oracle op {other}"),
    }
}

/// The value of a spec field: `-` is NULL, `~` is an empty string, and anything else is
/// read by `units`.
fn text(f: &Fields<'_>, key: &str) -> Option<Vec<u16>> {
    match f.get(key).copied() {
        None | Some("-") => None,
        Some("~") => Some(Vec::new()),
        Some(raw) => Some(units(raw)),
    }
}

/// Expands the spec escapes `\n \r \s \z` and `\X` (any other X stands for itself).
/// Only ASCII is accepted, as the harness also requires.
pub(super) fn units(raw: &str) -> Vec<u16> {
    let bytes = raw.as_bytes();
    let mut out = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] == b'\\' && at + 1 < bytes.len() {
            at += 1;
            out.push(match bytes[at] {
                b'n' => 0x0A,
                b'r' => 0x0D,
                b's' => 0x20,
                b'z' => 0,
                other => u16::from(other),
            });
        } else {
            assert!(bytes[at] < 0x7F, "non-ASCII input");
            out.push(u16::from(bytes[at]));
        }
        at += 1;
    }
    out
}

/// The inverse of `units` for the harness output. Empty text prints as `~`.
pub(super) fn render(s: &[u16]) -> String {
    if s.is_empty() {
        return "~".to_string();
    }
    let mut out = String::new();
    for &unit in s {
        match unit {
            0x0A => out.push_str("\\n"),
            0x0D => out.push_str("\\r"),
            0x20 => out.push_str("\\s"),
            0 => out.push_str("\\z"),
            0x21..=0x7E => out.push(char::from(unit as u8)),
            other => panic!("non-printable unit {other:x}"),
        }
    }
    out
}

fn shown(s: Option<&[u16]>) -> String {
    match s {
        None => "-".to_string(),
        Some(s) => render(s),
    }
}

fn hr_of(result: Result<(), HResult>) -> HResult {
    match result {
        Ok(()) => S_OK,
        Err(hr) => hr,
    }
}

fn hex(raw: &str) -> Vec<u8> {
    (0..raw.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&raw[at..at + 2], 16).expect("hex byte"))
        .collect()
}

fn hex_text(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn number<T: FromStr>(raw: &str) -> T
where
    T::Err: Debug,
{
    raw.parse().expect("hlink spec number")
}

fn flag(raw: &str) -> bool {
    raw == "1"
}

#[test]
fn oracle_rows_match_wine() {
    for &(spec, expected) in ROWS {
        assert_eq!(replay(spec), expected, "spec {spec}");
    }
}

#[test]
#[should_panic(expected = "unknown oracle op bogus")]
fn unknown_op_panics() {
    replay("op=bogus");
}
