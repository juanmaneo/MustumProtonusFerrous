//! Port of the portable logic in Wine's `dlls/mshtml.tlb/mshtml_tlb.idl`, which includes
//! `include/mshtml.idl` and so compiles its `MSHTML` library into `mshtml.tlb`.
//!
//! `mshtml.tlb` is data-only: `Makefile.in` passes `-Wb,--data-only` and there is no `.spec`
//! file, so it has no export table and no `@ stub` records to model. Its deterministic content
//! is the enum constants of the typelib, which is what this crate ports.
//!
//! Ported:
//! - The 12 `typedef enum` blocks of `include/mshtml.idl` (288 enumerators): one `pub const` per
//!   enumerator, and one member table per enum in IDL order, such as [`POINTER_GRAVITY`] and
//!   [`ELEMENT_TAG_ID`]. The other table names are [`MARKUP_CONTEXT_TYPE`],
//!   [`ELEMENT_ADJACENCY`], [`MOVEUNIT_ACTION`], [`SELECTION_TYPE`], [`COORD_SYSTEM`],
//!   [`DISPLAY_MOVEUNIT`], [`DISPLAY_GRAVITY`], [`CARET_DIRECTION`], [`DomConstructor`] and
//!   [`DOM_EVENT_PHASE`].
//! - Two lookups over the member tables, [`member_value`] and [`member_name`]. Wine has no such
//!   functions; they are port-side helpers.
//!
//! Skipped:
//! - The `library MSHTML` interfaces, dispinterfaces, coclasses, structs, typedef aliases and
//!   GUIDs, including the library's own `uuid` and `version(4.0)`. They are the typelib's type
//!   metadata, not code, and the port does not build a typelib.
//! - The `cpp_quote` lines of `include/mshtml.idl` (`DEFINE_GUID`, `#define`), which only reach
//!   the generated header, not the typelib.
//! - The `regtypelib` output (`#pragma makedep regtypelib`) and the `importlib("stdole2.tlb")`
//!   import.

mod enums;

pub use enums::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
