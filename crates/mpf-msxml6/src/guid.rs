/// A Windows GUID, with the field layout of include/guiddef.h.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    /// The lowercase 8-4-4-4-12 text that a uuid(...) attribute takes in an IDL file.
    pub fn to_uuid_string(&self) -> String {
        let tail: String = self.data4[2..]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{tail}",
            self.data1, self.data2, self.data3, self.data4[0], self.data4[1]
        )
    }
}
