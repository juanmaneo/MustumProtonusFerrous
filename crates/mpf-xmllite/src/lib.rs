//! xmllite subset for MPF, ported from Wine dlls/xmllite/reader.c and writer.c.
//!
//! Ported: the character classes; the encoding name table, the lookups by name and by code
//! page, and get_code_page; readerinput_detectencoding over a byte buffer; the NCName, Name,
//! QName, PubId, NmToken and whitespace validators; is_valid_xml_space_value;
//! writer_get_next_write_count, write_escaped_string, the bodies of xmlwriter_WriteCharEntity,
//! xmlwriter_WriteSurrogateCharEntity and xmlwriter_WriteComment; and reader_update_position
//! and get_predefined_entity.
//!
//! Skipped: the XML declaration parser and the reader and writer state machines (reference,
//! attribute and text parsing, WriteChars, WriteComment's state checks, write_node_indent),
//! the input streams, the COM objects, and is_wchar_encname, whose only caller is the
//! declaration parser.
//!
//! Divergences: the C reads past the end of its input in two places, and the port defines
//! those reads instead. readerinput_detectencoding reads missing bytes as 0 (a 3-byte input
//! reads byte 3), and get_encoding_name(Unknown) returns None where the C indexes past its
//! table. NULL has no slice form. Where the C accepts NULL it gives the same result as empty
//! text, except in the counted parse_encoding_prefix (NULL gives Unknown, an empty slice gives
//! UTF-16). C dereferences NULL in is_valid_qname, is_valid_nmtoken and is_valid_whitespace.
//! Wine's only counted caller (reader.c:1278) never passes an empty name.
//!
//! Kept from C: the encoding name lookup accepts any prefix of a name (`UTF-1` is UTF-16), and
//! the first-byte UTF-8 test reads the second byte as a signed char, so its three shift tests
//! never hold on x86. Case folding is ASCII only, as `_towlower_l` folds in msvcrt/wcs.c when
//! no CTYPE locale is loaded; a loaded CTYPE locale would also fold non-ASCII letters, which
//! the port does not model. Folding up or down gives the same result, because only names of
//! letters, digits and `-` can match, and those order the same either way.
//!
//! Text is UTF-16 code units, as in Wine.

mod chars;
mod encoding;
mod escape;
mod names;
mod position;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use chars::{
    is_char, is_namechar, is_namestartchar, is_ncnamechar, is_ncnamestartchar, is_pubchar,
    is_wchar_space,
};
pub use encoding::{
    detect_encoding, get_code_page, get_encoding_from_codepage, get_encoding_name,
    parse_encoding_name, parse_encoding_prefix, XmlEncoding,
};
pub use escape::{
    char_entity, escape_chars, escape_string, next_write_count, surrogate_entity, write_comment,
};
pub use names::{
    is_valid_name, is_valid_ncname, is_valid_nmtoken, is_valid_pubid, is_valid_qname,
    is_valid_whitespace, is_valid_xml_space_value,
};
pub use position::{advance, predefined_entity, Position};

pub const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;
pub const MX_E_INPUTEND: i32 = 0xc00c_ee01u32 as i32;
pub const WC_E_XMLCHARACTER: i32 = 0xc00c_ee2bu32 as i32;
pub const WC_E_NAMECHARACTER: i32 = 0xc00c_ee2cu32 as i32;
pub const WC_E_PUBLICID: i32 = 0xc00c_ee42u32 as i32;
pub const WR_E_INVALIDSURROGATEPAIR: i32 = 0xc00c_ef0cu32 as i32;

/// The units before the first NUL, which is how the C functions read a string.
pub(crate) fn c_str(units: &[u16]) -> &[u16] {
    let end = units
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(units.len());
    &units[..end]
}

/// Whether the units spell `text`, which must be ASCII.
pub(crate) fn eq_ascii(units: &[u16], text: &str) -> bool {
    units.len() == text.len()
        && units
            .iter()
            .zip(text.bytes())
            .all(|(&unit, byte)| unit == u16::from(byte))
}

#[cfg(test)]
pub(crate) fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}
