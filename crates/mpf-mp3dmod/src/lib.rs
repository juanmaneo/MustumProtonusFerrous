//! Port of the deterministic logic of Wine's `dlls/mp3dmod/mp3dmod.c`: the MP3 decoder DMO's
//! media type checks, size queries and frame arithmetic, and the Media Foundation output type list
//! and DMO-to-MF error mapping.
//!
//! Ported: [`types::set_input_type`], [`types::output_type`] and [`types::set_output_type`];
//! [`frame::input_size_info`], [`frame::output_size_info`], [`frame::framesize`] and
//! [`frame::frametime`]; [`mf::output_available_type`]; and [`hresult::convert_dmo_to_mf_error`].
//!
//! Skipped: COM plumbing (IUnknown, IMediaObject and IMFTransform entry points, object creation),
//! mpg123 calls (the caller runs `mpg123_format` with the returned [`types::Encoding`]), sample
//! buffers and decoding, flush, streaming, status and latency queries, MFTransform attributes,
//! streams and messages, and the `CoTaskMemAlloc` failure path. The accepted types are held by the
//! caller and passed in as `Option`.
//!
//! Deviations: `DMO_SET_TYPEF_CLEAR` and `DMO_SET_TYPEF_TEST_ONLY` are not modelled because the
//! caller decides whether to store a type. [`frame::frametime`] returns `None` for a sample rate of
//! zero, where Wine divides by zero.

pub mod frame;
pub mod hresult;
pub mod mf;
pub mod types;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// An `HRESULT`. `0` is `S_OK`.
pub type Hresult = u32;

fn check_stream(stream: u32) -> Result<(), Hresult> {
    if stream == 0 {
        Ok(())
    } else {
        Err(hresult::DMO_E_INVALIDSTREAMINDEX)
    }
}
