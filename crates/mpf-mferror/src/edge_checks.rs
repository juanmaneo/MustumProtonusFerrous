//! Edge-case tests for the ported mferror message table.
//!
//! These cover the constants, the composition of the message ID, the lookups and the table
//! invariants. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

/// `(SymbolicName, HRESULT)` for all 101 entries, from Wine's `include/mferror.h`, which `wmc`
/// generates from `dlls/mferror/mferror.mc`. Not oracle-derived from the sanitized harness: it
/// covers the 98 entries that UBSan flags there, as well as the 3 that it does not.
const WINE_HEADER_VALUES: [(&str, u32); 101] = [
    ("MF_E_PLATFORM_NOT_INITIALIZED", 0xC00D_36B0),
    ("MF_E_BUFFERTOOSMALL", 0xC00D_36B1),
    ("MF_E_INVALIDREQUEST", 0xC00D_36B2),
    ("MF_E_INVALIDSTREAMNUMBER", 0xC00D_36B3),
    ("MF_E_INVALIDMEDIATYPE", 0xC00D_36B4),
    ("MF_E_NOTACCEPTING", 0xC00D_36B5),
    ("MF_E_NOT_INITIALIZED", 0xC00D_36B6),
    ("MF_E_UNSUPPORTED_REPRESENTATION", 0xC00D_36B7),
    ("MF_E_NO_MORE_TYPES", 0xC00D_36B9),
    ("MF_E_UNSUPPORTED_SERVICE", 0xC00D_36BA),
    ("MF_E_UNEXPECTED", 0xC00D_36BB),
    ("MF_E_INVALIDNAME", 0xC00D_36BC),
    ("MF_E_INVALIDTYPE", 0xC00D_36BD),
    ("MF_E_INVALID_FILE_FORMAT", 0xC00D_36BE),
    ("MF_E_INVALIDINDEX", 0xC00D_36BF),
    ("MF_E_INVALID_TIMESTAMP", 0xC00D_36C0),
    ("MF_E_UNSUPPORTED_SCHEME", 0xC00D_36C3),
    ("MF_E_UNSUPPORTED_BYTESTREAM_TYPE", 0xC00D_36C4),
    ("MF_E_UNSUPPORTED_TIME_FORMAT", 0xC00D_36C5),
    ("MF_E_NO_SAMPLE_TIMESTAMP", 0xC00D_36C8),
    ("MF_E_NO_SAMPLE_DURATION", 0xC00D_36C9),
    ("MF_E_INVALID_STREAM_DATA", 0xC00D_36CB),
    ("MF_E_RT_UNAVAILABLE", 0xC00D_36CF),
    ("MF_E_UNSUPPORTED_RATE", 0xC00D_36D0),
    ("MF_E_THINNING_UNSUPPORTED", 0xC00D_36D1),
    ("MF_E_REVERSE_UNSUPPORTED", 0xC00D_36D2),
    ("MF_E_UNSUPPORTED_RATE_TRANSITION", 0xC00D_36D3),
    ("MF_E_RATE_CHANGE_PREEMPTED", 0xC00D_36D4),
    ("MF_E_NOT_FOUND", 0xC00D_36D5),
    ("MF_E_NOT_AVAILABLE", 0xC00D_36D6),
    ("MF_E_NO_CLOCK", 0xC00D_36D7),
    ("MF_S_MULTIPLE_BEGIN", 0x000D_36D8),
    ("MF_E_MULTIPLE_BEGIN", 0xC00D_36D9),
    ("MF_E_MULTIPLE_SUBSCRIBERS", 0xC00D_36DA),
    ("MF_E_TIMER_ORPHANED", 0xC00D_36DB),
    ("MF_E_STATE_TRANSITION_PENDING", 0xC00D_36DC),
    ("MF_E_UNSUPPORTED_STATE_TRANSITION", 0xC00D_36DD),
    ("MF_E_UNRECOVERABLE_ERROR_OCCURRED", 0xC00D_36DE),
    ("MF_E_SAMPLE_HAS_TOO_MANY_BUFFERS", 0xC00D_36DF),
    ("MF_E_SAMPLE_NOT_WRITABLE", 0xC00D_36E0),
    ("MF_E_INVALID_KEY", 0xC00D_36E2),
    ("MF_E_BAD_STARTUP_VERSION", 0xC00D_36E3),
    ("MF_E_UNSUPPORTED_CAPTION", 0xC00D_36E4),
    ("MF_E_INVALID_POSITION", 0xC00D_36E5),
    ("MF_E_ATTRIBUTENOTFOUND", 0xC00D_36E6),
    ("MF_E_PROPERTY_TYPE_NOT_ALLOWED", 0xC00D_36E7),
    ("MF_E_PROPERTY_TYPE_NOT_SUPPORTED", 0xC00D_36E8),
    ("MF_E_PROPERTY_EMPTY", 0xC00D_36E9),
    ("MF_E_PROPERTY_NOT_EMPTY", 0xC00D_36EA),
    ("MF_E_PROPERTY_VECTOR_NOT_ALLOWED", 0xC00D_36EB),
    ("MF_E_PROPERTY_VECTOR_REQUIRED", 0xC00D_36EC),
    ("MF_E_OPERATION_CANCELLED", 0xC00D_36ED),
    ("MF_E_BYTESTREAM_NOT_SEEKABLE", 0xC00D_36EE),
    ("MF_E_DISABLED_IN_SAFEMODE", 0xC00D_36EF),
    ("MF_E_CANNOT_PARSE_BYTESTREAM", 0xC00D_36F0),
    ("MF_E_SOURCERESOLVER_MUTUALLY_EXCLUSIVE_FLAGS", 0xC00D_36F1),
    ("MF_E_BYTESTREAM_UNKNOWN_LENGTH", 0xC00D_36FB),
    ("MF_E_INVALID_WORKQUEUE", 0xC00D_36FF),
    ("MF_E_NO_EVENTS_AVAILABLE", 0xC00D_3E80),
    ("MF_E_INVALID_STATE_TRANSITION", 0xC00D_3E82),
    ("MF_E_END_OF_STREAM", 0xC00D_3E84),
    ("MF_E_SHUTDOWN", 0xC00D_3E85),
    ("MF_E_NO_DURATION", 0xC00D_3E8A),
    ("MF_E_INVALID_FORMAT", 0xC00D_3E8C),
    ("MF_E_PROPERTY_NOT_FOUND", 0xC00D_3E8D),
    ("MF_E_PROPERTY_READ_ONLY", 0xC00D_3E8E),
    ("MF_E_PROPERTY_NOT_ALLOWED", 0xC00D_3E8F),
    ("MF_E_MEDIA_SOURCE_NOT_STARTED", 0xC00D_3E91),
    ("MF_E_UNSUPPORTED_FORMAT", 0xC00D_3E98),
    ("MF_E_MEDIA_SOURCE_WRONGSTATE", 0xC00D_3E9B),
    ("MF_E_MEDIA_SOURCE_NO_STREAMS_SELECTED", 0xC00D_3E9C),
    ("MF_E_UNSUPPORTED_CHARACTERISTICS", 0xC00D_3E9E),
    ("MF_E_STREAMSINK_REMOVED", 0xC00D_4A38),
    ("MF_E_STREAMSINKS_OUT_OF_SYNC", 0xC00D_4A3A),
    ("MF_E_STREAMSINKS_FIXED", 0xC00D_4A3B),
    ("MF_E_STREAMSINK_EXISTS", 0xC00D_4A3C),
    ("MF_E_SAMPLEALLOCATOR_CANCELED", 0xC00D_4A3D),
    ("MF_E_SAMPLEALLOCATOR_EMPTY", 0xC00D_4A3E),
    ("MF_E_SINK_ALREADYSTOPPED", 0xC00D_4A3F),
    ("MF_E_ASF_FILESINK_BITRATE_UNKNOWN", 0xC00D_4A40),
    ("MF_E_SINK_NO_STREAMS", 0xC00D_4A41),
    ("MF_S_SINK_NOT_FINALIZED", 0x000D_4A42),
    ("MF_E_METADATA_TOO_LONG", 0xC00D_4A43),
    ("MF_E_SINK_NO_SAMPLES_PROCESSED", 0xC00D_4A44),
    ("MF_E_SINK_HEADERS_NOT_FOUND", 0xC00D_4A45),
    ("MF_E_TOPO_INVALID_OPTIONAL_NODE", 0xC00D_520E),
    ("MF_E_TOPO_CANNOT_FIND_DECRYPTOR", 0xC00D_5211),
    ("MF_E_TOPO_CODEC_NOT_FOUND", 0xC00D_5212),
    ("MF_E_TOPO_CANNOT_CONNECT", 0xC00D_5213),
    ("MF_E_TOPO_UNSUPPORTED", 0xC00D_5214),
    ("MF_E_TOPO_INVALID_TIME_ATTRIBUTES", 0xC00D_5215),
    ("MF_E_TOPO_LOOPS_IN_TOPOLOGY", 0xC00D_5216),
    ("MF_E_TOPO_MISSING_PRESENTATION_DESCRIPTOR", 0xC00D_5217),
    ("MF_E_TOPO_MISSING_STREAM_DESCRIPTOR", 0xC00D_5218),
    ("MF_E_TOPO_STREAM_DESCRIPTOR_NOT_SELECTED", 0xC00D_5219),
    ("MF_E_TOPO_MISSING_SOURCE", 0xC00D_521A),
    ("MF_E_TOPO_SINK_ACTIVATES_UNSUPPORTED", 0xC00D_521B),
    ("MF_E_CLOCK_NO_TIME_SOURCE", 0xC00D_9C41),
    ("MF_E_CLOCK_STATE_ALREADY_SET", 0xC00D_9C42),
    ("MF_E_CLOCK_NOT_SIMPLE", 0xC00D_9C43),
    ("MF_S_CLOCK_STOPPED", 0x000D_9C44),
];

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/ntstatus.h, and FACILITY_MF from mferror.mc.
    assert_eq!(STATUS_SEVERITY_ERROR, 0x3);
    assert_eq!(STATUS_SEVERITY_SUCCESS, 0x0);
    assert_eq!(FACILITY_MF, 0xd);
}

#[test]
fn make_message_id_places_each_field() {
    // Not oracle-derived: the expression is copied from complete_msg, and these values are worked out by hand.
    assert_eq!(make_message_id(0, 0, 0), 0);
    assert_eq!(make_message_id(0, 0, 0xffff), 0xffff);
    assert_eq!(make_message_id(STATUS_SEVERITY_ERROR, 0, 0), 0xc000_0000);
    assert_eq!(
        make_message_id(STATUS_SEVERITY_SUCCESS, FACILITY_MF, 0),
        0x000d_0000
    );
    assert_eq!(make_message_id(0, 0, 0x1_0000), 0x1_0000);
}

#[test]
fn make_message_id_truncates_fields_past_their_width() {
    // Not oracle-derived: these inputs are outside the mferror.mc range. The C expression is
    // undefined for them, so the port keeps the low 32 bits.
    assert_eq!(make_message_id(4, 0, 0), 0);
    assert_eq!(make_message_id(0, 0x1_0000, 0), 0);
}

#[test]
fn find_by_symbol_is_case_sensitive_and_total() {
    // Not oracle-derived: lookup behaviour is defined by this port.
    assert_eq!(find_by_symbol("MF_E_UNEXPECTED").unwrap().id, 14011);
    assert!(find_by_symbol("mf_e_unexpected").is_none());
    assert!(find_by_symbol("").is_none());
    assert!(find_by_symbol("MF_E_NOT_A_MESSAGE").is_none());
}

#[test]
fn find_by_hresult_matches_only_table_values() {
    // Not oracle-derived: lookup behaviour is defined by this port. 0xC00D_36B8 is MessageId
    // 14008, which mferror.mc does not define.
    assert_eq!(
        find_by_hresult(0xc00d_36b0).unwrap().symbol,
        "MF_E_PLATFORM_NOT_INITIALIZED"
    );
    assert_eq!(
        find_by_hresult(0x000d_36d8).unwrap().symbol,
        "MF_S_MULTIPLE_BEGIN"
    );
    assert!(find_by_hresult(0).is_none());
    assert!(find_by_hresult(0xc00d_36b8).is_none());
}

#[test]
fn table_has_the_mc_entries_in_file_order() {
    // Not oracle-derived: 101 is `grep -c '^MessageId=' mferror.mc`, and the first and last symbols are the file's.
    assert_eq!(MESSAGES.len(), 101);
    assert_eq!(MESSAGES[0].symbol, "MF_E_PLATFORM_NOT_INITIALIZED");
    assert_eq!(MESSAGES[100].symbol, "MF_S_CLOCK_STOPPED");
}

#[test]
fn table_hresults_match_wine_header() {
    assert_eq!(WINE_HEADER_VALUES.len(), MESSAGES.len());
    for (message, (symbol, hresult)) in MESSAGES.iter().zip(WINE_HEADER_VALUES) {
        assert_eq!(message.symbol, symbol);
        assert_eq!(message.hresult(), hresult);
    }
}

#[test]
fn table_fields_are_consistent() {
    // Not oracle-derived: bit-field layout from the wmc formula in tools/wmc/mcy.y.
    for message in MESSAGES.iter() {
        assert_eq!(message.hresult() >> 30, message.severity);
        assert_eq!((message.hresult() >> 16) & 0xfff, FACILITY_MF);
        assert_eq!(message.hresult() & 0xffff, message.id);
        assert!(!message.text.is_empty());
    }
}

#[test]
fn table_symbols_and_hresults_are_unique() {
    // Not oracle-derived: a property of the table as transcribed from mferror.mc.
    for (i, a) in MESSAGES.iter().enumerate() {
        for b in &MESSAGES[i + 1..] {
            assert_ne!(a.symbol, b.symbol);
            assert_ne!(a.hresult(), b.hresult());
        }
    }
}

#[test]
fn success_entries_are_the_three_mc_entries() {
    // Not oracle-derived: the three `Severity=Success` entries of mferror.mc.
    let success: Vec<&str> = MESSAGES
        .iter()
        .filter(|message| message.severity == STATUS_SEVERITY_SUCCESS)
        .map(|message| message.symbol)
        .collect();
    assert_eq!(
        success,
        [
            "MF_S_MULTIPLE_BEGIN",
            "MF_S_SINK_NOT_FINALIZED",
            "MF_S_CLOCK_STOPPED"
        ]
    );
}
