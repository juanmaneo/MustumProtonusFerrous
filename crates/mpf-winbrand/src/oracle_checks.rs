//! Oracle-derived test vectors for the ported winbrand logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `BrandingFormatString`
//! (`dlls/winbrand/main.c`), `RtlExpandEnvironmentStrings` and `ENV_FindVariable`
//! (`dlls/ntdll/env.c`), and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`), with
//! `gcc -fsanitize=address,undefined -g -fshort-wchar` under `target/canon/winbrand/`, and running
//! the harness. The 37 printed rows are 17 `BrandingFormatString` rows, 6 expand rows and 14 stub
//! rows. ASan and UBSan flagged none of them, so none is excluded.
//!
//! `render` is the harness output mapping: NUL prints as `~` and the 0xAAAA sentinel as `.`.

use super::*;
use crate::format::BRANDING_ENVIRONMENT;

fn render(units: &[u16]) -> String {
    units
        .iter()
        .map(|&c| match c {
            0 => '~',
            0xAAAA => '.',
            _ => char::from_u32(u32::from(c)).unwrap_or('?'),
        })
        .collect()
}

/// `BrandingFormatString` with an allocator that either fails or returns a zeroed buffer.
fn branding(input: &str, alloc_fails: bool) -> Option<String> {
    let units: Vec<u16> = input.encode_utf16().collect();
    branding_format_string(&units, |size| {
        if alloc_fails {
            None
        } else {
            Some(vec![0; size])
        }
    })
    .map(|out| render(&out))
}

/// `RtlExpandEnvironmentStrings` into a 32-slot sentinel buffer of `count` slots.
fn expand(src: &str, count: usize) -> (NtStatus, usize, String) {
    let env: Vec<u16> = BRANDING_ENVIRONMENT.encode_utf16().collect();
    let src: Vec<u16> = src.encode_utf16().collect();
    let mut dst = [0xAAAA; 32];
    let (status, len) = expand_environment_strings(&env, &src, &mut dst[..count]);
    (status, len, render(&dst[..count]))
}

fn assert_stub(got: StubException, function: &'static str) {
    assert_eq!(
        got,
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "winbrand.dll",
            function,
        }
    );
}

#[test]
fn oracle_branding_format_string_rows() {
    // fmt 01 "%WINDOWS_SHORT%" alloc=ok -> len=11 out="Windows 10~"
    assert_eq!(
        branding("%WINDOWS_SHORT%", false).as_deref(),
        Some("Windows 10~")
    );
    // fmt 02 "%WINDOWS_LONG%" alloc=ok -> len=15 out="Windows 10 Pro~"
    assert_eq!(
        branding("%WINDOWS_LONG%", false).as_deref(),
        Some("Windows 10 Pro~")
    );
    // fmt 03 "Hello %WINDOWS_LONG% and %WINDOWS_SHORT%" alloc=ok -> len=36
    assert_eq!(
        branding("Hello %WINDOWS_LONG% and %WINDOWS_SHORT%", false).as_deref(),
        Some("Hello Windows 10 Pro and Windows 10~")
    );
    // fmt 04 "" alloc=ok -> len=1 out="~"
    assert_eq!(branding("", false).as_deref(), Some("~"));
    // fmt 05 "plain text" alloc=ok -> len=11 out="plain text~"
    assert_eq!(
        branding("plain text", false).as_deref(),
        Some("plain text~")
    );
    // fmt 06 "%windows_short%" alloc=ok -> len=11 out="Windows 10~"
    assert_eq!(
        branding("%windows_short%", false).as_deref(),
        Some("Windows 10~")
    );
    // fmt 07 "%WINDOWS%" alloc=ok -> len=10 out="%WINDOWS%~"
    assert_eq!(branding("%WINDOWS%", false).as_deref(), Some("%WINDOWS%~"));
    // fmt 08 "%MISSING%" alloc=ok -> len=10 out="%MISSING%~"
    assert_eq!(branding("%MISSING%", false).as_deref(), Some("%MISSING%~"));
    // fmt 09 "%WINDOWS_SHORT" alloc=ok -> len=15 out="%WINDOWS_SHORT~"
    assert_eq!(
        branding("%WINDOWS_SHORT", false).as_deref(),
        Some("%WINDOWS_SHORT~")
    );
    // fmt 10 "%%" alloc=ok -> len=3 out="%%~"
    assert_eq!(branding("%%", false).as_deref(), Some("%%~"));
    // fmt 11 "%%WINDOWS_SHORT%" alloc=ok -> len=17 out="%%WINDOWS_SHORT%~"
    assert_eq!(
        branding("%%WINDOWS_SHORT%", false).as_deref(),
        Some("%%WINDOWS_SHORT%~")
    );
    // fmt 12 "%WINDOWS_SHORT%%WINDOWS_LONG%" alloc=ok -> len=25
    assert_eq!(
        branding("%WINDOWS_SHORT%%WINDOWS_LONG%", false).as_deref(),
        Some("Windows 10Windows 10 Pro~")
    );
    // fmt 13 "%WINDOWS_LONG%x%" alloc=ok -> len=17 out="Windows 10 Prox%~"
    assert_eq!(
        branding("%WINDOWS_LONG%x%", false).as_deref(),
        Some("Windows 10 Prox%~")
    );
    // fmt 14 "%WINDOWS_SHORT=%" alloc=ok -> len=17 out="%WINDOWS_SHORT=%~"
    assert_eq!(
        branding("%WINDOWS_SHORT=%", false).as_deref(),
        Some("%WINDOWS_SHORT=%~")
    );
    // fmt 15 "%WINDOWS_SHORT%" alloc=fail -> NULL
    assert_eq!(branding("%WINDOWS_SHORT%", true), None);
    // fmt 16 "" alloc=fail -> NULL
    assert_eq!(branding("", true), None);
    // fmt 17 "A\0%WINDOWS_SHORT%" alloc=ok -> len=2 out="A~" (read up to the first NUL)
    assert_eq!(branding("A\0%WINDOWS_SHORT%", false).as_deref(), Some("A~"));
}

#[test]
fn oracle_expand_environment_strings_rows() {
    // expand 01 src="%WINDOWS_SHORT%" count=16 -> status=0x00000000 len=11 dst="Windows 10~....."
    assert_eq!(
        expand("%WINDOWS_SHORT%", 16),
        (STATUS_SUCCESS, 11, "Windows 10~.....".to_string())
    );
    // expand 02 src="%WINDOWS_SHORT%" count=10 -> status=0xc0000023 len=11 dst="~........."
    assert_eq!(
        expand("%WINDOWS_SHORT%", 10),
        (STATUS_BUFFER_TOO_SMALL, 11, "~.........".to_string())
    );
    // expand 03 src="ab%WINDOWS_SHORT%" count=6 -> status=0xc0000023 len=13 dst="ab~..."
    assert_eq!(
        expand("ab%WINDOWS_SHORT%", 6),
        (STATUS_BUFFER_TOO_SMALL, 13, "ab~...".to_string())
    );
    // expand 04 src="abcdef" count=3 -> status=0xc0000023 len=7 dst="ab."
    assert_eq!(
        expand("abcdef", 3),
        (STATUS_BUFFER_TOO_SMALL, 7, "ab.".to_string())
    );
    // expand 05 src="abc" count=0 -> status=0xc0000023 len=4 dst=""
    assert_eq!(
        expand("abc", 0),
        (STATUS_BUFFER_TOO_SMALL, 4, String::new())
    );
    // expand 06 src="%WINDOWS_SHORT%" count=11 -> status=0x00000000 len=11 dst="Windows 10~"
    assert_eq!(
        expand("%WINDOWS_SHORT%", 11),
        (STATUS_SUCCESS, 11, "Windows 10~".to_string())
    );
}

#[test]
fn oracle_stub_rows() {
    // stub 01 BrandingFormatStringForEdition -> code=0x80000100 flags=0x1 nparams=2
    assert_stub(
        branding_format_string_for_edition(),
        "BrandingFormatStringForEdition",
    );
    // stub 02 BrandingLoadBitmap
    assert_stub(branding_load_bitmap(), "BrandingLoadBitmap");
    // stub 03 BrandingLoadCursor
    assert_stub(branding_load_cursor(), "BrandingLoadCursor");
    // stub 04 BrandingLoadIcon
    assert_stub(branding_load_icon(), "BrandingLoadIcon");
    // stub 05 BrandingLoadImage
    assert_stub(branding_load_image(), "BrandingLoadImage");
    // stub 06 BrandingLoadNeutralStringForEdition
    assert_stub(
        branding_load_neutral_string_for_edition(),
        "BrandingLoadNeutralStringForEdition",
    );
    // stub 07 BrandingLoadString
    assert_stub(branding_load_string(), "BrandingLoadString");
    // stub 08 BrandingLoadStringForEdition
    assert_stub(
        branding_load_string_for_edition(),
        "BrandingLoadStringForEdition",
    );
    // stub 09 GetEULAFile
    assert_stub(get_eula_file(), "GetEULAFile");
    // stub 10 GetEULAFileEx
    assert_stub(get_eula_file_ex(), "GetEULAFileEx");
    // stub 11 GetEULAInCurrentUILanguage
    assert_stub(
        get_eula_in_current_ui_language(),
        "GetEULAInCurrentUILanguage",
    );
    // stub 12 GetHinstanceByNamespace
    assert_stub(get_hinstance_by_namespace(), "GetHinstanceByNamespace");
    // stub 13 GetInstalledEULAPath
    assert_stub(get_installed_eula_path(), "GetInstalledEULAPath");
    // stub 14 InstallEULA
    assert_stub(install_eula(), "InstallEULA");
}
