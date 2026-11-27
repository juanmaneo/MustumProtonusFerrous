//! Oracle-derived test vectors for the ported msxml4 tables.
//!
//! Vectors were captured by running `target/canon/msxml4/oracle.c`, built with
//! `gcc -fsanitize=address,undefined -g`, on the reference files in `dlls/msxml4`. The harness
//! printed 44 rows: the four `msxml4.spec` exports (rows 01-04), the `library MSXML2` LIBID
//! (row 05), the 36 coclass CLSIDs of `msxml4_tlb.idl` (rows 06-41), the manifest identity
//! (row 42), the `Makefile.in` module values (row 43) and the `rsrc.rc` manifest resource
//! (row 44). ASan and UBSan flagged no row, so no row is excluded.

use super::*;

/// `(name, convention, private, argc, forward)` for each `msxml4.spec` line. A `forward` of `-`
/// means the export has no target.
const SPEC_ROWS: [(&str, &str, bool, usize, &str); 4] = [
    ("DllCanUnloadNow", "stdcall", true, 0, "-"), // row 01
    (
        "DllGetClassObject",
        "stdcall",
        true,
        3,
        "msxml3.DllGetClassObject",
    ), // row 02
    ("DllRegisterServer", "stdcall", true, 0, "-"), // row 03
    ("DllUnregisterServer", "stdcall", true, 0, "-"), // row 04
];

/// `(name, uuid, LIBID)` for `library MSXML2`, row 05.
const LIBID_ROW: (&str, &str, Guid) = (
    "MSXML2",
    "f5078f18-c551-11d3-89b9-0000f81fe221",
    Guid {
        data1: 0xF5078F18,
        data2: 0xC551,
        data3: 0x11D3,
        data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
    },
);

/// `(name, uuid, CLSID)` for each coclass, in IDL order, rows 06-41.
const COCLASS_ROWS: [(&str, &str, Guid); 36] = [
    (
        "DOMDocument",
        "F6D90F11-9C73-11D3-B32E-00C04F990BB4",
        Guid {
            data1: 0xF6D90F11,
            data2: 0x9C73,
            data3: 0x11D3,
            data4: [0xB3, 0x2E, 0x00, 0xC0, 0x4F, 0x99, 0x0B, 0xB4],
        },
    ), // row 06
    (
        "DOMDocument26",
        "f5078f1b-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F1B,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 07
    (
        "DOMDocument30",
        "f5078f32-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F32,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 08
    (
        "DOMDocument40",
        "88d969c0-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969C0,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 09
    (
        "FreeThreadedDOMDocument",
        "F6D90F12-9C73-11D3-B32E-00C04F990BB4",
        Guid {
            data1: 0xF6D90F12,
            data2: 0x9C73,
            data3: 0x11D3,
            data4: [0xB3, 0x2E, 0x00, 0xC0, 0x4F, 0x99, 0x0B, 0xB4],
        },
    ), // row 10
    (
        "FreeThreadedDOMDocument26",
        "f5078f1c-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F1C,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 11
    (
        "FreeThreadedDOMDocument30",
        "f5078f33-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F33,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 12
    (
        "FreeThreadedDOMDocument40",
        "88d969c1-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969C1,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 13
    (
        "XMLHTTP",
        "f6d90f16-9c73-11d3-b32e-00c04f990bb4",
        Guid {
            data1: 0xF6D90F16,
            data2: 0x9C73,
            data3: 0x11D3,
            data4: [0xB3, 0x2E, 0x00, 0xC0, 0x4F, 0x99, 0x0B, 0xB4],
        },
    ), // row 14
    (
        "XMLHTTP26",
        "f5078f1e-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F1E,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 15
    (
        "XMLHTTP30",
        "f5078f35-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F35,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 16
    (
        "XMLHTTP40",
        "88d969c5-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969C5,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 17
    (
        "ServerXMLHTTP",
        "afba6b42-5692-48ea-8141-dc517dcf0ef1",
        Guid {
            data1: 0xAFBA6B42,
            data2: 0x5692,
            data3: 0x48EA,
            data4: [0x81, 0x41, 0xDC, 0x51, 0x7D, 0xCF, 0x0E, 0xF1],
        },
    ), // row 18
    (
        "ServerXMLHTTP30",
        "afb40ffd-b609-40a3-9828-f88bbe11e4e3",
        Guid {
            data1: 0xAFB40FFD,
            data2: 0xB609,
            data3: 0x40A3,
            data4: [0x98, 0x28, 0xF8, 0x8B, 0xBE, 0x11, 0xE4, 0xE3],
        },
    ), // row 19
    (
        "ServerXMLHTTP40",
        "88d969c6-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969C6,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 20
    (
        "XMLSchemaCache",
        "373984c9-b845-449b-91e7-45ac83036ade",
        Guid {
            data1: 0x373984C9,
            data2: 0xB845,
            data3: 0x449B,
            data4: [0x91, 0xE7, 0x45, 0xAC, 0x83, 0x03, 0x6A, 0xDE],
        },
    ), // row 21
    (
        "XMLSchemaCache26",
        "f5078f1d-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F1D,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 22
    (
        "XMLSchemaCache30",
        "f5078f34-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F34,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 23
    (
        "XMLSchemaCache40",
        "88d969c2-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969C2,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 24
    (
        "XSLTemplate",
        "2933BF94-7B36-11d2-B20E-00C04F983E60",
        Guid {
            data1: 0x2933BF94,
            data2: 0x7B36,
            data3: 0x11D2,
            data4: [0xB2, 0x0E, 0x00, 0xC0, 0x4F, 0x98, 0x3E, 0x60],
        },
    ), // row 25
    (
        "XSLTemplate26",
        "f5078f21-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F21,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 26
    (
        "XSLTemplate30",
        "f5078f36-c551-11d3-89b9-0000f81fe221",
        Guid {
            data1: 0xF5078F36,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ), // row 27
    (
        "XSLTemplate40",
        "88d969c3-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969C3,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 28
    (
        "SAXXMLReader",
        "079aa557-4a18-424a-8eee-e39f0a8d41b9",
        Guid {
            data1: 0x079AA557,
            data2: 0x4A18,
            data3: 0x424A,
            data4: [0x8E, 0xEE, 0xE3, 0x9F, 0x0A, 0x8D, 0x41, 0xB9],
        },
    ), // row 29
    (
        "SAXXMLReader30",
        "3124c396-fb13-4836-a6ad-1317f1713688",
        Guid {
            data1: 0x3124C396,
            data2: 0xFB13,
            data3: 0x4836,
            data4: [0xA6, 0xAD, 0x13, 0x17, 0xF1, 0x71, 0x36, 0x88],
        },
    ), // row 30
    (
        "SAXXMLReader40",
        "7c6e29bc-8b8b-4c3d-859e-af6cd158be0f",
        Guid {
            data1: 0x7C6E29BC,
            data2: 0x8B8B,
            data3: 0x4C3D,
            data4: [0x85, 0x9E, 0xAF, 0x6C, 0xD1, 0x58, 0xBE, 0x0F],
        },
    ), // row 31
    (
        "MXHTMLWriter",
        "a4c23ec3-6b70-4466-9127-550077239978",
        Guid {
            data1: 0xA4C23EC3,
            data2: 0x6B70,
            data3: 0x4466,
            data4: [0x91, 0x27, 0x55, 0x00, 0x77, 0x23, 0x99, 0x78],
        },
    ), // row 32
    (
        "MXHTMLWriter30",
        "853d1540-c1a7-4aa9-a226-4d3bd301146d",
        Guid {
            data1: 0x853D1540,
            data2: 0xC1A7,
            data3: 0x4AA9,
            data4: [0xA2, 0x26, 0x4D, 0x3B, 0xD3, 0x01, 0x14, 0x6D],
        },
    ), // row 33
    (
        "MXHTMLWriter40",
        "88d969c9-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969C9,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 34
    (
        "MXXMLWriter",
        "fc220ad8-a72a-4ee8-926e-0b7ad152a020",
        Guid {
            data1: 0xFC220AD8,
            data2: 0xA72A,
            data3: 0x4EE8,
            data4: [0x92, 0x6E, 0x0B, 0x7A, 0xD1, 0x52, 0xA0, 0x20],
        },
    ), // row 35
    (
        "MXXMLWriter30",
        "3d813dfe-6c91-4a4e-8f41-04346a841d9c",
        Guid {
            data1: 0x3D813DFE,
            data2: 0x6C91,
            data3: 0x4A4E,
            data4: [0x8F, 0x41, 0x04, 0x34, 0x6A, 0x84, 0x1D, 0x9C],
        },
    ), // row 36
    (
        "MXXMLWriter40",
        "88d969c8-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969C8,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 37
    (
        "MXNamespaceManager",
        "88d969d5-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969D5,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 38
    (
        "MXNamespaceManager40",
        "88d969d6-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969D6,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 39
    (
        "SAXAttributes",
        "4dd441ad-526d-4a77-9f1b-9841ed802fb0",
        Guid {
            data1: 0x4DD441AD,
            data2: 0x526D,
            data3: 0x4A77,
            data4: [0x9F, 0x1B, 0x98, 0x41, 0xED, 0x80, 0x2F, 0xB0],
        },
    ), // row 40
    (
        "SAXAttributes40",
        "88d969ca-f192-11d4-a65f-0040963251e5",
        Guid {
            data1: 0x88D969CA,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ), // row 41
];

/// The `forward` text that [`SPEC_ROWS`] uses for `target`: `-` when unbound.
fn target_text(target: ExportTarget) -> String {
    match target {
        ExportTarget::Unbound => "-".to_string(),
        ExportTarget::Forward { module, function } => format!("{module}.{function}"),
    }
}

/// The 8-4-4-4-12 text of `guid`, in uppercase hex.
fn uuid_text(guid: Guid) -> String {
    let d4 = guid.data4;
    format!(
        "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        guid.data1, guid.data2, guid.data3, d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7],
    )
}

#[test]
fn oracle_spec_rows() {
    assert_eq!(EXPORTS.len(), SPEC_ROWS.len());
    for (name, convention, private, argc, forward) in SPEC_ROWS {
        let entry = export_entry(name).expect("exported by msxml4.spec");
        assert_eq!(entry.convention, convention);
        assert_eq!(entry.private, private);
        assert_eq!(entry.params.len(), argc);
        assert_eq!(target_text(entry.target), forward);
    }
}

#[test]
fn oracle_libid_row() {
    let (name, uuid, guid) = LIBID_ROW;
    assert_eq!(name, "MSXML2");
    assert!(uuid_text(guid).eq_ignore_ascii_case(uuid));
    assert_eq!(LIBID_MSXML2, guid);
}

#[test]
fn oracle_coclass_rows() {
    assert_eq!(COCLASSES.len(), COCLASS_ROWS.len());
    for ((name, guid), (row_name, uuid, row_guid)) in COCLASSES.iter().zip(COCLASS_ROWS) {
        assert_eq!(*name, row_name);
        assert_eq!(*guid, row_guid);
        assert!(uuid_text(row_guid).eq_ignore_ascii_case(uuid));
        assert_eq!(coclass_clsid(row_name), Some(row_guid));
    }
}

#[test]
fn oracle_manifest_row() {
    // Row 42: type=win32 name=Microsoft.MSXML2 version=4.1.0.0 arch='' token=6bd6b9abf345378f
    // file=msxml4.dll
    assert_eq!(
        assembly_identity(),
        AssemblyIdentity {
            identity_type: "win32",
            name: "Microsoft.MSXML2",
            version: "4.1.0.0",
            processor_architecture: "",
            public_key_token: "6bd6b9abf345378f",
            file: "msxml4.dll",
        }
    );
}

#[test]
fn oracle_module_and_resource_rows() {
    // Row 43: MODULE=msxml4.dll VER_PRODUCTVERSION=4,20,9818,0 VER_FILEDESCRIPTION_STR="Wine MSXML 4.0"
    assert_eq!(MODULE_NAME, "msxml4.dll");
    assert_eq!(PRODUCT_VERSION, [4, 20, 9818, 0]);
    assert_eq!(FILE_DESCRIPTION, "Wine MSXML 4.0");
    // Row 44: type=24 file=msxml4.manifest
    assert_eq!(RT_MANIFEST, 24);
    assert_eq!(MANIFEST_FILE, "msxml4.manifest");
}
