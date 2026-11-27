//! Oracle-derived test vectors for the ported msxml2 logic.
//!
//! The vectors come from three C harnesses under `target/canon/msxml2/`. Each one was compiled
//! with `gcc -fsanitize=address,undefined -g` and run, and each left empty stderr, so ASan and
//! UBSan flagged no row and no row is excluded.
//!
//! - `guid_harness.c` reads the 37 `uuid(...)` strings of `msxml2_tlb.idl` (listed in
//!   `uuids.txt`) and prints each one as a braced, lowercase GUID.
//! - `enum_harness.c` evaluates the 13 `DOMNodeType` enumerators, copied from IDL lines 64 to
//!   79.
//! - `export_harness.c` runs the four `msxml2.spec` exports through the `check_undefined_exports`
//!   logic of `tools/winebuild/import.c`, copied verbatim with the flag values of
//!   `tools/winebuild/build.h`, once without and once with `--external-symbols`. It prints the
//!   flags and the error text of each export.
//!
//! That is 37 + 13 + 8 = 58 rows. The brief suggests 15 to 25 rows, but every IDL constant and
//! every spec export gets a row here, so the count is above that range.

use super::*;

const FLAG_FORWARD: u32 = 0x1000;
const FLAG_EXT_LINK: u32 = 0x2000;

/// `(name, braced GUID)` rows printed by `guid_harness`, in IDL order.
const ORACLE_GUID_ROWS: &[(&str, &str)] = &[
    ("MSXML2", "{f5078f18-c551-11d3-89b9-0000f81fe221}"),
    ("IXMLDOMNode", "{2933bf80-7b36-11d2-b20e-00c04f983e60}"),
    ("IXMLDOMDocument", "{2933bf81-7b36-11d2-b20e-00c04f983e60}"),
    (
        "IXMLDOMSchemaCollection",
        "{373984c8-b845-449b-91e7-45ac83036ade}",
    ),
    ("IXMLDOMDocument2", "{2933bf95-7b36-11d2-b20e-00c04f983e60}"),
    ("IXMLDOMNodeList", "{2933bf82-7b36-11d2-b20e-00c04f983e60}"),
    ("IXMLDOMSelection", "{aa634fc7-5888-44a7-a257-3a47150d3a0e}"),
    (
        "IXMLDOMNamedNodeMap",
        "{2933bf83-7b36-11d2-b20e-00c04f983e60}",
    ),
    (
        "IXMLDOMDocumentFragment",
        "{3efaa413-272f-11d2-836f-0000f87a7782}",
    ),
    (
        "IXMLDOMCharacterData",
        "{2933bf84-7b36-11d2-b20e-00c04f983e60}",
    ),
    ("IXMLDOMAttribute", "{2933bf85-7b36-11d2-b20e-00c04f983e60}"),
    ("IXMLDOMElement", "{2933bf86-7b36-11d2-b20e-00c04f983e60}"),
    ("IXMLDOMText", "{2933bf87-7b36-11d2-b20e-00c04f983e60}"),
    ("IXMLDOMComment", "{2933bf88-7b36-11d2-b20e-00c04f983e60}"),
    (
        "IXMLDOMProcessingInstruction",
        "{2933bf89-7b36-11d2-b20e-00c04f983e60}",
    ),
    (
        "IXMLDOMCDATASection",
        "{2933bf8a-7b36-11d2-b20e-00c04f983e60}",
    ),
    (
        "IXMLDOMDocumentType",
        "{2933bf8b-7b36-11d2-b20e-00c04f983e60}",
    ),
    ("IXMLDOMNotation", "{2933bf8c-7b36-11d2-b20e-00c04f983e60}"),
    ("IXMLDOMEntity", "{2933bf8d-7b36-11d2-b20e-00c04f983e60}"),
    (
        "IXMLDOMEntityReference",
        "{2933bf8e-7b36-11d2-b20e-00c04f983e60}",
    ),
    (
        "IXMLDOMImplementation",
        "{2933bf8f-7b36-11d2-b20e-00c04f983e60}",
    ),
    ("IXTLRuntime", "{3efaa425-272f-11d2-836f-0000f87a7782}"),
    (
        "XMLDOMDocumentEvents",
        "{3efaa427-272f-11d2-836f-0000f87a7782}",
    ),
    ("IXSLProcessor", "{2933bf92-7b36-11d2-b20e-00c04f983e60}"),
    ("IXSLTemplate", "{2933bf93-7b36-11d2-b20e-00c04f983e60}"),
    ("IXMLHTTPRequest", "{ed8c108d-4349-11d2-91a4-00c04f7969e8}"),
    (
        "IXMLDOMParseError",
        "{3efaa426-272f-11d2-836f-0000f87a7782}",
    ),
    ("DOMDocument", "{f6d90f11-9c73-11d3-b32e-00c04f990bb4}"),
    ("DOMDocument26", "{f5078f1b-c551-11d3-89b9-0000f81fe221}"),
    (
        "FreeThreadedDOMDocument",
        "{f6d90f12-9c73-11d3-b32e-00c04f990bb4}",
    ),
    (
        "FreeThreadedDOMDocument26",
        "{f5078f1c-c551-11d3-89b9-0000f81fe221}",
    ),
    ("XMLHTTP", "{f6d90f16-9c73-11d3-b32e-00c04f990bb4}"),
    ("XMLHTTP26", "{f5078f1e-c551-11d3-89b9-0000f81fe221}"),
    ("XMLSchemaCache", "{373984c9-b845-449b-91e7-45ac83036ade}"),
    ("XMLSchemaCache26", "{f5078f1d-c551-11d3-89b9-0000f81fe221}"),
    ("XSLTemplate", "{2933bf94-7b36-11d2-b20e-00c04f983e60}"),
    ("XSLTemplate26", "{f5078f21-c551-11d3-89b9-0000f81fe221}"),
];

/// `(name, value)` rows printed by `enum_harness`, in IDL order.
const ORACLE_ENUM_ROWS: &[(&str, i32)] = &[
    ("NODE_INVALID", 0),
    ("NODE_ELEMENT", 1),
    ("NODE_ATTRIBUTE", 2),
    ("NODE_TEXT", 3),
    ("NODE_CDATA_SECTION", 4),
    ("NODE_ENTITY_REFERENCE", 5),
    ("NODE_ENTITY", 6),
    ("NODE_PROCESSING_INSTRUCTION", 7),
    ("NODE_COMMENT", 8),
    ("NODE_DOCUMENT", 9),
    ("NODE_DOCUMENT_TYPE", 10),
    ("NODE_DOCUMENT_FRAGMENT", 11),
    ("NODE_NOTATION", 12),
];

/// `(external, name, flags, error)` rows printed by `export_harness`. `external` is set for
/// the run with `--external-symbols`.
const ORACLE_EXPORT_ROWS: &[(bool, &str, u32, Option<&str>)] = &[
    (
        false,
        "DllCanUnloadNow",
        0x0000,
        Some("msxml2.spec:1: function 'DllCanUnloadNow' not defined"),
    ),
    (false, "DllGetClassObject", 0x1000, None),
    (
        false,
        "DllRegisterServer",
        0x0000,
        Some("msxml2.spec:3: function 'DllRegisterServer' not defined"),
    ),
    (
        false,
        "DllUnregisterServer",
        0x0000,
        Some("msxml2.spec:4: function 'DllUnregisterServer' not defined"),
    ),
    (true, "DllCanUnloadNow", 0x2000, None),
    (true, "DllGetClassObject", 0x1000, None),
    (true, "DllRegisterServer", 0x2000, None),
    (true, "DllUnregisterServer", 0x2000, None),
];

#[test]
fn oracle_guid_rows() {
    for &(name, braced) in ORACLE_GUID_ROWS {
        assert_eq!(
            typelib_guid(name).map(|guid| guid.to_braced_string()),
            Some(braced.to_string()),
            "{name}"
        );
    }
}

#[test]
fn oracle_enum_rows() {
    for &(name, value) in ORACLE_ENUM_ROWS {
        assert_eq!(member_value(DOM_NODE_TYPE, name), Some(value), "{name}");
    }
}

#[test]
fn oracle_export_rows() {
    for &(external, name, flags, error) in ORACLE_EXPORT_ROWS {
        let kind = export_kind(name).expect("every oracle export is in EXPORTS");
        match kind {
            ExportKind::Forward(_) => {
                assert_eq!(flags, FLAG_FORWARD, "{name}");
                assert_eq!(error, None, "{name}");
            }
            ExportKind::Undefined => {
                assert_eq!(flags, if external { FLAG_EXT_LINK } else { 0 }, "{name}");
                assert_eq!(error.is_some(), !external, "{name}");
                if let Some(message) = error {
                    assert!(message.starts_with("msxml2.spec:"), "{name}");
                    assert!(
                        message.ends_with(&format!("function '{name}' not defined")),
                        "{name}"
                    );
                }
            }
        }
    }
}
