//! `ADPCM_FormatTagDetails`: the two format tags of the codec, PCM and IMA ADPCM.

use crate::format::{ADPCM_FORMATS, PCM_FORMATS};
use crate::word_tag;
use crate::{
    Error, ACM_DRIVERDETAILS_SUPPORTF_CODEC, IMAADPCMWAVEFORMAT_SIZE, PCMWAVEFORMAT_SIZE,
    WAVE_FORMAT_IMA_ADPCM, WAVE_FORMAT_PCM, WAVE_FORMAT_UNKNOWN,
};

pub const ACM_FORMATTAGDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: u32 = 1;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: u32 = 2;

/// `ACMFORMATTAGDETAILSW` fields. `name` is `szFormatTag`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatTagDetails {
    pub format_tag_index: u32,
    pub format_tag: u32,
    pub support: u32,
    pub format_size: u32,
    pub standard_formats: u32,
    pub name: &'static str,
}

/// `ADPCM_FormatTagDetails`. Index 0 is PCM and index 1 is IMA ADPCM, the
/// largest of the two. The index query accepts only index 0 and 1.
pub fn format_tag_details(details: &mut FormatTagDetails, query: u32) -> Result<(), Error> {
    match query {
        ACM_FORMATTAGDETAILSF_INDEX => {
            if details.format_tag_index >= 2 {
                return Err(Error::NotPossible);
            }
        }
        ACM_FORMATTAGDETAILSF_LARGESTSIZE
            if details.format_tag == u32::from(WAVE_FORMAT_UNKNOWN) =>
        {
            details.format_tag_index = 1;
        }
        ACM_FORMATTAGDETAILSF_LARGESTSIZE | ACM_FORMATTAGDETAILSF_FORMATTAG => {
            details.format_tag_index = match word_tag(details.format_tag) {
                Some(WAVE_FORMAT_PCM) => 0,
                Some(WAVE_FORMAT_IMA_ADPCM) => 1,
                _ => return Err(Error::NotPossible),
            };
        }
        _ => return Err(Error::NotSupported),
    }
    details.support = ACM_DRIVERDETAILS_SUPPORTF_CODEC;
    match details.format_tag_index {
        0 => {
            details.format_tag = u32::from(WAVE_FORMAT_PCM);
            details.format_size = PCMWAVEFORMAT_SIZE;
            details.standard_formats = PCM_FORMATS.len() as u32;
            details.name = "PCM";
        }
        _ => {
            details.format_tag = u32::from(WAVE_FORMAT_IMA_ADPCM);
            details.format_size = IMAADPCMWAVEFORMAT_SIZE;
            details.standard_formats = ADPCM_FORMATS.len() as u32;
            details.name = "IMA ADPCM";
        }
    }
    Ok(())
}
