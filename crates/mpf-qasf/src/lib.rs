//! Port of the deterministic logic of Wine's `dlls/qasf`: the buffer sizing of the ASF reader's
//! output pins (`allocator`), the stream count and names it opens with (`stream`), the times and
//! flags of the samples it delivers (`sample`), and the per-buffer planning of the DMO wrapper's
//! `process_output` (`output`).
//!
//! The COM interfaces, the critical sections, the media type copies, the `IWMReader` calls and the
//! `FIXME` stubs are not modelled. Sample times use 64-bit arithmetic, as Wine's `REFERENCE_TIME`
//! does; where Wine would overflow a signed sum, the port wraps.

pub mod allocator;
pub mod output;
pub mod sample;
pub mod stream;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A sample time as `IMediaSample::SetTime` takes it: a start, and a stop when one is given.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SampleTime {
    pub start: i64,
    pub stop: Option<i64>,
}

impl SampleTime {
    /// A start with no stop, as `SetTime` takes it with a null stop.
    pub fn at(start: i64) -> Self {
        Self { start, stop: None }
    }

    /// A start and the stop `length` after it.
    pub fn span(start: i64, length: i64) -> Self {
        Self {
            start,
            stop: Some(start.wrapping_add(length)),
        }
    }
}
