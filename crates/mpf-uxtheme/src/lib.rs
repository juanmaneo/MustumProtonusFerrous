//! Port of the deterministic parsing logic in Wine's `dlls/uxtheme`: the theme
//! INI reader in `uxini.c`, and the section name and property value parsers in
//! `msstyles.c`.
//!
//! Ported:
//! - [`ini`]: `UXINI_GetNextSection` and `UXINI_GetNextValue`, as
//!   [`ini::IniReader::next_section`] and [`ini::IniReader::next_value`].
//! - [`section`]: `MSSTYLES_ParseIniSectionName` as
//!   [`section::split_section_name`], and `parse_app_class_name` as
//!   [`section::split_app_class_name`].
//! - [`value`]: `MSSTYLES_GetPropertyBool`, `Color`, `Int`, `IntList`,
//!   `Position`, `String`, `Rect` and `Margins`, and `MSSTYLES_GetFont`.
//!
//! Skipped: the colour and non-client state handlers, which call
//! `SetSysColors` and `SystemParametersInfoW`; the stylemap tables and
//! `MSSTYLES_LookupPartState`, which the caller supplies; `FindSection` and
//! `FindValue`, which compare names with `CompareStringW`; and the DPI scaling
//! of the font size, which needs `MulDiv` and a DPI value.
//!
//! Deviations from Wine:
//! - Slices stand in for NUL-terminated buffers, and a unit past the end of a
//!   slice reads as 0. Wine also reads the unit after a value, which is only
//!   observable in [`value::parse_font`]; that function takes it as `after`.
//! - A font attribute name compares ASCII-case-insensitively. Wine's
//!   `lstrcmpiW` folds case by locale, which differs only for non-ASCII units
//!   that fold to ASCII.
//! - A section name whose colon is the last unit makes Wine read past the
//!   terminator, which is undefined. Here it yields `None`.
//! - A `name = value` line that starts with `=` makes Wine index the name
//!   before its start, which faults. Here the name is empty and the value is
//!   kept.
//! - Signed overflow in number parsing wraps, where Wine's is undefined.
//! - A `[` line with no closing bracket gives Wine a section length of
//!   `0xFFFFFFFF`. Here the section is empty, which `split_section_name` rejects
//!   just as Wine does.
//! - The font point size is returned unscaled. Callers compute
//!   `-MulDiv(point_size, dpi, 72)`.
//! - A negative `max_chars` in [`value::parse_string`] is read as unsigned, so
//!   it does not limit the copy, as Wine's `min` does.
//!
//! Quirks kept on purpose, so that the output matches Wine:
//! - A comma-less token followed by whitespace loses its last unit. For example
//!   the font face or attribute `bold` before a `\n` reads as `bol`, which
//!   makes [`value::parse_font`] keep the default weight.
//! - A name separator skips the unit after the colon. `app:Class` yields the
//!   class `lass`.
//! - A line with no `=` ends the value run in
//!   [`ini::IniReader::next_value`], and the line is consumed.

pub mod ini;
mod number;
pub mod section;
mod string;
pub mod value;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// An `HRESULT`, as returned by the `MSSTYLES_GetProperty*` functions.
pub type Hresult = i32;

/// `E_PROP_ID_UNSUPPORTED`: the property value could not be parsed.
pub const E_PROP_ID_UNSUPPORTED: Hresult = 0x8007_0490u32 as i32;

/// The unit at `index`, or 0 past the end, as a NUL-terminated buffer reads.
pub(crate) fn unit_at(text: &[u16], index: usize) -> u16 {
    text.get(index).copied().unwrap_or(0)
}

/// Whether `unit` is whitespace: the C locale `isspace` set, plus `\r`.
pub(crate) fn is_space(unit: u16) -> bool {
    matches!(unit, 0x09..=0x0d | 0x20)
}
