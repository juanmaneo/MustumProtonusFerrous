//! Headless subset of Wine's dlls/msgsm32.acm/msgsm32.c, the GSM 6.10 audio codec driver.
//!
//! The ported functions compute what Wine's C bodies compute from their
//! arguments. libgsm is reached through the [GsmCodec] trait. The stream state
//! handle, the libgsm allocation and the configure dialog are not modelled.
//!
//! Ported:
//! - GSM_FormatValidate: [format_validate].
//! - GSM_DriverDetails: [driver_details].
//! - GSM_FormatTagDetails: [format_tag_details].
//! - GSM_FormatDetails: [format_details].
//! - GSM_FormatSuggest: [format_suggest].
//! - GSM_StreamOpen checks: [stream_open_check]; the gsm_create and gsm_option calls are skipped.
//! - GSM_StreamSize: [stream_size].
//! - GSM_StreamConvert: [stream_convert], the frame layout over a [GsmCodec].
//! - GSM_DriverProc routing: [dispatch].
//!
//! Skipped:
//! - gsm_create, gsm_option, gsm_destroy and GSM_StreamClose: libgsm state, which [GsmCodec] stands in for.
//! - MessageBoxA in DRV_CONFIGURE: [dispatch] reports [Dispatch::Configure] and the caller shows the dialog.
//! - DefDriverProc: [dispatch] reports [Dispatch::DefDriverProc] and the caller forwards the message.
//! - MultiByteToWideChar, lstrcpyW and the szFormat clear: string conversion, so names are static strings.
//! - hicon: an icon handle.
//! - WARN, FIXME and TRACE output, including the libgsm decode failure that only FIXME reports.

/// Windows DWORD.
pub type DWORD = u32;
/// Windows LRESULT, the result of the driver entry point.
pub type LResult = isize;

/// MMSYSERR_NOERROR.
pub const MMSYSERR_NOERROR: LResult = 0;
/// MMSYSERR_NOTSUPPORTED.
pub const MMSYSERR_NOTSUPPORTED: LResult = 8;
/// MMSYSERR_INVALPARAM.
pub const MMSYSERR_INVALPARAM: LResult = 11;
/// ACMERR_NOTPOSSIBLE.
pub const ACMERR_NOTPOSSIBLE: LResult = 512;

/// DRVCNF_RESTART.
pub const DRVCNF_RESTART: LResult = 2;
pub const DRV_LOAD: DWORD = 1;
pub const DRV_ENABLE: DWORD = 2;
pub const DRV_OPEN: DWORD = 3;
pub const DRV_CLOSE: DWORD = 4;
pub const DRV_DISABLE: DWORD = 5;
pub const DRV_FREE: DWORD = 6;
pub const DRV_CONFIGURE: DWORD = 7;
pub const DRV_QUERYCONFIGURE: DWORD = 8;
pub const DRV_INSTALL: DWORD = 9;
pub const DRV_REMOVE: DWORD = 10;

/// ACMDM_BASE, the first ACM driver message.
pub const ACMDM_BASE: DWORD = 0x6000;
pub const ACMDM_DRIVER_NOTIFY: DWORD = ACMDM_BASE + 1;
pub const ACMDM_DRIVER_DETAILS: DWORD = ACMDM_BASE + 10;
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: DWORD = ACMDM_BASE + 20;
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: DWORD = ACMDM_BASE + 21;
pub const ACMDM_FORMATTAG_DETAILS: DWORD = ACMDM_BASE + 25;
pub const ACMDM_FORMAT_DETAILS: DWORD = ACMDM_BASE + 26;
pub const ACMDM_FORMAT_SUGGEST: DWORD = ACMDM_BASE + 27;
pub const ACMDM_FILTERTAG_DETAILS: DWORD = ACMDM_BASE + 50;
pub const ACMDM_FILTER_DETAILS: DWORD = ACMDM_BASE + 51;
pub const ACMDM_STREAM_OPEN: DWORD = ACMDM_BASE + 76;
pub const ACMDM_STREAM_CLOSE: DWORD = ACMDM_BASE + 77;
pub const ACMDM_STREAM_SIZE: DWORD = ACMDM_BASE + 78;
pub const ACMDM_STREAM_CONVERT: DWORD = ACMDM_BASE + 79;
pub const ACMDM_STREAM_RESET: DWORD = ACMDM_BASE + 80;
pub const ACMDM_STREAM_PREPARE: DWORD = ACMDM_BASE + 81;
pub const ACMDM_STREAM_UNPREPARE: DWORD = ACMDM_BASE + 82;

pub const ACM_FORMATTAGDETAILSF_INDEX: DWORD = 0;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: DWORD = 1;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: DWORD = 2;
pub const ACM_FORMATDETAILSF_INDEX: DWORD = 0;
pub const ACM_FORMATDETAILSF_FORMAT: DWORD = 1;
pub const ACM_FORMATSUGGESTF_WFORMATTAG: DWORD = 0x0001_0000;
pub const ACM_FORMATSUGGESTF_NCHANNELS: DWORD = 0x0002_0000;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: DWORD = 0x0004_0000;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: DWORD = 0x0008_0000;
pub const ACM_STREAMSIZEF_SOURCE: DWORD = 0;
pub const ACM_STREAMSIZEF_DESTINATION: DWORD = 1;
pub const ACM_STREAMCONVERTF_BLOCKALIGN: DWORD = 4;

pub const ACMDRIVERDETAILS_FCCTYPE_AUDIOCODEC: DWORD = u32::from_le_bytes(*b"audc");
pub const ACMDRIVERDETAILS_FCCCOMP_UNDEFINED: DWORD = 0;
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: DWORD = 1;
pub const MM_MICROSOFT: u16 = 1;
pub const MM_MSFT_ACM_GSM610: u16 = 36;

pub const WAVE_FORMAT_UNKNOWN: u16 = 0;
pub const WAVE_FORMAT_PCM: u16 = 1;
pub const WAVE_FORMAT_GSM610: u16 = 49;

/// sizeof(PCMWAVEFORMAT), also the smallest format a suggest call may pass.
pub const PCM_FORMAT_SIZE: DWORD = 16;
/// sizeof(GSM610WAVEFORMAT).
pub const GSM_FORMAT_SIZE: DWORD = 20;
/// nBlockAlign of a GSM 6.10 format.
pub const GSM_BLOCK_ALIGN: u16 = 65;
/// wSamplesPerBlock of a GSM 6.10 format.
pub const GSM_SAMPLES_PER_BLOCK: u16 = 320;
/// cbSize of a GSM 6.10 format, sizeof(WORD).
pub const GSM_CB_SIZE: u16 = 2;
/// The sample rates offered by format index, in index order.
pub const GSM_RATES: [DWORD; 6] = [8000, 11025, 22050, 44100, 48000, 96000];

/// PCM bytes of one GSM block: two frames of 320 bytes.
const BLOCK_PCM_BYTES: usize = 640;
/// Bytes of one GSM block: a 32-byte frame and a 33-byte frame.
const BLOCK_BYTES: usize = 65;
/// PCM bytes of one libgsm frame: 160 16-bit samples.
const FRAME_PCM_BYTES: usize = 320;

/// A WAVEFORMATEX, with the GSM 6.10 wSamplesPerBlock word after it.
/// The cb_size and samples_per_block fields are not read for PCM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaveFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: DWORD,
    pub avg_bytes_per_sec: DWORD,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
    pub samples_per_block: u16,
}

/// The ACMDRIVERDETAILSW fields that GSM_DriverDetails fills in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DriverDetails {
    pub fcc_type: DWORD,
    pub fcc_comp: DWORD,
    pub mid: u16,
    pub pid: u16,
    pub vdw_acm: DWORD,
    pub vdw_driver: DWORD,
    pub fdw_support: DWORD,
    pub format_tags: DWORD,
    pub filter_tags: DWORD,
    pub short_name: &'static str,
    pub long_name: &'static str,
    pub copyright: &'static str,
    pub licensing: &'static str,
    pub features: &'static str,
}

/// The ACMFORMATTAGDETAILSW fields used by GSM_FormatTagDetails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatTagDetails {
    pub index: DWORD,
    pub format_tag: DWORD,
    pub format_size: DWORD,
    pub support: DWORD,
    pub standard_formats: DWORD,
    pub name: &'static str,
}

/// The ACMFORMATDETAILSW fields used by GSM_FormatDetails; format is the pwfx it reads and writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatDetails {
    pub format_index: DWORD,
    pub format_tag: DWORD,
    pub format: WaveFormat,
    pub support: DWORD,
}

/// The ACMDRVFORMATSUGGEST fields used by GSM_FormatSuggest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatSuggest {
    pub fdw_suggest: DWORD,
    pub src: WaveFormat,
    pub src_size: DWORD,
    pub dst: WaveFormat,
    pub dst_size: DWORD,
}

/// The ACMDRVSTREAMSIZE fields used by GSM_StreamSize.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamSize {
    pub fdw_size: DWORD,
    pub src_length: DWORD,
    pub dst_length: DWORD,
}

/// The ACMDRVSTREAMHEADER fields used by GSM_StreamConvert. The buffer lengths are the slice lengths.
pub struct StreamHeader<'a> {
    pub fdw_convert: DWORD,
    pub src: &'a [u8],
    pub dst: &'a mut [u8],
    pub src_used: DWORD,
    pub dst_used: DWORD,
}

/// The libgsm calls that GSM_StreamConvert makes.
///
/// A block is two frames. encode writes a 32-byte frame and then a 33-byte
/// frame. decode reads a 33-byte frame and then a 32-byte frame. The frame index
/// toggles on every call, which is why the order differs.
pub trait GsmCodec {
    /// Sets the libgsm frame index back to 0 (GSM_OPT_FRAME_INDEX).
    fn reset_frame_index(&mut self);
    /// Encodes 320 bytes of PCM into frame, which is 32 or 33 bytes long.
    fn encode(&mut self, pcm: &[u8], frame: &mut [u8]);
    /// Decodes frame, which is 33 or 32 bytes long, into 320 bytes of PCM.
    /// Wine only logs a failed decode, so no status is returned.
    fn decode(&mut self, frame: &[u8], pcm: &mut [u8]);
}

/// What GSM_DriverProc does with a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dispatch {
    /// Returns the value without other work.
    Reply(LResult),
    /// DRV_CONFIGURE: shows the configure dialog, then returns 1.
    Configure,
    DriverDetails,
    FormatTagDetails,
    FormatDetails,
    FormatSuggest,
    StreamOpen,
    StreamClose,
    StreamSize,
    StreamConvert,
    /// Any message not handled here, forwarded to DefDriverProc.
    DefDriverProc,
}

/// GSM_DriverDetails. The C body always returns MMSYSERR_NOERROR.
pub fn driver_details() -> DriverDetails {
    DriverDetails {
        fcc_type: ACMDRIVERDETAILS_FCCTYPE_AUDIOCODEC,
        fcc_comp: ACMDRIVERDETAILS_FCCCOMP_UNDEFINED,
        mid: MM_MICROSOFT,
        pid: MM_MSFT_ACM_GSM610,
        vdw_acm: 0x0332_0000,
        vdw_driver: 0x0400_0000,
        fdw_support: ACMDRIVERDETAILS_SUPPORTF_CODEC,
        format_tags: 2,
        filter_tags: 0,
        short_name: "Microsoft GSM 6.10",
        long_name: "Wine GSM 6.10 libgsm codec",
        copyright: "Brought to you by the Wine team...",
        licensing: "Refer to LICENSE file",
        features: "",
    }
}

/// The average byte rate of GSM 6.10 at a sample rate, in the DWORD arithmetic of the C code.
fn gsm_avg_bytes(samples_per_sec: DWORD) -> DWORD {
    samples_per_sec.wrapping_mul(65) / 320
}

/// GSM_FormatValidate: whether a format is one this driver converts.
pub fn format_validate(wfx: &WaveFormat) -> bool {
    if wfx.channels != 1 {
        return false;
    }
    match wfx.format_tag {
        WAVE_FORMAT_PCM => {
            wfx.bits_per_sample == 16
                && wfx.block_align == 2
                && wfx.avg_bytes_per_sec
                    == u32::from(wfx.block_align).wrapping_mul(wfx.samples_per_sec)
        }
        WAVE_FORMAT_GSM610 => {
            wfx.cb_size >= GSM_CB_SIZE
                && wfx.bits_per_sample == 0
                && wfx.block_align == GSM_BLOCK_ALIGN
                && wfx.samples_per_block == GSM_SAMPLES_PER_BLOCK
                && wfx.avg_bytes_per_sec == gsm_avg_bytes(wfx.samples_per_sec)
        }
        _ => false,
    }
}

/// GSM_FormatTagDetails: aftd is read for the query and index or tag, then written.
pub fn format_tag_details(query: DWORD, aftd: &mut FormatTagDetails) -> LResult {
    match query {
        ACM_FORMATTAGDETAILSF_INDEX => {
            if aftd.index > 1 {
                return ACMERR_NOTPOSSIBLE;
            }
        }
        ACM_FORMATTAGDETAILSF_LARGESTSIZE
            if aftd.format_tag == DWORD::from(WAVE_FORMAT_UNKNOWN) =>
        {
            aftd.index = 1;
        }
        ACM_FORMATTAGDETAILSF_LARGESTSIZE | ACM_FORMATTAGDETAILSF_FORMATTAG => {
            aftd.index = match u16::try_from(aftd.format_tag) {
                Ok(WAVE_FORMAT_PCM) => 0,
                Ok(WAVE_FORMAT_GSM610) => 1,
                _ => return ACMERR_NOTPOSSIBLE,
            };
        }
        _ => return MMSYSERR_NOTSUPPORTED,
    }
    aftd.support = ACMDRIVERDETAILS_SUPPORTF_CODEC;
    aftd.standard_formats = GSM_RATES.len() as DWORD;
    if aftd.index == 0 {
        aftd.format_tag = DWORD::from(WAVE_FORMAT_PCM);
        aftd.format_size = PCM_FORMAT_SIZE;
        aftd.name = "PCM";
    } else {
        aftd.format_tag = DWORD::from(WAVE_FORMAT_GSM610);
        aftd.format_size = GSM_FORMAT_SIZE;
        aftd.name = "GSM 6.10";
    }
    MMSYSERR_NOERROR
}

/// GSM_FormatDetails. For an index query the format tag is written first and
/// stays written when the call fails, as in the C code.
pub fn format_details(query: DWORD, afd: &mut FormatDetails) -> LResult {
    match query {
        ACM_FORMATDETAILSF_FORMAT => {
            if !format_validate(&afd.format) {
                return ACMERR_NOTPOSSIBLE;
            }
        }
        ACM_FORMATDETAILSF_INDEX => {
            afd.format.format_tag = afd.format_tag as u16;
            let tag = match u16::try_from(afd.format_tag) {
                Ok(tag @ (WAVE_FORMAT_PCM | WAVE_FORMAT_GSM610)) => tag,
                _ => return MMSYSERR_INVALPARAM,
            };
            let Some(&rate) = GSM_RATES.get(afd.format_index as usize) else {
                return ACMERR_NOTPOSSIBLE;
            };
            let format = &mut afd.format;
            format.channels = 1;
            format.samples_per_sec = rate;
            if tag == WAVE_FORMAT_PCM {
                format.bits_per_sample = 16;
                format.block_align = 2;
                format.avg_bytes_per_sec = rate.wrapping_mul(2);
            } else {
                format.bits_per_sample = 0;
                format.block_align = GSM_BLOCK_ALIGN;
                format.avg_bytes_per_sec = gsm_avg_bytes(rate);
                format.cb_size = GSM_CB_SIZE;
                format.samples_per_block = GSM_SAMPLES_PER_BLOCK;
            }
        }
        _ => return MMSYSERR_NOTSUPPORTED,
    }
    afd.support = ACMDRIVERDETAILS_SUPPORTF_CODEC;
    MMSYSERR_NOERROR
}

/// GSM_FormatSuggest. The destination fields are written in place, and some
/// writes persist when the call returns ACMERR_NOTPOSSIBLE.
pub fn format_suggest(adfs: &mut FormatSuggest) -> LResult {
    if adfs.src_size < PCM_FORMAT_SIZE
        || adfs.dst_size < PCM_FORMAT_SIZE
        || !format_validate(&adfs.src)
    {
        return ACMERR_NOTPOSSIBLE;
    }
    let fdw = adfs.fdw_suggest;
    let src = adfs.src;
    let dst = &mut adfs.dst;
    if (fdw & ACM_FORMATSUGGESTF_NCHANNELS) == 0 {
        dst.channels = src.channels;
    }
    if (fdw & ACM_FORMATSUGGESTF_NSAMPLESPERSEC) == 0 {
        dst.samples_per_sec = src.samples_per_sec;
    }
    if (fdw & ACM_FORMATSUGGESTF_WBITSPERSAMPLE) == 0 {
        dst.bits_per_sample = if src.format_tag == WAVE_FORMAT_PCM {
            0
        } else {
            16
        };
    }
    if (fdw & ACM_FORMATSUGGESTF_WFORMATTAG) == 0 {
        dst.format_tag = if src.format_tag == WAVE_FORMAT_PCM {
            WAVE_FORMAT_GSM610
        } else {
            WAVE_FORMAT_PCM
        };
    }
    match dst.format_tag {
        WAVE_FORMAT_PCM => {
            dst.block_align = 2;
            dst.avg_bytes_per_sec = dst.samples_per_sec.wrapping_mul(2);
        }
        WAVE_FORMAT_GSM610 => {
            if dst.cb_size < GSM_CB_SIZE {
                return ACMERR_NOTPOSSIBLE;
            }
            dst.block_align = GSM_BLOCK_ALIGN;
            dst.avg_bytes_per_sec = gsm_avg_bytes(dst.samples_per_sec);
            dst.samples_per_block = GSM_SAMPLES_PER_BLOCK;
        }
        _ => return ACMERR_NOTPOSSIBLE,
    }
    if format_validate(dst) {
        MMSYSERR_NOERROR
    } else {
        ACMERR_NOTPOSSIBLE
    }
}

/// The checks of GSM_StreamOpen before it allocates libgsm state.
pub fn stream_open_check(src: &WaveFormat, dst: &WaveFormat) -> LResult {
    if src.format_tag != WAVE_FORMAT_GSM610 && dst.format_tag != WAVE_FORMAT_GSM610 {
        return MMSYSERR_NOTSUPPORTED;
    }
    if !format_validate(src) || !format_validate(dst) {
        return MMSYSERR_NOTSUPPORTED;
    }
    if src.samples_per_sec != dst.samples_per_sec {
        return MMSYSERR_NOTSUPPORTED;
    }
    MMSYSERR_NOERROR
}

/// GSM_StreamSize: converts a length in one direction, as the stream's formats allow.
pub fn stream_size(src: &WaveFormat, dst: &WaveFormat, size: &mut StreamSize) -> LResult {
    let pcm_to_gsm = src.format_tag == WAVE_FORMAT_PCM && dst.format_tag == WAVE_FORMAT_GSM610;
    let gsm_to_pcm = src.format_tag == WAVE_FORMAT_GSM610 && dst.format_tag == WAVE_FORMAT_PCM;
    match size.fdw_size {
        ACM_STREAMSIZEF_DESTINATION if pcm_to_gsm => {
            size.src_length = (size.dst_length / 65).wrapping_mul(640);
        }
        ACM_STREAMSIZEF_DESTINATION if gsm_to_pcm => {
            size.src_length = (size.dst_length / 640).wrapping_mul(65);
        }
        ACM_STREAMSIZEF_SOURCE if pcm_to_gsm => {
            size.dst_length = (size.src_length.wrapping_add(639) / 640).wrapping_mul(65);
        }
        ACM_STREAMSIZEF_SOURCE if gsm_to_pcm => {
            size.dst_length = (size.src_length / 65).wrapping_mul(640);
        }
        _ => return MMSYSERR_NOTSUPPORTED,
    }
    MMSYSERR_NOERROR
}

/// GSM_StreamConvert: the frame layout of a conversion, with the libgsm
/// calls made through codec. A GSM source is decoded; any other source is
/// encoded, zero-padded to a whole block unless ACM_STREAMCONVERTF_BLOCKALIGN
/// is set. On ACMERR_NOTPOSSIBLE the used lengths are left unchanged.
///
/// Buffer lengths are DWORD values as in the C header, so a buffer longer
/// than u32::MAX bytes panics. The size checks use DWORD arithmetic, so a
/// buffer of 436 MB or more can pass a wrapped check where the C code would
/// overrun; the slicing here then panics instead.
pub fn stream_convert<C: GsmCodec>(
    codec: &mut C,
    src_format: &WaveFormat,
    header: &mut StreamHeader<'_>,
) -> LResult {
    codec.reset_frame_index();
    let converted = if src_format.format_tag == WAVE_FORMAT_GSM610 {
        decode_blocks(codec, header.src, header.dst)
    } else {
        encode_blocks(codec, header.fdw_convert, header.src, header.dst)
    };
    match converted {
        Some((src_used, dst_used)) => {
            header.src_used = src_used;
            header.dst_used = dst_used;
            MMSYSERR_NOERROR
        }
        None => ACMERR_NOTPOSSIBLE,
    }
}

fn dword_len(bytes: &[u8]) -> DWORD {
    DWORD::try_from(bytes.len()).expect("ACM stream buffers are at most DWORD bytes")
}

fn decode_blocks<C: GsmCodec>(codec: &mut C, src: &[u8], dst: &mut [u8]) -> Option<(DWORD, DWORD)> {
    if (dword_len(src) / 65).wrapping_mul(640) > dword_len(dst) {
        return None;
    }
    let (mut n, mut d) = (0, 0);
    while n + BLOCK_BYTES <= src.len() {
        codec.decode(&src[n..n + 33], &mut dst[d..d + FRAME_PCM_BYTES]);
        n += 33;
        d += FRAME_PCM_BYTES;
        codec.decode(&src[n..n + 32], &mut dst[d..d + FRAME_PCM_BYTES]);
        n += 32;
        d += FRAME_PCM_BYTES;
    }
    Some((n as DWORD, d as DWORD))
}

fn encode_blocks<C: GsmCodec>(
    codec: &mut C,
    fdw_convert: DWORD,
    src: &[u8],
    dst: &mut [u8],
) -> Option<(DWORD, DWORD)> {
    if (dword_len(src).wrapping_add(639) / 640).wrapping_mul(65) > dword_len(dst) {
        return None;
    }
    let (mut n, mut d) = (0, 0);
    while n + BLOCK_PCM_BYTES <= src.len() {
        codec.encode(&src[n..n + FRAME_PCM_BYTES], &mut dst[d..d + 32]);
        n += FRAME_PCM_BYTES;
        d += 32;
        codec.encode(&src[n..n + FRAME_PCM_BYTES], &mut dst[d..d + 33]);
        n += FRAME_PCM_BYTES;
        d += 33;
    }
    if (fdw_convert & ACM_STREAMCONVERTF_BLOCKALIGN) == 0 && n < src.len() {
        let rest = &src[n..];
        let mut tail = [0u8; FRAME_PCM_BYTES];
        if rest.len() > FRAME_PCM_BYTES {
            codec.encode(&rest[..FRAME_PCM_BYTES], &mut dst[d..d + 32]);
            d += 32;
            let last = &rest[FRAME_PCM_BYTES..];
            tail[..last.len()].copy_from_slice(last);
            codec.encode(&tail, &mut dst[d..d + 33]);
            d += 33;
        } else {
            tail[..rest.len()].copy_from_slice(rest);
            codec.encode(&tail, &mut dst[d..d + 32]);
            d += 32;
            codec.encode(&[0u8; FRAME_PCM_BYTES], &mut dst[d..d + 33]);
            d += 33;
        }
        n = src.len();
    }
    Some((n as DWORD, d as DWORD))
}

/// Routes a driver message the way GSM_DriverProc does, without running the handler.
pub fn dispatch(msg: DWORD) -> Dispatch {
    match msg {
        DRV_LOAD | DRV_ENABLE | DRV_OPEN | DRV_CLOSE | DRV_DISABLE | DRV_FREE
        | DRV_QUERYCONFIGURE => Dispatch::Reply(1),
        DRV_CONFIGURE => Dispatch::Configure,
        DRV_INSTALL | DRV_REMOVE => Dispatch::Reply(DRVCNF_RESTART),
        ACMDM_DRIVER_NOTIFY | ACMDM_STREAM_PREPARE | ACMDM_STREAM_UNPREPARE => {
            Dispatch::Reply(MMSYSERR_NOERROR)
        }
        ACMDM_DRIVER_DETAILS => Dispatch::DriverDetails,
        ACMDM_FORMATTAG_DETAILS => Dispatch::FormatTagDetails,
        ACMDM_FORMAT_DETAILS => Dispatch::FormatDetails,
        ACMDM_FORMAT_SUGGEST => Dispatch::FormatSuggest,
        ACMDM_STREAM_OPEN => Dispatch::StreamOpen,
        ACMDM_STREAM_CLOSE => Dispatch::StreamClose,
        ACMDM_STREAM_SIZE => Dispatch::StreamSize,
        ACMDM_STREAM_CONVERT => Dispatch::StreamConvert,
        ACMDM_HARDWARE_WAVE_CAPS_INPUT
        | ACMDM_HARDWARE_WAVE_CAPS_OUTPUT
        | ACMDM_FILTERTAG_DETAILS
        | ACMDM_FILTER_DETAILS
        | ACMDM_STREAM_RESET => Dispatch::Reply(MMSYSERR_NOTSUPPORTED),
        _ => Dispatch::DefDriverProc,
    }
}

#[cfg(test)]
mod oracle_checks;
