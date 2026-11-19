//! Cases the oracle rows do not reach. The expectations follow the verbatim C in `main.c`,
//! and were cross-checked by running the same inputs through `target/canon/comsvcs/harness`.

use crate::new_moniker::{parse_displayname, NewMonikerTarget};
use crate::{guid_from_string, Guid};
use std::cell::Cell;

const GUID_TEXT: &str = "12345678-1234-1234-1234-123456789abc";
const GUID: Guid = Guid {
    data1: 0x1234_5678,
    data2: 0x1234,
    data3: 0x1234,
    data4: [0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc],
};

fn units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn guid_braced_and_bare_forms_agree() {
    assert_eq!(guid_from_string(&units(GUID_TEXT)), Some(GUID));
    assert_eq!(
        guid_from_string(&units(&format!("{{{GUID_TEXT}}}"))),
        Some(GUID)
    );
    assert_eq!(
        guid_from_string(&units(&GUID_TEXT.to_uppercase())),
        Some(GUID)
    );
}

#[test]
fn guid_extremes() {
    assert_eq!(
        guid_from_string(&units("FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF")),
        Some(Guid {
            data1: u32::MAX,
            data2: u16::MAX,
            data3: u16::MAX,
            data4: [0xff; 8],
        })
    );
    assert_eq!(
        guid_from_string(&units("00000000-0000-0000-0000-000000000000")),
        Some(Guid::default())
    );
}

#[test]
fn guid_rejects_a_bad_digit_in_every_digit_position() {
    for i in (0..36).filter(|i| ![8, 13, 18, 23].contains(i)) {
        let mut text = units(GUID_TEXT);
        text[i] = u16::from(b'g');
        assert_eq!(guid_from_string(&text), None, "bare, digit {i}");
        let mut braced = vec![u16::from(b'{')];
        braced.extend(&text);
        braced.push(u16::from(b'}'));
        assert_eq!(guid_from_string(&braced), None, "braced, digit {i}");
    }
}

#[test]
fn guid_rejects_a_bad_dash() {
    for i in [8, 13, 18, 23] {
        let mut text = units(GUID_TEXT);
        text[i] = u16::from(b'_');
        assert_eq!(guid_from_string(&text), None, "dash {i}");
    }
}

#[test]
fn guid_braces_must_pair() {
    assert_eq!(guid_from_string(&units(&format!("{{{GUID_TEXT}"))), None);
    assert_eq!(guid_from_string(&units(&format!("{{{GUID_TEXT}]"))), None);
    assert_eq!(guid_from_string(&units("{")), None);
}

#[test]
fn guid_ignores_text_after_the_guid() {
    assert_eq!(
        guid_from_string(&units(&format!("{GUID_TEXT}xyz"))),
        Some(GUID)
    );
    assert_eq!(
        guid_from_string(&units(&format!("{{{GUID_TEXT}}}xyz"))),
        Some(GUID)
    );
    // The closing brace of an unbraced GUID is not checked.
    assert_eq!(
        guid_from_string(&units(&format!("{GUID_TEXT}}}"))),
        Some(GUID)
    );
}

#[test]
fn guid_text_ends_at_the_first_nul() {
    assert_eq!(
        guid_from_string(&units(&format!("{GUID_TEXT}\0junk"))),
        Some(GUID)
    );
    assert_eq!(
        guid_from_string(&units(&format!("{{{GUID_TEXT}\0}}"))),
        None
    );
    assert_eq!(
        guid_from_string(&units("1234\x005678-1234-1234-1234-123456789abc")),
        None
    );
}

#[test]
fn guid_short_and_empty_text() {
    assert_eq!(guid_from_string(&[]), None);
    assert_eq!(guid_from_string(&units("1234")), None);
    assert_eq!(guid_from_string(&units(&GUID_TEXT[..35])), None);
    assert_eq!(guid_from_string(&units(&GUID_TEXT[..36])), Some(GUID));
}

#[test]
fn guid_rejects_non_ascii_digits() {
    let mut fullwidth = units(GUID_TEXT);
    fullwidth[0] = 0xff10; // FULLWIDTH DIGIT ZERO
    assert_eq!(guid_from_string(&fullwidth), None);
    let mut surrogate = units(GUID_TEXT);
    surrogate[0] = 0xd800;
    assert_eq!(guid_from_string(&surrogate), None);
}

/// A resolver that counts its calls and answers `answer`. Tests that must not reach the
/// lookup assert the count is zero, and tests that do reach it share this closure body.
fn counted(calls: &Cell<usize>, answer: Option<Guid>) -> impl FnOnce(&[u16]) -> Option<Guid> + '_ {
    move |_| {
        calls.set(calls.get() + 1);
        answer
    }
}

#[test]
fn clsid_names_never_reach_the_registry() {
    let name = units(&format!("new:{{{GUID_TEXT}}}"));
    let calls = Cell::new(0);
    let got = parse_displayname(&name, counted(&calls, None));
    assert_eq!(got, Some((NewMonikerTarget::Clsid(GUID), 42)));
    assert_eq!(calls.get(), 0);
}

#[test]
fn prefix_is_ascii_case_insensitive() {
    for prefix in ["new:", "NEW:", "NeW:", "nEW:"] {
        let name = units(&format!("{prefix}{GUID_TEXT}"));
        let calls = Cell::new(0);
        assert_eq!(
            parse_displayname(&name, counted(&calls, None)),
            Some((NewMonikerTarget::Clsid(GUID), 40)),
            "{prefix}"
        );
        assert_eq!(calls.get(), 0, "{prefix}");
    }
}

#[test]
fn bad_prefix_is_syntax_without_a_lookup() {
    for text in ["", "n", "ne", "new", "nex:", "xnew:", "new;", "n\u{435}w:"] {
        let name = units(text);
        let calls = Cell::new(0);
        assert_eq!(
            parse_displayname(&name, counted(&calls, Some(GUID))),
            None,
            "{text:?}"
        );
        assert_eq!(calls.get(), 0, "{text:?}");
    }
}

#[test]
fn progid_text_goes_to_the_resolver() {
    let name = units("new:Some.Prog");
    let mut asked = Vec::new();
    let got = parse_displayname(&name, |text| {
        asked.push(String::from_utf16_lossy(text));
        Some(GUID)
    });
    assert_eq!(
        got,
        Some((
            NewMonikerTarget::ProgId {
                clsid: GUID,
                name: &name[4..],
            },
            13
        ))
    );
    assert_eq!(asked, ["Some.Prog"]);
}

#[test]
fn resolver_refusal_is_syntax() {
    let mut asked = Vec::new();
    let empty = units("new:");
    let got = parse_displayname(&empty, |text| {
        asked.push(text.len());
        None
    });
    assert_eq!(got, None);
    assert_eq!(asked, [0]);
    let calls = Cell::new(0);
    assert_eq!(
        parse_displayname(&units("new:Foo"), counted(&calls, None)),
        None
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn name_ends_at_its_first_nul() {
    let name = units("new:Foo\0Bar");
    assert_eq!(
        parse_displayname(&name, |_| Some(GUID)),
        Some((
            NewMonikerTarget::ProgId {
                clsid: GUID,
                name: &name[4..7],
            },
            7
        ))
    );
    let calls = Cell::new(0);
    assert_eq!(
        parse_displayname(&units("new\0:Foo"), counted(&calls, Some(GUID))),
        None
    );
    assert_eq!(calls.get(), 0);
}
