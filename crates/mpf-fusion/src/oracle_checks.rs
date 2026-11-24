//! Replays of the oracle rows O01 to O23 in `target/canon/fusion`. The oracle compiles the verbatim
//! `dlls/fusion` ranges (the `asmname.c` parsers, the `assembly.c` helpers and `token_to_str`) with
//! `gcc -fsanitize=address,undefined -g`, using the Win32 shims in `mock.h`. Each `oNN` test
//! replays row ONN, and its expected values are the ones the C printed.
//!
//! Sanitizer-flagged vectors are left out. ASan reports a heap-buffer-overflow when a blob length
//! prefix is cut short: `GetData` on the one-byte inputs 0x80 and 0xC0. The port returns `None`
//! there, and `edge_checks::blob_prefix_cut_short_is_none` covers that.

use crate::{
    create_assembly_name, decode_blob_length, rva_to_offset, token_to_str, AssemblyName, Hresult,
    PeKind, Section, FUSION_E_INVALID_NAME, FUSION_E_PRIVATE_ASM_DISALLOWED, TOKEN_LENGTH,
};

pub(crate) const SECTIONS: [Section; 2] = [
    Section {
        virtual_address: 0x1000,
        size_of_raw_data: 0x200,
        pointer_to_raw_data: 0x400,
        virtual_size: 0x180,
    },
    Section {
        virtual_address: 0x2000,
        size_of_raw_data: 0,
        pointer_to_raw_data: 0x800,
        virtual_size: 0x100,
    },
];

pub(crate) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

pub(crate) fn parse(text: &str) -> Result<AssemblyName, Hresult> {
    create_assembly_name(Some(&wide(text)), 0)
}

/// The result for `display` with only the display and simple names set.
pub(crate) fn named(display: &str, name: &str) -> AssemblyName {
    AssemblyName {
        display_name: Some(wide(display)),
        name: Some(wide(name)),
        ..AssemblyName::default()
    }
}

/// `decode_blob_length` as the oracle prints it: the length and the bytes the prefix used.
pub(crate) fn blob(data: &[u8]) -> Option<(u32, usize)> {
    decode_blob_length(data).map(|(length, rest)| (length, data.len() - rest.len()))
}

#[test]
fn o01_bare_name() {
    assert_eq!(parse("Foo"), Ok(named("Foo", "Foo")));
}

#[test]
fn o02_version_four_parts() {
    let mut want = named("Foo, Version=1.2.3.4", "Foo");
    want.version = [1, 2, 3, 4];
    want.versize = 4;
    assert_eq!(parse("Foo, Version=1.2.3.4"), Ok(want));
}

#[test]
fn o03_version_parts_wrap_to_16_bits() {
    let mut want = named("Foo, Version=65537.-1", "Foo");
    want.version = [1, 65535, 0, 0];
    want.versize = 2;
    assert_eq!(parse("Foo, Version=65537.-1"), Ok(want));
}

#[test]
fn o04_repeated_version_overwrites_and_counts() {
    let mut want = named("Foo, Version=1.2, Version=5", "Foo");
    want.version = [5, 2, 0, 0];
    want.versize = 3;
    assert_eq!(parse("Foo, Version=1.2, Version=5"), Ok(want));
}

#[test]
fn o05_two_character_culture_is_kept() {
    let mut want = named("Foo, Culture=en", "Foo");
    want.culture = Some(wide("en"));
    assert_eq!(parse("Foo, Culture=en"), Ok(want));
}

#[test]
fn o06_other_culture_length_is_emptied() {
    let mut want = named("Foo, Culture=en-US", "Foo");
    want.culture = Some(Vec::new());
    assert_eq!(parse("Foo, Culture=en-US"), Ok(want));
}

#[test]
fn o07_public_key_token_is_read_as_bytes() {
    let mut want = named("Foo, PublicKeyToken=0123456789abcdef", "Foo");
    want.pubkey = Some([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
    assert_eq!(parse("Foo, PublicKeyToken=0123456789abcdef"), Ok(want));
}

#[test]
fn o08_public_key_ignores_text_after_16_hex_digits() {
    let mut want = named("Foo, PublicKeyToken=0123456789ABCDEFxyz", "Foo");
    want.pubkey = Some([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
    assert_eq!(parse("Foo, PublicKeyToken=0123456789ABCDEFxyz"), Ok(want));
}

#[test]
fn o09_public_key_null_is_disallowed() {
    assert_eq!(
        parse("Foo, PublicKeyToken=NULL"),
        Err(FUSION_E_PRIVATE_ASM_DISALLOWED)
    );
}

#[test]
fn o10_public_key_non_hex_digit_is_invalid() {
    assert_eq!(
        parse("Foo, PublicKeyToken=0123456789abcdeg"),
        Err(FUSION_E_INVALID_NAME)
    );
}

#[test]
fn o11_architecture_amd64() {
    let mut want = named("Foo, processorArchitecture=AMD64", "Foo");
    want.procarch = Some(wide("AMD64"));
    want.pekind = PeKind::Amd64;
    assert_eq!(parse("Foo, processorArchitecture=AMD64"), Ok(want));
}

#[test]
fn o12_unknown_architecture_is_invalid() {
    assert_eq!(
        parse("Foo, processorArchitecture=arm"),
        Err(FUSION_E_INVALID_NAME)
    );
}

#[test]
fn o13_space_without_comma_is_invalid() {
    assert_eq!(parse("Foo Bar"), Err(FUSION_E_INVALID_NAME));
}

#[test]
fn o14_quoted_version_value() {
    let mut want = named("Foo, Version=\"1.0\"", "Foo");
    want.version = [1, 0, 0, 0];
    want.versize = 2;
    assert_eq!(parse("Foo, Version=\"1.0\""), Ok(want));
}

#[test]
fn o15_unclosed_quote_is_invalid() {
    assert_eq!(parse("Foo, Version=\"1.0"), Err(FUSION_E_INVALID_NAME));
}

#[test]
fn o16_blob_length_one_byte() {
    assert_eq!(blob(&[0x03]), Some((3, 1)));
}

#[test]
fn o17_blob_length_two_bytes() {
    assert_eq!(blob(&[0x80, 0x80]), Some((128, 2)));
}

#[test]
fn o18_blob_length_four_bytes() {
    assert_eq!(blob(&[0xC0, 0x00, 0x40, 0x00]), Some((16384, 4)));
}

#[test]
fn o19_blob_lead_byte_above_c0_is_none() {
    assert_eq!(blob(&[0xE0, 0x00, 0x00, 0x00]), None);
}

#[test]
fn o20_rva_before_first_raw_pointer_is_unchanged() {
    assert_eq!(rva_to_offset(&SECTIONS, 0x300), 768);
}

#[test]
fn o21_rva_in_first_section() {
    assert_eq!(rva_to_offset(&SECTIONS, 0x1010), 1040);
}

#[test]
fn o22_rva_in_section_with_zero_raw_size() {
    assert_eq!(rva_to_offset(&SECTIONS, 0x2050), 2128);
}

#[test]
fn o23_public_key_token_hex() {
    let out = token_to_str(&[0xab, 0x00, 0x7f, 0x80, 0xfe, 0x10, 0x01, 0xc3]);
    let text = String::from_utf16(&out[..TOKEN_LENGTH - 1]).unwrap();
    assert_eq!(text, "ab007f80fe1001c3");
}
