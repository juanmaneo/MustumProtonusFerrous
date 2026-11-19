//! dwrite subset for MPF: the byte-level OpenType helpers of Wine's `dlls/dwrite/opentype.c`.
//!
//! The rest of the C file reads fonts through `IDWriteFontFileStream` and builds COM objects.
//! The helpers ported here take the table bytes or the code point as plain values, and keep the
//! control flow of the C bodies.
//!
//! Ported (names in `dlls/dwrite/opentype.c`; oracle rows in the `oracle_checks` tests):
//! - `table_read_ensure`, `table_read_be_word`: [`Table`].
//! - `opentype_get_gasp_flags`: [`gasp_flags`].
//! - `find_vdmx_group`, `opentype_get_vdmx_size`: [`vdmx_size`].
//! - `opentype_analyze_container_type`: [`container_type`], [`ContainerType`].
//! - `opentype_is_zero_width`, `opentype_is_default_ignorable`, `opentype_is_whitespace`:
//!   [`is_zero_width`], [`is_default_ignorable`], [`is_whitespace`].
//!
//! Divergences from the C bodies:
//! - Reads past the end of a [`Table`] are bounds-checked and give 0. The C VDMX header check
//!   does not cover the ratio offset array, so the C read can run past the table, and an odd
//!   offset is a misaligned `WORD` load. The port reads plain bytes at the offset. The oracle
//!   excludes the vectors that trip the sanitizer on either case.
//! - `opentype_analyze_container_type` reads its signature as a host-order `DWORD`. The port
//!   compares the four bytes in order, which is what the C code does on little-endian hosts.
//!
//! Skipped:
//! - The font file analysers and the `IDWriteFontFileStream` readers, which need a stream.
//! - The name table, cmap, `post`, `OS/2` and `head` metric readers, the layout and GDEF
//!   readers, and the CPAL and COLR readers, which are not ported yet.
//! - The glyph image format probes, which need an `IDWriteFontFace5`.
//! - `opentype_is_diacritic`, which uses `GetStringTypeW`, and the mirror table lookup.

mod chars;
mod container;
mod gasp;
mod table;
mod vdmx;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use chars::{is_default_ignorable, is_whitespace, is_zero_width};
pub use container::{container_type, ContainerType};
pub use gasp::gasp_flags;
pub use table::Table;
pub use vdmx::vdmx_size;
