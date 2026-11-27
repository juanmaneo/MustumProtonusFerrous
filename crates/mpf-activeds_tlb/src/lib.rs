//! Port of the portable logic in Wine's `dlls/activeds.tlb/activeds_tlb.idl`, which includes
//! `include/iads.idl` and so compiles its `ActiveDs` library into `activeds.tlb`.
//!
//! `activeds.tlb` is data-only: `Makefile.in` passes `-Wb,--data-only` and there is no `.spec`
//! file, so it has no export table and no `@ stub` records to model.
//!
//! Ported:
//! - The 12 `typedef enum` blocks of `include/iads.idl` (121 enumerators): one `pub const` per
//!   enumerator, and one member table per enum in IDL order, such as [`ADS_SCOPEENUM`] and
//!   [`ADSTYPEENUM`]. The table names are [`ADS_RIGHTS_ENUM`], [`ADS_AUTHENTICATION_ENUM`],
//!   [`ADS_SEARCHPREF_ENUM`], [`ADS_CHASE_REFERRALS_ENUM`], [`ADS_STATUSENUM`],
//!   [`ADS_PASSWORD_ENCODING_ENUM`], [`ADS_OPTION_ENUM`], [`ADS_SECURITY_INFO_ENUM`],
//!   [`ADS_SETTYPE_ENUM`] and [`ADS_FORMAT_ENUM`]
//! - Two lookups over the member tables, [`member_value`] and [`member_name`]. Wine has no such
//!   functions; they are port-side helpers.
//!
//! Skipped:
//! - The interfaces, coclasses, structs, unions, typedef aliases and GUIDs of `iads.idl`. They
//!   are the typelib's type metadata, not code, and the port does not build a typelib.
//! - The `regtypelib` output (`#pragma makedep regtypelib`) and the `importlib("stdole2.tlb")`
//!   import.

mod enums;

pub use enums::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
