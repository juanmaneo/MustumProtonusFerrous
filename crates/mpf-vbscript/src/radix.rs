//! `Global_Hex` and `Global_Oct` in `dlls/vbscript/global.c`. The caller passes the VARIANT value
//! as a `u32`. An `I2` is zero-extended with `as u16`, and an `I4` is reinterpreted as `u32`.

/// `Hex`: uppercase hexadecimal with no leading zeros, and `"0"` for zero.
pub fn hex(n: u32) -> String {
    format!("{n:X}")
}

/// `Oct`: octal with no leading zeros, and `"0"` for zero.
pub fn oct(n: u32) -> String {
    format!("{n:o}")
}
