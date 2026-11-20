//! Port of the deterministic logic in Wine's `dlls/rometadata`: the bit widths and coded indexes
//! from `assembly.c`, the compressed integer and heap readers from `assembly.c`, the PE RVA
//! mapping from `assembly.c`, the token enumerations from `mdtables.c`, and the CLR type-name
//! split and comparison from `mdtables.c`.
//!
//! Ported:
//! - `bit_width`, `index_size`, `assembly_get_coded_index_size` and
//!   `metadata_coded_value_as_token`: [`coded`].
//! - `decode_int`, `assembly_get_blob`, `assembly_get_string`, `assembly_get_guid`,
//!   `trim_string_heap` and the bounds test in `assembly_addr_offset`: [`heap`].
//! - `pe_rva_to_offset`: [`pe::rva_to_offset`].
//! - `type_split_name` and `clr_type_path_equals`: [`typepath`].
//! - `token_enum_range_create`, `token_enum_list_create`, `token_enum_get_entries`,
//!   `import_CountEnum` and `import_ResetEnum`, without the COM wrappers: [`enumeration`].
//!
//! Skipped:
//! - `debugstr_mdToken` and the `TRACE` calls. They only format trace output.
//! - `clr_type_path_join`. It converts through the ANSI code page and `MultiByteToWideChar`.
//! - The `token_array_*` helpers. They only allocate and grow buffers.
//! - Table and column lookups (`assembly_get_table`, `assembly_get_column`, `table_get_*`) and
//!   the file and PE header parsing in `assembly_parse_*`. They depend on the layout of a mapped
//!   file, so they are not reproduced here.
//! - The COM entry points: `QueryInterface`, `AddRef`, `Release`, and the `import_*` and
//!   `tables_*` methods that need a metadata object or a live `IMetaDataImport`.
//!
//! Divergences from C:
//! - A coded value whose tag is past the end of its table list: C asserts, and
//!   [`coded::coded_value_as_token`] returns `None`.
//! - An `addr` check on a zero-length buffer: C asserts that the address lies inside the buffer,
//!   and [`heap::span_in_bounds`] returns `false`.
//! - A type name with a leading dot compared with `clr_type_path_equals`: C asserts that an empty
//!   namespace has no first character, and [`typepath::TypePath::equals`] compares the parts.
//! - A range enumeration that starts at zero, counted with `import_CountEnum`: C asserts that the
//!   start is nonzero, and [`enumeration::TokenEnum::count`] returns `end + 1` with wrapping
//!   arithmetic.
//! - A range with `start > end` panics in [`enumeration::TokenEnum::range`], as the C assert in
//!   `token_enum_range_create` does. This is the same behaviour, not a divergence.
//! - `assembly_get_blob` does not check that a declared blob size fits in the heap, and
//!   `blob_at` mirrors that. The returned tail is the bytes that follow the length prefix, up to
//!   the end of the heap.
//! - The enumeration types take their tokens as owned `Vec`s, so the C `token_array` growth
//!   logic is not needed.

pub mod coded;
pub mod enumeration;
pub mod heap;
pub mod pe;
pub mod typepath;

/// Combines a row index with a table tag (C `TokenFromRid`).
pub const fn token_from_rid(rid: u32, token_type: u32) -> u32 {
    rid | token_type
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
