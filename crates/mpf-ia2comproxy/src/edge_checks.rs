//! Edge-case tests for the ported ia2comproxy data.
//!
//! The oracle rows cover the spec and Makefile data as printed by the reference harness. These
//! tests cover the lookup and formatting branches around them. Expectations that are not
//! oracle-derived say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: the CLSID field split is done by hand from the uuid text in
    // dlls/ia2comproxy/ia2_classes.idl. The module name is read from dlls/ia2comproxy/Makefile.in.
    assert_eq!(CLSID_PS_FACTORY_BUFFER.data1, 0x01c2_0f2b);
    assert_eq!(CLSID_PS_FACTORY_BUFFER.data2, 0x3dd2);
    assert_eq!(CLSID_PS_FACTORY_BUFFER.data3, 0x400f);
    assert_eq!(
        CLSID_PS_FACTORY_BUFFER.data4,
        [0x94, 0x9f, 0xad, 0x00, 0xbd, 0xab, 0x1d, 0x41]
    );
    assert_eq!(MODULE, "ia2comproxy.dll");
}

#[test]
fn lookups_are_exact_and_case_sensitive() {
    // Not oracle-derived: near-miss names around the exact match. The oracle covers a case-folded
    // name and the empty name; these cover extra, missing and padded characters.
    assert_eq!(find_export("DllGetClassObjectA"), None);
    assert_eq!(find_export("DllCanUnloadNow "), None);
    assert_eq!(find_export(" DllRegisterServer"), None);
    assert_eq!(find_export("DllUnregisterServe"), None);
    assert_eq!(find_export("DllRegisterServer"), Some(EXPORTS[2]));
    assert_eq!(find_export("DllUnregisterServer"), Some(EXPORTS[3]));
}

#[test]
fn idl_text_form_keeps_leading_zeros() {
    // Not oracle-derived: boundary values for the padding of each group.
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    assert_eq!(zero.to_idl_string(), "00000000-0000-0000-0000-000000000000");

    let low = Guid {
        data1: 0x0000_000a,
        data2: 0x000b,
        data3: 0x000c,
        data4: [0x0d, 0x0e, 0x0f, 0x01, 0x02, 0x03, 0x04, 0x05],
    };
    assert_eq!(low.to_idl_string(), "0000000a-000b-000c-0d0e-0f0102030405");

    let max = Guid {
        data1: u32::MAX,
        data2: u16::MAX,
        data3: u16::MAX,
        data4: [0xff; 8],
    };
    assert_eq!(max.to_idl_string(), "ffffffff-ffff-ffff-ffff-ffffffffffff");
}

#[test]
fn import_and_define_lookups_are_exact() {
    // Not oracle-derived: negative and near-miss lookups. Rows 14 and 15 of the oracle cover the
    // full lists.
    assert!(is_import("uuid"));
    assert!(is_import("rpcrt4"));
    assert!(!is_import("UUID"));
    assert!(!is_import("ole32.dll"));
    assert!(!is_import(""));
    assert!(has_dlldata_define("-DWINE_REGISTER_DLL"));
    assert!(has_dlldata_define("-DPROXY_DELEGATION"));
    assert!(!has_dlldata_define("PROXY_DELEGATION"));
    assert!(!has_dlldata_define("-DPROXY"));
    assert!(!has_dlldata_define(""));
}
