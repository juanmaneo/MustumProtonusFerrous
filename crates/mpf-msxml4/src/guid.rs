/// A `GUID` laid out like the C struct: `Data1`, `Data2`, `Data3` and `Data4[8]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// The `uuid()` of `library MSXML2` in `dlls/msxml4/msxml4_tlb.idl`.
pub const LIBID_MSXML2: Guid = Guid {
    data1: 0xF5078F18,
    data2: 0xC551,
    data3: 0x11D3,
    data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
};

/// The `coclass` names of `dlls/msxml4/msxml4_tlb.idl` with their CLSIDs, in IDL order.
pub const COCLASSES: [(&str, Guid); 36] = [
    (
        "DOMDocument",
        Guid {
            data1: 0xF6D90F11,
            data2: 0x9C73,
            data3: 0x11D3,
            data4: [0xB3, 0x2E, 0x00, 0xC0, 0x4F, 0x99, 0x0B, 0xB4],
        },
    ),
    (
        "DOMDocument26",
        Guid {
            data1: 0xF5078F1B,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "DOMDocument30",
        Guid {
            data1: 0xF5078F32,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "DOMDocument40",
        Guid {
            data1: 0x88D969C0,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "FreeThreadedDOMDocument",
        Guid {
            data1: 0xF6D90F12,
            data2: 0x9C73,
            data3: 0x11D3,
            data4: [0xB3, 0x2E, 0x00, 0xC0, 0x4F, 0x99, 0x0B, 0xB4],
        },
    ),
    (
        "FreeThreadedDOMDocument26",
        Guid {
            data1: 0xF5078F1C,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "FreeThreadedDOMDocument30",
        Guid {
            data1: 0xF5078F33,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "FreeThreadedDOMDocument40",
        Guid {
            data1: 0x88D969C1,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "XMLHTTP",
        Guid {
            data1: 0xF6D90F16,
            data2: 0x9C73,
            data3: 0x11D3,
            data4: [0xB3, 0x2E, 0x00, 0xC0, 0x4F, 0x99, 0x0B, 0xB4],
        },
    ),
    (
        "XMLHTTP26",
        Guid {
            data1: 0xF5078F1E,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "XMLHTTP30",
        Guid {
            data1: 0xF5078F35,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "XMLHTTP40",
        Guid {
            data1: 0x88D969C5,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "ServerXMLHTTP",
        Guid {
            data1: 0xAFBA6B42,
            data2: 0x5692,
            data3: 0x48EA,
            data4: [0x81, 0x41, 0xDC, 0x51, 0x7D, 0xCF, 0x0E, 0xF1],
        },
    ),
    (
        "ServerXMLHTTP30",
        Guid {
            data1: 0xAFB40FFD,
            data2: 0xB609,
            data3: 0x40A3,
            data4: [0x98, 0x28, 0xF8, 0x8B, 0xBE, 0x11, 0xE4, 0xE3],
        },
    ),
    (
        "ServerXMLHTTP40",
        Guid {
            data1: 0x88D969C6,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "XMLSchemaCache",
        Guid {
            data1: 0x373984C9,
            data2: 0xB845,
            data3: 0x449B,
            data4: [0x91, 0xE7, 0x45, 0xAC, 0x83, 0x03, 0x6A, 0xDE],
        },
    ),
    (
        "XMLSchemaCache26",
        Guid {
            data1: 0xF5078F1D,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "XMLSchemaCache30",
        Guid {
            data1: 0xF5078F34,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "XMLSchemaCache40",
        Guid {
            data1: 0x88D969C2,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "XSLTemplate",
        Guid {
            data1: 0x2933BF94,
            data2: 0x7B36,
            data3: 0x11D2,
            data4: [0xB2, 0x0E, 0x00, 0xC0, 0x4F, 0x98, 0x3E, 0x60],
        },
    ),
    (
        "XSLTemplate26",
        Guid {
            data1: 0xF5078F21,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "XSLTemplate30",
        Guid {
            data1: 0xF5078F36,
            data2: 0xC551,
            data3: 0x11D3,
            data4: [0x89, 0xB9, 0x00, 0x00, 0xF8, 0x1F, 0xE2, 0x21],
        },
    ),
    (
        "XSLTemplate40",
        Guid {
            data1: 0x88D969C3,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "SAXXMLReader",
        Guid {
            data1: 0x079AA557,
            data2: 0x4A18,
            data3: 0x424A,
            data4: [0x8E, 0xEE, 0xE3, 0x9F, 0x0A, 0x8D, 0x41, 0xB9],
        },
    ),
    (
        "SAXXMLReader30",
        Guid {
            data1: 0x3124C396,
            data2: 0xFB13,
            data3: 0x4836,
            data4: [0xA6, 0xAD, 0x13, 0x17, 0xF1, 0x71, 0x36, 0x88],
        },
    ),
    (
        "SAXXMLReader40",
        Guid {
            data1: 0x7C6E29BC,
            data2: 0x8B8B,
            data3: 0x4C3D,
            data4: [0x85, 0x9E, 0xAF, 0x6C, 0xD1, 0x58, 0xBE, 0x0F],
        },
    ),
    (
        "MXHTMLWriter",
        Guid {
            data1: 0xA4C23EC3,
            data2: 0x6B70,
            data3: 0x4466,
            data4: [0x91, 0x27, 0x55, 0x00, 0x77, 0x23, 0x99, 0x78],
        },
    ),
    (
        "MXHTMLWriter30",
        Guid {
            data1: 0x853D1540,
            data2: 0xC1A7,
            data3: 0x4AA9,
            data4: [0xA2, 0x26, 0x4D, 0x3B, 0xD3, 0x01, 0x14, 0x6D],
        },
    ),
    (
        "MXHTMLWriter40",
        Guid {
            data1: 0x88D969C9,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "MXXMLWriter",
        Guid {
            data1: 0xFC220AD8,
            data2: 0xA72A,
            data3: 0x4EE8,
            data4: [0x92, 0x6E, 0x0B, 0x7A, 0xD1, 0x52, 0xA0, 0x20],
        },
    ),
    (
        "MXXMLWriter30",
        Guid {
            data1: 0x3D813DFE,
            data2: 0x6C91,
            data3: 0x4A4E,
            data4: [0x8F, 0x41, 0x04, 0x34, 0x6A, 0x84, 0x1D, 0x9C],
        },
    ),
    (
        "MXXMLWriter40",
        Guid {
            data1: 0x88D969C8,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "MXNamespaceManager",
        Guid {
            data1: 0x88D969D5,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "MXNamespaceManager40",
        Guid {
            data1: 0x88D969D6,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
    (
        "SAXAttributes",
        Guid {
            data1: 0x4DD441AD,
            data2: 0x526D,
            data3: 0x4A77,
            data4: [0x9F, 0x1B, 0x98, 0x41, 0xED, 0x80, 0x2F, 0xB0],
        },
    ),
    (
        "SAXAttributes40",
        Guid {
            data1: 0x88D969CA,
            data2: 0xF192,
            data3: 0x11D4,
            data4: [0xA6, 0x5F, 0x00, 0x40, 0x96, 0x32, 0x51, 0xE5],
        },
    ),
];

/// The CLSID of coclass `name`, or `None` when the library does not define it. Names are
/// case-sensitive.
pub fn coclass_clsid(name: &str) -> Option<Guid> {
    COCLASSES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, guid)| guid)
}
