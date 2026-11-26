//! Edge-case tests for the ported tdh logic.
//!
//! These cover the buffer and size pointer boundaries, the short-buffer panic, the manifest
//! loaders' argument handling, the stub records and the export table. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/ntstatus.h, include/winternl.h
    // and include/winnt.h. The header size 8 is also the size that rows 02 and 07 report.
    assert_eq!(ERROR_SUCCESS, 0);
    assert_eq!(ERROR_INVALID_PARAMETER, 87);
    assert_eq!(ERROR_INSUFFICIENT_BUFFER, 122);
    assert_eq!(PROVIDER_ENUMERATION_HEADER_SIZE, 8);
    assert_eq!(STATUS_SUCCESS, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn null_size_pointer_is_invalid_for_any_buffer() {
    // Not oracle-derived: the oracle has a NULL size only with a NULL buffer (row 01). Here a
    // real buffer is passed; the C code returns before it reads or writes the buffer.
    let mut buf = vec![0xaa; 8];
    assert_eq!(
        tdh_enumerate_providers(Some(buf.as_mut_slice()), None),
        ERROR_INVALID_PARAMETER
    );
    assert_eq!(buf, vec![0xaa; 8]);
}

#[test]
fn null_buffer_with_a_size_below_the_header_is_invalid() {
    // Not oracle-derived: the oracle has a NULL buffer with sizes 0, 8 and 0xffffffff. A size of
    // 7 is non-zero, so the NULL buffer is rejected before the size is compared with the header,
    // and the size must be left alone.
    let mut size = 7;
    assert_eq!(
        tdh_enumerate_providers(None, Some(&mut size)),
        ERROR_INVALID_PARAMETER
    );
    assert_eq!(size, 7);
}

#[test]
fn zero_size_with_an_empty_buffer_reports_the_header_size() {
    // Not oracle-derived: the oracle has a zero size with a NULL buffer (row 02) and with an
    // 8-byte buffer (row 04). The C code never touches the buffer below the header size, so an
    // empty slice is accepted.
    let mut size = 0;
    assert_eq!(
        tdh_enumerate_providers(Some(&mut []), Some(&mut size)),
        ERROR_INSUFFICIENT_BUFFER
    );
    assert_eq!(size, PROVIDER_ENUMERATION_HEADER_SIZE);
}

#[test]
fn minimum_size_writes_exactly_four_bytes() {
    // Not oracle-derived: the oracle has size 8 with an 8-byte and a 24-byte buffer (rows 07
    // and 08). A 4-byte buffer is the smallest one the C write fits in, and it is accepted.
    let mut size = 8;
    let mut buf = vec![0xaa; 4];
    assert_eq!(
        tdh_enumerate_providers(Some(buf.as_mut_slice()), Some(&mut size)),
        ERROR_SUCCESS
    );
    assert_eq!(buf, [0, 0, 0, 0]);
    assert_eq!(size, 8);
}

#[test]
#[should_panic]
fn buffer_shorter_than_four_bytes_panics_where_c_would_overflow() {
    // Not oracle-derived: the C code writes NumberOfProviders with no length check, so a 3-byte
    // buffer with a size of 8 is a contract violation. The port panics instead of writing past
    // the end.
    let mut size = 8;
    let mut buf = vec![0xaa; 3];
    tdh_enumerate_providers(Some(buf.as_mut_slice()), Some(&mut size));
}

#[test]
fn manifest_loaders_ignore_their_argument() {
    // Not oracle-derived: the oracle passes NULL and L"x" only. The C bodies only trace the
    // argument, so an empty slice and a longer one give the same status.
    assert_eq!(tdh_load_manifest(Some(&[])), STATUS_SUCCESS);
    assert_eq!(tdh_load_manifest_from_binary(Some(&[])), STATUS_SUCCESS);
    let long: Vec<u16> = "C:\\manifest.man\0".encode_utf16().collect();
    assert_eq!(tdh_load_manifest(Some(&long)), STATUS_SUCCESS);
    assert_eq!(tdh_load_manifest_from_binary(Some(&long)), STATUS_SUCCESS);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of tdh.spec. The code and
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
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 34 lines of dlls/tdh/tdh.spec, in order, with the kind of each.
    assert_eq!(EXPORTS.len(), 34);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, k)| *k == ExportKind::Stub)
            .count(),
        31
    );
    assert_eq!(EXPORTS[0], ("TdhAggregatePayloadFilters", ExportKind::Stub));
    assert_eq!(EXPORTS[9], ("TdhEnumerateProviders", ExportKind::Ported));
    assert_eq!(EXPORTS[23], ("TdhLoadManifest", ExportKind::StatusSuccess));
    assert_eq!(
        EXPORTS[24],
        ("TdhLoadManifestFromBinary", ExportKind::StatusSuccess)
    );
    assert_eq!(EXPORTS[33], ("TdhpGetEventMapInfoWBEM", ExportKind::Stub));

    assert_eq!(
        export_kind("TdhEnumerateProviders"),
        Some(ExportKind::Ported)
    );
    assert_eq!(
        export_kind("TdhLoadManifest"),
        Some(ExportKind::StatusSuccess)
    );
    assert_eq!(
        export_kind("TdhLoadManifestFromBinary"),
        Some(ExportKind::StatusSuccess)
    );
    assert_eq!(export_kind("TdhGetProperty"), Some(ExportKind::Stub));
    assert_eq!(export_kind("tdhenumerateproviders"), None);
    assert_eq!(export_kind(""), None);
}
