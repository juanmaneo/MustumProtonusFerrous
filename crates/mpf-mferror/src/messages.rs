use crate::message_id::{
    make_message_id, FACILITY_MF, STATUS_SEVERITY_ERROR, STATUS_SEVERITY_SUCCESS,
};

/// One `MessageId` entry of `dlls/mferror/mferror.mc`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Message {
    /// The `MessageId`, which `wmc` combines with the severity and the facility.
    pub id: u32,
    /// The `Severity`: `STATUS_SEVERITY_ERROR` or `STATUS_SEVERITY_SUCCESS`.
    pub severity: u32,
    /// The `SymbolicName`.
    pub symbol: &'static str,
    /// The `Language=ENU` text.
    pub text: &'static str,
}

impl Message {
    /// The 32-bit value that `wmc` writes for this entry. `mferror.mc` gives every entry `Facility=MF`.
    pub const fn hresult(&self) -> u32 {
        make_message_id(self.severity, FACILITY_MF, self.id)
    }
}

/// Every `MessageId` entry of `mferror.mc`, in file order.
pub static MESSAGES: [Message; 101] = [
    Message {
        id: 14000,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PLATFORM_NOT_INITIALIZED",
        text: "Media Foundation platform is not initialized.",
    },
    Message {
        id: 14001,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_BUFFERTOOSMALL",
        text: "Buffer is too small.",
    },
    Message {
        id: 14002,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALIDREQUEST",
        text: "Invalid request.",
    },
    Message {
        id: 14003,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALIDSTREAMNUMBER",
        text: "Invalid stream number.",
    },
    Message {
        id: 14004,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALIDMEDIATYPE",
        text: "Invalid media type.",
    },
    Message {
        id: 14005,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NOTACCEPTING",
        text: "No more input is accepted.",
    },
    Message {
        id: 14006,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NOT_INITIALIZED",
        text: "Object is not initialized.",
    },
    Message {
        id: 14007,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_REPRESENTATION",
        text: "Representation is not supported.",
    },
    Message {
        id: 14009,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NO_MORE_TYPES",
        text: "No more types in the list of suggested media types.",
    },
    Message {
        id: 14010,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_SERVICE",
        text: "Unsupported service.",
    },
    Message {
        id: 14011,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNEXPECTED",
        text: "Unexpected error.",
    },
    Message {
        id: 14012,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALIDNAME",
        text: "Invalid name.",
    },
    Message {
        id: 14013,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALIDTYPE",
        text: "Invalid type.",
    },
    Message {
        id: 14014,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALID_FILE_FORMAT",
        text: "Invalid file format.",
    },
    Message {
        id: 14015,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALIDINDEX",
        text: "Invalid index.",
    },
    Message {
        id: 14016,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALID_TIMESTAMP",
        text: "Invalid timestamp.",
    },
    Message {
        id: 14019,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_SCHEME",
        text: "Unsupported scheme.",
    },
    Message {
        id: 14020,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_BYTESTREAM_TYPE",
        text: "Unsupported bytestream type.",
    },
    Message {
        id: 14021,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_TIME_FORMAT",
        text: "Unsupported time format.",
    },
    Message {
        id: 14024,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NO_SAMPLE_TIMESTAMP",
        text: "Timestamp is not set for the sample.",
    },
    Message {
        id: 14025,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NO_SAMPLE_DURATION",
        text: "No duration set for the sample.",
    },
    Message {
        id: 14027,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALID_STREAM_DATA",
        text: "Invalid stream data.",
    },
    Message {
        id: 14031,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_RT_UNAVAILABLE",
        text: "Realtime support is not available.",
    },
    Message {
        id: 14032,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_RATE",
        text: "Unsupported rate.",
    },
    Message {
        id: 14033,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_THINNING_UNSUPPORTED",
        text: "Unsupported thinning.",
    },
    Message {
        id: 14034,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_REVERSE_UNSUPPORTED",
        text: "Reversing is not supported.",
    },
    Message {
        id: 14035,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_RATE_TRANSITION",
        text: "Unsupported rate transition.",
    },
    Message {
        id: 14036,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_RATE_CHANGE_PREEMPTED",
        text: "Rate change was preempted.",
    },
    Message {
        id: 14037,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NOT_FOUND",
        text: "Object or value wasn't found.",
    },
    Message {
        id: 14038,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NOT_AVAILABLE",
        text: "Value is not available.",
    },
    Message {
        id: 14039,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NO_CLOCK",
        text: "Clock is not available.",
    },
    Message {
        id: 14040,
        severity: STATUS_SEVERITY_SUCCESS,
        symbol: "MF_S_MULTIPLE_BEGIN",
        text: "Begin request has already been made.",
    },
    Message {
        id: 14041,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_MULTIPLE_BEGIN",
        text: "Begin request has already been made.",
    },
    Message {
        id: 14042,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_MULTIPLE_SUBSCRIBERS",
        text: "Multiple subscribers are not supported.",
    },
    Message {
        id: 14043,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TIMER_ORPHANED",
        text: "The timer was orphaned.",
    },
    Message {
        id: 14044,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_STATE_TRANSITION_PENDING",
        text: "State transition is pending.",
    },
    Message {
        id: 14045,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_STATE_TRANSITION",
        text: "Unsupported state transition.",
    },
    Message {
        id: 14046,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNRECOVERABLE_ERROR_OCCURRED",
        text: "Unrecoverable error occurred.",
    },
    Message {
        id: 14047,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SAMPLE_HAS_TOO_MANY_BUFFERS",
        text: "Sample has too many buffers.",
    },
    Message {
        id: 14048,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SAMPLE_NOT_WRITABLE",
        text: "Sample is not writable.",
    },
    Message {
        id: 14050,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALID_KEY",
        text: "Key is invalid.",
    },
    Message {
        id: 14051,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_BAD_STARTUP_VERSION",
        text: "Bad startup version.",
    },
    Message {
        id: 14052,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_CAPTION",
        text: "Unsupported caption.",
    },
    Message {
        id: 14053,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALID_POSITION",
        text: "Invalid position.",
    },
    Message {
        id: 14054,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_ATTRIBUTENOTFOUND",
        text: "Attribute is not found.",
    },
    Message {
        id: 14055,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PROPERTY_TYPE_NOT_ALLOWED",
        text: "Property type is not allowed.",
    },
    Message {
        id: 14056,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PROPERTY_TYPE_NOT_SUPPORTED",
        text: "Property type is not supported.",
    },
    Message {
        id: 14057,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PROPERTY_EMPTY",
        text: "Property is empty.",
    },
    Message {
        id: 14058,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PROPERTY_NOT_EMPTY",
        text: "Property is not empty.",
    },
    Message {
        id: 14059,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PROPERTY_VECTOR_NOT_ALLOWED",
        text: "Vector property is not allowed.",
    },
    Message {
        id: 14060,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PROPERTY_VECTOR_REQUIRED",
        text: "Vector property is required.",
    },
    Message {
        id: 14061,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_OPERATION_CANCELLED",
        text: "Operation was canceled.",
    },
    Message {
        id: 14062,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_BYTESTREAM_NOT_SEEKABLE",
        text: "Bytestream is not seekable.",
    },
    Message {
        id: 14063,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_DISABLED_IN_SAFEMODE",
        text: "Platform is disabled in safe mode.",
    },
    Message {
        id: 14064,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_CANNOT_PARSE_BYTESTREAM",
        text: "Cannot parse bytestream.",
    },
    Message {
        id: 14065,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SOURCERESOLVER_MUTUALLY_EXCLUSIVE_FLAGS",
        text: "Mutually exclusive flags passed to source resolver.",
    },
    Message {
        id: 14075,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_BYTESTREAM_UNKNOWN_LENGTH",
        text: "Unknown bytestream length.",
    },
    Message {
        id: 14079,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALID_WORKQUEUE",
        text: "Invalid work queue index.",
    },
    Message {
        id: 16000,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NO_EVENTS_AVAILABLE",
        text: "No events available.",
    },
    Message {
        id: 16002,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALID_STATE_TRANSITION",
        text: "Invalid media source state transition.",
    },
    Message {
        id: 16004,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_END_OF_STREAM",
        text: "End of media stream has been reached.",
    },
    Message {
        id: 16005,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SHUTDOWN",
        text: "Shutdown() was called.",
    },
    Message {
        id: 16010,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_NO_DURATION",
        text: "Media stream has no duration set.",
    },
    Message {
        id: 16012,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_INVALID_FORMAT",
        text: "Media format was recognized but is invalid.",
    },
    Message {
        id: 16013,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PROPERTY_NOT_FOUND",
        text: "Property wasn't found.",
    },
    Message {
        id: 16014,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PROPERTY_READ_ONLY",
        text: "Property is read-only.",
    },
    Message {
        id: 16015,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_PROPERTY_NOT_ALLOWED",
        text: "Property is not allowed.",
    },
    Message {
        id: 16017,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_MEDIA_SOURCE_NOT_STARTED",
        text: "Media source is not started.",
    },
    Message {
        id: 16024,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_FORMAT",
        text: "Unsupported media format.",
    },
    Message {
        id: 16027,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_MEDIA_SOURCE_WRONGSTATE",
        text: "Media source is in wrong state.",
    },
    Message {
        id: 16028,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_MEDIA_SOURCE_NO_STREAMS_SELECTED",
        text: "No media streams were selected.",
    },
    Message {
        id: 16030,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_UNSUPPORTED_CHARACTERISTICS",
        text: "Unsupported media source characteristics.",
    },
    Message {
        id: 19000,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_STREAMSINK_REMOVED",
        text: "Stream sink was removed.",
    },
    Message {
        id: 19002,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_STREAMSINKS_OUT_OF_SYNC",
        text: "Stream sinks are out of sync.",
    },
    Message {
        id: 19003,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_STREAMSINKS_FIXED",
        text: "Media sink stream sinks set is fixed.",
    },
    Message {
        id: 19004,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_STREAMSINK_EXISTS",
        text: "Stream sink already exists.",
    },
    Message {
        id: 19005,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SAMPLEALLOCATOR_CANCELED",
        text: "Sample allocation was canceled.",
    },
    Message {
        id: 19006,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SAMPLEALLOCATOR_EMPTY",
        text: "Sample allocator is empty.",
    },
    Message {
        id: 19007,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SINK_ALREADYSTOPPED",
        text: "Sink was already stopped.",
    },
    Message {
        id: 19008,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_ASF_FILESINK_BITRATE_UNKNOWN",
        text: "Bitrate was unknown for ASF file sink.",
    },
    Message {
        id: 19009,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SINK_NO_STREAMS",
        text: "No streams were selected for the sink.",
    },
    Message {
        id: 19010,
        severity: STATUS_SEVERITY_SUCCESS,
        symbol: "MF_S_SINK_NOT_FINALIZED",
        text: "Sink has not been finalized.",
    },
    Message {
        id: 19011,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_METADATA_TOO_LONG",
        text: "Metadata was too long.",
    },
    Message {
        id: 19012,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SINK_NO_SAMPLES_PROCESSED",
        text: "No samples were processed by the sink.",
    },
    Message {
        id: 19013,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_SINK_HEADERS_NOT_FOUND",
        text: "Sink was not provided with required headers.",
    },
    Message {
        id: 21006,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_INVALID_OPTIONAL_NODE",
        text: "Optional node is invalid.",
    },
    Message {
        id: 21009,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_CANNOT_FIND_DECRYPTOR",
        text: "Cannot find decryptor.",
    },
    Message {
        id: 21010,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_CODEC_NOT_FOUND",
        text: "Codec was not found.",
    },
    Message {
        id: 21011,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_CANNOT_CONNECT",
        text: "Cannot connect topology nodes.",
    },
    Message {
        id: 21012,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_UNSUPPORTED",
        text: "Topology request is not supported.",
    },
    Message {
        id: 21013,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_INVALID_TIME_ATTRIBUTES",
        text: "Invalid topology time attributes.",
    },
    Message {
        id: 21014,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_LOOPS_IN_TOPOLOGY",
        text: "Found loops in topology.",
    },
    Message {
        id: 21015,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_MISSING_PRESENTATION_DESCRIPTOR",
        text: "Presentation descriptor is missing.",
    },
    Message {
        id: 21016,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_MISSING_STREAM_DESCRIPTOR",
        text: "Stream descriptor is missing.",
    },
    Message {
        id: 21017,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_STREAM_DESCRIPTOR_NOT_SELECTED",
        text: "Stream descriptor is not selected.",
    },
    Message {
        id: 21018,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_MISSING_SOURCE",
        text: "Source is missing.",
    },
    Message {
        id: 21019,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_TOPO_SINK_ACTIVATES_UNSUPPORTED",
        text: "Topology loader does not support sink activates.",
    },
    Message {
        id: 40001,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_CLOCK_NO_TIME_SOURCE",
        text: "Clock has no time source set.",
    },
    Message {
        id: 40002,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_CLOCK_STATE_ALREADY_SET",
        text: "Clock state was already set.",
    },
    Message {
        id: 40003,
        severity: STATUS_SEVERITY_ERROR,
        symbol: "MF_E_CLOCK_NOT_SIMPLE",
        text: "Clock is not simple",
    },
    Message {
        id: 40004,
        severity: STATUS_SEVERITY_SUCCESS,
        symbol: "MF_S_CLOCK_STOPPED",
        text: "Clock was stopped",
    },
];

/// The message whose `SymbolicName` is `symbol`. Names are case-sensitive.
pub fn find_by_symbol(symbol: &str) -> Option<&'static Message> {
    MESSAGES.iter().find(|message| message.symbol == symbol)
}

/// The message whose HRESULT is `hresult`.
pub fn find_by_hresult(hresult: u32) -> Option<&'static Message> {
    MESSAGES.iter().find(|message| message.hresult() == hresult)
}
