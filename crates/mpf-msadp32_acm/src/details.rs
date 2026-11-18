//! The driver details and the format tag and format queries of the driver
//! (`ADPCM_DriverDetails`, `ADPCM_FormatTagDetails`, `ADPCM_FormatDetails`).

use crate::format::{
    format_index, init_adpcm, WaveFormat, ADPCM_FORMATS, ADPCM_WAVEFORMAT_SIZE, PCM_FORMATS,
    PCM_WAVEFORMAT_SIZE,
};
use crate::{Error, WAVE_FORMAT_ADPCM, WAVE_FORMAT_PCM, WAVE_FORMAT_UNKNOWN};

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
/// `ACMDRIVERDETAILS_SUPPORTF_CODEC`.
pub const SUPPORT_CODEC: u32 = 1;

/// `ACMDRIVERDETAILSW` as filled by `ADPCM_DriverDetails`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

/// `ADPCM_DriverDetails`.
pub fn driver_details() -> DriverDetails {
    DriverDetails {
        fcc_type: u32::from_le_bytes(*b"audc"),
        fcc_comp: 0,
        manufacturer_id: 1,
        product_id: 0x21,
        acm_version: 0x0100_0000,
        driver_version: 0x0100_0000,
        support: SUPPORT_CODEC,
        format_tags: 2,
        filter_tags: 0,
        short_name: "MS-ADPCM",
        long_name: "Wine MS ADPCM converter",
        copyright: "Brought to you by the Wine team...",
        licensing: "Refer to LICENSE file",
        features: "",
    }
}

/// `ACMFORMATTAGDETAILSW`, the fields read and written by `ADPCM_FormatTagDetails`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FormatTagDetails {
    pub index: u32,
    pub format_tag: u32,
    pub support: u32,
    pub format_size: u32,
    pub standard_formats: u32,
    pub name: &'static str,
}

/// `ADPCM_FormatTagDetails`.
pub fn format_tag_details(query: u32, d: &mut FormatTagDetails) -> Result<(), Error> {
    match query {
        FORMATTAG_INDEX => {
            if d.index >= 2 {
                return Err(Error::NotPossible);
            }
        }
        FORMATTAG_LARGESTSIZE if d.format_tag == u32::from(WAVE_FORMAT_UNKNOWN) => d.index = 1,
        FORMATTAG_LARGESTSIZE | FORMATTAG_FORMATTAG => {
            d.index = match d.format_tag {
                tag if tag == u32::from(WAVE_FORMAT_PCM) => 0,
                tag if tag == u32::from(WAVE_FORMAT_ADPCM) => 1,
                _ => return Err(Error::NotPossible),
            };
        }
        _ => return Err(Error::NotSupported),
    }
    d.support = SUPPORT_CODEC;
    if d.index == 0 {
        d.format_tag = u32::from(WAVE_FORMAT_PCM);
        d.format_size = PCM_WAVEFORMAT_SIZE;
        d.standard_formats = PCM_FORMATS.len() as u32;
        d.name = "PCM";
    } else {
        d.format_tag = u32::from(WAVE_FORMAT_ADPCM);
        d.format_size = ADPCM_WAVEFORMAT_SIZE;
        d.standard_formats = ADPCM_FORMATS.len() as u32;
        d.name = "Microsoft ADPCM";
    }
    Ok(())
}

/// `ACMFORMATDETAILSW`, the fields read and written by `ADPCM_FormatDetails`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FormatDetails {
    pub format_tag: u32,
    pub format_index: u32,
    pub cb_wfx: u32,
    pub support: u32,
}

/// `ADPCM_FormatDetails`. The format tag is stored in `fmt` before the index
/// is checked, as Wine does.
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
            fmt.format_tag = d.format_tag as u16;
            if d.format_tag == u32::from(WAVE_FORMAT_PCM) {
                let &(channels, bits, rate) = PCM_FORMATS
                    .get(d.format_index as usize)
                    .ok_or(Error::NotPossible)?;
                fmt.channels = channels;
                fmt.samples_per_sec = rate;
                fmt.bits_per_sample = bits;
                fmt.block_align = ((u32::from(channels) * u32::from(bits)) / 8) as u16;
                fmt.avg_bytes_per_sec = rate.wrapping_mul(u32::from(fmt.block_align));
            } else if d.format_tag == u32::from(WAVE_FORMAT_ADPCM) {
                let &(channels, bits, rate) = ADPCM_FORMATS
                    .get(d.format_index as usize)
                    .ok_or(Error::NotPossible)?;
                if d.cb_wfx < ADPCM_WAVEFORMAT_SIZE {
                    return Err(Error::NotPossible);
                }
                fmt.channels = channels;
                fmt.samples_per_sec = rate;
                fmt.bits_per_sample = bits;
                init_adpcm(fmt);
            } else {
                return Err(Error::InvalidParam);
            }
        }
        _ => return Err(Error::NotSupported),
    }
    d.support = SUPPORT_CODEC;
    Ok(())
}
