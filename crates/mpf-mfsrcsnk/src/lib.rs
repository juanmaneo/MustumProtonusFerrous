//! Headless subset of Wine's dlls/mfsrcsnk, the Media Foundation source and sink module.
//!
//! The ported functions compute what Wine's C bodies compute from their
//! arguments. The COM objects, byte streams, sample buffers, event queues,
//! async results and work items are not modelled. [wave::WaveSink] keeps its
//! stream as an in-memory buffer with a position, and the rate and thinning
//! functions return their results instead of queueing events.
//!
//! Ported:
//! - The WAVE sink byte image in wave.c: the RIFF header, the JUNK pad, the
//!   data chunk, the finalize patch and the ProcessSample and Shutdown state
//!   checks: [wave::WaveSink].
//! - The derived nAvgBytesPerSec and nBlockAlign fields of MFCreateWAVEMediaSink:
//!   [wave::WaveFormatEx::with_derived_fields].
//! - GetSlowestRate, GetFastestRate, IsRateSupported and the SetRate checks in
//!   media_source.c: [rate].
//! - media_stream_accepts_sample, media_stream_set_thin and
//!   media_stream_send_sample in media_source.c: [thin].
//!
//! Skipped:
//! - Every COM vtable, reference count, critical section and clock hook.
//! - The IMFByteStream write errors and the IMFMediaBuffer lock path of ProcessSample.
//! - The async part of BeginFinalize: the callback is not run. The result status is returned.
//! - The async parameters and work item of SetRate: an Ok result means the caller queues the change.
//! - The thin argument of SetRate, which Wine forwards to those async parameters and not to the return value.
//! - MFCreateWaveFormatExFromMFMediaType and the event queue setup of MFCreateWAVEMediaSink.
//! - ERR, WARN, FIXME and TRACE output.
//!
//! Divergence: the C nBlockAlign computation is int arithmetic, which is
//! undefined behaviour on overflow. The port wraps as gcc does in practice.
//! The oracle excludes those vectors, and an edge test pins the wrapped value.

pub mod rate;
pub mod thin;
pub mod wave;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// HRESULT, as returned by the Media Foundation calls.
pub type HResult = u32;

/// S_OK.
pub const S_OK: HResult = 0;
/// E_INVALIDARG.
pub const E_INVALIDARG: HResult = 0x8007_0057;
/// E_FAIL.
pub const E_FAIL: HResult = 0x8000_4005;
/// MF_E_SHUTDOWN.
pub const MF_E_SHUTDOWN: HResult = 0xC00D_3E85;
/// MF_E_STREAMSINK_REMOVED.
pub const MF_E_STREAMSINK_REMOVED: HResult = 0xC00D_4A38;
/// MF_E_UNSUPPORTED_RATE.
pub const MF_E_UNSUPPORTED_RATE: HResult = 0xC00D_36D0;
/// MF_E_REVERSE_UNSUPPORTED.
pub const MF_E_REVERSE_UNSUPPORTED: HResult = 0xC00D_36D2;
