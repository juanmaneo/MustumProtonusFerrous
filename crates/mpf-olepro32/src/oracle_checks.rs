//! Oracle-derived test vectors for the ported olepro32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `DllGetClassObject`
//! (`dlls/olepro32/olepro32stubs.c`) and the verbatim `dlls/olepro32/olepro32.spec` lines, which
//! the harness parses, with `gcc -fsanitize=address,undefined -g` under `target/canon/olepro32/`.
//! The 21 printed rows (6 `DllGetClassObject` rows, 7 forward rows and 8 name lookups) all ran
//! clean under both sanitizers, so none are excluded.

use super::*;

/// `(ordinal, name, stack_bytes, target_dll, target_name)` from rows 07 to 13.
const FORWARD_ROWS: [(u16, &str, u32, &str, &str); 7] = [
    (248, "OleIconToCursor", 8, "oleaut32", "OleIconToCursor"),
    (
        249,
        "OleCreatePropertyFrameIndirect",
        4,
        "oleaut32",
        "OleCreatePropertyFrameIndirect",
    ),
    (
        250,
        "OleCreatePropertyFrame",
        44,
        "oleaut32",
        "OleCreatePropertyFrame",
    ),
    (251, "OleLoadPicture", 20, "oleaut32", "OleLoadPicture"),
    (
        252,
        "OleCreatePictureIndirect",
        16,
        "oleaut32",
        "OleCreatePictureIndirect",
    ),
    (
        253,
        "OleCreateFontIndirect",
        12,
        "oleaut32",
        "OleCreateFontIndirect",
    ),
    (
        254,
        "OleTranslateColor",
        12,
        "oleaut32",
        "OleTranslateColor",
    ),
];

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 01 class_object rclsid=NULL riid=NULL ppv=NULL -> hr=0
    assert_eq!(dll_get_class_object(), 0);
    // row 02 class_object rclsid=guid riid=NULL ppv=NULL -> hr=0
    assert_eq!(dll_get_class_object(), 0);
    // row 03 class_object rclsid=NULL riid=guid ppv=NULL -> hr=0
    assert_eq!(dll_get_class_object(), 0);
    // row 04 class_object rclsid=guid riid=guid ppv=slot -> hr=0
    assert_eq!(dll_get_class_object(), 0);
    // row 05 class_object rclsid=NULL riid=NULL ppv=slot -> hr=0
    assert_eq!(dll_get_class_object(), 0);
    // row 06 class_object rclsid=guid riid=guid ppv=NULL -> hr=0
    assert_eq!(dll_get_class_object(), 0);
}

#[test]
fn oracle_forward_rows() {
    for (i, &(ordinal, name, stack_bytes, target_dll, target_name)) in
        FORWARD_ROWS.iter().enumerate()
    {
        let expected = Forward {
            ordinal,
            name,
            stack_bytes,
            target_dll,
            target_name,
        };
        assert_eq!(FORWARDS[i], expected);
        assert_eq!(forward_for(name), Some(expected));
    }
}

#[test]
fn oracle_lookup_rows() {
    // row 14 lookup name="OleIconToCursor" -> ordinal=248 stack=8
    assert_eq!(
        forward_for("OleIconToCursor").map(|f| (f.ordinal, f.stack_bytes)),
        Some((248, 8))
    );
    // row 15 lookup name="OleTranslateColor" -> ordinal=254 stack=12
    assert_eq!(
        forward_for("OleTranslateColor").map(|f| (f.ordinal, f.stack_bytes)),
        Some((254, 12))
    );
    // row 16 lookup name="DllGetClassObject" -> none
    assert_eq!(forward_for("DllGetClassObject"), None);
    // row 17 lookup name="DllCanUnloadNow" -> none
    assert_eq!(forward_for("DllCanUnloadNow"), None);
    // row 18 lookup name="DllRegisterServer" -> none
    assert_eq!(forward_for("DllRegisterServer"), None);
    // row 19 lookup name="olecreatefontindirect" -> none
    assert_eq!(forward_for("olecreatefontindirect"), None);
    // row 20 lookup name="" -> none
    assert_eq!(forward_for(""), None);
    // row 21 lookup name="OleCreateFontIndirectX" -> none
    assert_eq!(forward_for("OleCreateFontIndirectX"), None);
}
