//! oleaut32 integer narrowing conversions from Wine's `dlls/oleaut32/vartype.c`.
//!
//! Each conversion accepts a source integer and returns the value in the
//! destination type when it fits, or `DISP_E_OVERFLOW` when it does not.
//! Bounds match the ranges asserted in `dlls/oleaut32/tests/vartype.c`
//! (`CONVERTRANGE` / `OVERFLOWRANGE`). Results are returned instead of
//! written through the `pcOut`/`pbOut` pointers the C API uses.

/// `DISP_E_OVERFLOW`, the HRESULT returned when a value does not fit.
pub const DISP_E_OVERFLOW: i32 = 0x8002000A_u32 as i32;

/// `Result` carrying the `HRESULT` Wine returns for failed conversions.
pub type VarResult<T> = Result<T, i32>;

/// Converts `value` into the integer range `min..=max`, or reports overflow.
fn narrow<S, T>(value: S, min: i64, max: i64, cast: fn(i64) -> T) -> VarResult<T>
where
    S: Into<i64>,
{
    let v: i64 = value.into();
    if v < min || v > max {
        return Err(DISP_E_OVERFLOW);
    }
    Ok(cast(v))
}

/// `VarI1FromI2`: `VT_I2` to `VT_I1`.
pub fn var_i1_from_i2(value: i16) -> VarResult<i8> {
    narrow(value, i8::MIN.into(), i8::MAX.into(), |v| v as i8)
}

/// `VarI1FromI4`: `VT_I4` to `VT_I1`.
pub fn var_i1_from_i4(value: i32) -> VarResult<i8> {
    narrow(value, i8::MIN.into(), i8::MAX.into(), |v| v as i8)
}

/// `VarI1FromUI1`: `VT_UI1` to `VT_I1`.
pub fn var_i1_from_ui1(value: u8) -> VarResult<i8> {
    narrow(value, i8::MIN.into(), i8::MAX.into(), |v| v as i8)
}

/// `VarI1FromUI2`: `VT_UI2` to `VT_I1`.
pub fn var_i1_from_ui2(value: u16) -> VarResult<i8> {
    narrow(value, i8::MIN.into(), i8::MAX.into(), |v| v as i8)
}

/// `VarUI1FromI1`: `VT_I1` to `VT_UI1`.
pub fn var_ui1_from_i1(value: i8) -> VarResult<u8> {
    narrow(value, u8::MIN.into(), u8::MAX.into(), |v| v as u8)
}

/// `VarUI1FromI2`: `VT_I2` to `VT_UI1`.
pub fn var_ui1_from_i2(value: i16) -> VarResult<u8> {
    narrow(value, u8::MIN.into(), u8::MAX.into(), |v| v as u8)
}

/// `VarUI1FromI4`: `VT_I4` to `VT_UI1`.
pub fn var_ui1_from_i4(value: i32) -> VarResult<u8> {
    narrow(value, u8::MIN.into(), u8::MAX.into(), |v| v as u8)
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
    fn overflow_hresult_matches_winerror() {
        assert_eq!(DISP_E_OVERFLOW as u32, 0x8002_000A);
    }
}
