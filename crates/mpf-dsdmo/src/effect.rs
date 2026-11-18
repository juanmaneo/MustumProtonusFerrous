//! The format state of a `struct effect` and the two setters that read and write it:
//! `effect_SetInputType` and `effect_SetOutputType`. The critical section around the stored
//! format is not modelled.

use crate::format::WaveFormat;
use crate::media::{check_input_type, check_output_type, MediaType};
use crate::Error;

/// `DMO_SET_TYPEF_CLEAR`. `DMO_SET_TYPEF_TEST_ONLY` is not handled by Wine's setters, so it does
/// not change their result.
pub const DMO_SET_TYPEF_CLEAR: u32 = 0x0000_0002;

/// The `format` field of an effect. A new effect holds a zeroed header, as `calloc` leaves it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TypeState {
    format: WaveFormat,
}

impl TypeState {
    /// The stored header.
    pub fn format(&self) -> WaveFormat {
        self.format
    }

    /// `effect_SetInputType`. With the clear flag it zeroes the stored header and ignores `media`.
    /// Otherwise it stores the header of an accepted type and keeps the old one on refusal.
    pub fn set_input_type(&mut self, flags: u32, media: &MediaType) -> Result<(), Error> {
        if flags & DMO_SET_TYPEF_CLEAR != 0 {
            self.format = WaveFormat::default();
            return Ok(());
        }
        self.format = check_input_type(media)?;
        Ok(())
    }

    /// `effect_SetOutputType`. With the clear flag it accepts without looking at `media`.
    /// Otherwise it accepts a type only when its header equals the stored one.
    pub fn set_output_type(&self, flags: u32, media: &MediaType) -> Result<(), Error> {
        if flags & DMO_SET_TYPEF_CLEAR != 0 {
            return Ok(());
        }
        check_output_type(media, self.format)
    }
}
