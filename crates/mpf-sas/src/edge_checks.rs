//! Edge-case tests for the ported sas export table and stub record.
//!
//! The oracle covers the record fields for the inputs in `oracle_checks.rs`. These tests cover the
//! constants, the export table and its lookup, and the `send_sas` helper. Expectations that are
//! not oracle-derived say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h. The harness shims
    // define the same values, and oracle rows 01-18 print them.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_has_only_send_sas_as_a_stub() {
    // Not oracle-derived: the single line of dlls/sas/sas.spec, `@ stub SendSAS`.
    assert_eq!(EXPORTS, [("SendSAS", ExportKind::Stub)]);
    assert_eq!(export_kind("SendSAS"), Some(ExportKind::Stub));
    assert_eq!(export_kind("sendsas"), None);
    assert_eq!(export_kind("SendSAS2"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn send_sas_is_the_stub_for_the_named_export() {
    // Not oracle-derived: the names come from MODULE in dlls/sas/Makefile.in and the spec line.
    // Oracle row 01 checks the same record through the C harness.
    let record = send_sas();
    assert_eq!(record.module, "sas.dll");
    assert_eq!(record.function, "SendSAS");
    assert_eq!(record, wine_spec_unimplemented_stub("sas.dll", "SendSAS"));
}

#[test]
fn stub_record_code_and_count_ignore_the_names() {
    // Not oracle-derived for the names chosen here. Oracle rows 10-12 and 18 show the same code,
    // flags and parameter count for empty and long names.
    for (module, function) in [("", ""), ("x.dll", ""), ("", "y")] {
        let record = wine_spec_unimplemented_stub(module, function);
        assert_eq!(record.code, EXCEPTION_WINE_STUB);
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(record.number_parameters, 2);
        assert_eq!(record.module, module);
        assert_eq!(record.function, function);
    }
}
