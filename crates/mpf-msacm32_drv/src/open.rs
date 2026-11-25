use std::ops::Range;

use crate::{
    PcmFormat, ACMERR_NOTPOSSIBLE, MMSYSERR_NOERROR, WAVERR_BADFORMAT, WAVE_FORMAT_DIRECT,
    WAVE_FORMAT_PCM,
};

/// The source `WAVEFORMATEX` fields that `wodOpen` / `widOpen` read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
}

/// The `WAVEMAPDATA` fields that `wodOpen` / `widOpen` set on success.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Opened {
    /// The device that accepted the stream.
    pub device: u32,
    /// True when an ACM conversion stream was opened (`hAcmStream != NULL` in C).
    pub converted: bool,
    pub avg_speed_inner: u32,
    pub samples_per_sec_inner: u32,
}

const RATES: [u32; 5] = [96000, 48000, 44100, 22050, 11025];

/// Which candidate sequence the search tries after the direct stage.
enum Ladder {
    /// PCM source: change the bit depth and channel count first, then the sample rate.
    Full,
    /// Non-PCM source: keep the rate and channel count, change only the bit depth.
    BitDepthOnly,
}

/// One `TRY` of the C ladder: a candidate PCM format on one device.
struct Probe {
    device: u32,
    samples_per_sec: u32,
    channels: u16,
    bits_per_sample: u16,
}

impl Ladder {
    /// The candidates in the order the C `TRY` macros run them.
    fn probes(self, src: &SourceFormat, devices: Range<u32>) -> Vec<Probe> {
        let mut probes = Vec::new();
        match self {
            Ladder::BitDepthOnly => {
                for device in devices {
                    for bits_per_sample in [16, 8] {
                        probes.push(Probe {
                            device,
                            samples_per_sec: src.samples_per_sec,
                            channels: src.channels,
                            bits_per_sample,
                        });
                    }
                }
            }
            Ladder::Full => {
                for device in devices.clone() {
                    for channels in [src.channels, src.channels ^ 3] {
                        for bits_per_sample in [16, 8] {
                            probes.push(Probe {
                                device,
                                samples_per_sec: src.samples_per_sec,
                                channels,
                                bits_per_sample,
                            });
                        }
                    }
                }
                for device in devices {
                    for bits_per_sample in [16, 8] {
                        for channels in [src.channels, src.channels ^ 3] {
                            for samples_per_sec in RATES {
                                probes.push(Probe {
                                    device,
                                    samples_per_sec,
                                    channels,
                                    bits_per_sample,
                                });
                            }
                        }
                    }
                }
            }
        }
        probes
    }
}

/// `wodOpen` after `device_range`. `flags` must not contain `WAVE_MAPPED`. `direct` opens a
/// device with the source format, and `convert` opens one candidate on a device. Both return
/// an `MMSYSERR_*` / `WAVERR_*` code, where 0 is success.
pub fn wod_open(
    src: &SourceFormat,
    devices: Range<u32>,
    flags: u32,
    direct: &mut dyn FnMut(u32) -> u32,
    convert: &mut dyn FnMut(u32, &PcmFormat) -> u32,
) -> Result<Opened, u32> {
    let ladder = if src.format_tag == WAVE_FORMAT_PCM {
        Ladder::Full
    } else {
        Ladder::BitDepthOnly
    };
    open_search(src, devices, flags, ladder, direct, convert)
}

/// `widOpen` after `device_range`. Unlike `wod_open`, it always runs the full ladder.
pub fn wid_open(
    src: &SourceFormat,
    devices: Range<u32>,
    flags: u32,
    direct: &mut dyn FnMut(u32) -> u32,
    convert: &mut dyn FnMut(u32, &PcmFormat) -> u32,
) -> Result<Opened, u32> {
    open_search(src, devices, flags, Ladder::Full, direct, convert)
}

fn open_search(
    src: &SourceFormat,
    devices: Range<u32>,
    flags: u32,
    ladder: Ladder,
    direct: &mut dyn FnMut(u32) -> u32,
    convert: &mut dyn FnMut(u32, &PcmFormat) -> u32,
) -> Result<Opened, u32> {
    for device in devices.clone() {
        if direct(device) == MMSYSERR_NOERROR {
            return Ok(Opened {
                device,
                converted: false,
                avg_speed_inner: src.avg_bytes_per_sec,
                samples_per_sec_inner: src.samples_per_sec,
            });
        }
    }
    if (flags & WAVE_FORMAT_DIRECT) != 0 {
        return Err(WAVERR_BADFORMAT);
    }
    for probe in ladder.probes(src, devices) {
        let format = PcmFormat::new(probe.samples_per_sec, probe.channels, probe.bits_per_sample);
        match convert(probe.device, &format) {
            MMSYSERR_NOERROR => {
                return Ok(Opened {
                    device: probe.device,
                    converted: true,
                    avg_speed_inner: format.avg_bytes_per_sec,
                    samples_per_sec_inner: format.samples_per_sec,
                });
            }
            WAVERR_BADFORMAT => {}
            // ACMERR_NOTPOSSIBLE ends the whole search, as the C `error:` label does.
            ACMERR_NOTPOSSIBLE => return Err(WAVERR_BADFORMAT),
            other => return Err(other),
        }
    }
    Err(WAVERR_BADFORMAT)
}
