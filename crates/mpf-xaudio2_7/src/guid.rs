/// A COM GUID with the Windows field layout.
#[derive(PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// Port of `IsEqualGUID`: all sixteen bytes must match.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}
