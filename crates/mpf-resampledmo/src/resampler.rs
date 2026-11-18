//! The stream state of Wine's `resampler.c`: `media_object_SetInputType` and `SetOutputType`,
//! `resampler_init`, `resampler_process_input`, and `resampler_process_output` with
//! `resampler_process_frame`. Wine's libswresample context is the [`Converter`].

use crate::format::AudioSpec;
use crate::media::{check_media_type, MediaType};
use crate::time::{self, NO_PTS};
use crate::Error;

/// `DMO_SET_TYPEF_TEST_ONLY`.
pub const SET_TYPE_TEST_ONLY: u32 = 0x1;
/// `DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT`.
pub const OUTPUT_SYNCPOINT: u32 = 0x1;
/// `DMO_OUTPUT_DATA_BUFFERF_TIME`.
pub const OUTPUT_TIME: u32 = 0x2;
/// `DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH`.
pub const OUTPUT_TIMELENGTH: u32 = 0x4;
/// `DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE`.
pub const OUTPUT_INCOMPLETE: u32 = 0x0100_0000;

/// The success results of `ProcessOutput`: `S_OK` once a buffer is filled, `S_FALSE` when the
/// converter needs more input first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Ready,
    NeedInput,
}

impl Status {
    /// The `HRESULT` Wine returns for this result.
    pub fn hresult(self) -> u32 {
        match self {
            Status::Ready => 0,
            Status::NeedInput => 1,
        }
    }
}

/// The sample converter behind a stream: `swr_init`, `swr_next_pts`, `swr_convert` and
/// `swr_get_out_samples` in Wine.
pub trait Converter: Sized {
    /// Builds a converter from the input and output layouts, or `None` when `swr_init` fails.
    fn new(input: &AudioSpec, output: &AudioSpec) -> Option<Self>;
    /// `swr_next_pts`: the output timestamp of the next sample for input timestamp `pts`.
    fn next_pts(&mut self, pts: i64) -> i64;
    /// `swr_convert`: converts `in_samples` frames of `input` into at most `out_samples` frames
    /// of `out`, returning the frames written, or a negative value on failure.
    fn convert(
        &mut self,
        out: &mut [u8],
        out_samples: usize,
        input: &[u8],
        in_samples: usize,
    ) -> i32;
    /// `swr_get_out_samples` with no input: the frames still buffered in the converter.
    fn pending(&self) -> i32;
}

/// The part of a `DMO_OUTPUT_DATA_BUFFER` that `ProcessOutput` fills in: the length the media
/// buffer was set to, the status flags and the timestamps.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OutputData {
    pub length: usize,
    pub status: u32,
    pub timestamp: i64,
    pub timelength: i64,
}

/// A resampler's types and, once both are set, its stream.
pub struct Resampler<C> {
    input_type: Option<MediaType>,
    output_type: Option<MediaType>,
    stream: Option<Stream<C>>,
}

impl<C> Default for Resampler<C> {
    fn default() -> Self {
        Self {
            input_type: None,
            output_type: None,
            stream: None,
        }
    }
}

impl<C: Converter> Resampler<C> {
    /// `media_object_SetInputType`. `None` clears the type. Otherwise the type is checked and, unless
    /// `flags` has [`SET_TYPE_TEST_ONLY`], stored. The stream is rebuilt when the output type is set
    /// too, and a type that cannot build one is cleared again.
    pub fn set_input_type(
        &mut self,
        index: u32,
        media: Option<&MediaType>,
        flags: u32,
    ) -> Result<(), Error> {
        if index != 0 {
            return Err(Error::InvalidStreamIndex);
        }
        let Some(media) = media else {
            self.input_type = None;
            self.stream = None;
            return Ok(());
        };
        check_media_type(media)?;
        if flags & SET_TYPE_TEST_ONLY != 0 {
            return Ok(());
        }
        self.input_type = Some(media.clone());
        self.renegotiate().inspect_err(|_| self.input_type = None)
    }

    /// `media_object_SetOutputType`, which needs the input type first.
    pub fn set_output_type(
        &mut self,
        index: u32,
        media: Option<&MediaType>,
        flags: u32,
    ) -> Result<(), Error> {
        if index != 0 {
            return Err(Error::InvalidStreamIndex);
        }
        let Some(media) = media else {
            self.output_type = None;
            self.stream = None;
            return Ok(());
        };
        if self.input_type.is_none() {
            return Err(Error::TypeNotSet);
        }
        check_media_type(media)?;
        if flags & SET_TYPE_TEST_ONLY != 0 {
            return Ok(());
        }
        self.output_type = Some(media.clone());
        self.renegotiate().inspect_err(|_| self.output_type = None)
    }

    /// `ProcessInput` with one buffer `data` stamped `pts`. The frames of `data` are kept, so the
    /// caller's buffer is not referenced after the call.
    pub fn process_input(&mut self, data: &[u8], pts: i64) -> Result<(), Error> {
        self.stream
            .as_mut()
            .ok_or(Error::TypeNotSet)?
            .accept(data, pts)
    }

    /// `ProcessOutput` for one output buffer, which fills `buffer` and `output`. The pending input is
    /// consumed whether or not the conversion succeeds.
    pub fn process_output(
        &mut self,
        count: u32,
        buffer: &mut [u8],
        output: &mut OutputData,
    ) -> Result<Status, Error> {
        if count != 1 {
            return Err(Error::InvalidArg);
        }
        let stream = self.stream.as_mut().ok_or(Error::TypeNotSet)?;
        let result = stream.produce(buffer, output);
        stream.current = Frame::empty();
        result
    }

    /// `resampler_init`: rebuilds the stream from both types, or leaves it unset until both are set.
    fn renegotiate(&mut self) -> Result<(), Error> {
        self.stream = None;
        let (Some(input), Some(output)) = (&self.input_type, &self.output_type) else {
            return Ok(());
        };
        let input = AudioSpec::from_format(&input.format).ok_or(Error::Fail)?;
        let output = AudioSpec::from_format(&output.format).ok_or(Error::Fail)?;
        let converter = C::new(&input, &output).ok_or(Error::Fail)?;
        self.stream = Some(Stream::new(input, output, converter));
        Ok(())
    }
}

/// The input frame waiting for conversion, `current_frame` in Wine: `samples` frames of `data`
/// stamped `pts` in stream time.
struct Frame {
    data: Vec<u8>,
    samples: usize,
    pts: i64,
}

impl Frame {
    fn empty() -> Self {
        Self {
            data: Vec::new(),
            samples: 0,
            pts: NO_PTS,
        }
    }
}

/// A live stream: both layouts, the converter and the pending input frame.
struct Stream<C> {
    input: AudioSpec,
    output: AudioSpec,
    time_den: i64,
    converter: C,
    current: Frame,
}

impl<C: Converter> Stream<C> {
    fn new(input: AudioSpec, output: AudioSpec, converter: C) -> Self {
        let time_den = i64::from(input.sample_rate) * i64::from(output.sample_rate);
        Self {
            input,
            output,
            time_den,
            converter,
            current: Frame::empty(),
        }
    }

    /// Stores a whole-frame copy of `data` as the pending input. Input is refused while earlier
    /// input is unconverted, and a buffer whose block align is too small for its channels is
    /// refused too, since Wine would read past it.
    fn accept(&mut self, data: &[u8], pts: i64) -> Result<(), Error> {
        if self.current.samples != 0 {
            return Err(Error::NotAccepting);
        }
        let (samples, size) = self.input.wrap(data.len()).ok_or(Error::Fail)?;
        self.current = Frame {
            data: data[..size].to_vec(),
            samples,
            pts: time::from_user(self.time_den, pts),
        };
        Ok(())
    }

    /// `resampler_process_frame`: converts the pending frame into `buffer`. Timestamps are
    /// written only when frames are produced.
    fn produce(&mut self, buffer: &mut [u8], output: &mut OutputData) -> Result<Status, Error> {
        output.length = 0;
        output.status = 0;
        let pts = self.converter.next_pts(self.current.pts);
        let (samples, length) = self.convert(buffer)?;
        if samples == 0 {
            return Ok(Status::NeedInput);
        }
        output.length = length;
        output.status = OUTPUT_SYNCPOINT;
        if self.converter.pending() != 0 {
            output.status |= OUTPUT_INCOMPLETE;
        }
        output.timestamp = time::to_user(self.time_den, pts);
        if output.timestamp != NO_PTS {
            output.status |= OUTPUT_TIME;
        }
        output.timelength = time::duration(samples as i64, i64::from(self.output.sample_rate));
        output.status |= OUTPUT_TIMELENGTH;
        Ok(Status::Ready)
    }

    /// Converts the pending frame into `buffer`, returning the frames produced and the bytes they
    /// fill. When whole output frames fit, the converter writes into the buffer. Otherwise it
    /// writes into a staging buffer of one rate's worth of frames, and the result is copied out.
    fn convert(&mut self, buffer: &mut [u8]) -> Result<(usize, usize), Error> {
        let block_align = self.output.block_align as usize;
        match self.output.wrap(buffer.len()) {
            Some((count, size)) => {
                let ret = self.converter.convert(
                    &mut buffer[..size],
                    count,
                    &self.current.data,
                    self.current.samples,
                );
                let produced = frames_produced(ret, count)?;
                Ok((produced, produced * block_align))
            }
            None => {
                let rate = self.output.sample_rate as usize;
                let count = (buffer.len() / block_align).min(rate);
                let mut staging = vec![0; self.output.frame_bytes(rate)];
                let ret = self.converter.convert(
                    &mut staging,
                    count,
                    &self.current.data,
                    self.current.samples,
                );
                let produced = frames_produced(ret, count)?;
                let length = self.output.frame_bytes(produced);
                if length > buffer.len() {
                    return Err(Error::Fail);
                }
                buffer[..length].copy_from_slice(&staging[..length]);
                Ok((produced, length))
            }
        }
    }
}

/// The frame count `swr_convert` returned. A negative count is a failure, and so is one beyond
/// the room the converter was given.
fn frames_produced(ret: i32, room: usize) -> Result<usize, Error> {
    match usize::try_from(ret) {
        Ok(produced) if produced <= room => Ok(produced),
        _ => Err(Error::Fail),
    }
}
