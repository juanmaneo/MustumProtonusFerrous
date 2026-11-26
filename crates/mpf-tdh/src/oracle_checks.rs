//! Oracle-derived test vectors for the ported tdh logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `TdhLoadManifest`,
//! `TdhLoadManifestFromBinary` and `TdhEnumerateProviders` (`dlls/tdh/tdh_main.c`) and of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/tdh/`, and running the harness. The
//! 45 printed rows are rows 01 to 10 for `TdhEnumerateProviders`, 11 to 14 for the two manifest
//! loaders, and 15 to 45 for the 31 `@ stub` exports. None of them was flagged by ASan or UBSan,
//! so none is excluded.

use super::*;

/// The buffer the C harness starts with: `0xaa` everywhere, then `NumberOfProviders` is
/// `0xdeadbeef` and `Reserved` is `0x12345678`. Every oracle buffer is at least eight bytes.
fn sentinel_buffer(cap: usize) -> Vec<u8> {
    let mut buf = vec![0xaa; cap];
    buf[..4].copy_from_slice(&0xdead_beefu32.to_le_bytes());
    buf[4..8].copy_from_slice(&0x1234_5678u32.to_le_bytes());
    buf
}

/// The sentinel buffer of `cap` bytes after `NumberOfProviders` has been written as `nprov`.
fn with_providers(cap: usize, nprov: u32) -> Vec<u8> {
    let mut buf = sentinel_buffer(cap);
    buf[..4].copy_from_slice(&nprov.to_le_bytes());
    buf
}

/// Calls `tdh_enumerate_providers` with a sentinel buffer of `cap` bytes (`None` is a NULL
/// buffer) and a size of `size` (`None` is a NULL size pointer). Returns the return value, the
/// size after the call and the buffer after the call.
fn call(cap: Option<usize>, size: Option<u32>) -> (u32, Option<u32>, Option<Vec<u8>>) {
    let mut buf = cap.map(sentinel_buffer);
    let mut size = size;
    let ret = tdh_enumerate_providers(buf.as_deref_mut(), size.as_mut());
    (ret, size, buf)
}

#[test]
fn oracle_tdh_enumerate_providers_rows() {
    // row 01 buf=NULL size=NULL -> ret=87 size=- nprov=- reserved=-
    assert_eq!(call(None, None), (87, None, None));

    // row 02 buf=NULL size=0 -> ret=122 size=8 nprov=- reserved=-
    assert_eq!(call(None, Some(0)), (122, Some(8), None));

    // row 03 buf=NULL size=8 -> ret=87 size=8 nprov=- reserved=-
    assert_eq!(call(None, Some(8)), (87, Some(8), None));

    // row 04 buf=cap8 size=0 -> ret=122 size=8 nprov=3735928559 reserved=305419896
    assert_eq!(
        call(Some(8), Some(0)),
        (122, Some(8), Some(sentinel_buffer(8)))
    );

    // row 05 buf=cap8 size=4 -> ret=122 size=8 nprov=3735928559 reserved=305419896
    assert_eq!(
        call(Some(8), Some(4)),
        (122, Some(8), Some(sentinel_buffer(8)))
    );

    // row 06 buf=cap8 size=7 -> ret=122 size=8 nprov=3735928559 reserved=305419896
    assert_eq!(
        call(Some(8), Some(7)),
        (122, Some(8), Some(sentinel_buffer(8)))
    );

    // row 07 buf=cap8 size=8 -> ret=0 size=8 nprov=0 reserved=305419896
    assert_eq!(
        call(Some(8), Some(8)),
        (0, Some(8), Some(with_providers(8, 0)))
    );

    // row 08 buf=cap24 size=24 -> ret=0 size=24 nprov=0 reserved=305419896
    assert_eq!(
        call(Some(24), Some(24)),
        (0, Some(24), Some(with_providers(24, 0)))
    );

    // row 09 buf=cap64 size=0xffffffff -> ret=0 size=4294967295 nprov=0 reserved=305419896
    assert_eq!(
        call(Some(64), Some(0xffff_ffff)),
        (0, Some(0xffff_ffff), Some(with_providers(64, 0)))
    );

    // row 10 buf=NULL size=0xffffffff -> ret=87 size=4294967295 nprov=- reserved=-
    assert_eq!(call(None, Some(0xffff_ffff)), (87, Some(0xffff_ffff), None));
}

#[test]
fn oracle_manifest_loader_rows() {
    // L"x" in C is two WCHARs: 'x' and the terminating NUL.
    let x = [u16::from(b'x'), 0];

    // row 11 TdhLoadManifest(NULL) -> ret=0
    assert_eq!(tdh_load_manifest(None), 0);

    // row 12 TdhLoadManifest(L"x") -> ret=0
    assert_eq!(tdh_load_manifest(Some(&x)), 0);

    // row 13 TdhLoadManifestFromBinary(NULL) -> ret=0
    assert_eq!(tdh_load_manifest_from_binary(None), 0);

    // row 14 TdhLoadManifestFromBinary(L"x") -> ret=0
    assert_eq!(tdh_load_manifest_from_binary(Some(&x)), 0);
}

#[test]
fn oracle_stub_rows() {
    // rows 15 to 45: __wine_spec_unimplemented_stub("tdh.dll", name) raises
    // code=0x80000100 flags=0x1 nparams=2 p0=tdh.dll p1=name, for each @ stub export.
    type StubFn = fn() -> StubException;
    let rows: [(StubFn, &str); 31] = [
        (tdh_aggregate_payload_filters, "TdhAggregatePayloadFilters"),
        (tdh_apply_payload_filter, "TdhApplyPayloadFilter"),
        (
            tdh_cleanup_payload_event_filter_descriptor,
            "TdhCleanupPayloadEventFilterDescriptor",
        ),
        (tdh_close_decoding_handle, "TdhCloseDecodingHandle"),
        (tdh_create_payload_filter, "TdhCreatePayloadFilter"),
        (tdh_delete_payload_filter, "TdhDeletePayloadFilter"),
        (
            tdh_enumerate_manifest_provider_events,
            "TdhEnumerateManifestProviderEvents",
        ),
        (
            tdh_enumerate_provider_field_information,
            "TdhEnumerateProviderFieldInformation",
        ),
        (
            tdh_enumerate_provider_filters,
            "TdhEnumerateProviderFilters",
        ),
        (
            tdh_enumerate_remote_wbem_provider_field_information,
            "TdhEnumerateRemoteWBEMProviderFieldInformation",
        ),
        (
            tdh_enumerate_remote_wbem_providers,
            "TdhEnumerateRemoteWBEMProviders",
        ),
        (tdh_format_property, "TdhFormatProperty"),
        (tdh_get_all_events_information, "TdhGetAllEventsInformation"),
        (tdh_get_decoding_parameter, "TdhGetDecodingParameter"),
        (tdh_get_event_information, "TdhGetEventInformation"),
        (tdh_get_event_map_information, "TdhGetEventMapInformation"),
        (
            tdh_get_manifest_event_information,
            "TdhGetManifestEventInformation",
        ),
        (tdh_get_property, "TdhGetProperty"),
        (
            tdh_get_property_offset_and_size,
            "TdhGetPropertyOffsetAndSize",
        ),
        (tdh_get_property_size, "TdhGetPropertySize"),
        (tdh_get_wpp_message, "TdhGetWppMessage"),
        (tdh_get_wpp_property, "TdhGetWppProperty"),
        (tdh_open_decoding_handle, "TdhOpenDecodingHandle"),
        (
            tdh_query_provider_field_information,
            "TdhQueryProviderFieldInformation",
        ),
        (
            tdh_query_remote_wbem_provider_field_information,
            "TdhQueryRemoteWBEMProviderFieldInformation",
        ),
        (tdh_set_decoding_parameter, "TdhSetDecodingParameter"),
        (tdh_unload_manifest, "TdhUnloadManifest"),
        (tdh_validate_payload_filter, "TdhValidatePayloadFilter"),
        (
            tdhp_find_match_class_from_wbem,
            "TdhpFindMatchClassFromWBEM",
        ),
        (
            tdhp_get_best_trace_event_info_wbem,
            "TdhpGetBestTraceEventInfoWBEM",
        ),
        (tdhp_get_event_map_info_wbem, "TdhpGetEventMapInfoWBEM"),
    ];
    for (stub, name) in rows {
        assert_eq!(
            stub(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "tdh.dll",
                function: name,
            }
        );
    }
}
