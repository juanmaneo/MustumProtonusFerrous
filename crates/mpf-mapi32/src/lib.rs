//! mapi32 subset for MPF: the deterministic helpers of Wine's `dlls/mapi32`.
//!
//! Most of the DLL forwards to MAPI providers, keeps COM objects, or reads caller memory behind
//! `IsBadReadPtr`-style checks. The pure helpers run here on byte, word and value slices, and
//! follow the C control flow. A byte string is a slice that is read up to its first NUL, as the
//! C code reads it.
//!
//! Ported (names in `dlls/mapi32`; oracle rows in the `oracle_checks` tests):
//! - `util.c` hex helpers: [`bin_from_hex`], [`hex_from_bin`], [`ul_from_sz_hex`].
//! - `util.c` byte-swap and string helpers: [`swap_plong`], [`swap_pword`], [`uf_from_sz`],
//!   [`cb_of_encoded`], [`lstrlen_w`], [`lstrcpy_w`].
//! - `util.c` FILETIME arithmetic: [`FileTime`], [`ft_add_ft`], [`ft_sub_ft`], [`ft_mul_dw`],
//!   [`ft_mul_dw_dw`], [`ft_neg_ft`].
//! - `prop.c` property-tag helpers: [`prop_type`], [`prop_id`], [`fbad_prop_tag`],
//!   [`fbad_column_set`], [`find_prop_index`], [`lval_find_prop_index`].
//!
//! Not ported: the provider forwarding and `ScInitMapiUtil`, the COM objects (`imalloc.c` and
//! `IPropData`), `CreateIProp`, `OpenStreamOnFile`, `FEqualNames` and `MNLS_CompareStringW`
//! (which compare strings with locale collation), `UlPropSize`, the string branches of
//! `LPropCompareProp`, and the pointer checks (`IsBadReadPtr`, `IsBadStringPtr`).
//!
//! Divergences from the C bodies:
//! - `FBinFromHex` writes the pairs before a bad one, and then returns `FALSE`. [`bin_from_hex`]
//!   returns `None` and writes nothing, because the output of a failed call is not specified.
//! - `HexFromBin` writes a NUL terminator. [`hex_from_bin`] returns a `String` without one.
//! - `MAPI32_FtSubFt` returns `right - left`, although its comment says `left - right`. The port
//!   keeps the code, so [`ft_sub_ft`] returns `right - left`.
//! - The C FILETIME helpers overflow a signed 64-bit value, which is undefined behaviour. The
//!   port wraps instead, see [`ft_add_ft`].
//! - A NULL pointer is `None` where the C code takes one ([`cb_of_encoded`]). Slices replace the
//!   `(pointer, count)` pairs, so the length comes from the slice.
//! - [`lstrcpy_w`] returns `None` where the C code would write past `dest` or read past `src`.

mod filetime;
mod hex;
mod prop;
mod util;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use filetime::{ft_add_ft, ft_mul_dw, ft_mul_dw_dw, ft_neg_ft, ft_sub_ft, FileTime};
pub use hex::{bin_from_hex, hex_from_bin, ul_from_sz_hex};
pub use prop::{
    fbad_column_set, fbad_prop_tag, find_prop_index, lval_find_prop_index, prop_id, prop_type,
    MV_FLAG, PROP_TYPE_MASK, PT_APPTIME, PT_BINARY, PT_BOOLEAN, PT_CLSID, PT_CURRENCY, PT_DOUBLE,
    PT_ERROR, PT_I2, PT_I8, PT_LONG, PT_NULL, PT_OBJECT, PT_R4, PT_STRING8, PT_SYSTIME, PT_UNICODE,
    PT_UNSPECIFIED,
};
pub use util::{cb_of_encoded, lstrcpy_w, lstrlen_w, swap_plong, swap_pword, uf_from_sz};

/// The bytes that a C string reads: everything before the first NUL.
fn c_str(text: &[u8]) -> &[u8] {
    let end = text.iter().position(|&b| b == 0).unwrap_or(text.len());
    &text[..end]
}
