//! Edge-case tests for the ported hrtfapo logic.
//!
//! These cover the constants, the out-parameter contract of `CreateHrtfApo`, and the stub and
//! export tables. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn create_hrtf_apo_never_writes_the_out_parameter() {
    // Not oracle-derived: the oracle only stored 0 and 0xdead in `*xapo`. These boundary values
    // check that the port leaves every stored value alone, including the largest one.
    for before in [0, 1, u32::MAX] {
        let mut slot = before;
        assert_eq!(create_hrtf_apo::<u8, u32>(None, Some(&mut slot)), E_NOTIMPL);
        assert_eq!(slot, before);
    }
}

#[test]
fn create_hrtf_apo_ignores_the_init_contents() {
    // Not oracle-derived: the oracle used the init bytes 0x01 and 0xff. This also checks the
    // lowest and highest byte values.
    for byte in [0u8, u8::MAX] {
        assert_eq!(create_hrtf_apo::<u8, u32>(Some(&byte), None), E_NOTIMPL);
    }
}

#[test]
fn wine_stub_record_passes_names_through_unchanged() {
    // Not oracle-derived: the oracle only passed the fixed names of hrtfapo.spec. The code and
    // flags are the same for any names.
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

#[test]
fn stub_helpers_name_their_own_export() {
    // Not oracle-derived: the names come from the four `@ stub` lines of hrtfapo.spec. The
    // oracle rows check the same names through the verbatim C wrappers.
    for (name, stub) in [
        (
            "CreateHrtfApoWithDatasetType",
            create_hrtf_apo_with_dataset_type(),
        ),
        ("CreateHrtfEngineFactory", create_hrtf_engine_factory()),
        ("IsHrtfApoAvailable", is_hrtf_apo_available()),
        (
            "GetHrtfEngineMinFrameCount",
            get_hrtf_engine_min_frame_count(),
        ),
    ] {
        assert_eq!(stub.module, "hrtfapo.dll");
        assert_eq!(stub.function, name);
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the five lines of dlls/hrtfapo/hrtfapo.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("CreateHrtfApo", ExportKind::Stdcall),
            ("CreateHrtfApoWithDatasetType", ExportKind::Stub),
            ("CreateHrtfEngineFactory", ExportKind::Stub),
            ("IsHrtfApoAvailable", ExportKind::Stub),
            ("GetHrtfEngineMinFrameCount", ExportKind::Stub),
        ]
    );
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: lookups of names the spec does and does not export.
    assert_eq!(export_kind("CreateHrtfApo"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("GetHrtfEngineMinFrameCount"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("createhrtfapo"), None);
    assert_eq!(export_kind("CreateHrtfApo "), None);
    assert_eq!(export_kind(""), None);
}
