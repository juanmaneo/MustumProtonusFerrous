//! `audio_format_from_codec_params` from `dlls/winedmo/unix_media_type.c`: the WAVEFORMATEX family
//! written for an audio stream.

use crate::codec::{CodecId, CodecParams, CodecTables};
use crate::guid::media_guid;
use crate::status::{check_capacity, FormatError};

const WAVE_FORMAT_EXTENSIBLE: u32 = 0xfffe;
const WAVE_FORMAT_MPEG_HEAAC: u16 = 0x1610;
/// `sizeof(WAVEFORMATEX)`.
const WAVEFORMATEX_SIZE: usize = 18;
/// `sizeof(WAVEFORMATEXTENSIBLE)`.
const WAVEFORMATEXTENSIBLE_SIZE: usize = 40;
/// `sizeof(WAVEFORMATEXTENSIBLE) - sizeof(WAVEFORMATEX)`.
const EXTENSIBLE_EXTRA_SIZE: usize = 22;
/// `sizeof(HEAACWAVEINFO)`.
const HEAACWAVEINFO_SIZE: usize = 30;

/// `audio_format_from_codec_params`. The returned bytes are the whole format structure, including
/// the extradata and the zero padding up to the format size.
pub fn audio_format_from_codec_params(
    params: &CodecParams,
    capacity: u32,
    tables: &dyn CodecTables,
) -> Result<Vec<u8>, FormatError> {
    if params.codec_id == CodecId::Aac {
        return heaac_wave_format(params, capacity, tables);
    }

    let channels = if params.channels == 0 {
        1
    } else {
        params.channels
    };
    let channel_mask = if params.channel_order_native {
        params.channel_mask
    } else {
        0
    };
    let format_tag = tables.wave_format_tag(params.codec_id);
    if format_tag == WAVE_FORMAT_EXTENSIBLE
        || format_tag >> 16 != 0
        || (channels > 2 && channel_mask != 0)
    {
        return extensible_wave_format(params, capacity, tables, format_tag, channel_mask);
    }

    let format_size = params
        .codec_id
        .wave_format_size()
        .max(WAVEFORMATEX_SIZE + params.extradata.len());
    check_capacity(format_size, capacity)?;
    let header = wave_format_ex(params, tables, format_size, format_tag as u16);
    Ok(assemble(&header, params.extradata, format_size))
}

/// `wave_format_extensible_from_codec_params`. The subtype is `MFAudioFormat_Base` with the format
/// tag as its `Data1`.
fn extensible_wave_format(
    params: &CodecParams,
    capacity: u32,
    tables: &dyn CodecTables,
    format_tag: u32,
    channel_mask: u64,
) -> Result<Vec<u8>, FormatError> {
    let format_size = (params.codec_id.wave_format_size() + EXTENSIBLE_EXTRA_SIZE)
        .max(WAVEFORMATEXTENSIBLE_SIZE + params.extradata.len());
    check_capacity(format_size, capacity)?;
    let mut header = [0; WAVEFORMATEXTENSIBLE_SIZE];
    header[..WAVEFORMATEX_SIZE].copy_from_slice(&wave_format_ex(
        params,
        tables,
        format_size,
        WAVE_FORMAT_EXTENSIBLE as u16,
    ));
    // `Samples.wValidBitsPerSample` stays zero.
    header[20..24].copy_from_slice(&(channel_mask as u32).to_le_bytes());
    header[24..40].copy_from_slice(&media_guid(format_tag).to_bytes());
    Ok(assemble(&header, params.extradata, format_size))
}

/// `heaac_wave_format_from_codec_params`. The three trailing fields of `HEAACWAVEINFO` stay zero.
fn heaac_wave_format(
    params: &CodecParams,
    capacity: u32,
    tables: &dyn CodecTables,
) -> Result<Vec<u8>, FormatError> {
    let format_size = HEAACWAVEINFO_SIZE + params.extradata.len();
    check_capacity(format_size, capacity)?;
    let mut header = [0; HEAACWAVEINFO_SIZE];
    header[..WAVEFORMATEX_SIZE].copy_from_slice(&wave_format_ex(
        params,
        tables,
        format_size,
        WAVE_FORMAT_MPEG_HEAAC,
    ));
    Ok(assemble(&header, params.extradata, format_size))
}

/// `wave_format_ex_init`: the WAVEFORMATEX fields. `cbSize` is the format size beyond the 18-byte
/// header.
fn wave_format_ex(
    params: &CodecParams,
    tables: &dyn CodecTables,
    format_size: usize,
    format_tag: u16,
) -> [u8; WAVEFORMATEX_SIZE] {
    let mut bits = tables.bits_per_sample(params.codec_id) as u16;
    if bits == 0 {
        bits = params.bits_per_coded_sample as u16;
    }
    let channels = params.channels as u16;
    let samples_per_sec = params.sample_rate as u32;
    let mut block_align = params.block_align as u16;
    if block_align == 0 {
        block_align = (u32::from(bits) * u32::from(channels) / 8) as u16;
    }
    let mut avg_bytes_per_sec = (params.bit_rate / 8) as u32;
    if avg_bytes_per_sec == 0 {
        avg_bytes_per_sec = samples_per_sec.wrapping_mul(u32::from(block_align));
    }

    let mut header = [0; WAVEFORMATEX_SIZE];
    header[0..2].copy_from_slice(&format_tag.to_le_bytes());
    header[2..4].copy_from_slice(&channels.to_le_bytes());
    header[4..8].copy_from_slice(&samples_per_sec.to_le_bytes());
    header[8..12].copy_from_slice(&avg_bytes_per_sec.to_le_bytes());
    header[12..14].copy_from_slice(&block_align.to_le_bytes());
    header[14..16].copy_from_slice(&bits.to_le_bytes());
    header[16..18].copy_from_slice(&((format_size - WAVEFORMATEX_SIZE) as u16).to_le_bytes());
    header
}

/// The header, then the extradata, then zeros up to `format_size`.
fn assemble(header: &[u8], extradata: &[u8], format_size: usize) -> Vec<u8> {
    let mut format = Vec::with_capacity(format_size);
    format.extend_from_slice(header);
    format.extend_from_slice(extradata);
    format.resize(format_size, 0);
    format
}
