//! Port of the portable logic in Wine's `dlls/mfwmaaec/mfwmaaec.c`.
//!
//! Ported:
//! - `check_dmo_media_type`: [`check_dmo_media_type`], over the [`DmoMediaType`] fields
//! - The checks and `cbSize` computation of `media_object_SetOutputType`:
//!   [`check_dmo_media_type`] and [`output_buffer_size`]
//!
//! Skipped:
//! - `aaec_factory_CreateInstance` and the `IUnknown`, `IMFTransform`, `IMediaObject`,
//!   and `IPropertyStore` vtables: COM objects, AddRef/Release, and interface queries.
//! - `get_capture_device`, `start_audio_capture`, `stop_audio_capture`: audio endpoint
//!   and capture-thread work through OS APIs.
//! - The copy step of `media_object_SetOutputType` (`MoCopyMediaType`, `MoFreeMediaType`)
//!   and the `aaec` state update.
//! - `media_object_SetInputType` and the other `FIXME` stubs returning `E_NOTIMPL`.

/// `DMO_E_INVALIDTYPE` from `mediaerr.h`.
pub const DMO_E_INVALIDTYPE: u32 = 0x8004_0202;
/// `DMO_E_TYPE_NOT_ACCEPTED` from `mediaerr.h`.
pub const DMO_E_TYPE_NOT_ACCEPTED: u32 = 0x8004_0205;

const WAVE_FORMAT_PCM: u16 = 1;

/// `sizeof(WAVEFORMATEX)`. The struct is packed (`#pragma pack(1)` in `mmreg.h`).
const WAVEFORMATEX_SIZE: usize = 18;

/// `max_frame_size_ms` in `mfwmaaec.c`.
const MAX_FRAME_SIZE_MS: u32 = 40;

/// `supported_rates` in `mfwmaaec.c`.
const SUPPORTED_RATES: [u32; 4] = [8000, 11025, 16000, 22050];

/// `GUID` from `guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const MEDIATYPE_AUDIO: Guid = Guid {
    data1: 0x7364_7561,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

pub const FORMAT_WAVEFORMATEX: Guid = Guid {
    data1: 0x0558_9f81,
    data2: 0xc356,
    data3: 0x11ce,
    data4: [0xbf, 0x01, 0x00, 0xaa, 0x00, 0x55, 0x59, 0x5a],
};

pub const MEDIASUBTYPE_PCM: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// The fields of `DMO_MEDIA_TYPE` that `check_dmo_media_type` reads. `format` is
/// `pbFormat` with `cbFormat` bytes.
pub struct DmoMediaType<'a> {
    pub majortype: Guid,
    pub subtype: Guid,
    pub formattype: Guid,
    pub format: &'a [u8],
}

/// The `WAVEFORMATEX` fields read by `check_dmo_media_type`, decoded from the
/// packed little-endian bytes of `pbFormat`.
struct WaveFormatEx {
    format_tag: u16,
    channels: u16,
    samples_per_sec: u32,
    avg_bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
}

impl WaveFormatEx {
    fn read(b: &[u8]) -> Self {
        Self {
            format_tag: u16::from_le_bytes([b[0], b[1]]),
            channels: u16::from_le_bytes([b[2], b[3]]),
            samples_per_sec: u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            avg_bytes_per_sec: u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
            block_align: u16::from_le_bytes([b[12], b[13]]),
            bits_per_sample: u16::from_le_bytes([b[14], b[15]]),
        }
    }
}

/// `check_dmo_media_type`. Returns `Ok(())` for `S_OK`, otherwise the failing HRESULT.
pub fn check_dmo_media_type(media_type: &DmoMediaType) -> Result<(), u32> {
    if media_type.majortype != MEDIATYPE_AUDIO {
        return Err(DMO_E_TYPE_NOT_ACCEPTED);
    }
    if media_type.formattype != FORMAT_WAVEFORMATEX || media_type.format.len() < WAVEFORMATEX_SIZE {
        return Err(DMO_E_TYPE_NOT_ACCEPTED);
    }
    if media_type.subtype != MEDIASUBTYPE_PCM {
        return Err(DMO_E_TYPE_NOT_ACCEPTED);
    }

    let wfx = WaveFormatEx::read(media_type.format);
    if wfx.bits_per_sample == 0 || wfx.avg_bytes_per_sec == 0 || wfx.block_align == 0 {
        return Err(DMO_E_INVALIDTYPE);
    }
    if wfx.channels != 1 || wfx.format_tag != WAVE_FORMAT_PCM {
        return Err(DMO_E_TYPE_NOT_ACCEPTED);
    }
    if !SUPPORTED_RATES.contains(&wfx.samples_per_sec) {
        return Err(DMO_E_TYPE_NOT_ACCEPTED);
    }
    Ok(())
}

/// The `cbSize` that `media_object_SetOutputType` stores for one frame of
/// `MAX_FRAME_SIZE_MS`. The C operands are 32-bit `unsigned int`, so the product
/// wraps modulo 2^32 before the division.
pub fn output_buffer_size(samples_per_sec: u32, bits_per_sample: u16) -> u32 {
    MAX_FRAME_SIZE_MS
        .wrapping_mul(samples_per_sec)
        .wrapping_mul(u32::from(bits_per_sample))
        / (8 * 1000)
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
