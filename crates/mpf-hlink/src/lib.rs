//! hlink subset for MPF: the deterministic logic of Wine's `dlls/hlink`.
//!
//! Strings are `&[u16]`. As in the C code, a string ends at its first NUL unit, and
//! every entry point reads it that way. Save headers and string records are byte
//! slices in the little-endian layout that `IPersistStream` writes to an `IStream`.
//!
//! Ported (table tests against a verbatim Wine C oracle, see `oracle_checks.rs`):
//! - `split_target`: the `#` split of `HlinkCreateFromString`.
//! - `strip_file_scheme`, `is_shortcut`, `is_absolute_display_name` and
//!   `fallback_is_url`: the display-name checks of `HlinkParseDisplayName` and
//!   `HlinkIsShortcut`, and the URL-or-file fallback of `SetStringReference`.
//! - `save_flags`, `unknown_flags`, `parse_header`, `encode_string`, `decode_string`
//!   and `size_max`: the save header and string layout of `IPersistStream`.
//! - `normalize_headers`: the CRLF rule of `ExtServ_ImplSetAdditionalHeaders`.
//! - `special_value_name`: the switch in `HlinkGetSpecialReference`.
//! - `resolve_ref_type`, `check_get_string_reference`, `set_moniker_reference_plan`
//!   and `set_string_reference_plan`: the flag checks of `IHlink` and
//!   `GetStringReference`.
//! - `check_update_stack_item` and `check_clone`: the argument checks of
//!   `HlinkUpdateStackItem` and `HlinkClone`.
//!
//! Divergences from the C bodies:
//! - C converts a string's unit count to a DWORD byte count. From 2^31 units up the
//!   count wraps, so C always fails with `STG_E_READFAULT`. The port decodes the string
//!   when its bytes are present. Testing this needs more than 4 GiB of input, so the
//!   oracle cannot cover it.
//! - Case folding is ASCII only. Wine's `wcsnicmp` and `wcsicmp` fold with
//!   `ntdll_towupper`. Under the Unicode simple mapping, U+0131 (dotless i) folds to
//!   `I`, so Wine accepts `fıle:` as `file:` and the port does not. This is
//!   inferred from the mapping and was not run. `.url` is unaffected.
//!   `strip_file_does_not_fold_dotless_i` pins the port's behaviour.
//!
//! Kept from C:
//! - `set_moniker_reference_plan` returns the flags as the HRESULT when neither the
//!   target nor the location bit is set.
//! - `set_string_reference_plan` returns flags from 4 to 0xFFFF_FFFC as the HRESULT.
//!   Flags from 0xFFFF_FFFD up take the normal path.
//! - `size_max` reports the size even when the moniker is absent. The total wraps
//!   modulo 2^64, as the C `ULONGLONG` sum does.
//!
//! Skipped (needs OS, registry, files, network, COM or the moniker stream): the
//! `IPersistStream` Load and Save orchestration, the `IHlink` string and moniker
//! setters, `HlinkNavigate`, `HlinkNavigateToStringReference`, `HlinkOnNavigate`,
//! `HlinkCreateFromData`, `HlinkQueryCreateFromData` (FIXME in C), `HlinkTranslateURL`
//! (FIXME stub), `HlinkResolveMonikerForData`, the browse context, the registry read
//! in `HlinkGetSpecialReference` (the port returns the value name only), ExtServ
//! authentication and `CreateExtensionServices`, `hlink.spec`, and TRACE and FIXME
//! logging.

mod args;
mod create;
mod display;
mod headers;
mod reference;
mod save;
mod special;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use args::{check_clone, check_update_stack_item};
pub use create::split_target;
pub use display::{is_absolute_display_name, is_shortcut, strip_file_scheme};
pub use headers::normalize_headers;
pub use reference::{
    check_get_string_reference, fallback_is_url, resolve_ref_type, set_moniker_reference_plan,
    set_string_reference_plan, MonikerPlan, StringPlan,
};
pub use save::{decode_string, encode_string, parse_header, save_flags, size_max, unknown_flags};
pub use special::special_value_name;

/// An `HRESULT`, as the C code returns it.
pub type HResult = i32;

pub const S_OK: HResult = 0;
pub const S_FALSE: HResult = 1;
pub const E_FAIL: HResult = 0x8000_4005_u32 as HResult;
pub const E_NOTIMPL: HResult = 0x8000_4001_u32 as HResult;
pub const E_INVALIDARG: HResult = 0x8007_0057_u32 as HResult;
pub const STG_E_READFAULT: HResult = 0x8003_001e_u32 as HResult;

/// The prefix of `units` up to its first NUL, which is where the C string functions stop.
fn c_str(units: &[u16]) -> &[u16] {
    &units[..units
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(units.len())]
}

fn non_empty(units: Option<&[u16]>) -> Option<&[u16]> {
    units.map(c_str).filter(|units| !units.is_empty())
}

/// Compares ASCII case-insensitively. A non-ASCII unit never matches an ASCII byte.
fn eq_ascii_ci(units: &[u16], text: &[u8]) -> bool {
    units.len() == text.len()
        && units.iter().zip(text).all(|(&unit, &byte)| {
            u8::try_from(unit).is_ok_and(|ascii| ascii.eq_ignore_ascii_case(&byte))
        })
}
