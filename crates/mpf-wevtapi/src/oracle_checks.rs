//! Oracle-derived test vectors for the ported wevtapi logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/wevtapi/main.c` and
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/wevtapi/`, and running `harness.c`. Two harness adaptations, neither of
//! which changes a value: the path literal `L"..."` became `u"..."` so it is 16-bit on Linux, and
//! the stub's `RtlRaiseException` recorder longjmps out instead of returning, because the Wine loop
//! never exits. The harness printed 55 rows: rows 01 to 25 for the explicit functions and rows 26
//! to 55 for the 30 `@ stub` exports. ASan and UBSan printed no report, so no row was excluded.

use super::*;

/// Calls `get_channel_config_property` with a starting `used` value of `used_in`, and returns the
/// outcome, the variant if the C code wrote one, and the value the call left in `used`.
fn property(
    property_id: u32,
    size: u32,
    present: bool,
    used_in: u32,
) -> (Outcome<bool>, Option<PropertyVariant>, u32) {
    let mut used = used_in;
    let (outcome, variant) = get_channel_config_property(property_id, size, present, &mut used);
    (outcome, variant, used)
}

fn log_path_variant() -> Option<PropertyVariant> {
    Some(PropertyVariant {
        variant_type: 1,
        string_val: "C:\\windows\\temp\\evt.log"
            .encode_utf16()
            .chain([0])
            .collect(),
    })
}

#[test]
fn oracle_explicit_rows() {
    // row 01 EvtOpenSession -> ret=(nil) err=-
    assert_eq!(evt_open_session(), NULL_HANDLE);
    // row 02 EvtOpenLog -> ret=(nil) err=-
    assert_eq!(evt_open_log(), NULL_HANDLE);

    // row 03 id=9 size=64 buf=1 used_in=0x5a5a5a5a -> ret=1 err=- used=64 touched=1 type=1 str=path
    assert_eq!(
        property(9, 64, true, 0x5a5a_5a5a),
        (Outcome::returned(true), log_path_variant(), 64)
    );
    // row 04 id=9 size=100 buf=1 -> ret=1 err=- used=64 touched=1 type=1 str=path
    assert_eq!(
        property(9, 100, true, 0x5a5a_5a5a),
        (Outcome::returned(true), log_path_variant(), 64)
    );
    // row 05 id=9 size=63 buf=1 -> ret=0 err=122 used=64 touched=0
    assert_eq!(
        property(9, 63, true, 0x5a5a_5a5a),
        (Outcome::failed(false, 122), None, 64)
    );
    // row 06 id=9 size=0 buf=1 -> ret=0 err=122 used=64 touched=0
    assert_eq!(
        property(9, 0, true, 0x5a5a_5a5a),
        (Outcome::failed(false, 122), None, 64)
    );
    // row 07 id=9 size=64 buf=0 -> ret=0 err=122 used=64 touched=0
    assert_eq!(
        property(9, 64, false, 0x5a5a_5a5a),
        (Outcome::failed(false, 122), None, 64)
    );
    // row 08 id=9 size=0 buf=0 -> ret=0 err=122 used=64 touched=0
    assert_eq!(
        property(9, 0, false, 0x5a5a_5a5a),
        (Outcome::failed(false, 122), None, 64)
    );
    // row 09 id=0 size=64 buf=1 -> ret=0 err=120 used untouched
    assert_eq!(
        property(0, 64, true, 0x5a5a_5a5a),
        (Outcome::failed(false, 120), None, 0x5a5a_5a5a)
    );
    // row 10 id=8 size=64 buf=1 -> ret=0 err=120 used untouched
    assert_eq!(
        property(8, 64, true, 0x5a5a_5a5a),
        (Outcome::failed(false, 120), None, 0x5a5a_5a5a)
    );
    // row 11 id=21 size=64 buf=1 -> ret=0 err=120 used untouched
    assert_eq!(
        property(21, 64, true, 0x5a5a_5a5a),
        (Outcome::failed(false, 120), None, 0x5a5a_5a5a)
    );
    // row 12 id=9 size=65 buf=1 used_in=7 -> ret=1 err=- used=64 touched=1 type=1 str=path
    assert_eq!(
        property(9, 65, true, 7),
        (Outcome::returned(true), log_path_variant(), 64)
    );

    // row 13 EvtSetChannelConfigProperty -> ret=1 err=-
    assert!(evt_set_channel_config_property());
    // row 14 EvtSubscribe -> ret=(nil) err=-
    assert_eq!(evt_subscribe(), NULL_HANDLE);
    // row 15 EvtOpenChannelEnum -> ret=(nil) err=-
    assert_eq!(evt_open_channel_enum(), NULL_HANDLE);
    // row 16 EvtNextChannelPath -> ret=0 err=-
    assert!(!evt_next_channel_path());
    // row 17 EvtOpenChannelConfig -> ret=0xdeadbeef err=-
    assert_eq!(evt_open_channel_config(), 0xdead_beef);
    // row 18 EvtQuery -> ret=(nil) err=-
    assert_eq!(evt_query(), NULL_HANDLE);
    // row 19 EvtSaveChannelConfig -> ret=1 err=-
    assert!(evt_save_channel_config());
    // row 20 EvtClose -> ret=1 err=-
    assert!(evt_close());
    // row 21 EvtNext -> ret=0 err=120
    assert_eq!(evt_next(), Outcome::failed(false, 120));
    // row 22 EvtExportLog -> ret=0 err=120
    assert_eq!(evt_export_log(), Outcome::failed(false, 120));
    // row 23 EvtCreateBookmark -> ret=(nil) err=120
    assert_eq!(evt_create_bookmark(), Outcome::failed(NULL_HANDLE, 120));
    // row 24 EvtCreateRenderContext -> ret=(nil) err=120
    assert_eq!(
        evt_create_render_context(),
        Outcome::failed(NULL_HANDLE, 120)
    );
    // row 25 EvtRender -> ret=0 err=120
    assert_eq!(evt_render(), Outcome::failed(false, 120));
}

#[test]
fn oracle_stub_rows() {
    // rows 26 to 55: each `@ stub` export raises code=0x80000100 flags=0x1 nparams=2
    // module=wevtapi.dll function=<export name>.
    type StubFn = fn() -> StubException;
    let rows: [(&'static str, StubFn); 30] = [
        ("EvtArchiveExportedLog", evt_archive_exported_log),
        ("EvtCancel", evt_cancel),
        ("EvtClearLog", evt_clear_log),
        ("EvtFormatMessage", evt_format_message),
        ("EvtGetEventInfo", evt_get_event_info),
        (
            "EvtGetEventMetadataProperty",
            evt_get_event_metadata_property,
        ),
        ("EvtGetExtendedStatus", evt_get_extended_status),
        ("EvtGetLogInfo", evt_get_log_info),
        ("EvtGetObjectArrayProperty", evt_get_object_array_property),
        ("EvtGetObjectArraySize", evt_get_object_array_size),
        (
            "EvtGetPublisherMetadataProperty",
            evt_get_publisher_metadata_property,
        ),
        ("EvtGetQueryInfo", evt_get_query_info),
        ("EvtIntAssertConfig", evt_int_assert_config),
        (
            "EvtIntCreateBinXMLFromCustomXML",
            evt_int_create_bin_xml_from_custom_xml,
        ),
        ("EvtIntCreateLocalLogfile", evt_int_create_local_logfile),
        (
            "EvtIntGetClassicLogDisplayName",
            evt_int_get_classic_log_display_name,
        ),
        (
            "EvtIntRenderResourceEventTemplate",
            evt_int_render_resource_event_template,
        ),
        (
            "EvtIntReportAuthzEventAndSourceAsync",
            evt_int_report_authz_event_and_source_async,
        ),
        (
            "EvtIntReportEventAndSourceAsync",
            evt_int_report_event_and_source_async,
        ),
        ("EvtIntRetractConfig", evt_int_retract_config),
        ("EvtIntSysprepCleanup", evt_int_sysprep_cleanup),
        (
            "EvtIntWriteXmlEventToLocalLogfile",
            evt_int_write_xml_event_to_local_logfile,
        ),
        ("EvtNextEventMetadata", evt_next_event_metadata),
        ("EvtNextPublisherId", evt_next_publisher_id),
        ("EvtOpenEventMetadataEnum", evt_open_event_metadata_enum),
        ("EvtOpenPublisherEnum", evt_open_publisher_enum),
        ("EvtOpenPublisherMetadata", evt_open_publisher_metadata),
        ("EvtSeek", evt_seek),
        ("EvtSetObjectArrayProperty", evt_set_object_array_property),
        ("EvtUpdateBookmark", evt_update_bookmark),
    ];
    for (i, (name, stub)) in rows.into_iter().enumerate() {
        assert_eq!(
            stub(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "wevtapi.dll",
                function: name,
            },
            "row {}",
            26 + i
        );
    }
}
