//! Edge cases of the port. Most rows are spec strings that the Wine C harness ran, so
//! their expected values come from C. The direct tests cover what the harness cannot
//! reach: non-ASCII input, embedded NULs, the divergences listed in `lib.rs`, and the
//! derived traits.

use super::*;
use crate::oracle_checks::{render, replay, units};
use crate::save::string_size;

/// Edge specs recorded from the verbatim Wine C code: `(spec, harness output)`.
#[rustfmt::skip]
const EDGE_ROWS: &[(&str, &str)] = &[
    // split_no_target
    ("op=split tgt=- loc=x", "tgt=- loc=x"),
    // split_empty_target
    ("op=split tgt=~ loc=-", "tgt=~ loc=-"),
    // split_hash_only
    ("op=split tgt=# loc=-", "tgt=- loc=~"),
    // strip_file_upper
    ("op=strip_file s=FILE:x", "eaten=5"),
    // strip_file_bare
    ("op=strip_file s=file:", "eaten=5"),
    // shortcut_txt
    ("op=shortcut s=a.txt", "hr=00000001"),
    // shortcut_empty
    ("op=shortcut s=~", "hr=00000001"),
    // parse_header_truncated_magic
    ("op=parse_header hex=02", "hr=80004005"),
    // read_string_empty
    ("op=read_string hex=00000000", "hr=00000000 s=~ used=4"),
    // read_string_nul_only
    ("op=read_string hex=010000000000", "hr=00000000 s=\\z used=6"),
    // read_string_len_without_body
    ("op=read_string hex=0200000041", "hr=8003001e"),
    // read_string_len_short_field
    ("op=read_string hex=0300", "hr=8003001e"),
    // write_string_empty
    ("op=write_string s=~", "010000000000"),
    // size_max_friendly_only
    ("op=size_max tf=- fn=abc loc=- mon=-", "size=20 hr=80004005"),
    // headers_null
    ("op=headers s=-", "s=-"),
    // headers_empty
    ("op=headers s=~", "s=~"),
    // headers_cr_tail
    ("op=headers s=a\\r", "s=a\\r"),
    // headers_lf_only_in_middle
    ("op=headers s=a\\nb", "s=a\\nb\\r\\n"),
    // special_home
    ("op=special ref=0", "hr=00000000 name=Start\\sPage"),
    // special_search
    ("op=special ref=1", "hr=00000000 name=Search\\sPage"),
    // special_history
    ("op=special ref=2", "hr=80004001"),
    // special_unknown
    ("op=special ref=9", "hr=80070057"),
    // check_get_ref_sentinel
    ("op=check_get_ref which=4294967295", "hr=00000000"),
    // check_get_ref_bad_bit
    ("op=check_get_ref which=4", "hr=80070057"),
    // check_get_ref_valid_bits
    ("op=check_get_ref which=3", "hr=00000000"),
    // resolve_ref_default
    ("op=resolve_ref which=0", "ref=2"),
    // resolve_ref_absolute
    ("op=resolve_ref which=1", "ref=1"),
    // set_moniker_zero
    ("op=set_moniker flags=0", "hr=80070057"),
    // set_moniker_passthrough
    ("op=set_moniker flags=4", "hr=00000004"),
    // set_moniker_both
    ("op=set_moniker flags=3", "hr=00000000 target=1 location=1"),
    // set_string_passthrough
    ("op=set_string flags=4 tgt=a loc=b", "hr=00000004"),
    // set_string_empty_location
    ("op=set_string flags=3 tgt=- loc=~", "hr=00000000 clear=1 tgt=- repl=1 loc=-"),
    // set_string_zero
    ("op=set_string flags=0 tgt=a loc=b", "hr=00000000 clear=0 tgt=- repl=0 loc=-"),
    // set_string_location_only
    ("op=set_string flags=2 tgt=- loc=c", "hr=00000000 clear=0 tgt=- repl=1 loc=c"),
    // set_string_target_only
    ("op=set_string flags=1 tgt=abc loc=-", "hr=00000000 clear=1 tgt=abc repl=0 loc=-"),
    // fallback_url
    ("op=fallback tgt=http:x", "kind=url"),
    // fallback_drive
    ("op=fallback tgt=C:x", "kind=file"),
    // abs_name_colon
    ("op=abs_name s=C:x", "abs=1"),
    // abs_name_none
    ("op=abs_name s=-", "abs=0"),
    // update_stack_null
    ("op=update_stack frame=0 bc=0", "hr=80070057"),
    // update_stack_frame
    ("op=update_stack frame=1 bc=0", "hr=00000000"),
    // clone_missing_obj
    ("op=clone hlink=1 riid=1 obj=0", "hr=80070057"),
    // clone_all
    ("op=clone hlink=1 riid=1 obj=1", "hr=00000000"),
    // save_flags_none
    ("op=save_flags mon=0 abs=0 loc=0 fn=0 tf=0", "flags=00000000"),
    // save_flags_friendly
    ("op=save_flags mon=0 abs=0 loc=0 fn=1 tf=0", "flags=00000014"),
    // parse_header_absolute_ok
    ("op=parse_header hex=0200000002000000", "hr=00000000 flags=00000002 unk=00000000"),
    // read_string_huge_len_short
    ("op=read_string hex=ffffffff41", "hr=8003001e"),
    // set_string_high_flags
    ("op=set_string flags=4294967294 tgt=a loc=b", "hr=00000000 clear=0 tgt=- repl=1 loc=b"),
    // set_string_passthrough_top
    ("op=set_string flags=4294967292 tgt=a loc=b", "hr=fffffffc"),
    // strip_file_short
    ("op=strip_file s=ab", "eaten=0"),
];

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn edge_rows_match_wine() {
    for &(spec, expected) in EDGE_ROWS {
        assert_eq!(replay(spec), expected, "row {spec}");
    }
}

#[test]
fn shortcut_ignores_non_ascii_units() {
    assert_eq!(is_shortcut(Some(wide(".\u{ff35}rl").as_slice())), S_FALSE);
}

#[test]
fn strip_file_does_not_fold_dotless_i() {
    let name = wide("f\u{131}le:x");
    assert_eq!(strip_file_scheme(&name), name.as_slice());
}

#[test]
fn strip_file_slashes_stop_at_nul() {
    let name = wide("file:\0/x");
    assert_eq!(strip_file_scheme(&name), &name[5..]);
}

#[test]
fn embedded_nul_ends_each_string() {
    let target = wide("a\0#b");
    let location = wide("c\0d");
    assert_eq!(
        split_target(Some(target.as_slice()), Some(location.as_slice())),
        (Some(wide("a").as_slice()), Some(wide("c").as_slice()))
    );
    assert_eq!(is_shortcut(Some(wide(".url\0x").as_slice())), S_OK);
    assert!(!is_absolute_display_name(Some(wide("C\0:x").as_slice())));
    assert!(!fallback_is_url(&wide("ab\0:x")));
    assert_eq!(string_size(&wide("a\0b")), 8);
    assert_eq!(encode_string(&wide("a\0b")), encode_string(&wide("a")));
    assert_eq!(
        normalize_headers(Some(wide("a\0b").as_slice())),
        Some(wide("a\r\n"))
    );
    let blank_target = wide("\0a");
    let plan = set_string_reference_plan(1, Some(blank_target.as_slice()), None).expect("plan");
    assert_eq!(plan.target, None);
}

#[test]
fn encode_decode_round_trip_non_ascii() {
    let text = wide("n\u{e9}\u{20ac}\u{1f600}");
    let bytes = encode_string(&text);
    let (decoded, used) = decode_string(&bytes).expect("round trip");
    let mut expected = text.clone();
    expected.push(0);
    assert_eq!(decoded, expected);
    assert_eq!(used, 16);
    assert_eq!(string_size(&text), 16);
}

#[test]
fn decode_ignores_trailing_bytes() {
    assert_eq!(
        decode_string(&[1, 0, 0, 0, 0x61, 0, 0xAA, 0xBB]),
        Ok((vec![0x61], 6))
    );
}

#[test]
fn decode_rejects_short_input() {
    assert_eq!(decode_string(&[1, 0]), Err(STG_E_READFAULT));
    assert_eq!(decode_string(&[0xFF; 4]), Err(STG_E_READFAULT));
    assert_eq!(
        decode_string(&[0, 0, 0, 0x80, 0x61, 0]),
        Err(STG_E_READFAULT)
    );
}

#[test]
fn size_max_with_and_without_moniker() {
    assert_eq!(size_max(None, None, None, None), (E_FAIL, 8));
    assert_eq!(size_max(None, None, None, Some(0)), (S_OK, 8));
    let abc = wide("abc");
    let xy = wide("xy");
    assert_eq!(
        size_max(Some(abc.as_slice()), None, Some(xy.as_slice()), Some(10)),
        (S_OK, 40)
    );
    assert_eq!(string_size(&abc), 12);
    assert_eq!(size_max(None, None, None, Some(u64::MAX)), (S_OK, 7));
}

#[test]
fn units_expands_every_escape() {
    assert_eq!(
        units(r"a\sb\z\\\n\r"),
        [0x61, 0x20, 0x62, 0, 0x5C, 0x0A, 0x0D]
    );
    assert_eq!(units("a\\"), [0x61, 0x5C]);
}

#[test]
#[should_panic(expected = "non-ASCII input")]
fn units_rejects_non_ascii() {
    units("\u{e9}");
}

#[test]
#[should_panic(expected = "non-printable unit 7f")]
fn render_rejects_del() {
    render(&[0x7F]);
}

#[test]
fn set_string_passthrough_boundary() {
    assert!(set_string_reference_plan(3, None, None).is_ok());
    assert_eq!(set_string_reference_plan(4, None, None).err(), Some(4));
    assert_eq!(
        set_string_reference_plan(0xFFFF_FFFC, None, None).err(),
        Some(-4)
    );
    assert!(set_string_reference_plan(0xFFFF_FFFD, None, None).is_ok());
}

#[test]
fn plans_format_and_compare() {
    assert_eq!(
        format!("{:?}", set_moniker_reference_plan(1).expect("plan")),
        "MonikerPlan { target: true, location: false }"
    );
    assert_eq!(
        set_moniker_reference_plan(3),
        Ok(MonikerPlan {
            target: true,
            location: true
        })
    );
    let ab = wide("ab");
    let plan = set_string_reference_plan(3, Some(ab.as_slice()), None).expect("plan");
    assert_eq!(
        format!("{plan:?}"),
        "StringPlan { clear_target: true, target: Some([97, 98]), replace_location: true, location: None }"
    );
    assert_eq!(
        plan,
        StringPlan {
            clear_target: true,
            target: Some(ab.as_slice()),
            replace_location: true,
            location: None
        }
    );
}

#[test]
fn argument_checks_cover_each_operand() {
    assert_eq!(check_update_stack_item(false, false), Err(E_INVALIDARG));
    assert_eq!(check_update_stack_item(true, false), Ok(()));
    assert_eq!(check_update_stack_item(false, true), Ok(()));
    assert_eq!(check_clone(false, true, true), Err(E_INVALIDARG));
    assert_eq!(check_clone(true, false, true), Err(E_INVALIDARG));
    assert_eq!(check_clone(true, true, false), Err(E_INVALIDARG));
    assert_eq!(check_clone(true, true, true), Ok(()));
}

#[test]
fn fallback_url_needs_two_letters_before_colon() {
    assert!(fallback_is_url(&wide("ab:x")));
    assert!(!fallback_is_url(&wide("C:x")));
    assert!(!fallback_is_url(&wide("abc")));
}

#[test]
fn unknown_flags_masks_known_bits() {
    assert_eq!(unknown_flags(0x14), 0);
    assert_eq!(unknown_flags(0xC0), 0x40);
}

#[test]
fn save_flags_round_trip_through_parse_header() {
    for bits in 0u8..32 {
        let flags = save_flags(
            (bits & 1) != 0,
            (bits & 2) != 0,
            (bits & 4) != 0,
            (bits & 8) != 0,
            (bits & 16) != 0,
        );
        let mut header = vec![2u8, 0, 0, 0];
        header.extend(flags.to_le_bytes());
        assert_eq!(parse_header(&header), Ok(flags));
        assert_eq!(unknown_flags(flags), 0);
    }
}

#[test]
fn eq_ascii_ci_requires_ascii_and_equal_length() {
    assert!(eq_ascii_ci(&wide("FiLe"), b"file"));
    assert!(!eq_ascii_ci(&wide("file"), b"files"));
    assert!(!eq_ascii_ci(&[0xE9], b"e"));
}
