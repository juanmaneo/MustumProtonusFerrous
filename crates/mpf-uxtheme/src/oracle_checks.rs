//! Rows derived from the verbatim-C oracle in `target/canon/uxtheme`, which
//! compiles Wine's `uxini.c` and `msstyles.c` with AddressSanitizer and UBSan.
//! Each test is named after the oracle row id it checks, and its expected value
//! is the output recorded for that row in `oracle.jsonl`.
//!
//! The row `ini-value-first-on-line` is not here: the sanitizer flagged it, so
//! it is excluded. See `edge_checks.rs` for the behaviour kept for it.

use super::*;

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

// num-neg-prefix: "OK -12 4"
#[test]
fn oracle_num_neg_prefix() {
    assert_eq!(number::next_number(&wide("a-12x"), 0, 5), Some((-12, 4)));
}

// num-minus-anywhere: "OK -5 4"
#[test]
fn oracle_num_minus_anywhere() {
    assert_eq!(number::next_number(&wide("a-b5"), 0, 4), Some((-5, 4)));
}

// tok-comma-separated: "OK Tahoma 6"
#[test]
fn oracle_tok_comma_separated() {
    assert_eq!(
        string::next_token(&wide("Tahoma,12"), 0, 9, 32),
        Some((wide("Tahoma"), 6))
    );
}

// tok-last-char-lost: "OK bol 4"
#[test]
fn oracle_tok_last_char_lost() {
    assert_eq!(
        string::next_token(&wide("bold\n"), 0, 4, 32),
        Some((wide("bol"), 4))
    );
}

// tok-truncated-to-buffer: "OK it 6"
#[test]
fn oracle_tok_truncated_to_buffer() {
    assert_eq!(
        string::next_token(&wide("italic"), 0, 6, 3),
        Some((wide("it"), 6))
    );
}

// tok-two-spaces-before-bound: "OK x 3"
#[test]
fn oracle_tok_two_spaces_before_bound() {
    assert_eq!(
        string::next_token(&wide("xy \n"), 0, 3, 32),
        Some((wide("x"), 3))
    );
}

// color-wrap-bytes: "OK 393004", that is 0x05FF2C with 300 -> 0x2C, -1 -> 0xFF
#[test]
fn oracle_color_wrap_bytes() {
    assert_eq!(value::parse_color(&wide("300 -1 5")), Ok(393_004));
}

// pos-with-punct: "OK 10 20"
#[test]
fn oracle_pos_with_punct() {
    assert_eq!(value::parse_position(&wide("10 , 20")), Ok((10, 20)));
}

// ilist-capped-402: "OK 402" followed by 402 sevens
#[test]
fn oracle_ilist_capped_402() {
    let values = value::parse_int_list(&wide(&" 7".repeat(500)));
    assert_eq!(values, vec![7; 402]);
}

// rect-missing-bottom: "ERR 1 2 3 4000", the harness pre-fills bottom with 4000
#[test]
fn oracle_rect_missing_bottom() {
    let mut rect = value::Rect {
        bottom: 4000,
        ..Default::default()
    };
    assert_eq!(
        value::parse_rect(&wide("1 2 3"), &mut rect),
        Err(E_PROP_ID_UNSUPPORTED)
    );
    assert_eq!(
        rect,
        value::Rect {
            left: 1,
            top: 2,
            right: 3,
            bottom: 4000,
        }
    );
}

// margins-two-values: "ERR 1 2 13 14", the harness pre-fills top with 13 and bottom with 14
#[test]
fn oracle_margins_two_values() {
    let mut margins = value::Margins {
        top_height: 13,
        bottom_height: 14,
        ..Default::default()
    };
    assert_eq!(
        value::parse_margins(&wide("1 2"), &mut margins),
        Err(E_PROP_ID_UNSUPPORTED)
    );
    assert_eq!(
        margins,
        value::Margins {
            left_width: 1,
            right_width: 2,
            top_height: 13,
            bottom_height: 14,
        }
    );
}

// string-negative-cch-unbounded: "abcdef" in full
#[test]
fn oracle_string_negative_cch_unbounded() {
    assert_eq!(value::parse_string(&wide("abcdef"), -1), wide("abcdef"));
}

// font-face-size-bold: "OK Tahoma 12 700 0 0 0 1"
#[test]
fn oracle_font_face_size_bold() {
    let font = value::parse_font(&wide("Tahoma,12,bold"), 0).unwrap();
    assert_eq!(font.face_name, wide("Tahoma"));
    assert_eq!(font.point_size, 12);
    assert_eq!(font.weight, 700);
    assert!(!font.italic && !font.underline && !font.strike_out);
    assert_eq!(font.char_set, 1);
}

// font-last-token-cut: "OK Tahoma 12 400 0 0 0 1". The value ends at 14, and the unit after it is '\n'.
#[test]
fn oracle_font_last_token_cut() {
    let font = value::parse_font(&wide("Tahoma,12,bold"), u16::from(b'\n')).unwrap();
    assert_eq!(font.face_name, wide("Tahoma"));
    assert_eq!(font.point_size, 12);
    assert_eq!(font.weight, 400);
}

// font-attrs-after-value: "OK Courier New 9 400 1 1 1 1"
#[test]
fn oracle_font_attrs_after_value() {
    let font = value::parse_font(&wide("Courier New,9,Italic,Underline,StrikeOut,"), 0).unwrap();
    assert_eq!(font.face_name, wide("Courier New"));
    assert_eq!(font.point_size, 9);
    assert_eq!(font.weight, 400);
    assert!(font.italic && font.underline && font.strike_out);
    assert_eq!(font.char_set, 1);
}

// split-app-class-part-state: app "app", class "Class", part "Part", state "State"
#[test]
fn oracle_split_app_class_part_state() {
    let name = section::split_section_name(&wide("app::Class.Part(State)")).unwrap();
    assert_eq!(name.app, wide("app"));
    assert_eq!(name.class, wide("Class"));
    assert_eq!(name.part, Some(wide("Part")));
    assert_eq!(name.state, Some(wide("State")));
}

// split-empty-part-null: "Class.(State)" gives part NULL and state "State"
#[test]
fn oracle_split_class_part_null() {
    let name = section::split_section_name(&wide("Class.(State)")).unwrap();
    assert!(name.app.is_empty());
    assert_eq!(name.class, wide("Class"));
    assert_eq!(name.part, None);
    assert_eq!(name.state, Some(wide("State")));
}

// split-single-colon-skips: "OK app lass - -"
#[test]
fn oracle_split_single_colon_skips() {
    let name = section::split_section_name(&wide("app:Class")).unwrap();
    assert_eq!(name.app, wide("app"));
    assert_eq!(name.class, wide("lass"));
    assert_eq!(name.part, None);
    assert_eq!(name.state, None);
}

// split-part-not-cut-at-dot-in-state: class "Class(a", part "b)", no state
#[test]
fn oracle_split_part_not_cut_at_dot_in_state() {
    let name = section::split_section_name(&wide("Class(a.b)")).unwrap();
    assert!(name.app.is_empty());
    assert_eq!(name.class, wide("Class(a"));
    assert_eq!(name.part, Some(wide("b)")));
    assert_eq!(name.state, None);
}

// appclass-first-separator-wins: app "a", class "b::c"
#[test]
fn oracle_appclass_first_separator_wins() {
    assert_eq!(
        section::split_app_class_name(&wide("a::b::c")),
        (wide("a"), wide("b::c"))
    );
}

// ini-section-then-value: "S 1 A", "V k=v", "V none"
#[test]
fn oracle_ini_section_then_value() {
    let text = wide("[A]\nk=v\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&wide("A")[..]));
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("k")[..],
            value: &wide("v")[..],
        })
    );
    assert_eq!(ini.next_value(), None);
}

// ini-comment-and-spaces: "S 3 Sec", "V name=value", "V none"
#[test]
fn oracle_ini_comment_and_spaces() {
    let text = wide("; comment\n[Sec]\nname = value  \n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&wide("Sec")[..]));
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("name")[..],
            value: &wide("value")[..],
        })
    );
    assert_eq!(ini.next_value(), None);
}

// ini-unclosed-section: "S 2 Se", the closing bracket is missing
#[test]
fn oracle_ini_unclosed_section() {
    let text = wide("[Sec\nk=1");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&wide("Se")[..]));
}

// ini-lone-bracket-wraps: "S 4294967295", the length wraps in Wine. Here the section is
// empty, which split_section_name rejects just as Wine does.
#[test]
fn oracle_ini_lone_bracket_wraps() {
    let text = wide("[\n");
    let mut ini = ini::IniReader::new(&text);
    let section = ini.next_section().unwrap();
    assert!(section.is_empty());
    assert_eq!(section::split_section_name(section), None);
}

// ini-crlf-lines: "S 1 X", "V k=1", "V none", "S none"
#[test]
fn oracle_ini_crlf_lines() {
    let text = wide("[X]\r\nk=1\r\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&wide("X")[..]));
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("k")[..],
            value: &wide("1")[..],
        })
    );
    assert_eq!(ini.next_value(), None);
    assert_eq!(ini.next_section(), None);
}
