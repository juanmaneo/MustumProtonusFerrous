//! Edge-case tests for the ported odbcbcp export table and stub records.
//!
//! These cover the export table's size and ends, the lookup's misses, and the stub's name
//! handling. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived as constants: the values come from include/winternl.h and
    // include/winnt.h. The oracle rows show the same values through the stub.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_has_the_spec_stub_count_and_ends() {
    // Not oracle-derived: `grep -c '^@ stub' dlls/odbcbcp/odbcbcp.spec` gives 28, and the first
    // and last lines of the spec are bcp_batch and SQLLinkedServers.
    assert_eq!(EXPORTS.len(), 28);
    assert_eq!(EXPORTS[0].0, "bcp_batch");
    assert_eq!(EXPORTS[27].0, "SQLLinkedServers");
}

#[test]
fn lookup_finds_every_spec_name_and_only_those() {
    // Not oracle-derived: the lookup misses. The Rust helper names, case variants, padded names
    // and the empty name are not spec exports, so they return None.
    assert_eq!(export_stub("bcp_init_a"), None);
    assert_eq!(export_stub("BCP_BATCH"), None);
    assert_eq!(export_stub("sqllinkedservers"), None);
    assert_eq!(export_stub("bcp_batch "), None);
    assert_eq!(export_stub(""), None);
    assert_eq!(
        export_stub("bcp_batch").map(|stub| stub.function),
        Some("bcp_batch")
    );
    assert_eq!(
        export_stub("SQLLinkedServers").map(|stub| stub.function),
        Some("SQLLinkedServers")
    );
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the 28 fixed names of odbcbcp.spec. The code,
    // flags and parameter count are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}
