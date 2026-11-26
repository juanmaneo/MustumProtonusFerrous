/// A `GUID`: `Data1`, `Data2`, `Data3` and the eight bytes of `Data4`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `CLSID_PSFactoryBuffer`: the `uuid(...)` of the `PSFactoryBuffer` coclass in
/// `dlls/ia2comproxy/ia2_classes.idl`. The value is also the IID of `IAccessibleHyperlink`
/// in `include/wine/iaccessible2.idl`.
pub const CLSID_PS_FACTORY_BUFFER: Guid = Guid {
    data1: 0x01c2_0f2b,
    data2: 0x3dd2,
    data3: 0x400f,
    data4: [0x94, 0x9f, 0xad, 0x00, 0xbd, 0xab, 0x1d, 0x41],
};

impl Guid {
    /// The lower-case `8-4-4-4-12` form used in IDL `uuid(...)` attributes.
    pub fn to_idl_string(self) -> String {
        let [b0, b1, b2, b3, b4, b5, b6, b7] = self.data4;
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1, self.data2, self.data3, b0, b1, b2, b3, b4, b5, b6, b7
        )
    }
}
