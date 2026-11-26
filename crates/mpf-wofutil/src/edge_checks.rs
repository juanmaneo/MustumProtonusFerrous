//! Edge-case tests for the ported wofutil logic.
//!
//! These cover the out-parameter writes of the two explicit functions, the stub record and the
//! export table. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/windef.h, include/winternl.h,
    // include/winnt.h and dlls/wofutil/Makefile.in.
    assert_eq!(S_OK, 0);
    assert_eq!(FALSE, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(WOFUTIL_DLL_NAME, "wofutil.dll");
}

#[test]
fn is_external_file_never_reads_path_or_ptr() {
    // Not oracle-derived: the C body never reads `path` or `ptr`, so an empty path and an
    // arbitrary opaque pointer give the same result as NULL. Row 15 of the oracle covers `ptr`
    // alone with the value 1.
    let mut result = 1;
    let empty: [u16; 0] = [];
    assert_eq!(
        wof_is_external_file(Some(&empty[..]), Some(&mut result), None, usize::MAX, None),
        S_OK
    );
    assert_eq!(result, FALSE);
}

#[test]
fn is_external_file_writes_zero_over_previous_values() {
    // Not oracle-derived: the body writes unconditionally when a pointer is non-NULL, so values
    // that are already 0 or FALSE are written again and still read back as 0.
    let mut result = FALSE;
    let mut provider = 7;
    let mut length = 9;
    assert_eq!(
        wof_is_external_file(
            None,
            Some(&mut result),
            Some(&mut provider),
            0,
            Some(&mut length)
        ),
        S_OK
    );
    assert_eq!((result, provider, length), (0, 0, 0));
}

#[test]
fn should_compress_binaries_ignores_volume_and_alg() {
    // Not oracle-derived: the body returns FALSE for any volume, including an empty one, and
    // never writes `alg`. The NULL pair is the other edge.
    let mut alg = 42;
    let empty: [u16; 0] = [];
    assert_eq!(
        wof_should_compress_binaries(Some(&empty[..]), Some(&mut alg)),
        FALSE
    );
    assert_eq!(alg, 42);
    assert_eq!(wof_should_compress_binaries(None, None), FALSE);
}

#[test]
fn stub_record_carries_the_arguments_it_is_given() {
    // Not oracle-derived: the record copies the module and function arguments unchanged, so the
    // helpers depend only on the name they pass.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Fn"),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "other.dll",
            function: "Fn",
        }
    );
}

#[test]
fn stub_helpers_match_the_export_table_in_spec_order() {
    // Not oracle-derived: every `@ stub` name in EXPORTS has a helper in STUB_EXPORTS, in the
    // same order, and each helper names its own export under wofutil.dll.
    let stub_names: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let helper_names: Vec<&str> = STUB_EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(helper_names, stub_names);
    assert_eq!(helper_names.len(), 9);
    for (name, helper) in STUB_EXPORTS {
        let record = helper();
        assert_eq!(record.module, WOFUTIL_DLL_NAME);
        assert_eq!(record.function, *name);
        assert_eq!(record.code, EXCEPTION_WINE_STUB);
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(record.number_parameters, 2);
    }
}

#[test]
fn export_kind_finds_each_export_and_rejects_others() {
    // Not oracle-derived: the spec exports 2 `@ stdcall` functions and 9 `@ stub` functions, and
    // lookup is case-sensitive.
    for (name, kind) in EXPORTS {
        assert_eq!(export_kind(name), Some(kind));
    }
    assert_eq!(export_kind("WofIsExternalFile"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("WofShouldCompressBinaries"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("wofisexternalfile"), None);
    assert_eq!(export_kind("WofMissing"), None);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stdcall)
            .count(),
        2
    );
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stub)
            .count(),
        9
    );
}
