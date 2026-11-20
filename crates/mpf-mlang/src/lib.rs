//! Deterministic part of Wine's `dlls/mlang/mlang.c`: the Japanese JIS and Shift-JIS codec, the
//! Japanese code detection, and the codepage family checks behind `IsConvertINetStringAvailable`.
//!
//! Ported (names in `dlls/mlang/mlang.c`):
//! - `DetectJapaneseCode`: [detect::japanese_code].
//! - `ConvertJIS2SJIS` and `ConvertSJIS2JIS`: [jis::jis_to_sjis] and [jis::sjis_to_jis].
//! - `jis2sjis`, `sjis2jis` and `han2zen`: [jis::jis2sjis], [jis::sjis2jis] and [jis::han2zen].
//! - `enter_shift` and `exit_shift` (here `enter_jis` and `exit_jis`), and the byte-class macros
//!   `HANKATA`, `ISEUC`, `NOTEUC`, `SJIS1`, `SJIS2`, `ISMARU` and `ISNIGORI`: private helpers in
//!   the `jis`, `detect` and `classes` modules.
//! - `GetFamilyCodePage` and the codepage lists of `mlang_data`: [family::family_codepage].
//! - `IsConvertINetStringAvailable`: [is_convert_inet_string_available].
//!
//! Skipped (OS, locale or COM): `ConvertINetMultiByteToUnicode`, `ConvertINetUnicodeToMultiByte`,
//! `ConvertINetString`, `ConvertJISJapaneseToUnicode`, `ConvertUnknownJapaneseToUnicode` and
//! `ConvertJapaneseUnicodeToJIS`, which call `MultiByteToWideChar` and `WideCharToMultiByte`;
//! `LcidToRfc1766` and `Rfc1766ToLcid`, which use the locale API; the font-mapping data; the
//! `IEnumXXX`, `IMultiLanguage` and `IMLangFontLink` objects; the class factory and
//! `DllGetClassObject`; and the MIME descriptions, charsets and flags of the `*_cp` tables.
//!
//! Deviations from Wine:
//! - [jis::jis_to_sjis] and [jis::sjis_to_jis] return `None` where `ConvertJIS2SJIS` and
//!   `ConvertSJIS2JIS` return `0` on truncated input. A successful empty conversion is `Some` of
//!   an empty vector, so the two cases stay apart.
//! - [jis::sjis2jis] has undefined behaviour in C for a lead byte below 112 or from 160 to 175.
//!   The Rust result is the two's complement wrap that gcc gives. [jis::sjis_to_jis] never passes
//!   such a byte.

pub mod detect;
pub mod family;
pub mod jis;

mod classes;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

use family::family_codepage;

/// `HRESULT`. This crate returns only [S_OK] and [S_FALSE].
pub type HResult = u32;
pub const S_OK: HResult = 0;
pub const S_FALSE: HResult = 1;

/// The `CP_UNICODE` pseudo codepage. It is also the family of the Unicode group.
pub const CP_UNICODE: u32 = 1200;

/// `IsConvertINetStringAvailable`: [S_OK] when both codepages belong to known families and the
/// families match, or when either family is Unicode. Otherwise [S_FALSE].
pub fn is_convert_inet_string_available(src: u32, dst: u32) -> HResult {
    let (Some(src_family), Some(dst_family)) = (family_codepage(src), family_codepage(dst)) else {
        return S_FALSE;
    };
    if src_family == dst_family || src_family == CP_UNICODE || dst_family == CP_UNICODE {
        S_OK
    } else {
        S_FALSE
    }
}
