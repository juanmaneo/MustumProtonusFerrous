//! The sample thinning of media_source.c: media_stream_accepts_sample, media_stream_set_thin and media_stream_send_sample.

use crate::HResult;

/// An event that the C queues on the stream's event queue.
#[derive(Debug, PartialEq, Eq)]
pub enum StreamEvent {
    /// MEStreamThinMode, with the new thin flag as the value.
    ThinMode(i32),
    /// MEMediaSample.
    Sample,
}

/// The thinning state of one media stream.
pub struct StreamThinning {
    pub thin: bool,
    pub pending_thin: bool,
}

impl StreamThinning {
    /// media_stream_accepts_sample. `clean_point` is the MFSampleExtension_CleanPoint
    /// value, or None when the sample does not carry it.
    ///
    /// While thinning, only a clean-point sample is accepted. A sample without the
    /// attribute is rejected.
    pub fn accepts_sample(&self, thin: bool, clean_point: Option<u32>) -> bool {
        let thinning = thin || self.thin;
        match clean_point {
            Some(keyframe) if thinning => keyframe != 0,
            _ => !thinning,
        }
    }

    /// media_stream_set_thin. Returns the MEStreamThinMode event when the flag changes.
    pub fn set_thin(&mut self, thin: bool) -> Option<StreamEvent> {
        if self.thin == thin {
            return None;
        }
        self.thin = thin;
        Some(StreamEvent::ThinMode(i32::from(thin)))
    }

    /// media_stream_send_sample. `token` is None when the sample has no token,
    /// and otherwise the result of setting the token on the sample.
    ///
    /// A failed token leaves the stream unchanged and returns the error. The C
    /// reports that failure only through ERR, so the port returns it as the error.
    /// Thinning stays on until a sample is sent, unless a pending thin change is set.
    pub fn send_sample(
        &mut self,
        token: Option<Result<(), HResult>>,
    ) -> Result<Vec<StreamEvent>, HResult> {
        if let Some(Err(hr)) = token {
            return Err(hr);
        }
        let mut events = Vec::new();
        if !self.pending_thin {
            events.extend(self.set_thin(false));
        }
        events.push(StreamEvent::Sample);
        Ok(events)
    }
}
