//! Pure, deterministic helpers corresponding to `dlls/vccorlib140/vccorlib.c`.
//!
//! The Windows-facing wrappers are intentionally omitted. This crate models their
//! value-level behavior: type descriptor identity, string formatting, and GUID copying.
//!
//! Wine calls `vswprintf` with the C `%g` conversion. Float formatting here follows
//! the C default precision and exponent thresholds for the tested C-locale values.
//! `char16_to_string` returns UTF-16 code units rather than a Rust `String`, so even
//! NUL and unpaired surrogate values are preserved exactly.
//!
//! `GetIidsFn`'s allocation failure and HRESULT behavior are not modeled: the slice
//! helper represents only its successful count-and-copy path. COM, heap ownership,
//! activation, SEH, and thread-dependent exports are omitted.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A GUID matching Wine's `GUID` layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// The descriptor identity referenced by a `Platform.Type` instance.
///
/// `identity` is payload, not the value used for equality; Wine compares descriptor
/// pointers, so two distinct descriptors with equal payloads remain distinct.
#[derive(Debug)]
pub struct TypeDescriptor {
    pub identity: u8,
}

/// A safe value-level model of Wine's `struct platform_type`.
#[derive(Debug)]
pub struct PlatformType<'a> {
    pub descriptor: Option<&'a TypeDescriptor>,
}

/// Type codes declared by Wine's `enum typecode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TypeCode {
    Boolean = 3,
    Char16 = 4,
    UInt8 = 6,
    Int16 = 7,
    UInt16 = 8,
    Int32 = 9,
    UInt32 = 10,
    Int64 = 11,
    UInt64 = 12,
    Single = 13,
    Double = 14,
    DateTime = 16,
    String = 18,
    Point = 20,
    Size = 21,
    Rect = 22,
    Guid = 23,
}

/// `platform_type_Equals_Object`: compares instance pointers, then descriptor pointers.
pub fn platform_type_equals_object(
    this: Option<&PlatformType<'_>>,
    object: Option<&PlatformType<'_>>,
) -> bool {
    match (this, object) {
        (None, None) => true,
        (Some(this), Some(object)) => {
            std::ptr::eq(this, object)
                || match (this.descriptor, object.descriptor) {
                    (Some(this_desc), Some(object_desc)) => std::ptr::eq(this_desc, object_desc),
                    (None, None) => true,
                    _ => false,
                }
        }
        _ => false,
    }
}

/// `Guid_ToString`: formats a GUID as `{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}`.
pub fn guid_to_string(guid: &Guid) -> String {
    format!(
        "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
        guid.data1,
        guid.data2,
        guid.data3,
        guid.data4[0],
        guid.data4[1],
        guid.data4[2],
        guid.data4[3],
        guid.data4[4],
        guid.data4[5],
        guid.data4[6],
        guid.data4[7],
    )
}

/// `Boolean_ToString`: any nonzero Windows `boolean` value is true.
pub fn boolean_to_string(value: u8) -> String {
    if value != 0 {
        String::from("true")
    } else {
        String::from("false")
    }
}

/// `char16_ToString`: preserve the single UTF-16 code unit emitted by `%c`.
pub fn char16_to_string(value: u16) -> Vec<u16> {
    vec![value]
}

/// `float32_ToString`: C `%g` formatting with its default precision of six digits.
pub fn float32_to_string(value: f32) -> String {
    format_float_g(f64::from(value))
}

/// `float64_ToString`: C `%g` formatting with its default precision of six digits.
pub fn float64_to_string(value: f64) -> String {
    format_float_g(value)
}

/// Decimal formatting corresponding to Wine's integer `*_ToString` functions.
pub fn int8_to_string(value: i8) -> String {
    value.to_string()
}

pub fn int16_to_string(value: i16) -> String {
    value.to_string()
}

pub fn int32_to_string(value: i32) -> String {
    value.to_string()
}

pub fn int64_to_string(value: i64) -> String {
    value.to_string()
}

pub fn uint8_to_string(value: u8) -> String {
    value.to_string()
}

pub fn uint16_to_string(value: u16) -> String {
    value.to_string()
}

pub fn uint32_to_string(value: u32) -> String {
    value.to_string()
}

pub fn uint64_to_string(value: u64) -> String {
    value.to_string()
}

/// Successful count-and-copy behavior of Wine's `GetIidsFn`.
pub fn get_iids_fn(src: &[Guid]) -> Vec<Guid> {
    src.to_vec()
}

/// The name returned by Wine's `debugstr_typecode`; unlisted values render as decimal.
pub fn typecode_name(code: i32) -> String {
    match code {
        3 => String::from("Boolean"),
        4 => String::from("char16"),
        6 => String::from("uint8"),
        7 => String::from("int16"),
        8 => String::from("uint16"),
        9 => String::from("int32"),
        10 => String::from("uint32"),
        11 => String::from("int64"),
        12 => String::from("uint64"),
        13 => String::from("float32"),
        14 => String::from("double"),
        16 => String::from("DateTime"),
        18 => String::from("String"),
        20 => String::from("Point"),
        21 => String::from("Size"),
        22 => String::from("Rect"),
        23 => String::from("Guid"),
        _ => code.to_string(),
    }
}

fn format_float_g(value: f64) -> String {
    if value.is_nan() {
        return String::from("nan");
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            String::from("-inf")
        } else {
            String::from("inf")
        };
    }
    if value == 0.0 {
        return if value.is_sign_negative() {
            String::from("-0")
        } else {
            String::from("0")
        };
    }

    let scientific = format!("{value:.5e}");
    let (mantissa, exponent) = scientific
        .split_once('e')
        .expect("Rust scientific formatting includes an exponent");
    let exponent: i32 = exponent
        .parse()
        .expect("Rust scientific formatting emits a decimal exponent");

    if !(-4..6).contains(&exponent) {
        let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        let sign = if exponent < 0 { '-' } else { '+' };
        let digits = exponent.unsigned_abs();
        let exponent_digits = if digits < 10 {
            format!("0{digits}")
        } else {
            digits.to_string()
        };
        format!("{mantissa}e{sign}{exponent_digits}")
    } else {
        let precision = (5 - exponent) as usize;
        let fixed = format!("{value:.precision$}");
        if fixed.contains('.') {
            fixed
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            fixed
        }
    }
}
