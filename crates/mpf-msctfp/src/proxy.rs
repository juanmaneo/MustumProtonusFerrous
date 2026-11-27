/// `GUID` from `guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// The `PSFactoryBuffer` coclass CLSID, `uuid(b5f8fb3b-393f-4f7c-84cb-504924c2705a)` in
/// `dlls/msctfp/msctfp.idl`. `dlldata_EXTRADEFS` passes it to the proxy code as `PROXY_CLSID`.
pub const CLSID_PSFACTORYBUFFER: Guid = Guid {
    data1: 0xb5f8_fb3b,
    data2: 0x393f,
    data3: 0x4f7c,
    data4: [0x84, 0xcb, 0x50, 0x49, 0x24, 0xc2, 0x70, 0x5a],
};

/// The CLSID half of `GetProxyDllInfo` from `DLLDATA_GETPROXYDLLINFO` in `include/rpcproxy.h`.
/// The C function also stores the `ProxyFileInfo` list, which midl generates and the reference
/// does not contain.
pub fn get_proxy_dll_info() -> &'static Guid {
    &CLSID_PSFACTORYBUFFER
}
