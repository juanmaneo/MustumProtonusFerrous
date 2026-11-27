//! Oracle-derived test vectors for the ported msxml6 records.
//!
//! Wine's dlls/msxml6 has no C sources, so there are no C bodies to compile. The vectors come
//! from the oracle harness in target/canon/msxml6/oracle.c, built with
//! gcc -fsanitize=address,undefined -g. The harness reads the verbatim dlls/msxml6/msxml6.spec
//! and include/msxml6.idl at run time and prints 17 rows: 4 exports, 1 library and 12 coclasses.
//! ASan and UBSan reported nothing, so no row is excluded.

use super::*;

/// One row of the export oracle: name, parameter types, and the forward target if any.
type ExportRow = (
    &'static str,
    &'static [&'static str],
    Option<(&'static str, &'static str)>,
);

/// The four lines of msxml6.spec, in spec order.
const EXPORT_ROWS: [ExportRow; 4] = [
    // row 01 export name=DllCanUnloadNow conv=stdcall private=yes params=0 forward=none
    ("DllCanUnloadNow", &[], None),
    // row 02 export name=DllGetClassObject conv=stdcall private=yes params=3 forward=msxml3.DllGetClassObject
    (
        "DllGetClassObject",
        &["ptr", "ptr", "ptr"],
        Some(("msxml3", "DllGetClassObject")),
    ),
    // row 03 export name=DllRegisterServer conv=stdcall private=yes params=0 forward=none
    ("DllRegisterServer", &[], None),
    // row 04 export name=DllUnregisterServer conv=stdcall private=yes params=0 forward=none
    ("DllUnregisterServer", &[], None),
];

/// One row of the coclass oracle: name, progid, threading, uuid text, then the GUID fields.
type CoClassRow = (
    &'static str,
    &'static str,
    Threading,
    &'static str,
    u32,
    u16,
    u16,
    [u8; 8],
);

/// The twelve coclasses, in file order.
const COCLASS_ROWS: [CoClassRow; 12] = [
    // row 05 coclass name=FreeThreadedXMLHTTP60 threading=both uuid=88d96a09-f192-11d4-a65f-0040963251e5
    (
        "FreeThreadedXMLHTTP60",
        "Msxml2.FreeThreadedXMLHTTP60.6.0",
        Threading::Both,
        "88d96a09-f192-11d4-a65f-0040963251e5",
        0x88d9_6a09,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 07 coclass name=DOMDocument60 threading=both uuid=88d96a05-f192-11d4-a65f-0040963251e5
    (
        "DOMDocument60",
        "Msxml2.DOMDocument.6.0",
        Threading::Both,
        "88d96a05-f192-11d4-a65f-0040963251e5",
        0x88d9_6a05,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 08 coclass name=FreeThreadedDOMDocument60 threading=both uuid=88d96a06-f192-11d4-a65f-0040963251e5
    (
        "FreeThreadedDOMDocument60",
        "Msxml2.FreeThreadedDOMDocument.6.0",
        Threading::Both,
        "88d96a06-f192-11d4-a65f-0040963251e5",
        0x88d9_6a06,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 09 coclass name=SAXXMLReader60 threading=both uuid=88d96a0c-f192-11d4-a65f-0040963251e5
    (
        "SAXXMLReader60",
        "Msxml2.SAXXMLReader.6.0",
        Threading::Both,
        "88d96a0c-f192-11d4-a65f-0040963251e5",
        0x88d9_6a0c,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 10 coclass name=XMLSchemaCache60 threading=both uuid=88d96a07-f192-11d4-a65f-0040963251e5
    (
        "XMLSchemaCache60",
        "Msxml2.XMLSchemaCache.6.0",
        Threading::Both,
        "88d96a07-f192-11d4-a65f-0040963251e5",
        0x88d9_6a07,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 11 coclass name=MXHTMLWriter60 threading=both uuid=88d96a10-f192-11d4-a65f-0040963251e5
    (
        "MXHTMLWriter60",
        "Msxml2.MXHTMLWriter.6.0",
        Threading::Both,
        "88d96a10-f192-11d4-a65f-0040963251e5",
        0x88d9_6a10,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 12 coclass name=MXXMLWriter60 threading=both uuid=88d96a0f-f192-11d4-a65f-0040963251e5
    (
        "MXXMLWriter60",
        "Msxml2.MXXMLWriter.6.0",
        Threading::Both,
        "88d96a0f-f192-11d4-a65f-0040963251e5",
        0x88d9_6a0f,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 13 coclass name=MXNamespaceManager60 threading=both uuid=88d96a11-f192-11d4-a65f-0040963251e5
    (
        "MXNamespaceManager60",
        "Msxml2.MXNamespaceManager.6.0",
        Threading::Both,
        "88d96a11-f192-11d4-a65f-0040963251e5",
        0x88d9_6a11,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 14 coclass name=SAXAttributes60 threading=both uuid=88d96a0e-f192-11d4-a65f-0040963251e5
    (
        "SAXAttributes60",
        "Msxml2.SAXAttributes.6.0",
        Threading::Both,
        "88d96a0e-f192-11d4-a65f-0040963251e5",
        0x88d9_6a0e,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 15 coclass name=XSLTemplate60 threading=both uuid=88d96a08-f192-11d4-a65f-0040963251e5
    (
        "XSLTemplate60",
        "Msxml2.XSLTemplate.6.0",
        Threading::Both,
        "88d96a08-f192-11d4-a65f-0040963251e5",
        0x88d9_6a08,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 16 coclass name=XMLHTTP60 threading=apartment uuid=88d96a0a-f192-11d4-a65f-0040963251e5
    (
        "XMLHTTP60",
        "Msxml2.XMLHTTP.6.0",
        Threading::Apartment,
        "88d96a0a-f192-11d4-a65f-0040963251e5",
        0x88d9_6a0a,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
    // row 17 coclass name=ServerXMLHTTP60 threading=apartment uuid=88d96a0b-f192-11d4-a65f-0040963251e5
    (
        "ServerXMLHTTP60",
        "Msxml2.ServerXMLHTTP.6.0",
        Threading::Apartment,
        "88d96a0b-f192-11d4-a65f-0040963251e5",
        0x88d9_6a0b,
        0xf192,
        0x11d4,
        [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
    ),
];

#[test]
fn oracle_export_rows() {
    for (name, params, forward) in EXPORT_ROWS {
        let kind = match forward {
            Some((module, function)) => ExportKind::Forward { module, function },
            None => ExportKind::Private,
        };
        assert_eq!(export(name), Some(Export { name, params, kind }), "{name}");
    }
}

#[test]
fn oracle_library_row() {
    // row 06 library name=MSXML2 version=6.0 uuid=f5078f18-c551-11d3-89b9-0000f81fe221
    //        data1=0xf5078f18 data2=0xc551 data3=0x11d3 data4=89,b9,00,00,f8,1f,e2,21
    assert_eq!(
        TYPELIB,
        TypeLib {
            name: "MSXML2",
            libid: Guid {
                data1: 0xf507_8f18,
                data2: 0xc551,
                data3: 0x11d3,
                data4: [0x89, 0xb9, 0x00, 0x00, 0xf8, 0x1f, 0xe2, 0x21],
            },
            version: (6, 0),
        }
    );
    assert_eq!(
        TYPELIB.libid.to_uuid_string(),
        "f5078f18-c551-11d3-89b9-0000f81fe221"
    );
}

#[test]
fn oracle_coclass_rows() {
    assert_eq!(COCLASSES.len(), COCLASS_ROWS.len());
    for (entry, &(name, progid, threading, uuid, data1, data2, data3, data4)) in
        COCLASSES.iter().zip(&COCLASS_ROWS)
    {
        let clsid = Guid {
            data1,
            data2,
            data3,
            data4,
        };
        assert_eq!(
            *entry,
            CoClass {
                name,
                progid,
                threading,
                clsid,
            },
            "{name}"
        );
        assert_eq!(clsid.to_uuid_string(), uuid, "{name}");
    }
}
