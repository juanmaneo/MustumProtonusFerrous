/// One `ForceRemove` class key of `dlls/msxml/msxml.rgs`, with the values the script sets under it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComClass {
    /// The ProgID key, for example `MSXML.DOMDocument`.
    pub prog_id: &'static str,
    /// The default value of the ProgID key.
    pub description: &'static str,
    /// The `CLSID` value.
    pub clsid: &'static str,
    /// The `CurVer` value.
    pub cur_ver: &'static str,
}

/// The class keys of `msxml.rgs`, in script order.
pub const COM_CLASSES: [ComClass; 2] = [
    ComClass {
        prog_id: "MSXML.DOMDocument",
        description: "XML DOM Document",
        clsid: "{2933BF90-7B36-11D2-B20E-00C04F983E60}",
        cur_ver: "Microsoft.XMLDOM.1.0",
    },
    ComClass {
        prog_id: "MSXML.FreeThreadedDOMDocument",
        description: "Free Threaded XML DOM Document",
        clsid: "{2933BF91-7B36-11D2-B20E-00C04F983E60}",
        cur_ver: "Microsoft.FreeThreadedXMLDOM.1.0",
    },
];

/// The class registered under `prog_id`, or `None` when the script does not register it. The
/// match is exact; the Windows registry compares key names case-insensitively, which is not modelled.
pub fn com_class(prog_id: &str) -> Option<ComClass> {
    COM_CLASSES
        .iter()
        .find(|class| class.prog_id == prog_id)
        .copied()
}
