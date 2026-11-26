//! Byte layout of `DVCOMPRESSIONINFO` for the pointer width of the target ABI.

/// Target ABI. The pointer width sets the layout of `DVCOMPRESSIONINFO`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abi {
    /// 32-bit: 4-byte pointers.
    Win32,
    /// 64-bit: 8-byte pointers.
    Win64,
}

const DWORD_BYTES: usize = 4;
/// `dwSize` (4 bytes) followed by `guidType` (16 bytes).
const HEADER_BYTES: usize = 20;

impl Abi {
    pub const fn pointer_size(self) -> usize {
        match self {
            Abi::Win32 => 4,
            Abi::Win64 => 8,
        }
    }

    /// Offset of `lpszName`. Pointers are aligned to their own width.
    pub const fn name_offset(self) -> usize {
        align_up(HEADER_BYTES, self.pointer_size())
    }

    pub const fn description_offset(self) -> usize {
        self.name_offset() + self.pointer_size()
    }

    pub const fn flags_offset(self) -> usize {
        self.description_offset() + self.pointer_size()
    }

    pub const fn max_bits_offset(self) -> usize {
        self.flags_offset() + DWORD_BYTES
    }

    /// `sizeof(DVCOMPRESSIONINFO)`. The struct is padded to the pointer alignment.
    pub const fn struct_size(self) -> usize {
        align_up(self.max_bits_offset() + DWORD_BYTES, self.pointer_size())
    }
}

/// Rounds `value` up to a multiple of `align`, which must be a power of two.
const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}
