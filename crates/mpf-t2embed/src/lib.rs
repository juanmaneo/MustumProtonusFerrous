//! t2embed subset for MPF: the embedding-permission logic of Wine's
//! `dlls/t2embed/main.c`.
//!
//! Ported (pure computation over values the caller supplies):
//! - `TTGetEmbeddingType`, `_TTGetEmbeddingType@8`: `embedding_type` maps `fsType` to `EMBED_*`; the GDI read of `fsType` and the `hDC`/`status` checks are skipped.
//! - `TTIsEmbeddingEnabledForFacename`, `_TTIsEmbeddingEnabledForFacename@8`: `embedding_enabled_for_facename` applies the lookup to registry values already read; opening the registry key and the `enabled` out-pointer are skipped.
//!
//! Skipped:
//! - `TTIsEmbeddingEnabled`, `_TTIsEmbeddingEnabled@8`: the facename comes from `GetOutlineTextMetricsA` on a live HDC.
//! - `TTLoadEmbeddedFont`, `_TTLoadEmbeddedFont@40`, `TTEmbedFont`, `_TTEmbedFont@44`, `TTDeleteEmbeddedFont`: Wine's bodies only log a FIXME and return `E_API_NOTIMPL`, so there is no logic to port.
//! - `_TTDeleteEmbeddedFont@12`, `TTCharToUnicode`, `TTEmbedFontFromFileA`, `TTEnableEmbeddingForFacename`, `TTGetEmbeddedFontInfo`, `TTRunValidationTests`, `_TTCharToUnicode@24`, `_TTEmbedFontFromFileA@52`, `_TTEnableEmbeddingForFacename@8`, `_TTGetEmbeddedFontInfo@28`, `_TTRunValidationTests@8`, `TTEmbedFontEx`, `TTRunValidationTestsEx`, `TTGetNewFontName`: `stub` in the spec, so Wine does not implement them.

use std::ffi::CStr;

/// `E_FACENAMEINVALID`: the facename pointer is null.
pub const E_FACENAMEINVALID: i32 = 0x0113;

/// `LF_FACESIZE`. `RegEnumValueA` reports `ERROR_MORE_DATA` for names of this many characters or more.
const LF_FACESIZE: usize = 32;

const LICENSE_TYPE_MASK: u16 = 0x000f;
const LICENSE_NOEMBEDDING: u16 = 0x0002;
const LICENSE_PREVIEWPRINT: u16 = 0x0004;
const LICENSE_EDITABLE: u16 = 0x0008;

/// The `EMBED_*` status reported by `TTGetEmbeddingType`. The discriminants are Wine's values.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbeddingType {
    PreviewPrint = 1,
    Editable = 2,
    Installable = 3,
    NoEmbedding = 4,
}

/// Classifies a font's `fsType` as `TTGetEmbeddingType` does.
///
/// Wine reads `fsType` into a `WORD` and keeps its low four bits. Editable is tested first, then preview-print, then no-embedding. Installable (`LICENSE_INSTALLABLE`, `0`) is the fallback, and so is any unassigned bit such as `0x1`, which Wine also reports as installable.
pub fn embedding_type(fs_type: u16) -> EmbeddingType {
    let flags = fs_type & LICENSE_TYPE_MASK;
    if flags & LICENSE_EDITABLE != 0 {
        EmbeddingType::Editable
    } else if flags & LICENSE_PREVIEWPRINT != 0 {
        EmbeddingType::PreviewPrint
    } else if flags & LICENSE_NOEMBEDDING != 0 {
        EmbeddingType::NoEmbedding
    } else {
        EmbeddingType::Installable
    }
}

/// A value under the t2embed registry key, as the caller read it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegistryValue<'a> {
    /// The value name, without a terminating NUL.
    pub name: &'a [u8],
    /// The `REG_DWORD` data, or `None` for any other registry type.
    pub dword: Option<u32>,
}

/// Whether embedding is enabled for `facename`, as `TTIsEmbeddingEnabledForFacename` decides it.
///
/// `values` lists the key's values in enumeration order. Wine skips values that are not `REG_DWORD`, and names of `LF_FACESIZE` characters or more. The first remaining value whose name matches `facename` decides, and nonzero data means enabled. With no decision, embedding is enabled, which is also the result for a missing or empty key.
///
/// `None` is a null facename and yields `E_FACENAMEINVALID`. Names are compared ASCII-case-insensitively. Wine's `lstrcmpiA` folds through the active code page, so non-ASCII names may compare differently.
pub fn embedding_enabled_for_facename(
    facename: Option<&CStr>,
    values: &[RegistryValue<'_>],
) -> Result<bool, i32> {
    let facename = facename.ok_or(E_FACENAMEINVALID)?.to_bytes();
    let decision = values.iter().find_map(|value| {
        let dword = value.dword?;
        (value.name.len() < LF_FACESIZE && value.name.eq_ignore_ascii_case(facename))
            .then_some(dword != 0)
    });
    Ok(decision.unwrap_or(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    /// `fsType` rows and the `EMBED_*` status Wine's `TTGetEmbeddingType` reports for each.
    #[test]
    fn embedding_type_matches_wine_oracle() {
        let vectors: [(u16, u32); 9] = [
            (0x0000, 3),
            (0x0001, 3),
            (0x0002, 4),
            (0x0004, 1),
            (0x0006, 1),
            (0x0008, 2),
            (0x000f, 2),
            (0x0102, 4),
            (0x8000, 3),
        ];
        for (fs_type, status) in vectors {
            assert_eq!(
                embedding_type(fs_type) as u32,
                status,
                "fsType {fs_type:#06x}"
            );
        }
    }

    /// A facename, the registry values it is looked up in, and the expected result.
    type Case<'a> = (Option<&'a str>, &'a [RegistryValue<'a>], Result<bool, i32>);

    fn value(name: &[u8], dword: Option<u32>) -> RegistryValue<'_> {
        RegistryValue { name, dword }
    }

    /// Facename rows and the result Wine's `TTIsEmbeddingEnabledForFacename` reports for each.
    #[test]
    fn facename_lookup_matches_wine_oracle() {
        let thirty_one = "a".repeat(31);
        let thirty_two = "a".repeat(32);
        let cases: [Case<'_>; 10] = [
            (Some("Arial"), &[], Ok(true)),
            (Some("Arial"), &[value(b"Arial", Some(0))], Ok(false)),
            (Some("Arial"), &[value(b"Arial", Some(1))], Ok(true)),
            (Some("arial"), &[value(b"ARIAL", Some(0))], Ok(false)),
            (Some("Tahoma"), &[value(b"Arial", Some(0))], Ok(true)),
            (
                Some("Arial"),
                &[value(b"Arial", Some(0)), value(b"Arial", Some(1))],
                Ok(false),
            ),
            (
                Some("Arial"),
                &[value(b"Arial", None), value(b"arial", Some(0))],
                Ok(false),
            ),
            (
                Some(&thirty_two),
                &[value(thirty_two.as_bytes(), Some(0))],
                Ok(true),
            ),
            (
                Some(&thirty_one),
                &[value(thirty_one.as_bytes(), Some(0))],
                Ok(false),
            ),
            (None, &[], Err(E_FACENAMEINVALID)),
        ];
        for (facename, values, expected) in cases {
            let facename = facename.map(|name| CString::new(name).unwrap());
            assert_eq!(
                embedding_enabled_for_facename(facename.as_deref(), values),
                expected,
                "facename {facename:?}"
            );
        }
    }
}
