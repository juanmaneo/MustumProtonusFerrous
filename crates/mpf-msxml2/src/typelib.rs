//! The `uuid(...)` attributes of `dlls/msxml2/msxml2_tlb.idl`: the LIBID of the `MSXML2`
//! library, the IIDs of its 26 interfaces and dispinterface, and the CLSIDs of its 10
//! coclasses. The constants use the IDL names in upper case, with MIDL's `LIBID_`, `IID_`,
//! `DIID_` and `CLSID_` prefixes. [`TYPELIB_GUIDS`] lists all 37 in IDL order.

use crate::guid::Guid;

pub const LIBID_MSXML2: Guid = Guid {
    data1: 0xf507_8f18,
    data2: 0xc551,
    data3: 0x11d3,
    data4: [0x89, 0xb9, 0x00, 0x00, 0xf8, 0x1f, 0xe2, 0x21],
};

pub const IID_IXMLDOMNODE: Guid = Guid {
    data1: 0x2933_bf80,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMDOCUMENT: Guid = Guid {
    data1: 0x2933_bf81,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMSCHEMACOLLECTION: Guid = Guid {
    data1: 0x3739_84c8,
    data2: 0xb845,
    data3: 0x449b,
    data4: [0x91, 0xe7, 0x45, 0xac, 0x83, 0x03, 0x6a, 0xde],
};

pub const IID_IXMLDOMDOCUMENT2: Guid = Guid {
    data1: 0x2933_bf95,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMNODELIST: Guid = Guid {
    data1: 0x2933_bf82,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMSELECTION: Guid = Guid {
    data1: 0xaa63_4fc7,
    data2: 0x5888,
    data3: 0x44a7,
    data4: [0xa2, 0x57, 0x3a, 0x47, 0x15, 0x0d, 0x3a, 0x0e],
};

pub const IID_IXMLDOMNAMEDNODEMAP: Guid = Guid {
    data1: 0x2933_bf83,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMDOCUMENTFRAGMENT: Guid = Guid {
    data1: 0x3efa_a413,
    data2: 0x272f,
    data3: 0x11d2,
    data4: [0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82],
};

pub const IID_IXMLDOMCHARACTERDATA: Guid = Guid {
    data1: 0x2933_bf84,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMATTRIBUTE: Guid = Guid {
    data1: 0x2933_bf85,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMELEMENT: Guid = Guid {
    data1: 0x2933_bf86,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMTEXT: Guid = Guid {
    data1: 0x2933_bf87,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMCOMMENT: Guid = Guid {
    data1: 0x2933_bf88,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMPROCESSINGINSTRUCTION: Guid = Guid {
    data1: 0x2933_bf89,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMCDATASECTION: Guid = Guid {
    data1: 0x2933_bf8a,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMDOCUMENTTYPE: Guid = Guid {
    data1: 0x2933_bf8b,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMNOTATION: Guid = Guid {
    data1: 0x2933_bf8c,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMENTITY: Guid = Guid {
    data1: 0x2933_bf8d,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMENTITYREFERENCE: Guid = Guid {
    data1: 0x2933_bf8e,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLDOMIMPLEMENTATION: Guid = Guid {
    data1: 0x2933_bf8f,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXTLRUNTIME: Guid = Guid {
    data1: 0x3efa_a425,
    data2: 0x272f,
    data3: 0x11d2,
    data4: [0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82],
};

pub const DIID_XMLDOMDOCUMENTEVENTS: Guid = Guid {
    data1: 0x3efa_a427,
    data2: 0x272f,
    data3: 0x11d2,
    data4: [0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82],
};

pub const IID_IXSLPROCESSOR: Guid = Guid {
    data1: 0x2933_bf92,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXSLTEMPLATE: Guid = Guid {
    data1: 0x2933_bf93,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const IID_IXMLHTTPREQUEST: Guid = Guid {
    data1: 0xed8c_108d,
    data2: 0x4349,
    data3: 0x11d2,
    data4: [0x91, 0xa4, 0x00, 0xc0, 0x4f, 0x79, 0x69, 0xe8],
};

pub const IID_IXMLDOMPARSEERROR: Guid = Guid {
    data1: 0x3efa_a426,
    data2: 0x272f,
    data3: 0x11d2,
    data4: [0x83, 0x6f, 0x00, 0x00, 0xf8, 0x7a, 0x77, 0x82],
};

pub const CLSID_DOMDOCUMENT: Guid = Guid {
    data1: 0xf6d9_0f11,
    data2: 0x9c73,
    data3: 0x11d3,
    data4: [0xb3, 0x2e, 0x00, 0xc0, 0x4f, 0x99, 0x0b, 0xb4],
};

pub const CLSID_DOMDOCUMENT26: Guid = Guid {
    data1: 0xf507_8f1b,
    data2: 0xc551,
    data3: 0x11d3,
    data4: [0x89, 0xb9, 0x00, 0x00, 0xf8, 0x1f, 0xe2, 0x21],
};

pub const CLSID_FREETHREADEDDOMDOCUMENT: Guid = Guid {
    data1: 0xf6d9_0f12,
    data2: 0x9c73,
    data3: 0x11d3,
    data4: [0xb3, 0x2e, 0x00, 0xc0, 0x4f, 0x99, 0x0b, 0xb4],
};

pub const CLSID_FREETHREADEDDOMDOCUMENT26: Guid = Guid {
    data1: 0xf507_8f1c,
    data2: 0xc551,
    data3: 0x11d3,
    data4: [0x89, 0xb9, 0x00, 0x00, 0xf8, 0x1f, 0xe2, 0x21],
};

pub const CLSID_XMLHTTP: Guid = Guid {
    data1: 0xf6d9_0f16,
    data2: 0x9c73,
    data3: 0x11d3,
    data4: [0xb3, 0x2e, 0x00, 0xc0, 0x4f, 0x99, 0x0b, 0xb4],
};

pub const CLSID_XMLHTTP26: Guid = Guid {
    data1: 0xf507_8f1e,
    data2: 0xc551,
    data3: 0x11d3,
    data4: [0x89, 0xb9, 0x00, 0x00, 0xf8, 0x1f, 0xe2, 0x21],
};

pub const CLSID_XMLSCHEMACACHE: Guid = Guid {
    data1: 0x3739_84c9,
    data2: 0xb845,
    data3: 0x449b,
    data4: [0x91, 0xe7, 0x45, 0xac, 0x83, 0x03, 0x6a, 0xde],
};

pub const CLSID_XMLSCHEMACACHE26: Guid = Guid {
    data1: 0xf507_8f1d,
    data2: 0xc551,
    data3: 0x11d3,
    data4: [0x89, 0xb9, 0x00, 0x00, 0xf8, 0x1f, 0xe2, 0x21],
};

pub const CLSID_XSLTEMPLATE: Guid = Guid {
    data1: 0x2933_bf94,
    data2: 0x7b36,
    data3: 0x11d2,
    data4: [0xb2, 0x0e, 0x00, 0xc0, 0x4f, 0x98, 0x3e, 0x60],
};

pub const CLSID_XSLTEMPLATE26: Guid = Guid {
    data1: 0xf507_8f21,
    data2: 0xc551,
    data3: 0x11d3,
    data4: [0x89, 0xb9, 0x00, 0x00, 0xf8, 0x1f, 0xe2, 0x21],
};

/// Every `uuid(...)` of `msxml2_tlb.idl` in IDL order, keyed by its IDL name.
pub const TYPELIB_GUIDS: [(&str, Guid); 37] = [
    ("MSXML2", LIBID_MSXML2),
    ("IXMLDOMNode", IID_IXMLDOMNODE),
    ("IXMLDOMDocument", IID_IXMLDOMDOCUMENT),
    ("IXMLDOMSchemaCollection", IID_IXMLDOMSCHEMACOLLECTION),
    ("IXMLDOMDocument2", IID_IXMLDOMDOCUMENT2),
    ("IXMLDOMNodeList", IID_IXMLDOMNODELIST),
    ("IXMLDOMSelection", IID_IXMLDOMSELECTION),
    ("IXMLDOMNamedNodeMap", IID_IXMLDOMNAMEDNODEMAP),
    ("IXMLDOMDocumentFragment", IID_IXMLDOMDOCUMENTFRAGMENT),
    ("IXMLDOMCharacterData", IID_IXMLDOMCHARACTERDATA),
    ("IXMLDOMAttribute", IID_IXMLDOMATTRIBUTE),
    ("IXMLDOMElement", IID_IXMLDOMELEMENT),
    ("IXMLDOMText", IID_IXMLDOMTEXT),
    ("IXMLDOMComment", IID_IXMLDOMCOMMENT),
    (
        "IXMLDOMProcessingInstruction",
        IID_IXMLDOMPROCESSINGINSTRUCTION,
    ),
    ("IXMLDOMCDATASection", IID_IXMLDOMCDATASECTION),
    ("IXMLDOMDocumentType", IID_IXMLDOMDOCUMENTTYPE),
    ("IXMLDOMNotation", IID_IXMLDOMNOTATION),
    ("IXMLDOMEntity", IID_IXMLDOMENTITY),
    ("IXMLDOMEntityReference", IID_IXMLDOMENTITYREFERENCE),
    ("IXMLDOMImplementation", IID_IXMLDOMIMPLEMENTATION),
    ("IXTLRuntime", IID_IXTLRUNTIME),
    ("XMLDOMDocumentEvents", DIID_XMLDOMDOCUMENTEVENTS),
    ("IXSLProcessor", IID_IXSLPROCESSOR),
    ("IXSLTemplate", IID_IXSLTEMPLATE),
    ("IXMLHTTPRequest", IID_IXMLHTTPREQUEST),
    ("IXMLDOMParseError", IID_IXMLDOMPARSEERROR),
    ("DOMDocument", CLSID_DOMDOCUMENT),
    ("DOMDocument26", CLSID_DOMDOCUMENT26),
    ("FreeThreadedDOMDocument", CLSID_FREETHREADEDDOMDOCUMENT),
    ("FreeThreadedDOMDocument26", CLSID_FREETHREADEDDOMDOCUMENT26),
    ("XMLHTTP", CLSID_XMLHTTP),
    ("XMLHTTP26", CLSID_XMLHTTP26),
    ("XMLSchemaCache", CLSID_XMLSCHEMACACHE),
    ("XMLSchemaCache26", CLSID_XMLSCHEMACACHE26),
    ("XSLTemplate", CLSID_XSLTEMPLATE),
    ("XSLTemplate26", CLSID_XSLTEMPLATE26),
];

/// Returns the GUID that `msxml2_tlb.idl` gives the library, interface, dispinterface or
/// coclass called `name`, or `None` when the IDL declares no such name. Names are
/// case-sensitive.
pub fn typelib_guid(name: &str) -> Option<Guid> {
    TYPELIB_GUIDS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, guid)| guid)
}
