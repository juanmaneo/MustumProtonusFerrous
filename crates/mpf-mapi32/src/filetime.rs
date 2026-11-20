//! FILETIME arithmetic from `dlls/mapi32/util.c`. The C code reads a `FILETIME` as one signed
//! 64-bit value, with the low half first, as on little-endian targets.

/// A `FILETIME`: the low and high 32-bit halves of a 64-bit count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTime {
    pub low: u32,
    pub high: u32,
}

impl FileTime {
    /// The `FILETIME` with the bit pattern of `value`.
    pub const fn from_i64(value: i64) -> Self {
        let bits = value as u64;
        Self {
            low: bits as u32,
            high: (bits >> 32) as u32,
        }
    }

    /// The signed 64-bit value of the `FILETIME`, as the C `LONGLONG` cast reads it.
    pub const fn to_i64(self) -> i64 {
        (((self.high as u64) << 32) | self.low as u64) as i64
    }
}

/// `MAPI32_FtAddFt`. The sum of the two values. The C code overflows on a large sum, which is
/// undefined behaviour. The port wraps.
pub const fn ft_add_ft(left: FileTime, right: FileTime) -> i64 {
    left.to_i64().wrapping_add(right.to_i64())
}

/// `MAPI32_FtSubFt`. The C code returns `right - left`, although its comment says `left - right`.
/// The port keeps the code's result.
pub const fn ft_sub_ft(left: FileTime, right: FileTime) -> i64 {
    right.to_i64().wrapping_sub(left.to_i64())
}

/// `MAPI32_FtMulDw`. The product of a `DWORD` and a `FILETIME`.
pub const fn ft_mul_dw(dw_left: u32, ft_right: FileTime) -> i64 {
    (dw_left as i64).wrapping_mul(ft_right.to_i64())
}

/// `MAPI32_FtMulDwDw`. The product of two `DWORD`s, as a signed 64-bit value.
pub const fn ft_mul_dw_dw(dw_left: u32, dw_right: u32) -> i64 {
    (dw_left as i64).wrapping_mul(dw_right as i64)
}

/// `MAPI32_FtNegFt`. The negation of a `FILETIME`. Negating `i64::MIN` is undefined behaviour in
/// the C code. The port wraps.
pub const fn ft_neg_ft(ft: FileTime) -> i64 {
    ft.to_i64().wrapping_neg()
}
