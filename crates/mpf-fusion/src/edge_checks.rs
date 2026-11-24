//! Edge checks for the rows E01 to E52 in `target/canon/fusion`, from the oracle's `edge_main.c`
//! (which has no E21). Each test names the rows it replays, and the expected values are the ones
//! the verbatim C printed. The module also covers the port's own behaviour: the `CANOF` argument
//! check, the NUL terminator of `token_to_str`, the HRESULT values and the derived trait impls,
//! which the rows do not call.

use crate::oracle_checks::{blob, named, parse, wide, SECTIONS};
use crate::{
    create_assembly_name, failed, rva_to_offset, token_to_str, AssemblyName, PeKind, Section,
    CANOF_PARSE_DISPLAY_NAME, E_INVALIDARG, FUSION_E_INVALID_NAME, FUSION_E_PRIVATE_ASM_DISALLOWED,
    S_OK, TOKEN_LENGTH,
};

/// `named("Foo", "Foo")` with a version and its part count.
fn versioned(display: &str, version: [u16; 4], versize: u32) -> AssemblyName {
    let mut name = named(display, "Foo");
    name.version = version;
    name.versize = versize;
    name
}

#[test]
fn e01_e03_version_parts_and_empty_values() {
    // E01: a fifth part is never read.
    assert_eq!(
        parse("Foo, Version=1.2.3.4.5"),
        Ok(versioned("Foo, Version=1.2.3.4.5", [1, 2, 3, 4], 4))
    );
    // E03: an empty value is accepted, and the next key still parses.
    let mut want = named("Foo, Version=,Culture=x", "Foo");
    want.culture = Some(Vec::new());
    assert_eq!(parse("Foo, Version=,Culture=x"), Ok(want));
}

#[test]
fn e02_e04_missing_equals_or_value_is_invalid() {
    assert_eq!(parse("Foo, Version="), Err(FUSION_E_INVALID_NAME));
    assert_eq!(parse("Foo, Version"), Err(FUSION_E_INVALID_NAME));
}

#[test]
fn e05_key_with_space_before_equals_is_unknown() {
    assert_eq!(
        parse("Foo, Version = 1.0"),
        Ok(named("Foo, Version = 1.0", "Foo"))
    );
}

#[test]
fn e06_e07_spaces_and_unknown_keys() {
    assert_eq!(
        parse("Foo,   Version=2.0"),
        Ok(versioned("Foo,   Version=2.0", [2, 0, 0, 0], 2))
    );
    assert_eq!(
        parse("Foo, Custom=1, Version=3"),
        Ok(versioned("Foo, Custom=1, Version=3", [3, 0, 0, 0], 1))
    );
}

#[test]
fn e08_lone_sign_reads_as_zero() {
    assert_eq!(
        parse("Foo, Version=-"),
        Ok(versioned("Foo, Version=-", [0, 0, 0, 0], 1))
    );
}

#[test]
fn e09_e10_values_clamp_to_long_then_truncate() {
    assert_eq!(
        parse("Foo, Version=2147483648"),
        Ok(versioned("Foo, Version=2147483648", [65535, 0, 0, 0], 1))
    );
    assert_eq!(
        parse("Foo, Version=-2147483649"),
        Ok(versioned("Foo, Version=-2147483649", [0, 0, 0, 0], 1))
    );
}

#[test]
fn e11_e12_leading_whitespace_and_plus_sign() {
    assert_eq!(
        parse("Foo, Version= \t 7"),
        Ok(versioned("Foo, Version= \t 7", [7, 0, 0, 0], 1))
    );
    assert_eq!(
        parse("Foo, Version=+4"),
        Ok(versioned("Foo, Version=+4", [4, 0, 0, 0], 1))
    );
}

#[test]
fn e13_name_is_cut_at_nul() {
    assert_eq!(parse("Foo\0, Version=1"), Ok(named("Foo", "Foo")));
}

#[test]
fn e14_space_before_comma_is_allowed() {
    let mut want = named("Foo Bar, Version=1", "Foo Bar");
    want.version = [1, 0, 0, 0];
    want.versize = 1;
    assert_eq!(parse("Foo Bar, Version=1"), Ok(want));
}

#[test]
fn e15_e17_short_or_non_hex_public_key_is_invalid() {
    assert_eq!(
        parse("Foo, PublicKeyToken=0123456789abcde"),
        Err(FUSION_E_INVALID_NAME)
    );
    assert_eq!(
        parse("Foo, PublicKeyToken=nullify0123456"),
        Err(FUSION_E_INVALID_NAME)
    );
}

#[test]
fn e16_e36_null_public_key_in_any_case_is_disallowed() {
    assert_eq!(
        parse("Foo, PublicKeyToken=NuLL"),
        Err(FUSION_E_PRIVATE_ASM_DISALLOWED)
    );
    assert_eq!(
        parse("Foo, Version=1.2, PublicKeyToken=null"),
        Err(FUSION_E_PRIVATE_ASM_DISALLOWED)
    );
}

#[test]
fn e18_architecture_ignores_case() {
    let mut want = named("Foo, processorArchitecture=X86", "Foo");
    want.procarch = Some(wide("X86"));
    want.pekind = PeKind::I386;
    assert_eq!(parse("Foo, processorArchitecture=X86"), Ok(want));
}

#[test]
fn e19_e20_architecture_must_match_exactly() {
    assert_eq!(
        parse("Foo, processorArchitecture=msil "),
        Err(FUSION_E_INVALID_NAME)
    );
    assert_eq!(
        parse("Foo, processorArchitecture="),
        Err(FUSION_E_INVALID_NAME)
    );
}

#[test]
fn e22_text_after_closing_quote_is_ignored() {
    assert_eq!(
        parse("Foo, Version=\"1.0\"x"),
        Ok(versioned("Foo, Version=\"1.0\"x", [1, 0, 0, 0], 2))
    );
}

#[test]
fn e23_e24_unbalanced_quotes_are_invalid() {
    assert_eq!(parse("Foo, Version=a\"b"), Err(FUSION_E_INVALID_NAME));
    assert_eq!(parse("Foo, Version=\""), Err(FUSION_E_INVALID_NAME));
}

#[test]
fn e25_e26_empty_and_quoted_values() {
    assert_eq!(
        parse("Foo, Version=\"\""),
        Ok(named("Foo, Version=\"\"", "Foo"))
    );
    let mut want = named("Foo, Culture=\"en\"", "Foo");
    want.culture = Some(wide("en"));
    assert_eq!(parse("Foo, Culture=\"en\""), Ok(want));
}

#[test]
fn e27_e28_empty_name_and_empty_key() {
    assert_eq!(parse(""), Ok(named("", "")));
    assert_eq!(parse("Foo, =x"), Ok(named("Foo, =x", "Foo")));
}

#[test]
fn e29_leading_space_without_comma_is_invalid() {
    assert_eq!(parse(" Foo"), Err(FUSION_E_INVALID_NAME));
}

#[test]
fn e30_e32_version_separators_and_empty_parts() {
    assert_eq!(
        parse("Foo,Version=1"),
        Ok(versioned("Foo,Version=1", [1, 0, 0, 0], 1))
    );
    // E31: an empty part still counts toward versize.
    assert_eq!(
        parse("Foo, Version=1..2"),
        Ok(versioned("Foo, Version=1..2", [1, 0, 2, 0], 3))
    );
    // E32: a trailing dot ends the parse.
    assert_eq!(
        parse("Foo, Version=1.2."),
        Ok(versioned("Foo, Version=1.2.", [1, 2, 0, 0], 2))
    );
}

#[test]
fn e33_e35_separator_without_a_pair_is_invalid() {
    assert_eq!(parse("Foo,"), Err(FUSION_E_INVALID_NAME));
    assert_eq!(parse("Foo, "), Err(FUSION_E_INVALID_NAME));
    assert_eq!(parse("Foo, Version=,"), Err(FUSION_E_INVALID_NAME));
}

#[test]
fn e37_quoted_version_then_culture() {
    let mut want = versioned("Foo, Version=\"1.0\", Culture=ab", [1, 0, 0, 0], 2);
    want.culture = Some(wide("ab"));
    assert_eq!(parse("Foo, Version=\"1.0\", Culture=ab"), Ok(want));
}

#[test]
fn e38_repeated_version_keeps_the_later_parts() {
    assert_eq!(
        parse("Foo, Version=1.2.3.4, Version=9"),
        Ok(versioned(
            "Foo, Version=1.2.3.4, Version=9",
            [9, 2, 3, 4],
            5
        ))
    );
}

#[test]
fn e39_part_wraps_to_16_bits_and_stops_at_x() {
    assert_eq!(
        parse("Foo, Version=65536.0x1"),
        Ok(versioned("Foo, Version=65536.0x1", [0, 0, 0, 0], 2))
    );
}

#[test]
fn e40_e45_blob_length_prefixes() {
    assert_eq!(blob(&[0x7F]), Some((127, 1)));
    assert_eq!(blob(&[0x80, 0x00]), Some((0, 2)));
    assert_eq!(blob(&[0xBF, 0xFF]), Some((16383, 2)));
    assert_eq!(blob(&[0xC0, 0x00, 0x00, 0x00]), Some((0, 4)));
    assert_eq!(blob(&[0xDF, 0xFF, 0xFF, 0xFF]), Some((536_870_911, 4)));
    assert_eq!(blob(&[0xFF]), None);
}

#[test]
fn blob_prefix_cut_short_is_none() {
    assert_eq!(blob(&[0x80]), None);
    assert_eq!(blob(&[0xC0, 0x00, 0x00]), None);
    assert_eq!(blob(&[]), None);
}

#[test]
fn e46_e50_rva_inside_and_outside_sections() {
    assert_eq!(rva_to_offset(&SECTIONS, 0x11ff), 1535);
    assert_eq!(rva_to_offset(&SECTIONS, 0x1200), 0);
    assert_eq!(rva_to_offset(&SECTIONS, 0x500), 0);
    assert_eq!(rva_to_offset(&SECTIONS, 0x2100), 0);
    assert_eq!(rva_to_offset(&SECTIONS, 0x2000), 2048);
    assert_eq!(rva_to_offset(&[], 0x1000), 0);
}

#[test]
fn e51_section_without_raw_pointer_keeps_the_rva() {
    let sections = [
        Section {
            virtual_address: 0x1000,
            size_of_raw_data: 0x200,
            pointer_to_raw_data: 0x400,
            virtual_size: 0x180,
        },
        Section {
            virtual_address: 0x3000,
            size_of_raw_data: 0x100,
            pointer_to_raw_data: 0,
            virtual_size: 0x100,
        },
    ];
    assert_eq!(rva_to_offset(&sections, 0x3010), 12304);
}

#[test]
fn e52_section_past_32_bit_space_matches_nothing() {
    let sections = [Section {
        virtual_address: 0xFFFF_FF00,
        size_of_raw_data: 0x200,
        pointer_to_raw_data: 0x400,
        virtual_size: 0,
    }];
    assert_eq!(rva_to_offset(&sections, 0xFFFF_FF80), 0);
}

#[test]
fn create_flag_rejects_null_or_empty_name() {
    assert_eq!(
        create_assembly_name(None, CANOF_PARSE_DISPLAY_NAME),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        create_assembly_name(Some(&[0u16]), CANOF_PARSE_DISPLAY_NAME),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        create_assembly_name(Some(&wide("Foo")), CANOF_PARSE_DISPLAY_NAME),
        Ok(named("Foo", "Foo"))
    );
}

#[test]
fn null_name_without_flag_is_an_empty_result() {
    assert_eq!(create_assembly_name(None, 0), Ok(AssemblyName::default()));
    assert_eq!(create_assembly_name(Some(&[0u16]), 0), Ok(named("", "")));
}

#[test]
fn non_ascii_text_does_not_fold_to_a_key_name() {
    // The fullwidth letters are not ASCII, so they do not match "null" even at the same length.
    assert_eq!(
        parse("Foo, PublicKeyToken=\u{FF2E}\u{FF35}\u{FF2C}\u{FF2C}"),
        Err(FUSION_E_INVALID_NAME)
    );
}

#[test]
fn public_key_non_hex_at_each_nibble_is_invalid() {
    assert_eq!(
        parse("Foo, PublicKeyToken=g123456789abcdef"),
        Err(FUSION_E_INVALID_NAME)
    );
    assert_eq!(
        parse("Foo, PublicKeyToken=0123456789abcdgf"),
        Err(FUSION_E_INVALID_NAME)
    );
}

#[test]
fn token_to_str_ends_with_nul_and_uses_lowercase_hex() {
    let out = token_to_str(&[0xff; 8]);
    assert_eq!(out[TOKEN_LENGTH - 1], 0);
    assert_eq!(
        String::from_utf16(&out[..TOKEN_LENGTH - 1]).unwrap(),
        "ffffffffffffffff"
    );
}

#[test]
fn hresult_values_match_winerror_and_corerror() {
    assert_eq!(E_INVALIDARG as u32, 0x8007_0057);
    assert_eq!(FUSION_E_INVALID_NAME as u32, 0x8013_1047);
    assert_eq!(FUSION_E_PRIVATE_ASM_DISALLOWED as u32, 0x8013_1044);
    assert_eq!(CANOF_PARSE_DISPLAY_NAME, 0x1);
    assert!(failed(FUSION_E_INVALID_NAME));
    assert!(!failed(S_OK));
}

#[test]
fn derived_traits_format_and_clone() {
    assert_eq!(format!("{:?}", PeKind::NoKind), "NoKind");
    assert_eq!(format!("{:?}", PeKind::Msil), "Msil");
    assert_eq!(format!("{:?}", PeKind::I386), "I386");
    assert_eq!(format!("{:?}", PeKind::Ia64), "Ia64");
    assert_eq!(format!("{:?}", PeKind::Amd64), "Amd64");
    assert_eq!(PeKind::default(), PeKind::NoKind);
    let name = named("Foo", "Foo");
    assert_eq!(name.clone(), name);
    assert!(format!("{name:?}").starts_with("AssemblyName {"));
}
