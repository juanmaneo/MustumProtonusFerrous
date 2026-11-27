/// `MODULE` in `dlls/qmgrprxy/Makefile.in`.
pub const MODULE: &str = "qmgrprxy.dll";

/// `IMPORTS` in `dlls/qmgrprxy/Makefile.in`.
pub const IMPORTS: &[&str] = &["rpcrt4"];

/// A Windows `GUID`: `Data1`, `Data2`, `Data3` and the eight bytes of `Data4`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// The `uuid` of the `PSFactoryBuffer` coclass in `dlls/qmgrprxy/qmgrprxy.idl`. The value is the
/// IID of `IBackgroundCopyManager` in `include/bits.idl`. Nothing in this build defines
/// `PROXY_CLSID` or `PROXY_CLSID_IS`, so the generated `GetProxyDllInfo` reports the IID of the
/// first stub vtable in the proxy file list instead (`GET_DLL_CLSID`).
pub const PS_FACTORY_BUFFER_CLSID: Guid = Guid {
    data1: 0x5ce3_4c0d,
    data2: 0x0dc9,
    data3: 0x4c1f,
    data4: [0x89, 0x7c, 0xda, 0xa1, 0xb7, 0x8c, 0xee, 0x7c],
};

/// `guid` in the lowercase `8-4-4-4-12` text form used by the IDL `uuid` attribute.
pub fn guid_string(guid: &Guid) -> String {
    let d = guid.data4;
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}
