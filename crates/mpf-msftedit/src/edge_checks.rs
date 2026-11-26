//! Edge-case tests for the ported msftedit logic.
//!
//! These cover the constants, the export table, the stub helpers, and the resource bounds that the
//! C `DllGetVersion` default does not check. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

/// The six-byte `VS_VERSION_INFO` header: `wLength` (not read by the port), `wValueLength`, `wType`.
fn header(val_len: u16) -> Vec<u8> {
    let mut out = vec![0; 6];
    out[2..4].copy_from_slice(&val_len.to_le_bytes());
    out
}

#[test]
fn constants_are_pinned() {
    // Oracle-derived: the sizes line of the C harness (DLLVERSIONINFO=20, DLLVERSIONINFO2=32).
    assert_eq!(DLLVERSIONINFO_SIZE, 20);
    assert_eq!(DLLVERSIONINFO2_SIZE, 32);

    // Not oracle-derived: values from include/winnt.h, include/winerror.h, include/shlwapi.h,
    // include/winternl.h, and the name passed in dlls/msftedit/msftedit_main.c.
    assert_eq!(DLL_PROCESS_DETACH, 0);
    assert_eq!(DLL_PROCESS_ATTACH, 1);
    assert_eq!(DLL_THREAD_ATTACH, 2);
    assert_eq!(DLL_THREAD_DETACH, 3);
    assert_eq!(RICHEDIT_DLL_NAME, "riched20.dll");
    assert_eq!(S_OK, 0);
    assert_eq!(E_INVALIDARG, 0x8007_0057);
    assert_eq!(DLLVER_PLATFORM_NT, 2);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the fourteen export lines of dlls/msftedit/msftedit.spec, in order, without
    // their ordinals.
    assert_eq!(
        EXPORTS,
        [
            ("IID_IRichEditOle", ExportKind::Data),
            ("IID_IRichEditOleCallback", ExportKind::Data),
            (
                "CreateTextServices",
                ExportKind::Forward("riched20.CreateTextServices")
            ),
            (
                "IID_ITextServices",
                ExportKind::Forward("riched20.IID_ITextServices")
            ),
            (
                "IID_ITextHost",
                ExportKind::Forward("riched20.IID_ITextHost")
            ),
            (
                "IID_ITextHost2",
                ExportKind::Forward("riched20.IID_ITextHost2")
            ),
            (
                "REExtendedRegisterClass",
                ExportKind::Forward("riched20.REExtendedRegisterClass")
            ),
            (
                "RichEdit10ANSIWndProc",
                ExportKind::Forward("riched20.RichEdit10ANSIWndProc")
            ),
            (
                "RichEditANSIWndProc",
                ExportKind::Forward("riched20.RichEditANSIWndProc")
            ),
            ("SetCustomTextOutHandlerEx", ExportKind::Stub),
            ("DllGetVersion", ExportKind::WinecrtDefault),
            ("RichEditWndProc", ExportKind::Stub),
            ("RichListBoxWndProc", ExportKind::Stub),
            ("RichComboBoxWndProc", ExportKind::Stub),
        ]
    );
    assert_eq!(export_kind("RichEditWndProc"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("DllGetVersion"),
        Some(ExportKind::WinecrtDefault)
    );
    assert_eq!(
        export_kind("IID_ITextHost2"),
        Some(ExportKind::Forward("riched20.IID_ITextHost2"))
    );
    assert_eq!(export_kind("richeditwndproc"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_helpers_match_stub_exports() {
    // Not oracle-derived: the four `stub` lines of the spec, paired with their helpers. The module
    // name is oracle-derived (`msftedit.dll` in rows 11-14).
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(
        stubs,
        [
            "SetCustomTextOutHandlerEx",
            "RichEditWndProc",
            "RichListBoxWndProc",
            "RichComboBoxWndProc",
        ]
    );
    let helpers = [
        set_custom_text_out_handler_ex(),
        rich_edit_wnd_proc(),
        rich_list_box_wnd_proc(),
        rich_combo_box_wnd_proc(),
    ];
    for (helper, name) in helpers.iter().zip(stubs) {
        assert_eq!(helper.function, name);
        assert_eq!(helper.module, "msftedit.dll");
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the msftedit.dll names. The fields are copied as given.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);
}

#[test]
fn truncated_resources_fail_without_reading_past_the_end() {
    // Not oracle-derived: the C code reads past the end of each of these resources (undefined
    // behaviour). The port returns E_INVALIDARG and leaves the buffer untouched.
    let untouched = DllVersionInfo {
        cb_size: 20,
        ..Default::default()
    };

    // Shorter than the header.
    let mut info = untouched;
    assert_eq!(
        dll_get_version(Some(&mut info), Some(&[0; 5])),
        E_INVALIDARG
    );
    assert_eq!(info, untouched);

    // The key runs to the end of the resource with no NUL unit.
    let mut unterminated = header(52);
    unterminated.extend_from_slice(&[0x41, 0, 0x42, 0]);
    let mut info = untouched;
    assert_eq!(
        dll_get_version(Some(&mut info), Some(&unterminated)),
        E_INVALIDARG
    );
    assert_eq!(info, untouched);

    // The key's NUL ends the resource, so the value would start at offset 12, past the 10 bytes.
    let mut no_value = header(52);
    no_value.extend_from_slice(&[0x41, 0, 0, 0]);
    let mut info = untouched;
    assert_eq!(
        dll_get_version(Some(&mut info), Some(&no_value)),
        E_INVALIDARG
    );
    assert_eq!(info, untouched);
}

#[test]
fn empty_key_value_starts_at_the_aligned_offset() {
    // Not oracle-derived: the harness only used the keys "VS_VERSION_INFO" and "A". This applies the
    // GET_VERSION_VALUE rounding to an empty key. The NUL is at offset 6, so the value is at
    // (6 + 2 + 3) & !3 = 8.
    let mut fixed = vec![0; 52];
    fixed[0..4].copy_from_slice(&0xfeef_04bd_u32.to_le_bytes());
    fixed[8..12].copy_from_slice(&0x0001_0002_u32.to_le_bytes());
    fixed[12..16].copy_from_slice(&0x0003_0004_u32.to_le_bytes());
    let mut res = header(52);
    res.extend_from_slice(&[0, 0]);
    res.extend_from_slice(&fixed);

    let mut info = DllVersionInfo {
        cb_size: 20,
        ..Default::default()
    };
    assert_eq!(dll_get_version(Some(&mut info), Some(&res)), S_OK);
    assert_eq!((info.major, info.minor, info.build), (1, 2, 3));
}

#[test]
fn null_info_and_missing_resource_both_fail() {
    // Not oracle-derived: rows 15-17 each pair one failure with a valid other argument.
    assert_eq!(dll_get_version(None, None), E_INVALIDARG);
}
