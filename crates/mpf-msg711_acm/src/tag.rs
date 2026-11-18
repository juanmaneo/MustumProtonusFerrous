//! The `ACMDM_FORMATTAG_DETAILS` query (`G711_FormatTagDetails`).

use crate::format::{ALAW_FORMATS, PCM_FORMATS, ULAW_FORMATS};
use crate::{
    word_tag, Error, PCMWAVEFORMAT_SIZE, WAVEFORMATEX_SIZE, WAVE_FORMAT_ALAW, WAVE_FORMAT_MULAW,
    WAVE_FORMAT_PCM, WAVE_FORMAT_UNKNOWN,
};

pub const ACM_FORMATTAGDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: u32 = 1;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: u32 = 2;

/// The outputs of `ACMFORMATTAGDETAILSW` that the query sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatTagDetails {
    pub format_tag: u32,
    pub format_tag_index: u32,
    pub format_size: u32,
    pub standard_formats: u32,
    pub name: &'static str,
}

struct Tag {
    format_tag: u16,
    format_size: u32,
    standard_formats: u32,
    name: &'static str,
}

const TAGS: [Tag; 3] = [
    Tag {
        format_tag: WAVE_FORMAT_PCM,
        format_size: PCMWAVEFORMAT_SIZE,
        standard_formats: PCM_FORMATS.len() as u32,
        name: "PCM",
    },
    Tag {
        format_tag: WAVE_FORMAT_ALAW,
        format_size: WAVEFORMATEX_SIZE,
        standard_formats: ALAW_FORMATS.len() as u32,
        name: "A-Law",
    },
    Tag {
        format_tag: WAVE_FORMAT_MULAW,
        format_size: WAVEFORMATEX_SIZE,
        standard_formats: ULAW_FORMATS.len() as u32,
        name: "U-Law",
    },
];

/// `G711_FormatTagDetails`. `ACM_FORMATTAGDETAILSF_INDEX` looks up the tag at
/// `format_tag_index`. The other two queries look up `format_tag`, and
/// `LARGESTSIZE` with `WAVE_FORMAT_UNKNOWN` reports A-law.
pub fn format_tag_details(
    query: u32,
    format_tag: u32,
    format_tag_index: u32,
) -> Result<FormatTagDetails, Error> {
    let index = match query {
        ACM_FORMATTAGDETAILSF_INDEX => {
            if format_tag_index >= TAGS.len() as u32 {
                return Err(Error::NotPossible);
            }
            format_tag_index
        }
        ACM_FORMATTAGDETAILSF_LARGESTSIZE if format_tag == u32::from(WAVE_FORMAT_UNKNOWN) => 1,
        ACM_FORMATTAGDETAILSF_FORMATTAG | ACM_FORMATTAGDETAILSF_LARGESTSIZE => {
            match word_tag(format_tag) {
                Some(WAVE_FORMAT_PCM) => 0,
                Some(WAVE_FORMAT_ALAW) => 1,
                Some(WAVE_FORMAT_MULAW) => 2,
                _ => return Err(Error::NotPossible),
            }
        }
        _ => return Err(Error::NotSupported),
    };
    let tag = &TAGS[index as usize];
    Ok(FormatTagDetails {
        format_tag: u32::from(tag.format_tag),
        format_tag_index: index,
        format_size: tag.format_size,
        standard_formats: tag.standard_formats,
        name: tag.name,
    })
}
