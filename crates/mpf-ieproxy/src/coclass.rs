/// A GUID with the `Data1` to `Data4` layout of `include/guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    /// The braced, lowercase form that the IDL `uuid(...)` attribute uses.
    pub fn to_braced_string(&self) -> String {
        let d = self.data4;
        format!(
            "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
            self.data1, self.data2, self.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
        )
    }
}

/// The `uuid(...)` of `coclass PSFactoryBuffer` in both wrapper IDL files. The same value is
/// `CLSID_PSFactoryBuffer`, which `dlls/ieproxy/Makefile.in` passes as `PROXY_CLSID`.
pub const PS_FACTORY_BUFFER_CLSID: Guid = Guid {
    data1: 0xa4a1_a128,
    data2: 0x768f,
    data3: 0x41e0,
    data4: [0xbf, 0x75, 0xe4, 0xfd, 0xdd, 0x70, 0x1c, 0xba],
};

/// A coclass from the wrapper IDL files.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coclass {
    pub name: &'static str,
    pub clsid: Guid,
    /// The `threading(...)` attribute.
    pub threading: &'static str,
    pub interfaces: &'static [&'static str],
}

/// `coclass PSFactoryBuffer { interface IPSFactoryBuffer; }` with `threading(both)`.
pub const PS_FACTORY_BUFFER: Coclass = Coclass {
    name: "PSFactoryBuffer",
    clsid: PS_FACTORY_BUFFER_CLSID,
    threading: "both",
    interfaces: &["IPSFactoryBuffer"],
};
