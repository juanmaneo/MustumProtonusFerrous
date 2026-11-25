//! Oracle-derived test vectors for the ported msacm32.drv logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies (`MulDiv`,
//! `wodGetPosition` / `widGetPosition`, `wodOpen` / `widOpen` and their helpers) with
//! `gcc -std=gnu11 -fsanitize=address,undefined -g` under `target/canon/msacm32.drv/`
//! and running the harness. The 25 printed rows are 4 MulDiv, 7 position and 14 open rows.
//! One open row, `wid_mapped_invalid_id2`, is excluded: LeakSanitizer reports that the C
//! `widOpen` invalid `WAVE_MAPPED` path returns without freeing its `WAVEMAPDATA`. The
//! leak is in Wine's code, not the port. The 24 remaining rows are below.

use std::cell::Cell;

use super::*;

/// `pcm44` in the harness: 44.1 kHz stereo 16-bit PCM.
pub(super) const PCM44: SourceFormat = SourceFormat {
    format_tag: WAVE_FORMAT_PCM,
    channels: 2,
    samples_per_sec: 44100,
    avg_bytes_per_sec: 176400,
};

/// `adpcm` in the harness: 22.05 kHz mono, format tag 2 (non-PCM).
pub(super) const ADPCM: SourceFormat = SourceFormat {
    format_tag: 2,
    channels: 1,
    samples_per_sec: 22050,
    avg_bytes_per_sec: 11025,
};

/// The fake device layer behind [`run_open`]. Each field names the one format or device that
/// is accepted or failed. Every other call returns `WAVERR_BADFORMAT`.
pub(super) struct Stub {
    /// Device that accepts the source format directly (`WAVE_FORMAT_DIRECT` stage).
    pub(super) direct_dev: Option<u32>,
    /// `(device, (samples_per_sec, channels, bits))` that accepts a converted candidate.
    pub(super) target: Option<(u32, (u32, u16, u16))>,
    /// Candidate that `acmStreamOpen` rejects with `ACMERR_NOTPOSSIBLE`.
    pub(super) abort: Option<(u32, u16, u16)>,
    /// Candidate whose device open fails with `MMSYSERR_NOMEM`.
    pub(super) nomem: Option<(u32, u16, u16)>,
}

pub(super) const NONE: Stub = Stub {
    direct_dev: None,
    target: None,
    abort: None,
    nomem: None,
};

/// Runs `wod_open`, or `wid_open` when `wid` is set, against `stub`. Returns the result and
/// the number of candidates probed (the harness `acmStreamOpen` count).
pub(super) fn run_open(
    wid: bool,
    src: &SourceFormat,
    devices: Range<u32>,
    flags: u32,
    stub: Stub,
) -> (Result<Opened, u32>, u32) {
    let probes = Cell::new(0);
    let mut direct = |device: u32| {
        if stub.direct_dev == Some(device) {
            MMSYSERR_NOERROR
        } else {
            WAVERR_BADFORMAT
        }
    };
    let mut convert = |device: u32, format: &PcmFormat| {
        probes.set(probes.get() + 1);
        let triple = (
            format.samples_per_sec,
            format.channels,
            format.bits_per_sample,
        );
        if stub.abort == Some(triple) {
            ACMERR_NOTPOSSIBLE
        } else if stub.nomem == Some(triple) {
            MMSYSERR_NOMEM
        } else if stub.target == Some((device, triple)) {
            MMSYSERR_NOERROR
        } else {
            WAVERR_BADFORMAT
        }
    };
    let result = if wid {
        wid_open(src, devices, flags, &mut direct, &mut convert)
    } else {
        wod_open(src, devices, flags, &mut direct, &mut convert)
    };
    (result, probes.get())
}

#[test]
fn oracle_muldiv_rows() {
    assert_eq!(mul_div(7, 3, 2), 11);
    assert_eq!(mul_div(7, 3, -2), -11);
    assert_eq!(mul_div(1000, 1, 0), -1);
    assert_eq!(mul_div(2147483647, 2, 1), -1);
}

#[test]
fn oracle_position_rows() {
    // wod_bytes_aligned
    let rates = Rates {
        avg_speed_outer: 16000,
        avg_speed_inner: 192000,
        samples_per_sec_outer: 8000,
        samples_per_sec_inner: 48000,
    };
    let req = MmTime {
        wtype: TIME_BYTES,
        u: [0, 0, 0, 0, 0xaa, 0xaa, 0xaa, 0xaa],
    };
    let reply = MmTime {
        wtype: TIME_BYTES,
        u: [0xe8, 0x03, 0, 0, 0, 0, 0, 0],
    };
    assert_eq!(position_query(&req).wtype, TIME_BYTES);
    assert_eq!(
        get_position(&rates, &req, &reply),
        MmTime {
            wtype: TIME_BYTES,
            u: [0x54, 0, 0, 0, 0xaa, 0xaa, 0xaa, 0xaa],
        }
    );

    // wod_ms_from_bytes
    let rates = Rates {
        avg_speed_outer: 16000,
        avg_speed_inner: 16000,
        samples_per_sec_outer: 8000,
        samples_per_sec_inner: 8000,
    };
    let req = MmTime {
        wtype: TIME_MS,
        u: [0; 8],
    };
    let reply = MmTime {
        wtype: TIME_BYTES,
        u: [0x50, 0, 0, 0, 0, 0, 0, 0],
    };
    assert_eq!(position_query(&req).wtype, TIME_BYTES);
    assert_eq!(
        get_position(&rates, &req, &reply),
        MmTime {
            wtype: TIME_MS,
            u: [0x05, 0, 0, 0, 0, 0, 0, 0],
        }
    );

    // wod_samples
    let rates = Rates {
        avg_speed_outer: 16000,
        avg_speed_inner: 96000,
        samples_per_sec_outer: 8000,
        samples_per_sec_inner: 48000,
    };
    let req = MmTime {
        wtype: TIME_SAMPLES,
        u: [0; 8],
    };
    let reply = MmTime {
        wtype: TIME_SAMPLES,
        u: [0xc0, 0x12, 0, 0, 0, 0, 0, 0],
    };
    assert_eq!(position_query(&req).wtype, TIME_SAMPLES);
    assert_eq!(
        get_position(&rates, &req, &reply),
        MmTime {
            wtype: TIME_SAMPLES,
            u: [0x20, 0x03, 0, 0, 0, 0, 0, 0],
        }
    );

    // wod_smpte_raw
    let req = MmTime {
        wtype: TIME_SMPTE,
        u: [0; 8],
    };
    let reply = MmTime {
        wtype: TIME_SMPTE,
        u: [1, 2, 3, 4, 5, 6, 7, 8],
    };
    assert_eq!(position_query(&req).wtype, TIME_SMPTE);
    assert_eq!(
        get_position(&rates, &req, &reply),
        MmTime {
            wtype: TIME_SMPTE,
            u: [1, 2, 3, 4, 5, 6, 7, 8],
        }
    );

    // wod_ms_samples_raw
    let rates = Rates {
        avg_speed_outer: 16000,
        avg_speed_inner: 192000,
        samples_per_sec_outer: 8000,
        samples_per_sec_inner: 48000,
    };
    let req = MmTime {
        wtype: TIME_MS,
        u: [0; 8],
    };
    let reply = MmTime {
        wtype: TIME_SAMPLES,
        u: [0xc0, 0x12, 0, 0, 0, 0, 0, 0],
    };
    assert_eq!(position_query(&req).wtype, TIME_BYTES);
    assert_eq!(
        get_position(&rates, &req, &reply),
        MmTime {
            wtype: TIME_MS,
            u: [0xc0, 0x12, 0, 0, 0, 0, 0, 0],
        }
    );

    // wod_bytes_no_align
    let rates = Rates {
        avg_speed_outer: 192000,
        avg_speed_inner: 16000,
        samples_per_sec_outer: 48000,
        samples_per_sec_inner: 8000,
    };
    let req = MmTime {
        wtype: TIME_BYTES,
        u: [0; 8],
    };
    let reply = MmTime {
        wtype: TIME_BYTES,
        u: [0xe0, 0x2e, 0, 0, 0, 0, 0, 0],
    };
    assert_eq!(position_query(&req).wtype, TIME_BYTES);
    assert_eq!(
        get_position(&rates, &req, &reply),
        MmTime {
            wtype: TIME_BYTES,
            u: [0x80, 0x32, 0x02, 0, 0, 0, 0, 0],
        }
    );

    // wid_bytes_aligned (widGetPosition has the same body; the request tail is zero here)
    let rates = Rates {
        avg_speed_outer: 16000,
        avg_speed_inner: 192000,
        samples_per_sec_outer: 8000,
        samples_per_sec_inner: 48000,
    };
    let req = MmTime {
        wtype: TIME_BYTES,
        u: [0; 8],
    };
    let reply = MmTime {
        wtype: TIME_BYTES,
        u: [0xe8, 0x03, 0, 0, 0, 0, 0, 0],
    };
    assert_eq!(position_query(&req).wtype, TIME_BYTES);
    assert_eq!(
        get_position(&rates, &req, &reply),
        MmTime {
            wtype: TIME_BYTES,
            u: [0x54, 0, 0, 0, 0, 0, 0, 0],
        }
    );
}

#[test]
fn oracle_open_rows() {
    // wod_pcm_stereo_44100_probe1
    let stub = Stub {
        target: Some((0, (44100, 2, 16))),
        ..NONE
    };
    assert_eq!(
        run_open(false, &PCM44, 0..1, 0, stub),
        (
            Ok(Opened {
                device: 0,
                converted: true,
                avg_speed_inner: 176400,
                samples_per_sec_inner: 44100,
            }),
            1
        )
    );

    // wod_pcm_mono_44100_probe3
    let stub = Stub {
        target: Some((0, (44100, 1, 16))),
        ..NONE
    };
    assert_eq!(
        run_open(false, &PCM44, 0..1, 0, stub),
        (
            Ok(Opened {
                device: 0,
                converted: true,
                avg_speed_inner: 88200,
                samples_per_sec_inner: 44100,
            }),
            3
        )
    );

    // wod_pcm_stereo_96000_probe5
    let stub = Stub {
        target: Some((0, (96000, 2, 16))),
        ..NONE
    };
    assert_eq!(
        run_open(false, &PCM44, 0..1, 0, stub),
        (
            Ok(Opened {
                device: 0,
                converted: true,
                avg_speed_inner: 384000,
                samples_per_sec_inner: 96000,
            }),
            5
        )
    );

    // wod_pcm_mono_11025_8bit_probe24
    let stub = Stub {
        target: Some((0, (11025, 1, 8))),
        ..NONE
    };
    assert_eq!(
        run_open(false, &PCM44, 0..1, 0, stub),
        (
            Ok(Opened {
                device: 0,
                converted: true,
                avg_speed_inner: 11025,
                samples_per_sec_inner: 11025,
            }),
            24
        )
    );

    // wod_pcm_dev1_of_2_probe5
    let stub = Stub {
        target: Some((1, (44100, 2, 16))),
        ..NONE
    };
    assert_eq!(
        run_open(false, &PCM44, 0..2, 0, stub),
        (
            Ok(Opened {
                device: 1,
                converted: true,
                avg_speed_inner: 176400,
                samples_per_sec_inner: 44100,
            }),
            5
        )
    );

    // wod_pcm_abort_probe3
    let stub = Stub {
        abort: Some((44100, 1, 16)),
        ..NONE
    };
    assert_eq!(
        run_open(false, &PCM44, 0..1, 0, stub),
        (Err(WAVERR_BADFORMAT), 3)
    );

    // wod_pcm_nomem_probe1
    let stub = Stub {
        nomem: Some((44100, 2, 16)),
        ..NONE
    };
    assert_eq!(
        run_open(false, &PCM44, 0..1, 0, stub),
        (Err(MMSYSERR_NOMEM), 1)
    );

    // wod_pcm_direct_flag_no_probe
    assert_eq!(
        run_open(false, &PCM44, 0..1, WAVE_FORMAT_DIRECT, NONE),
        (Err(WAVERR_BADFORMAT), 0)
    );

    // wod_pcm_direct_dev0
    let stub = Stub {
        direct_dev: Some(0),
        ..NONE
    };
    assert_eq!(
        run_open(false, &PCM44, 0..1, 0, stub),
        (
            Ok(Opened {
                device: 0,
                converted: false,
                avg_speed_inner: 176400,
                samples_per_sec_inner: 44100,
            }),
            0
        )
    );

    // wod_adpcm_8bit_probe2
    let stub = Stub {
        target: Some((0, (22050, 1, 8))),
        ..NONE
    };
    assert_eq!(
        run_open(false, &ADPCM, 0..1, 0, stub),
        (
            Ok(Opened {
                device: 0,
                converted: true,
                avg_speed_inner: 22050,
                samples_per_sec_inner: 22050,
            }),
            2
        )
    );

    // wid_adpcm_full_probe3
    let stub = Stub {
        target: Some((0, (22050, 2, 16))),
        ..NONE
    };
    assert_eq!(
        run_open(true, &ADPCM, 0..1, 0, stub),
        (
            Ok(Opened {
                device: 0,
                converted: true,
                avg_speed_inner: 88200,
                samples_per_sec_inner: 22050,
            }),
            3
        )
    );

    // wod_mapped_dev1_probe1: WAVE_MAPPED is stripped before the search
    assert_eq!(device_range(WAVE_MAPPED, 1, 2), Ok(1..2));
    let stub = Stub {
        target: Some((1, (44100, 2, 16))),
        ..NONE
    };
    assert_eq!(
        run_open(false, &PCM44, 1..2, 0, stub),
        (
            Ok(Opened {
                device: 1,
                converted: true,
                avg_speed_inner: 176400,
                samples_per_sec_inner: 44100,
            }),
            1
        )
    );

    // wod_mapped_invalid_id2: rejected before any probe
    assert_eq!(device_range(WAVE_MAPPED, 2, 2), Err(MMSYSERR_INVALPARAM));
}
