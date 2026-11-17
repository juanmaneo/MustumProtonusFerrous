//! oleaut32 integer-to-integer conversions from Wine's `dlls/oleaut32/vartype.c`.
//!
//! Each `VarX FromY` conversion between the integer `VT_` types (`I1`, `UI1`,
//! `I2`, `UI2`, `I4`, `UI4`, `I8`, `UI8`) is either a lossless widening
//! (`SIMPLE`) or a range check (`NEGTST`/`POSTST`/`BOTHTST`) that reports
//! `DISP_E_OVERFLOW`. Both reduce to "the value fits the destination type", so
//! every conversion shares one rule. Function names follow `VarX FromY` as
//! `var_x_from_y`. Results are returned instead of written through the C
//! `pcOut`/`pbOut` pointers. Floating point, `DATE`, `CY`, `DECIMAL`, `BSTR`,
//! `DISPATCH` and `BOOL` sources are not ported.

/// `DISP_E_OVERFLOW`, the HRESULT returned when a value does not fit.
pub const DISP_E_OVERFLOW: i32 = 0x8002000A_u32 as i32;

/// `Result` carrying the `HRESULT` Wine returns for failed conversions.
pub type VarResult<T> = Result<T, i32>;

/// Converts `value` to `T` when it fits, or reports overflow.
fn convert<S, T>(value: S) -> VarResult<T>
where
    S: Into<i128>,
    T: TryFrom<i128>,
{
    T::try_from(value.into()).ok().ok_or(DISP_E_OVERFLOW)
}

macro_rules! int_conversions {
    ($($name:ident: $src:ty => $dst:ty;)*) => {
        $(
            pub fn $name(value: $src) -> VarResult<$dst> {
                convert(value)
            }
        )*

        #[cfg(test)]
        #[test]
        fn every_conversion_matches_its_range() {
            $(check_conversion::<$src, $dst>($name);)*
        }
    };
}

int_conversions! {
    var_i1_from_i2: i16 => i8;
    var_i1_from_i4: i32 => i8;
    var_i1_from_i8: i64 => i8;
    var_i1_from_ui1: u8 => i8;
    var_i1_from_ui2: u16 => i8;
    var_i1_from_ui4: u32 => i8;
    var_i1_from_ui8: u64 => i8;
    var_ui1_from_i1: i8 => u8;
    var_ui1_from_i2: i16 => u8;
    var_ui1_from_i4: i32 => u8;
    var_ui1_from_i8: i64 => u8;
    var_ui1_from_ui2: u16 => u8;
    var_ui1_from_ui4: u32 => u8;
    var_ui1_from_ui8: u64 => u8;
    var_i2_from_i1: i8 => i16;
    var_i2_from_i4: i32 => i16;
    var_i2_from_i8: i64 => i16;
    var_i2_from_ui1: u8 => i16;
    var_i2_from_ui2: u16 => i16;
    var_i2_from_ui4: u32 => i16;
    var_i2_from_ui8: u64 => i16;
    var_ui2_from_i1: i8 => u16;
    var_ui2_from_i2: i16 => u16;
    var_ui2_from_i4: i32 => u16;
    var_ui2_from_i8: i64 => u16;
    var_ui2_from_ui1: u8 => u16;
    var_ui2_from_ui4: u32 => u16;
    var_ui2_from_ui8: u64 => u16;
    var_i4_from_i1: i8 => i32;
    var_i4_from_i2: i16 => i32;
    var_i4_from_i8: i64 => i32;
    var_i4_from_ui1: u8 => i32;
    var_i4_from_ui2: u16 => i32;
    var_i4_from_ui4: u32 => i32;
    var_i4_from_ui8: u64 => i32;
    var_ui4_from_i1: i8 => u32;
    var_ui4_from_i2: i16 => u32;
    var_ui4_from_i4: i32 => u32;
    var_ui4_from_i8: i64 => u32;
    var_ui4_from_ui1: u8 => u32;
    var_ui4_from_ui2: u16 => u32;
    var_ui4_from_ui8: u64 => u32;
    var_i8_from_i1: i8 => i64;
    var_i8_from_i2: i16 => i64;
    var_i8_from_ui1: u8 => i64;
    var_i8_from_ui2: u16 => i64;
    var_i8_from_ui4: u32 => i64;
    var_i8_from_ui8: u64 => i64;
    var_ui8_from_i1: i8 => u64;
    var_ui8_from_i2: i16 => u64;
    var_ui8_from_i8: i64 => u64;
    var_ui8_from_ui1: u8 => u64;
    var_ui8_from_ui2: u16 => u64;
    var_ui8_from_ui4: u32 => u64;
}

/// Values covering every integer width's minimum, maximum and their neighbours.
#[cfg(test)]
const PROBES: &[i128] = &[
    -4_294_967_297,
    -4_294_967_296,
    -2_147_483_649,
    -2_147_483_648,
    -70_000,
    -32_769,
    -32_768,
    -129,
    -128,
    -1,
    0,
    1,
    127,
    128,
    255,
    256,
    32_767,
    32_768,
    65_535,
    65_536,
    2_147_483_647,
    2_147_483_648,
    4_294_967_295,
    4_294_967_296,
    9_223_372_036_854_775_807,
    9_223_372_036_854_775_808,
    18_446_744_073_709_551_615,
    18_446_744_073_709_551_616,
];

/// Checks that `convert_fn` accepts exactly the probes `T` can represent,
/// and reports `DISP_E_OVERFLOW` for the rest.
#[cfg(test)]
fn check_conversion<S, T>(convert_fn: fn(S) -> VarResult<T>)
where
    S: TryFrom<i128> + Into<i128> + Copy,
    T: TryFrom<i128> + PartialEq + std::fmt::Debug,
{
    for &probe in PROBES {
        if let Ok(value) = S::try_from(probe) {
            let wide: i128 = value.into();
            assert_eq!(convert_fn(value).ok(), T::try_from(wide).ok());
            // `unwrap_or` keeps lossless pairs free of an unreachable error branch.
            assert_eq!(
                convert_fn(value).err().unwrap_or(DISP_E_OVERFLOW),
                DISP_E_OVERFLOW
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i1_from_i2_bounds() {
        assert_eq!(var_i1_from_i2(-128), Ok(-128));
        assert_eq!(var_i1_from_i2(127), Ok(127));
        assert_eq!(var_i1_from_i2(-129), Err(DISP_E_OVERFLOW));
        assert_eq!(var_i1_from_i2(128), Err(DISP_E_OVERFLOW));
        assert_eq!(var_i1_from_i2(i16::MIN), Err(DISP_E_OVERFLOW));
    }

    #[test]
    fn i1_from_i4_bounds() {
        assert_eq!(var_i1_from_i4(-128), Ok(-128));
        assert_eq!(var_i1_from_i4(127), Ok(127));
        assert_eq!(var_i1_from_i4(128), Err(DISP_E_OVERFLOW));
        assert_eq!(var_i1_from_i4(i32::MIN), Err(DISP_E_OVERFLOW));
    }

    #[test]
    fn i1_from_unsigned_bounds() {
        assert_eq!(var_i1_from_ui1(127), Ok(127));
        assert_eq!(var_i1_from_ui1(128), Err(DISP_E_OVERFLOW));
        assert_eq!(var_i1_from_ui2(127), Ok(127));
        assert_eq!(var_i1_from_ui2(128), Err(DISP_E_OVERFLOW));
        assert_eq!(var_i1_from_ui2(u16::MAX), Err(DISP_E_OVERFLOW));
    }

    #[test]
    fn ui1_from_signed_bounds() {
        assert_eq!(var_ui1_from_i1(0), Ok(0));
        assert_eq!(var_ui1_from_i1(127), Ok(127));
        assert_eq!(var_ui1_from_i1(-1), Err(DISP_E_OVERFLOW));
        assert_eq!(var_ui1_from_i2(255), Ok(255));
        assert_eq!(var_ui1_from_i2(256), Err(DISP_E_OVERFLOW));
        assert_eq!(var_ui1_from_i2(-1), Err(DISP_E_OVERFLOW));
        assert_eq!(var_ui1_from_i4(255), Ok(255));
        assert_eq!(var_ui1_from_i4(256), Err(DISP_E_OVERFLOW));
        assert_eq!(var_ui1_from_i4(i32::MIN), Err(DISP_E_OVERFLOW));
    }

    #[test]
    fn sixty_four_bit_sources_check_both_ends() {
        assert_eq!(var_i8_from_ui8(i64::MAX as u64), Ok(i64::MAX));
        assert_eq!(var_i8_from_ui8(i64::MAX as u64 + 1), Err(DISP_E_OVERFLOW));
        assert_eq!(var_ui8_from_i8(-1), Err(DISP_E_OVERFLOW));
        assert_eq!(var_ui4_from_i8(u32::MAX.into()), Ok(u32::MAX));
        assert_eq!(
            var_ui4_from_ui8(u64::from(u32::MAX) + 1),
            Err(DISP_E_OVERFLOW)
        );
    }

    #[test]
    fn widening_is_lossless() {
        assert_eq!(var_i2_from_i1(-128), Ok(-128));
        assert_eq!(var_ui2_from_ui1(u8::MAX), Ok(255));
        assert_eq!(var_i8_from_ui4(u32::MAX), Ok(i64::from(u32::MAX)));
    }

    #[test]
    fn overflow_hresult_matches_winerror() {
        assert_eq!(DISP_E_OVERFLOW as u32, 0x8002_000A);
    }
}
