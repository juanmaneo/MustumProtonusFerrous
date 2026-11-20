//! Deterministic property-storage name conversion from Wine's `dlls/coml2/stg_prop.c`.
//!
//! The crate ports `FmtIdToPropStgName` and `PropStgNameToFmtId`. Storage, streams, COM objects,
//! and handle-based exports are outside this portable subset.

mod fmtid;
mod prop_name;

pub use fmtid::{
    Fmtid, FMTID_DOC_SUMMARY_INFORMATION, FMTID_SUMMARY_INFORMATION, FMTID_USER_DEFINED_PROPERTIES,
};
pub use prop_name::{fmt_id_to_prop_stg_name, prop_stg_name_to_fmt_id};

const E_INVALIDARG: i32 = 0x8007_0057_u32 as i32;
const STG_E_INVALIDNAME: i32 = 0x8003_00FC_u32 as i32;

/// A failed property-storage name conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Wine returns `E_INVALIDARG` when the FMTID pointer is null.
    InvalidArg,
    /// Wine returns `STG_E_INVALIDNAME` for a null or invalid property-storage name.
    InvalidName,
}

impl Error {
    /// Returns the HRESULT Wine reports for this error.
    pub const fn hresult(self) -> i32 {
        match self {
            Self::InvalidArg => E_INVALIDARG,
            Self::InvalidName => STG_E_INVALIDNAME,
        }
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
