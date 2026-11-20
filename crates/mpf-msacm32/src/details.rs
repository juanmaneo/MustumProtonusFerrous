//! The driver queries (`PCM_DriverDetails`, `PCM_FormatTagDetails` and
//! `PCM_FormatDetails`) and the driver open check (`PCM_drvOpen`).

use crate::format::{
    fill_block_fields, format_index, WaveFormat, PCM_FORMATS, PCM_WAVEFORMAT_SIZE,
};
use crate::{Error, WAVE_FORMAT_PCM, WAVE_FORMAT_UNKNOWN};

/// `ACMDRIVERDETAILS_FCCTYPE_AUDIOCODEC`, `mmioFOURCC('a', 'u', 'd', 'c')`.
pub const FCC_TYPE_AUDIOCODEC: u32 = u32::from_le_bytes(*b"audc");
/// `ACMDRIVERDETAILS_FCCCOMP_UNDEFINED`.
pub const FCC_COMP_UNDEFINED: u32 = 0;
/// `MM_MICROSOFT`.
pub const MM_MICROSOFT: u16 = 1;
/// `MM_MSFT_ACM_PCM`.
pub const MM_MSFT_ACM_PCM: u16 = 0x26;
/// `ACMDRIVERDETAILS_SUPPORTF_CONVERTER`.
pub const SUPPORT_CONVERTER: u32 = 2;

/// `ACM_FORMATTAGDETAILSF_INDEX`.
pub const FORMATTAG_INDEX: u32 = 0;
/// `ACM_FORMATTAGDETAILSF_FORMATTAG`.
pub const FORMATTAG_FORMATTAG: u32 = 1;
/// `ACM_FORMATTAGDETAILSF_LARGESTSIZE`.
pub const FORMATTAG_LARGESTSIZE: u32 = 2;

/// `ACM_FORMATDETAILSF_INDEX`.
pub const FORMAT_INDEX: u32 = 0;
/// `ACM_FORMATDETAILSF_FORMAT`.
pub const FORMAT_FORMAT: u32 = 1;

/// The `ACMDRIVERDETAILSW` fields that `PCM_DriverDetails` fills in. The icon is not ported.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DriverDetails {
    pub fcc_type: u32,
    pub fcc_comp: u32,
    pub manufacturer_id: u16,
    pub product_id: u16,
    pub acm_version: u32,
    pub driver_version: u32,
    pub support: u32,
    pub format_tags: u32,
    pub filter_tags: u32,
    pub short_name: &'static str,
    pub long_name: &'static str,
    pub copyright: &'static str,
    pub licensing: &'static str,
    pub features: &'static str,
}

/// `PCM_DriverDetails`.
pub fn driver_details() -> DriverDetails {
    DriverDetails {
        fcc_type: FCC_TYPE_AUDIOCODEC,
        fcc_comp: FCC_COMP_UNDEFINED,
        manufacturer_id: MM_MICROSOFT,
        product_id: MM_MSFT_ACM_PCM,
        acm_version: 0x0100_0000,
        driver_version: 0x0100_0000,
        support: SUPPORT_CONVERTER,
        format_tags: 1,
        filter_tags: 0,
        short_name: "MS-PCM",
        long_name: "Wine PCM converter",
        copyright: "Brought to you by the Wine team...",
        licensing: "Refer to LICENSE file",
        features: "",
    }
}

/// The `ACMFORMATTAGDETAILSW` fields that the format tag query reads and writes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FormatTagDetails {
    pub index: u32,
    pub format_tag: u32,
    pub support: u32,
    pub format_size: u32,
    pub standard_formats: u32,
}

/// `PCM_FormatTagDetails`. A query that does not match the given index or tag
/// is refused and leaves the fields unchanged.
pub fn format_tag_details(query: u32, d: &mut FormatTagDetails) -> Result<(), Error> {
    let matches = match query {
        FORMATTAG_INDEX => d.index == 0,
        FORMATTAG_FORMATTAG => d.format_tag == u32::from(WAVE_FORMAT_PCM),
        FORMATTAG_LARGESTSIZE => {
            d.format_tag == u32::from(WAVE_FORMAT_UNKNOWN)
                || d.format_tag == u32::from(WAVE_FORMAT_PCM)
        }
        _ => return Err(Error::NotSupported),
    };
    if !matches {
        return Err(Error::NotPossible);
    }
    d.index = 0;
    d.format_tag = u32::from(WAVE_FORMAT_PCM);
    d.format_size = PCM_WAVEFORMAT_SIZE;
    d.support = SUPPORT_CONVERTER;
    d.standard_formats = PCM_FORMATS.len() as u32;
    Ok(())
}

/// The `ACMFORMATDETAILSW` fields that the format query reads and writes, with
/// the format itself passed separately.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FormatDetails {
    pub format_tag: u32,
    pub format_index: u32,
    pub cb_wfx: u32,
    pub support: u32,
}

/// `PCM_FormatDetails`. The format query checks the given format; the index query
/// writes the standard layout at `format_index` into `fmt`.
pub fn format_details(
    query: u32,
    d: &mut FormatDetails,
    fmt: &mut WaveFormat,
) -> Result<(), Error> {
    match query {
        FORMAT_FORMAT => {
            if format_index(fmt).is_none() {
                return Err(Error::NotPossible);
            }
        }
        FORMAT_INDEX => {
            let &(channels, bits, rate) = PCM_FORMATS
                .get(d.format_index as usize)
                .ok_or(Error::InvalidParam)?;
            fmt.format_tag = WAVE_FORMAT_PCM;
            fmt.channels = channels;
            fmt.samples_per_sec = rate;
            fmt.bits_per_sample = bits;
            fill_block_fields(fmt);
        }
        _ => return Err(Error::NotSupported),
    }
    d.format_tag = u32::from(WAVE_FORMAT_PCM);
    d.support = SUPPORT_CONVERTER;
    d.cb_wfx = PCM_WAVEFORMAT_SIZE;
    Ok(())
}

/// The `ACMDRVOPENDESCW` fields that `PCM_drvOpen` checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenDescription {
    pub fcc_type: u32,
    pub fcc_comp: u32,
}

/// `PCM_drvOpen`: an open without a description is accepted, and one with a
/// description only when it asks for the audio codec type with no compressor.
pub fn driver_open_accepts(desc: Option<&OpenDescription>) -> bool {
    desc.is_none_or(|d| d.fcc_type == FCC_TYPE_AUDIOCODEC && d.fcc_comp == FCC_COMP_UNDEFINED)
}
