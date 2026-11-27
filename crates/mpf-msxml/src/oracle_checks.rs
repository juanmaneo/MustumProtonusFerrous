//! Oracle-derived test vectors for the ported msxml logic.
//!
//! Vectors were captured by compiling `target/canon/msxml/oracle.c` with
//! `gcc -fsanitize=address,undefined -g` and running it against `dlls/msxml`. The two winebuild
//! helpers in it are verbatim: `get_dll_name` (`tools/winebuild/import.c:284-291`) and the
//! statements of `split_forward` (`import.c:423-427`). The export table, the class table and
//! `MODULE` are read from `msxml.spec`, `msxml.rgs` and `Makefile.in` at run time. The harness
//! printed 17 rows, and neither sanitizer flagged a row, so none is excluded.

use super::*;

#[test]
fn oracle_export_rows() {
    // row 01 export [DllCanUnloadNow] -> args=0 forward=none
    assert_eq!(
        export("DllCanUnloadNow"),
        Some(Export {
            name: "DllCanUnloadNow",
            arg_count: 0,
            forward: None,
        })
    );

    // row 02 export [DllGetClassObject] -> args=3 forward=msxml3.DllGetClassObject
    assert_eq!(
        export("DllGetClassObject"),
        Some(Export {
            name: "DllGetClassObject",
            arg_count: 3,
            forward: Some("msxml3.DllGetClassObject"),
        })
    );

    // row 03 export [DllRegisterServer] -> args=0 forward=none
    assert_eq!(
        export("DllRegisterServer"),
        Some(Export {
            name: "DllRegisterServer",
            arg_count: 0,
            forward: None,
        })
    );

    // row 04 export [DllUnregisterServer] -> args=0 forward=none
    assert_eq!(
        export("DllUnregisterServer"),
        Some(Export {
            name: "DllUnregisterServer",
            arg_count: 0,
            forward: None,
        })
    );

    // row 05 export [DllMain] -> none
    assert_eq!(export("DllMain"), None);

    // row 06 export [dllcanunloadnow] -> none
    assert_eq!(export("dllcanunloadnow"), None);

    // row 07 export [msxml3.DllGetClassObject] -> none
    assert_eq!(export("msxml3.DllGetClassObject"), None);
}

#[test]
fn oracle_forward_rows() {
    // row 08 split_forward [msxml3.DllGetClassObject] -> dll_name=msxml3.dll api_name=DllGetClassObject
    let (module, api) = split_forward("msxml3.DllGetClassObject").unwrap();
    assert_eq!(get_dll_name(module), "msxml3.dll");
    assert_eq!(api, "DllGetClassObject");

    // row 09 split_forward [a.b.c] -> dll_name=a.b api_name=c
    let (module, api) = split_forward("a.b.c").unwrap();
    assert_eq!(get_dll_name(module), "a.b");
    assert_eq!(api, "c");

    // row 10 get_dll_name [msxml3] -> msxml3.dll
    assert_eq!(get_dll_name("msxml3"), "msxml3.dll");

    // row 11 get_dll_name [msxml3.dll] -> msxml3.dll
    assert_eq!(get_dll_name("msxml3.dll"), "msxml3.dll");

    // row 12 get_dll_name [a.b] -> a.b
    assert_eq!(get_dll_name("a.b"), "a.b");
}

#[test]
fn oracle_com_class_rows() {
    // row 13 com_class [MSXML.DOMDocument] -> description=XML DOM Document
    //        clsid={2933BF90-7B36-11D2-B20E-00C04F983E60} cur_ver=Microsoft.XMLDOM.1.0
    assert_eq!(
        com_class("MSXML.DOMDocument"),
        Some(ComClass {
            prog_id: "MSXML.DOMDocument",
            description: "XML DOM Document",
            clsid: "{2933BF90-7B36-11D2-B20E-00C04F983E60}",
            cur_ver: "Microsoft.XMLDOM.1.0",
        })
    );

    // row 14 com_class [MSXML.FreeThreadedDOMDocument] -> description=Free Threaded XML DOM Document
    //        clsid={2933BF91-7B36-11D2-B20E-00C04F983E60} cur_ver=Microsoft.FreeThreadedXMLDOM.1.0
    assert_eq!(
        com_class("MSXML.FreeThreadedDOMDocument"),
        Some(ComClass {
            prog_id: "MSXML.FreeThreadedDOMDocument",
            description: "Free Threaded XML DOM Document",
            clsid: "{2933BF91-7B36-11D2-B20E-00C04F983E60}",
            cur_ver: "Microsoft.FreeThreadedXMLDOM.1.0",
        })
    );

    // row 15 com_class [Microsoft.XMLDOM.1.0] -> none
    assert_eq!(com_class("Microsoft.XMLDOM.1.0"), None);

    // row 16 com_class [] -> none
    assert_eq!(com_class(""), None);
}

#[test]
fn oracle_module_row() {
    // row 17 MODULE -> msxml.dll
    assert_eq!(MODULE_NAME, "msxml.dll");
}
