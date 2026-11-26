//! Edge-case tests for the ported wevtapi logic.
//!
//! These cover the property-id and buffer-size boundaries, the variant string, the outcome
//! constructors, and the export table. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winevt.h, include/winnt.h and
    // include/winternl.h. The oracle printed the same values through the C constants.
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
    assert_eq!(ERROR_INSUFFICIENT_BUFFER, 122);
    assert_eq!(LOG_FILE_PATH_PROPERTY, 9);
    assert_eq!(EVT_VAR_TYPE_STRING, 1);
    assert_eq!(EVT_VARIANT_SIZE, 16);
    assert_eq!(NULL_HANDLE, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(WEVTAPI_DLL_NAME, "wevtapi.dll");
}

#[test]
fn neighbouring_property_ids_are_rejected_without_writing_used() {
    // Not oracle-derived: the oracle only sent ids 0, 8 and 21. Ids 8 and 10 sit next to the
    // property the port handles, and u32::MAX is the largest value a DWORD can hold.
    for id in [8, 10, u32::MAX] {
        let mut used = 0x1234;
        let (outcome, variant) = get_channel_config_property(id, u32::MAX, true, &mut used);
        assert_eq!(outcome, Outcome::failed(false, ERROR_CALL_NOT_IMPLEMENTED));
        assert_eq!(variant, None);
        assert_eq!(used, 0x1234);
    }
}

#[test]
fn oversized_buffer_is_accepted() {
    // Not oracle-derived: the oracle sent 100 and 65 bytes, but not u32::MAX. The C check is only
    // a lower bound, so any size at or above 64 succeeds and `used` stays 64.
    let mut used = 0;
    let (outcome, variant) =
        get_channel_config_property(LOG_FILE_PATH_PROPERTY, u32::MAX, true, &mut used);
    assert_eq!(outcome, Outcome::returned(true));
    assert!(variant.is_some());
    assert_eq!(used, 64);
}

#[test]
fn size_boundary_is_exact() {
    // Not oracle-derived: the oracle sent 63 (rejected) and 64 (accepted), which also pin this
    // boundary. The check is `size < needed`, so `needed` itself is the first accepted size.
    let mut used = 0;
    let (short, short_variant) = get_channel_config_property(
        LOG_FILE_PATH_PROPERTY,
        EVT_VARIANT_SIZE + 47,
        true,
        &mut used,
    );
    assert_eq!(short, Outcome::failed(false, ERROR_INSUFFICIENT_BUFFER));
    assert_eq!(short_variant, None);
    assert_eq!(used, 64);

    let (exact, exact_variant) = get_channel_config_property(
        LOG_FILE_PATH_PROPERTY,
        EVT_VARIANT_SIZE + 48,
        true,
        &mut used,
    );
    assert_eq!(exact, Outcome::returned(true));
    assert!(exact_variant.is_some());
}

#[test]
fn path_variant_is_nul_terminated_utf16() {
    // Not oracle-derived for the code units themselves. The oracle printed the path and the
    // 48-byte `sizeof(log_pathW)`, which is 24 UTF-16 units including the NUL.
    let mut used = 0;
    let (_, variant) = get_channel_config_property(LOG_FILE_PATH_PROPERTY, 64, true, &mut used);
    let variant = variant.expect("the path is written at size 64");
    assert_eq!(variant.string_val.len() * 2, 48);
    assert_eq!(variant.string_val.last(), Some(&0));
    assert_eq!(variant.string_val.iter().filter(|&&u| u == 0).count(), 1);
}

#[test]
fn outcome_constructors_set_last_error_only_when_failed() {
    // Not oracle-derived: the oracle showed err=- for the `returned` rows and err=120 or 122 for
    // the `failed` rows. These assertions pin the struct layout the rows rely on.
    assert_eq!(
        Outcome::returned(true),
        Outcome {
            ret: true,
            last_error: None
        }
    );
    assert_eq!(
        Outcome::failed(false, ERROR_INSUFFICIENT_BUFFER),
        Outcome {
            ret: false,
            last_error: Some(122)
        }
    );
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness passed the fixed names of wevtapi.spec. The code and flags
    // are the same for any names.
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
fn export_table_matches_spec_order_and_kinds() {
    // Not oracle-derived: the 46 lines of dlls/wevtapi/wevtapi.spec, in order. The oracle rows
    // cover each export's behaviour; this pins the table itself.
    assert_eq!(EXPORTS.len(), 46);
    assert_eq!(EXPORTS[0], ("EvtArchiveExportedLog", ExportKind::Stub));
    assert_eq!(EXPORTS[3], ("EvtClose", ExportKind::Stdcall));
    assert_eq!(EXPORTS[45], ("EvtUpdateBookmark", ExportKind::Stub));

    let stdcall = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stdcall)
        .count();
    assert_eq!(stdcall, 16);
    assert_eq!(EXPORTS.len() - stdcall, 30);

    assert_eq!(export_kind("EvtClose"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("EvtCancel"), Some(ExportKind::Stub));
    assert_eq!(export_kind("evtclose"), None);
    assert_eq!(export_kind(""), None);
}
