//! `process_output` of the DMO wrapper, from Wine's `dlls/qasf/dmowrapper.c`: what it does with
//! each output buffer the DMO has filled. The COM calls and the downstream `Receive` are left to
//! the caller, which delivers the planned samples in order and stops at the first failure.

use crate::SampleTime;

/// `DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT`.
pub const DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT: u32 = 0x1;
/// `DMO_OUTPUT_DATA_BUFFERF_TIME`.
pub const DMO_OUTPUT_DATA_BUFFERF_TIME: u32 = 0x2;
/// `DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH`.
pub const DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH: u32 = 0x4;
/// `DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE`.
pub const DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE: u32 = 0x0100_0000;

/// One output buffer after `ProcessOutput`: its status and time, and the length the DMO set on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputBuffer {
    pub status: u32,
    pub timestamp: i64,
    pub timelength: i64,
    pub len: u32,
}

/// What `process_output` does with one filled output sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputPlan {
    pub len: u32,
    /// The time to set, if the status has `TIME`. The stop is set only with `TIMELENGTH`.
    pub time: Option<SampleTime>,
    pub sync_point: bool,
    /// Only a non-empty sample goes downstream.
    pub deliver: bool,
    pub incomplete: bool,
}

/// One pass of `process_output` over its output buffers.
#[derive(Debug, PartialEq, Eq)]
pub struct OutputRound {
    /// Whether the DMO has more output to give, so the caller runs another pass.
    pub more_data: bool,
    /// One entry per output; `None` for an output with no buffer, which is skipped.
    pub plans: Vec<Option<OutputPlan>>,
}

/// Plans one pass over the output buffers. An incomplete buffer means another pass is needed.
pub fn plan_output(buffers: &[Option<OutputBuffer>]) -> OutputRound {
    let plans: Vec<Option<OutputPlan>> = buffers
        .iter()
        .map(|buffer| buffer.map(plan_buffer))
        .collect();
    let more_data = plans.iter().flatten().any(|plan| plan.incomplete);
    OutputRound { more_data, plans }
}

fn plan_buffer(buffer: OutputBuffer) -> OutputPlan {
    let time = (buffer.status & DMO_OUTPUT_DATA_BUFFERF_TIME != 0).then(|| {
        if buffer.status & DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH != 0 {
            SampleTime::span(buffer.timestamp, buffer.timelength)
        } else {
            SampleTime::at(buffer.timestamp)
        }
    });
    OutputPlan {
        len: buffer.len,
        time,
        sync_point: buffer.status & DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT != 0,
        deliver: buffer.len != 0,
        incomplete: buffer.status & DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE != 0,
    }
}
