//! `ACMDM_FORMATTAG_DETAILS` for the MPEG Layer-3 codec. It offers three format
//! tags, PCM, MPEG Layer-3 and MPEG-1, in that index order.

use crate::format::PCM_FORMATS;
use crate::{
    word_tag, Error, ACM_DRIVERDETAILS_SUPPORTF_CODEC, MPEG1WAVEFORMAT_SIZE,
    MPEGLAYER3WAVEFORMAT_SIZE, PCMWAVEFORMAT_SIZE, WAVE_FORMAT_MPEG, WAVE_FORMAT_MPEGLAYER3,
    WAVE_FORMAT_PCM, WAVE_FORMAT_UNKNOWN,
};

pub const ACM_FORMATTAGDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: u32 = 1;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: u32 = 2;

/// `ACMFORMATTAGDETAILSW`. The `szFormatTag` buffer is `name`.
pub struct FormatTagDetails {
    pub format_tag_index: u32,
    pub format_tag: u32,
    pub support: u32,
    pub format_size: u32,
    pub standard_formats: u32,
    pub name: &'static str,
}

/// `MPEG3_FormatTagDetails`: answers a format-tag query by index, by tag, or
/// for the largest size, and fills in the tag's fields.
pub fn format_tag_details(details: &mut FormatTagDetails, query: u32) -> Result<(), Error> {
    match query {
        ACM_FORMATTAGDETAILSF_INDEX => {
            if details.format_tag_index > 2 {
                return Err(Error::NotPossible);
            }
        }
        ACM_FORMATTAGDETAILSF_LARGESTSIZE
            if word_tag(details.format_tag) == Some(WAVE_FORMAT_UNKNOWN) =>
        {
            // MPEG-1 has the largest format structure of the three.
            details.format_tag_index = 2;
        }
        ACM_FORMATTAGDETAILSF_FORMATTAG | ACM_FORMATTAGDETAILSF_LARGESTSIZE => {
            details.format_tag_index = match word_tag(details.format_tag) {
                Some(WAVE_FORMAT_PCM) => 0,
                Some(WAVE_FORMAT_MPEGLAYER3) => 1,
                Some(WAVE_FORMAT_MPEG) => 2,
                _ => return Err(Error::NotPossible),
            };
        }
        _ => return Err(Error::NotSupported),
    }
    details.support = ACM_DRIVERDETAILS_SUPPORTF_CODEC;
    let (format_tag, format_size, standard_formats, name) = match details.format_tag_index {
        0 => (
            WAVE_FORMAT_PCM,
            PCMWAVEFORMAT_SIZE,
            PCM_FORMATS.len() as u32,
            "PCM",
        ),
        1 => (
            WAVE_FORMAT_MPEGLAYER3,
            MPEGLAYER3WAVEFORMAT_SIZE,
            0,
            "MPeg3",
        ),
        _ => (WAVE_FORMAT_MPEG, MPEG1WAVEFORMAT_SIZE, 0, "MPeg"),
    };
    details.format_tag = u32::from(format_tag);
    details.format_size = format_size;
    details.standard_formats = standard_formats;
    details.name = name;
    Ok(())
}
