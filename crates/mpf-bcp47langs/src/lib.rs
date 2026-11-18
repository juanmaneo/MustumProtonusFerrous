//! Port of the portable logic in Wine's `dlls/bcp47langs/main.c`.
//!
//! `bcp47langs.spec` exports 71 functions. Only `GetFontFallbackLanguageList`
//! has logic in `main.c`; the other 70 are spec stubs or need the OS. The
//! port takes the language tag as a UTF-16 slice. Wine's NULL-`lang` branch
//! reads the user default locale from the OS, so the caller passes that
//! resolved tag instead.
//!
//! Ported (1 of 71):
//! - `GetFontFallbackLanguageList`: [`get_font_fallback_language_list`]; the NULL-`lang` branch is skipped, since it reads the OS default locale.
//!
//! Skipped (70 of 71):
//! - `GetUserLanguages`: Wine logs a FIXME and returns `S_OK` without writing `*langs`; the result is an HSTRING, which needs the WinRT string runtime.
//! - `GetUnIsoRegionCode`: stub in Wine, no body in main.c, nothing to port.
//! - `AppendUserLanguageInputMethods`: stub in Wine, no body in main.c, nothing to port.
//! - `AppendUserLanguageInternal`: stub in Wine, no body in main.c, nothing to port.
//! - `AppendUserLanguages`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47BufferFromLcid`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47FindClosestLanguage`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47FromCompactTagInternal`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47FromHkl`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47FromLcid`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetAbbreviation`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetDirectionality`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetDistance`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetExtensionSingletons`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetExtensionSubstring`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetIsoLanguageCode`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetIsoScriptCode`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetLanguageName`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetMuiForm`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetNeutralForm`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetNlsForm`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetSubtagMapInternal`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47GetUnIsoRegionCode`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47IsWellFormed`: stub in Wine, no body in main.c, nothing to port.
//! - `Bcp47Normalize`: stub in Wine, no body in main.c, nothing to port.
//! - `ClearApplicationLanguageOverride`: stub in Wine, no body in main.c, nothing to port.
//! - `ClearApplicationManifestLanguages`: stub in Wine, no body in main.c, nothing to port.
//! - `ClearHttpAcceptLanguageOptOut`: stub in Wine, no body in main.c, nothing to port.
//! - `ClearUserDisplayLanguageOverride`: stub in Wine, no body in main.c, nothing to port.
//! - `ClearUserLocaleFromLanguageProfileOptOut`: stub in Wine, no body in main.c, nothing to port.
//! - `CompactTagFromBcp47Internal`: stub in Wine, no body in main.c, nothing to port.
//! - `DllGetActivationFactory`: COM class-factory entry point; stub in Wine.
//! - `DllGetClassObject`: COM class-factory entry point; stub in Wine.
//! - `GetApplicationLanguageOverride`: stub in Wine, no body in main.c, nothing to port.
//! - `GetApplicationLanguages`: stub in Wine, no body in main.c, nothing to port.
//! - `GetApplicationManifestLanguages`: stub in Wine, no body in main.c, nothing to port.
//! - `GetAppropriateUserLocaleForUserLanguages`: stub in Wine, no body in main.c, nothing to port.
//! - `GetAppropriateUserPreferredAndDisplayLanguagesForUser`: stub in Wine, no body in main.c, nothing to port.
//! - `GetClosestMatchingUserLanguage`: stub in Wine, no body in main.c, nothing to port.
//! - `GetDisplayLanguagesForAllUsers`: stub in Wine, no body in main.c, nothing to port.
//! - `GetHttpAcceptLanguageOptOut`: stub in Wine, no body in main.c, nothing to port.
//! - `GetInputMethodOverrideForUser`: stub in Wine, no body in main.c, nothing to port.
//! - `GetPendingUserDisplayLanguage`: stub in Wine, no body in main.c, nothing to port.
//! - `GetRelevantLocalesFromLanguageTags`: stub in Wine, no body in main.c, nothing to port.
//! - `GetSerializedUserLanguageProfile`: stub in Wine, no body in main.c, nothing to port.
//! - `GetSerializedUserLanguagesForUser`: stub in Wine, no body in main.c, nothing to port.
//! - `GetStartingUserDisplayLanguage`: stub in Wine, no body in main.c, nothing to port.
//! - `GetUserDisplayLanguageOverride`: stub in Wine, no body in main.c, nothing to port.
//! - `GetUserLanguageInputMethods`: stub in Wine, no body in main.c, nothing to port.
//! - `GetUserLanguageInputMethodsForUser`: stub in Wine, no body in main.c, nothing to port.
//! - `GetUserLanguagesForAllUsers`: stub in Wine, no body in main.c, nothing to port.
//! - `GetUserLanguagesForUser`: stub in Wine, no body in main.c, nothing to port.
//! - `GetUserLocaleFromLanguageProfileOptOut`: stub in Wine, no body in main.c, nothing to port.
//! - `IsTransientLcid`: stub in Wine, no body in main.c, nothing to port.
//! - `LanguageListAsMuiForm`: stub in Wine, no body in main.c, nothing to port.
//! - `LcidFromBcp47`: stub in Wine, no body in main.c, nothing to port.
//! - `RemoveInputsForAllLanguagesInternal`: stub in Wine, no body in main.c, nothing to port.
//! - `RemoveUserLanguageInputMethods`: stub in Wine, no body in main.c, nothing to port.
//! - `ResolveLanguages`: stub in Wine, no body in main.c, nothing to port.
//! - `SetApplicationLanguageOverride`: stub in Wine, no body in main.c, nothing to port.
//! - `SetApplicationManifestLanguages`: stub in Wine, no body in main.c, nothing to port.
//! - `SetHttpAcceptLanguageOptOut`: stub in Wine, no body in main.c, nothing to port.
//! - `SetInputMethodOverride`: stub in Wine, no body in main.c, nothing to port.
//! - `SetPreviousUserDisplayLanguages`: stub in Wine, no body in main.c, nothing to port.
//! - `SetStartingUserDisplayLanguage`: stub in Wine, no body in main.c, nothing to port.
//! - `SetUserDisplayLanguageOverride`: stub in Wine, no body in main.c, nothing to port.
//! - `SetUserLanguageInputMethods`: stub in Wine, no body in main.c, nothing to port.
//! - `SetUserLanguagesInternal`: stub in Wine, no body in main.c, nothing to port.
//! - `SetUserLanguagesInternalCore`: stub in Wine, no body in main.c, nothing to port.
//! - `SetUserLocaleFromLanguageProfileOptOut`: stub in Wine, no body in main.c, nothing to port.

/// Success HRESULT.
pub const S_OK: i32 = 0;
/// Invalid argument HRESULT.
pub const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;
/// Insufficient buffer HRESULT, `HRESULT_FROM_WIN32(ERROR_INSUFFICIENT_BUFFER)`.
pub const E_NOT_SUFFICIENT_BUFFER: i32 = 0x8007_007au32 as i32;

/// Length of the UTF-16 string before its first NUL, or the slice length.
fn wide_len(s: &[u16]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
}

/// Copies `lang` into `buffer` as a NUL-terminated UTF-16 string, the
/// non-NULL-`lang` path of `GetFontFallbackLanguageList`.
///
/// `lang` is read up to its first NUL, or to the end of the slice when it has
/// none. A `None` or empty `buffer` is `E_INVALIDARG` and leaves
/// `required_buffer_length` untouched. Otherwise `required_buffer_length`
/// receives the length including the terminator, even when the buffer is too
/// small and the result is `E_NOT_SUFFICIENT_BUFFER`.
pub fn get_font_fallback_language_list(
    lang: &[u16],
    buffer: Option<&mut [u16]>,
    required_buffer_length: &mut usize,
) -> i32 {
    let buffer = match buffer {
        Some(buffer) if !buffer.is_empty() => buffer,
        _ => return E_INVALIDARG,
    };
    let tag = &lang[..wide_len(lang)];
    *required_buffer_length = tag.len() + 1;
    if buffer.len() < *required_buffer_length {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    buffer[..tag.len()].copy_from_slice(tag);
    buffer[tag.len()] = 0;
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Value `harness.c` fills every output slot with before the call.
    const FILL: u16 = 0xcccc;
    /// Value `harness.c` stores in `*required_buffer_length` before the call.
    const UNTOUCHED: usize = 0xdead_beef;

    /// One oracle row, from `target/canon/bcp47langs/output.txt`. `capacity`
    /// `None` passes a NULL buffer. `required` `None` means the length is not
    /// written. `output` is the written prefix of the buffer, with the rest
    /// still `FILL`.
    struct Vector {
        name: &'static str,
        lang: &'static str,
        capacity: Option<usize>,
        hr: i32,
        required: Option<usize>,
        output: &'static str,
    }

    /// Oracle vectors from the verbatim Wine body, built with
    /// `gcc -fsanitize=address,undefined -g` and run clean under both.
    const VECTORS: [Vector; 21] = [
        Vector {
            name: "en-US cap128",
            lang: "en-US",
            capacity: Some(128),
            hr: S_OK,
            required: Some(6),
            output: "en-US\0",
        },
        Vector {
            name: "en-US cap6 exact",
            lang: "en-US",
            capacity: Some(6),
            hr: S_OK,
            required: Some(6),
            output: "en-US\0",
        },
        Vector {
            name: "en-US cap5",
            lang: "en-US",
            capacity: Some(5),
            hr: E_NOT_SUFFICIENT_BUFFER,
            required: Some(6),
            output: "",
        },
        Vector {
            name: "en-US cap1",
            lang: "en-US",
            capacity: Some(1),
            hr: E_NOT_SUFFICIENT_BUFFER,
            required: Some(6),
            output: "",
        },
        Vector {
            name: "en-US cap0",
            lang: "en-US",
            capacity: Some(0),
            hr: E_INVALIDARG,
            required: None,
            output: "",
        },
        Vector {
            name: "en-US null buffer",
            lang: "en-US",
            capacity: None,
            hr: E_INVALIDARG,
            required: None,
            output: "",
        },
        Vector {
            name: "empty cap1",
            lang: "",
            capacity: Some(1),
            hr: S_OK,
            required: Some(1),
            output: "\0",
        },
        Vector {
            name: "empty cap0",
            lang: "",
            capacity: Some(0),
            hr: E_INVALIDARG,
            required: None,
            output: "",
        },
        Vector {
            name: "deadbeef cap16",
            lang: "deadbeef",
            capacity: Some(16),
            hr: S_OK,
            required: Some(9),
            output: "deadbeef\0",
        },
        Vector {
            name: "zh-Hant-TW cap16",
            lang: "zh-Hant-TW",
            capacity: Some(16),
            hr: S_OK,
            required: Some(11),
            output: "zh-Hant-TW\0",
        },
        Vector {
            name: "sr-Latn-RS cap10",
            lang: "sr-Latn-RS",
            capacity: Some(10),
            hr: E_NOT_SUFFICIENT_BUFFER,
            required: Some(11),
            output: "",
        },
        Vector {
            name: "sr-Latn-RS cap11",
            lang: "sr-Latn-RS",
            capacity: Some(11),
            hr: S_OK,
            required: Some(11),
            output: "sr-Latn-RS\0",
        },
        Vector {
            name: "en embedded NUL",
            lang: "en\0US",
            capacity: Some(16),
            hr: S_OK,
            required: Some(3),
            output: "en\0",
        },
        Vector {
            name: "en-US NUL trailing",
            lang: "en-US\0garbage",
            capacity: Some(6),
            hr: S_OK,
            required: Some(6),
            output: "en-US\0",
        },
        Vector {
            name: "surrogate pair",
            lang: "a\u{1F600}",
            capacity: Some(16),
            hr: S_OK,
            required: Some(4),
            output: "a\u{1F600}\0",
        },
        Vector {
            name: "latin-1 e-acute",
            lang: "\u{e9}-FR",
            capacity: Some(16),
            hr: S_OK,
            required: Some(5),
            output: "\u{e9}-FR\0",
        },
        Vector {
            name: "x-a38 cap41",
            lang: "x-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            capacity: Some(41),
            hr: S_OK,
            required: Some(41),
            output: "x-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\0",
        },
        Vector {
            name: "x-a38 cap40",
            lang: "x-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            capacity: Some(40),
            hr: E_NOT_SUFFICIENT_BUFFER,
            required: Some(41),
            output: "",
        },
        Vector {
            name: "zh-Hans cap3",
            lang: "zh-Hans",
            capacity: Some(3),
            hr: E_NOT_SUFFICIENT_BUFFER,
            required: Some(8),
            output: "",
        },
        Vector {
            name: "ru-RU cap6",
            lang: "ru-RU",
            capacity: Some(6),
            hr: S_OK,
            required: Some(6),
            output: "ru-RU\0",
        },
        Vector {
            name: "ja-JP cap16",
            lang: "ja-JP",
            capacity: Some(16),
            hr: S_OK,
            required: Some(6),
            output: "ja-JP\0",
        },
    ];

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().collect()
    }

    #[test]
    fn matches_oracle_vectors() {
        for v in &VECTORS {
            let lang = wide(v.lang);
            let mut required = UNTOUCHED;
            let mut buffer = v.capacity.map(|capacity| vec![FILL; capacity]);
            let hr = get_font_fallback_language_list(&lang, buffer.as_deref_mut(), &mut required);
            assert_eq!(hr, v.hr, "{}", v.name);
            assert_eq!(required, v.required.unwrap_or(UNTOUCHED), "{}", v.name);
            if let Some(buffer) = buffer {
                let mut expected = wide(v.output);
                expected.resize(buffer.len(), FILL);
                assert_eq!(buffer, expected, "{}", v.name);
            }
        }
    }

    #[test]
    fn lang_without_nul_is_taken_whole() {
        // Not an oracle vector: the C body reads past a missing NUL, so this
        // pins only the port's defined behaviour.
        let mut buffer = [FILL; 4];
        let mut required = UNTOUCHED;
        let hr = get_font_fallback_language_list(&wide("ab"), Some(&mut buffer), &mut required);
        assert_eq!(hr, S_OK);
        assert_eq!(required, 3);
        assert_eq!(buffer, [0x61, 0x62, 0, FILL]);

        let hr = get_font_fallback_language_list(&[], Some(&mut buffer), &mut required);
        assert_eq!(hr, S_OK);
        assert_eq!(required, 1);
        assert_eq!(buffer, [0, 0x62, 0, FILL]);
    }
}
