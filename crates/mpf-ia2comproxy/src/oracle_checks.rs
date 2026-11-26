//! Oracle-derived rows for the ported ia2comproxy data.
//!
//! The rows are printed by `target/canon/ia2comproxy/oracle.c`, built with
//! `gcc -fsanitize=address,undefined -g -Wall -Wextra`. The harness parses the verbatim reference
//! text that `gen.sh` copies in: `ia2comproxy.spec`, the `MODULE`, `IMPORTS` and `dlldata_EXTRADEFS`
//! lines of `Makefile.in`, and the `uuid(...)` attributes of `ia2_classes.idl` and
//! `iaccessible2.idl`. The 17 printed rows are all checked here. ASan and UBSan flagged none of
//! them, so 0 rows are excluded.

use super::*;

#[test]
fn oracle_export_rows() {
    // row 01 spec name=DllCanUnloadNow conv=stdcall private=1 params=0 types=-
    assert_eq!(
        EXPORTS[0],
        Export {
            name: "DllCanUnloadNow",
            convention: CallingConvention::Stdcall,
            private: true,
            params: &[],
        }
    );
    // row 02 spec name=DllGetClassObject conv=stdcall private=1 params=3 types=ptr,ptr,ptr
    assert_eq!(
        EXPORTS[1],
        Export {
            name: "DllGetClassObject",
            convention: CallingConvention::Stdcall,
            private: true,
            params: &["ptr", "ptr", "ptr"],
        }
    );
    // row 03 spec name=DllRegisterServer conv=stdcall private=1 params=0 types=-
    assert_eq!(
        EXPORTS[2],
        Export {
            name: "DllRegisterServer",
            convention: CallingConvention::Stdcall,
            private: true,
            params: &[],
        }
    );
    // row 04 spec name=DllUnregisterServer conv=stdcall private=1 params=0 types=-
    assert_eq!(
        EXPORTS[3],
        Export {
            name: "DllUnregisterServer",
            convention: CallingConvention::Stdcall,
            private: true,
            params: &[],
        }
    );
    // row 05 order names=DllCanUnloadNow,DllGetClassObject,DllRegisterServer,DllUnregisterServer
    assert_eq!(
        EXPORTS.map(|export| export.name).join(","),
        "DllCanUnloadNow,DllGetClassObject,DllRegisterServer,DllUnregisterServer"
    );
    // row 06 counts exports=4 private=4 stub=0. The table has no stub kind, so stub=0 holds by
    // construction and is not asserted here.
    assert_eq!(EXPORTS.len(), 4);
    assert_eq!(EXPORTS.iter().filter(|export| export.private).count(), 4);
}

#[test]
fn oracle_lookup_rows() {
    // row 07 lookup name="DllGetClassObject" index=1
    assert_eq!(find_export("DllGetClassObject"), Some(EXPORTS[1]));
    // row 08 lookup name="DllCanUnloadNow" index=0
    assert_eq!(find_export("DllCanUnloadNow"), Some(EXPORTS[0]));
    // row 09 lookup name="dllgetclassobject" index=-1
    assert_eq!(find_export("dllgetclassobject"), None);
    // row 10 lookup name="DllMain" index=-1
    assert_eq!(find_export("DllMain"), None);
    // row 11 lookup name="MergeFontPackage" index=-1
    assert_eq!(find_export("MergeFontPackage"), None);
    // row 12 lookup name="" index=-1
    assert_eq!(find_export(""), None);
}

#[test]
fn oracle_makefile_rows() {
    // row 13 makefile MODULE=ia2comproxy.dll
    assert_eq!(MODULE, "ia2comproxy.dll");
    // row 14 makefile IMPORTS=uuid,oleacc,oleaut32,ole32,rpcrt4 count=5
    assert_eq!(IMPORTS.join(","), "uuid,oleacc,oleaut32,ole32,rpcrt4");
    // row 15 makefile dlldata_EXTRADEFS=-DWINE_REGISTER_DLL,-DPROXY_DELEGATION count=2
    assert_eq!(
        DLLDATA_EXTRADEFS.join(","),
        "-DWINE_REGISTER_DLL,-DPROXY_DELEGATION"
    );
}

#[test]
fn oracle_guid_rows() {
    // row 16 coclass PSFactoryBuffer canon=01c20f2b-3dd2-400f-949f-ad00bdab1d41
    assert_eq!(
        CLSID_PS_FACTORY_BUFFER.to_idl_string(),
        "01c20f2b-3dd2-400f-949f-ad00bdab1d41"
    );
    // row 17 iid IAccessibleHyperlink canon=01c20f2b-3dd2-400f-949f-ad00bdab1d41 same_as_coclass=1
    // iaccessible2.idl writes the IID in upper case, so the comparison is case-folded.
    assert_eq!(
        CLSID_PS_FACTORY_BUFFER.to_idl_string(),
        "01C20F2B-3DD2-400f-949F-AD00BDAB1D41".to_ascii_lowercase()
    );
}
