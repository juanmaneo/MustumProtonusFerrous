use crate::guid::Guid;

/// The threading(...) attribute of a coclass in include/msxml6.idl.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Threading {
    Both,
    Apartment,
}

/// A coclass record from include/msxml6.idl.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoClass {
    pub name: &'static str,
    pub progid: &'static str,
    pub threading: Threading,
    pub clsid: Guid,
}

/// The library record from include/msxml6.idl: its name, its LIBID and its version(major.minor).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeLib {
    pub name: &'static str,
    pub libid: Guid,
    pub version: (u16, u16),
}

/// The library MSXML2 from include/msxml6.idl.
pub const TYPELIB: TypeLib = TypeLib {
    name: "MSXML2",
    libid: Guid {
        data1: 0xf507_8f18,
        data2: 0xc551,
        data3: 0x11d3,
        data4: [0x89, 0xb9, 0x00, 0x00, 0xf8, 0x1f, 0xe2, 0x21],
    },
    version: (6, 0),
};

/// The coclasses in include/msxml6.idl, in file order.
pub const COCLASSES: [CoClass; 12] = [
    CoClass {
        name: "FreeThreadedXMLHTTP60",
        progid: "Msxml2.FreeThreadedXMLHTTP60.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a09,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "DOMDocument60",
        progid: "Msxml2.DOMDocument.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a05,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "FreeThreadedDOMDocument60",
        progid: "Msxml2.FreeThreadedDOMDocument.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a06,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "SAXXMLReader60",
        progid: "Msxml2.SAXXMLReader.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a0c,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "XMLSchemaCache60",
        progid: "Msxml2.XMLSchemaCache.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a07,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "MXHTMLWriter60",
        progid: "Msxml2.MXHTMLWriter.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a10,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "MXXMLWriter60",
        progid: "Msxml2.MXXMLWriter.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a0f,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "MXNamespaceManager60",
        progid: "Msxml2.MXNamespaceManager.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a11,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "SAXAttributes60",
        progid: "Msxml2.SAXAttributes.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a0e,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "XSLTemplate60",
        progid: "Msxml2.XSLTemplate.6.0",
        threading: Threading::Both,
        clsid: Guid {
            data1: 0x88d9_6a08,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "XMLHTTP60",
        progid: "Msxml2.XMLHTTP.6.0",
        threading: Threading::Apartment,
        clsid: Guid {
            data1: 0x88d9_6a0a,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
    CoClass {
        name: "ServerXMLHTTP60",
        progid: "Msxml2.ServerXMLHTTP.6.0",
        threading: Threading::Apartment,
        clsid: Guid {
            data1: 0x88d9_6a0b,
            data2: 0xf192,
            data3: 0x11d4,
            data4: [0xa6, 0x5f, 0x00, 0x40, 0x96, 0x32, 0x51, 0xe5],
        },
    },
];

/// The coclass named name, or None. Names are case-sensitive.
pub fn coclass(name: &str) -> Option<CoClass> {
    COCLASSES.iter().find(|entry| entry.name == name).copied()
}
