//! Edge cases for each function: bounds, buffer limits, error paths and the
//! documented deviations from Wine. Where the same input has a row in the
//! verbatim-C oracle, the test cites its id. The other tests are Rust-only.

use super::*;

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn split(text: &str) -> Option<section::SectionName> {
    section::split_section_name(&wide(text))
}

// number.rs

#[test]
fn number_without_digits_is_none() {
    assert_eq!(number::next_number(&wide("x-y"), 0, 3), None); // oracle num-no-digits
    assert_eq!(number::next_number(&wide("a5"), 0, 1), None); // oracle num-bound-before-digit
    assert_eq!(number::next_number(&wide("-"), 0, 1), None); // oracle num-minus-only
    assert_eq!(number::next_number(&wide("5"), 1, 1), None);
}

#[test]
fn number_reads_from_start_and_stops_at_end() {
    assert_eq!(number::next_number(&wide("9-8"), 1, 3), Some((-8, 3))); // oracle num-start-offset
    assert_eq!(number::next_number(&wide("12-3"), 0, 4), Some((12, 2))); // oracle num-stops-at-minus
    assert_eq!(number::next_number(&wide("007"), 0, 3), Some((7, 3))); // oracle num-leading-zeros
    assert_eq!(number::next_number(&wide("  42"), 0, 4), Some((42, 4))); // oracle num-leading-spaces
}

#[test]
fn number_minus_negates_the_value_once() {
    assert_eq!(number::next_number(&wide("--5"), 0, 3), Some((-5, 3))); // oracle num-double-minus
    assert_eq!(number::next_number(&wide("-0"), 0, 2), Some((0, 2)));
}

#[test]
fn number_overflow_wraps() {
    // Wine's signed arithmetic is undefined here. This port wraps.
    assert_eq!(
        number::next_number(&wide("2147483648"), 0, 10),
        Some((i32::MIN, 10))
    );
    assert_eq!(
        number::next_number(&wide("-2147483648"), 0, 11),
        Some((i32::MIN, 11))
    );
}

// string.rs

#[test]
fn copy_units_respects_the_buffer_size() {
    let text = [97, 98, 99];
    assert!(string::copy_units(&text, 0, 0).is_empty());
    assert!(string::copy_units(&text, 0, 1).is_empty());
    assert_eq!(string::copy_units(&text, 0, 3), vec![97, 98]);
    assert_eq!(string::copy_units(&text, 1, 10), vec![98, 99]);
    assert!(string::copy_units(&text, 5, 10).is_empty());
}

#[test]
fn copy_units_stops_at_nul() {
    assert_eq!(string::copy_units(&[97, 0, 98], 0, 10), vec![97]);
    assert!(string::copy_units(&[0, 97], 0, 10).is_empty());
}

#[test]
fn token_reports_the_comma_that_ends_it() {
    assert_eq!(
        string::next_token(&wide("  Tahoma , 12"), 0, 13, 32),
        Some((wide("Tahoma "), 9))
    );
    assert_eq!(
        string::next_token(&wide("a b ,"), 0, 5, 32),
        Some((wide("a b "), 4))
    ); // oracle tok-space-inside-kept
}

#[test]
fn token_with_no_text_is_none() {
    assert_eq!(string::next_token(&wide(" , ,"), 0, 4, 32), None); // oracle tok-only-separators
    assert_eq!(string::next_token(&wide("abc"), 3, 3, 32), None);
}

#[test]
fn token_buffer_limits_the_copy() {
    assert_eq!(
        string::next_token(&wide("Tahoma,12"), 0, 9, 0),
        Some((Vec::new(), 6))
    ); // oracle tok-zero-buffer
    assert_eq!(
        string::next_token(&wide("Tahoma,12"), 0, 9, 1),
        Some((Vec::new(), 6))
    ); // oracle tok-one-unit-buffer
    assert_eq!(
        string::next_token(&wide("xy "), 0, 3, 32),
        Some((wide("xy "), 3))
    ); // oracle tok-trailing-space-at-bound
    assert_eq!(
        string::next_token(&wide("bold"), 0, 4, 32),
        Some((wide("bold"), 4))
    ); // oracle tok-last-no-space
}

// value.rs

#[test]
fn bool_reads_only_the_first_unit() {
    assert!(value::parse_bool(&wide("True"))); // oracle bool-true-upper
    assert!(value::parse_bool(&wide("t")));
    assert!(!value::parse_bool(&wide("false"))); // oracle bool-false
    assert!(!value::parse_bool(&wide(""))); // oracle bool-empty
    assert!(!value::parse_bool(&wide(" t")));
}

#[test]
fn color_needs_all_three_numbers() {
    assert_eq!(value::parse_color(&wide("10 20 30")), Ok(1_971_210)); // oracle color-plain
    assert_eq!(value::parse_color(&wide("")), Err(E_PROP_ID_UNSUPPORTED));
    assert_eq!(value::parse_color(&wide("1")), Err(E_PROP_ID_UNSUPPORTED));
    assert_eq!(value::parse_color(&wide("1 2")), Err(E_PROP_ID_UNSUPPORTED)); // oracle color-missing-blue
}

#[test]
fn int_reads_the_first_number() {
    assert_eq!(value::parse_int(&wide("-42")), Ok(-42)); // oracle int-negative
    assert_eq!(value::parse_int(&wide("7 8")), Ok(7));
    assert_eq!(value::parse_int(&wide("x")), Err(E_PROP_ID_UNSUPPORTED)); // oracle int-not-numeric
}

#[test]
fn int_list_stops_at_the_first_failure() {
    assert_eq!(value::parse_int_list(&wide("1 2 3")), vec![1, 2, 3]); // oracle ilist-three
    assert!(value::parse_int_list(&wide("")).is_empty()); // oracle ilist-empty
    assert_eq!(value::parse_int_list(&wide("5,6,x")), vec![5, 6]); // oracle ilist-stops-at-junk
}

#[test]
fn position_needs_both_numbers() {
    assert_eq!(value::parse_position(&wide("10 , 20")), Ok((10, 20))); // oracle pos-with-punct
    assert_eq!(value::parse_position(&wide("")), Err(E_PROP_ID_UNSUPPORTED));
    assert_eq!(
        value::parse_position(&wide("5")),
        Err(E_PROP_ID_UNSUPPORTED)
    ); // oracle pos-missing-y
}

#[test]
fn string_is_cut_to_the_buffer() {
    assert_eq!(value::parse_string(&wide("abcdef"), 4), wide("abc")); // oracle string-cut-to-cch
    assert!(value::parse_string(&wide("abcdef"), 0).is_empty()); // oracle string-zero-cch
    assert!(value::parse_string(&wide("abcdef"), 1).is_empty());
    assert_eq!(value::parse_string(&wide("abc"), 10), wide("abc")); // oracle string-fits
    assert!(value::parse_string(&wide(""), 5).is_empty());
}

#[test]
fn string_stops_at_nul() {
    assert_eq!(value::parse_string(&[97, 0, 98], -1), vec![97]);
}

#[test]
fn rect_reads_four_fields() {
    let mut rect = value::Rect::default();
    assert_eq!(value::parse_rect(&wide("1 2 3 4"), &mut rect), Ok(())); // oracle rect-full
    assert_eq!(
        rect,
        value::Rect {
            left: 1,
            top: 2,
            right: 3,
            bottom: 4,
        }
    );
}

#[test]
fn rect_leaves_unparsed_fields_unchanged() {
    let mut rect = value::Rect {
        left: 0,
        top: 0,
        right: 9,
        bottom: 9,
    };
    assert_eq!(
        value::parse_rect(&wide("1 2"), &mut rect),
        Err(E_PROP_ID_UNSUPPORTED)
    );
    assert_eq!(
        rect,
        value::Rect {
            left: 1,
            top: 2,
            right: 9,
            bottom: 9,
        }
    );

    let mut rect = value::Rect {
        left: 5,
        top: 6,
        right: 7,
        bottom: 8,
    };
    assert_eq!(
        value::parse_rect(&wide(""), &mut rect),
        Err(E_PROP_ID_UNSUPPORTED)
    );
    assert_eq!(
        rect,
        value::Rect {
            left: 5,
            top: 6,
            right: 7,
            bottom: 8,
        }
    );
}

#[test]
fn margins_read_four_fields_in_order() {
    let mut margins = value::Margins::default();
    assert_eq!(value::parse_margins(&wide("1 2 3 4"), &mut margins), Ok(())); // oracle margins-full
    assert_eq!(
        margins,
        value::Margins {
            left_width: 1,
            right_width: 2,
            top_height: 3,
            bottom_height: 4,
        }
    );
}

#[test]
fn margins_leave_unparsed_fields_unchanged() {
    let mut margins = value::Margins {
        left_width: 0,
        right_width: 0,
        top_height: 13,
        bottom_height: 14,
    };
    assert_eq!(
        value::parse_margins(&wide("1 2 3"), &mut margins),
        Err(E_PROP_ID_UNSUPPORTED)
    ); // oracle margins-missing-bottom
    assert_eq!(
        margins,
        value::Margins {
            left_width: 1,
            right_width: 2,
            top_height: 3,
            bottom_height: 14,
        }
    );

    let mut margins = value::Margins::default();
    assert_eq!(
        value::parse_margins(&wide(""), &mut margins),
        Err(E_PROP_ID_UNSUPPORTED)
    );
    assert_eq!(margins, value::Margins::default());
}

#[test]
fn font_needs_a_face_name_and_a_size() {
    assert!(value::parse_font(&wide(""), 0).is_err());
    assert!(value::parse_font(&wide(",,"), 0).is_err());
    assert!(value::parse_font(&wide("Tahoma"), 0).is_err()); // oracle font-missing-size
}

#[test]
fn font_attribute_names_ignore_case() {
    let font = value::parse_font(&wide("Arial,10,BOLD"), 0).unwrap(); // oracle font-bold-upper
    assert_eq!(font.face_name, wide("Arial"));
    assert_eq!(font.point_size, 10);
    assert_eq!(font.weight, value::FW_BOLD);
}

#[test]
fn font_ignores_unknown_attributes() {
    let font = value::parse_font(&wide("Tahoma,12,Foo,Italic"), 0).unwrap();
    assert!(font.italic);
    assert!(!font.underline && !font.strike_out);
    assert_eq!(font.weight, value::FW_REGULAR);
    assert_eq!(font.char_set, value::DEFAULT_CHARSET);
}

#[test]
fn font_attribute_with_a_non_ascii_unit_does_not_match() {
    // U+0142 has the low byte 0x42, which is 'B'. A byte-wise compare would match.
    let font = value::parse_font(&wide("Tahoma,12,\u{0142}old"), 0).unwrap();
    assert_eq!(font.weight, value::FW_REGULAR);

    let mut text = wide("Tahoma,12,");
    text.extend([0xD800, 0x6F, 0x6C, 0x64]);
    assert_eq!(
        value::parse_font(&text, 0).unwrap().weight,
        value::FW_REGULAR
    );
}

#[test]
fn font_trims_the_last_token_only_against_whitespace_after() {
    let font = value::parse_font(&wide("Tahoma,12,bold"), b'x'.into()).unwrap();
    assert_eq!(font.weight, value::FW_BOLD);
    let font = value::parse_font(&wide("Tahoma,12,bold"), b'\n'.into()).unwrap();
    assert_eq!(font.weight, value::FW_REGULAR); // oracle font-last-token-cut
}

// section.rs

#[test]
fn section_name_parts() {
    let name = split("Class").unwrap(); // oracle split-class-only
    assert!(name.app.is_empty());
    assert_eq!(name.class, wide("Class"));
    assert_eq!(name.part, None);
    assert_eq!(name.state, None);

    let name = split("Class(State)").unwrap(); // oracle split-class-state
    assert_eq!(name.class, wide("Class"));
    assert_eq!(name.part, None);
    assert_eq!(name.state, Some(wide("State")));

    let name = split("Class.Part").unwrap(); // oracle split-class-part
    assert_eq!(name.part, Some(wide("Part")));
    assert_eq!(name.state, None);

    let name = split("Class()").unwrap(); // oracle split-empty-state
    assert_eq!(name.state, None);

    let name = split("Class.()").unwrap();
    assert_eq!(name.part, None);
    assert_eq!(name.state, None);
}

#[test]
fn section_name_without_a_class_is_none() {
    assert_eq!(split("()"), None); // oracle split-empty-class
    assert_eq!(split(":x"), None); // oracle split-colon-then-nothing-class
    assert_eq!(split(".Part"), None);
}

#[test]
fn section_name_with_an_unclosed_state_is_none() {
    assert_eq!(split("app::Class(State"), None); // oracle split-missing-close
    assert_eq!(split("Class(State"), None);
    assert_eq!(split("Class.Part(State"), None);
}

#[test]
fn section_colon_as_last_unit_is_none() {
    // Wine reads past the terminator here, which is undefined.
    assert_eq!(split("app:"), None);
}

#[test]
fn section_names_are_cut_to_59_units() {
    let name = split(&format!("{}::b", "a".repeat(70))).unwrap(); // oracle split-app-name-long
    assert_eq!(name.app.len(), 59);

    let name = split(&format!("c{}(s)", "a".repeat(70))).unwrap(); // oracle split-class-name-long
    assert_eq!(name.class.len(), 59);
    assert_eq!(name.class[0], u16::from(b'c'));
    assert_eq!(name.state, Some(wide("s")));
}

#[test]
fn section_is_read_up_to_254_units() {
    let name = split(&format!("{}:Class", "a".repeat(260))).unwrap();
    assert!(name.app.is_empty());
    assert_eq!(name.class, wide(&"a".repeat(59)));
}

#[test]
fn app_class_name_splits_at_the_first_separator() {
    assert_eq!(
        section::split_app_class_name(&wide("a::")),
        (wide("a"), Vec::new())
    ); // oracle appclass-empty-class
    assert_eq!(
        section::split_app_class_name(&wide("::x")),
        (Vec::new(), wide("x"))
    ); // oracle appclass-leading-separator
    assert_eq!(
        section::split_app_class_name(&wide("abc")),
        (Vec::new(), wide("abc"))
    ); // oracle appclass-no-separator
}

#[test]
fn app_class_name_is_cut_to_59_units() {
    let (app, class) = section::split_app_class_name(&wide(&format!("a::{}", "b".repeat(70))));
    assert_eq!(app, wide("a"));
    assert_eq!(class.len(), 59);
    let (app, _) = section::split_app_class_name(&wide(&format!("{}::b", "a".repeat(70))));
    assert_eq!(app.len(), 59); // oracle appclass-app-long
}

#[test]
fn app_class_name_ends_at_nul() {
    assert_eq!(
        section::split_app_class_name(&[97, 0, 58, 58, 98]),
        (Vec::new(), vec![97])
    );
}

// ini.rs

#[test]
fn value_with_no_name_keeps_the_value() {
    // Wine indexes the name before its start here, which faults. This port keeps the value.
    let text = wide("=abc\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &[],
            value: &wide("abc"),
        })
    );
}

#[test]
fn line_without_equals_ends_the_run_and_is_consumed() {
    let text = wide("noeq\nk=v\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_value(), None); // oracle ini-no-equals-skipped
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("k"),
            value: &wide("v"),
        })
    );
}

#[test]
fn next_section_skips_value_comment_and_blank_lines() {
    // uxini.c UXINI_GetNextSection loops past lines that do not start with '['
    let text = wide("k=v\n; note\n\n  \n[S]\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&wide("S")[..]));
    assert_eq!(ini.next_section(), None);
}

#[test]
fn section_line_is_left_for_next_section() {
    let text = wide("k=v\n[S]\nj=1\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("k"),
            value: &wide("v"),
        })
    );
    assert_eq!(ini.next_value(), None);
    assert_eq!(ini.next_section(), Some(&wide("S")[..]));
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("j"),
            value: &wide("1"),
        })
    );
}

#[test]
fn section_skips_lines_before_its_bracket() {
    // oracle ini-value-after-section-skip
    let text = wide("[A]\nk=1\n[B]\nj=2\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&wide("A")[..]));
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("k"),
            value: &wide("1"),
        })
    );
    assert_eq!(ini.next_section(), Some(&wide("B")[..]));
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("j"),
            value: &wide("2"),
        })
    );
}

#[test]
fn blank_and_comment_lines_are_skipped() {
    let text = wide("; a\n\n  ;b\n   \nk=v");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("k"),
            value: &wide("v"),
        })
    );
    assert_eq!(ini.next_value(), None);
}

#[test]
fn comment_runs_to_the_end_of_the_text() {
    let text = wide("k=v ; tail");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("k"),
            value: &wide("v"),
        })
    );
    let text = wide("[S] ; tail");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&wide("S")[..]));
    assert_eq!(ini.next_section(), None);
}

#[test]
fn empty_and_blank_texts_have_no_entries() {
    let mut ini = ini::IniReader::new(&[]);
    assert_eq!(ini.next_section(), None);
    assert_eq!(ini.next_value(), None);

    let text = wide("   \n\t\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_value(), None); // oracle ini-whitespace-only-lines
}

#[test]
fn value_is_trimmed_at_both_ends() {
    let text = wide("  a b  =\t c  ;x\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("a b"),
            value: &wide("c"),
        })
    );

    let text = wide("k =\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(
        ini.next_value(),
        Some(ini::Entry {
            name: &wide("k"),
            value: &[],
        })
    ); // oracle ini-empty-value
}

#[test]
fn section_names_keep_inner_text_and_trim_the_bracket_span() {
    let text = wide("[]\n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&[][..])); // oracle ini-empty-section-name

    let text = wide("[x");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&[][..]));

    let text = wide("[A]   \n");
    let mut ini = ini::IniReader::new(&text);
    assert_eq!(ini.next_section(), Some(&wide("A")[..]));
}
