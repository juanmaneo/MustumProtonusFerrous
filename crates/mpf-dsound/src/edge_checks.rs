//! Edge-case tests for the ported dsound logic.
//!
//! These cover the NULL and aggregation rules of the create entry points, the create and
//! Initialize failure paths, the class table, the volume and pan conversions, the sample
//! readers, the vector NaN rules, the angle branches, the speaker mask, the export table and the
//! HRESULT macros. Expectations that are not oracle-derived say so in a comment.

use super::*;
use crate::oracle_checks::{
    amp_to_vol_pan, bits3, factory_index, run, v, Entry, IID_IDIRECTSOUND, OBJECT, SENTINEL,
};

fn entries() -> [Entry; 5] {
    [
        Entry::Create,
        Entry::Create8,
        Entry::CaptureCreate,
        Entry::CaptureCreate8,
        Entry::FullDuplexCreate,
    ]
}

#[test]
fn null_out_returns_invalid_param_without_calls() {
    // Not oracle-derived: each C entry point checks for a NULL out pointer before anything else.
    for outer in [false, true] {
        for entry in entries() {
            let r = run(entry, None, outer, Ok(OBJECT), DS_OK);
            assert_eq!(r.ret, DSERR_INVALIDPARAM);
            assert!(r.out.is_none());
            assert_eq!((r.creates, r.initializes, r.releases), (0, 0, 0));
        }
    }
}

#[test]
fn create_aggregated_clears_out_and_fails() {
    // Not oracle-derived: the aggregated DirectSoundCreate call from body_dsound.c clears the out
    // pointer and returns DSERR_INVALIDPARAM without creating an object.
    let r = run(Entry::Create, Some(Some(SENTINEL)), true, Ok(OBJECT), DS_OK);
    assert_eq!(r.ret, DSERR_INVALIDPARAM);
    assert_eq!(r.out, Some(None));
    assert_eq!((r.creates, r.initializes, r.releases), (0, 0, 0));
}

#[test]
fn create8_tolerates_already_initialized() {
    // Not oracle-derived: DirectSoundCreate8 counts DSERR_ALREADYINITIALIZED as success. r03 covers
    // DirectSoundCreate with the same Initialize result.
    let r = run(
        Entry::Create8,
        Some(None),
        false,
        Ok(OBJECT),
        DSERR_ALREADYINITIALIZED,
    );
    assert_eq!(r.ret, DS_OK);
    assert_eq!(r.out, Some(Some(OBJECT)));
    assert_eq!((r.initializes, r.releases), (1, 0));
}

#[test]
fn capture8_and_full_duplex_reject_already_initialized() {
    // Not oracle-derived: the strict Initialize rule for these two entry points. r07 covers
    // DirectSoundCaptureCreate.
    for entry in [Entry::CaptureCreate8, Entry::FullDuplexCreate] {
        let r = run(
            entry,
            Some(None),
            false,
            Ok(OBJECT),
            DSERR_ALREADYINITIALIZED,
        );
        assert_eq!(r.ret, 0x8878_0082);
        assert_eq!(r.out, Some(None));
        assert_eq!((r.initializes, r.releases), (1, 1));
    }
}

#[test]
fn capture8_and_full_duplex_aggregation_returns_noaggregation() {
    // Not oracle-derived: the aggregated calls return DSERR_NOAGGREGATION and clear the out pointer.
    for entry in [Entry::CaptureCreate8, Entry::FullDuplexCreate] {
        let r = run(entry, Some(Some(SENTINEL)), true, Ok(OBJECT), DS_OK);
        assert_eq!(r.ret, DSERR_NOAGGREGATION);
        assert_eq!(r.out, Some(None));
        assert_eq!((r.creates, r.initializes, r.releases), (0, 0, 0));
    }
}

#[test]
fn create_failure_skips_initialize_and_release() {
    // Not oracle-derived, except that r05 covers DirectSoundCreate8 with E_OUTOFMEMORY. The create
    // failure's HRESULT is returned, and the out pointer is set to NULL.
    for entry in entries() {
        let r = run(entry, Some(None), false, Err(E_OUTOFMEMORY), DS_OK);
        assert_eq!(r.ret, E_OUTOFMEMORY);
        assert_eq!(r.out, Some(None));
        assert_eq!((r.creates, r.initializes, r.releases), (1, 0, 0));
    }
}

#[test]
fn class_table_guids_match_headers() {
    // Not oracle-derived: values from include/dsound.h (CLSID_DirectSound, CLSID_DirectSound8,
    // CLSID_DirectSoundCapture, CLSID_DirectSoundCapture8, CLSID_DirectSoundFullDuplex) and
    // include/dsconf.h (CLSID_DirectSoundPrivate). The comparisons use == so no Debug output
    // is needed.
    let expected = [
        Guid {
            data1: 0x47d4_d946,
            data2: 0x62e8,
            data3: 0x11cf,
            data4: [0x93, 0xbc, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
        },
        Guid {
            data1: 0x3901_cc3f,
            data2: 0x84b5,
            data3: 0x4fa4,
            data4: [0xba, 0x35, 0xaa, 0x81, 0x72, 0xb8, 0xa0, 0x9b],
        },
        Guid {
            data1: 0xb021_0780,
            data2: 0x89cd,
            data3: 0x11d0,
            data4: [0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16],
        },
        Guid {
            data1: 0xe4bc_ac13,
            data2: 0x7f99,
            data3: 0x4908,
            data4: [0x9a, 0x8e, 0x74, 0xe3, 0xbf, 0x24, 0xb6, 0xe1],
        },
        Guid {
            data1: 0xfea4_300c,
            data2: 0x7959,
            data3: 0x4147,
            data4: [0xb2, 0x6a, 0x23, 0x77, 0xb9, 0xe7, 0xa9, 0x1d],
        },
        Guid {
            data1: 0x11ab_3ec0,
            data2: 0x25ec,
            data3: 0x11d1,
            data4: [0xa4, 0xd8, 0x00, 0xc0, 0x4f, 0xc2, 0x8a, 0xca],
        },
    ];
    assert_eq!(CLASS_FACTORIES.len(), expected.len());
    for (cf, want) in CLASS_FACTORIES.iter().zip(expected) {
        assert!(cf.clsid == want);
    }
    assert!(
        IID_UNKNOWN
            == Guid {
                data1: 0,
                data2: 0,
                data3: 0,
                data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
            }
    );
    assert!(
        IID_CLASS_FACTORY
            == Guid {
                data1: 1,
                data2: 0,
                data3: 0,
                data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
            }
    );
}

#[test]
fn dll_get_class_object_clears_ppv_before_checks() {
    // Not oracle-derived: the output is cleared before the riid check, so a rejected riid leaves
    // NULL even when ppv held a factory.
    let mut ppv = Some(&CLASS_FACTORIES[0]);
    let ret = dll_get_class_object(&CLASS_FACTORIES[0].clsid, &IID_IDIRECTSOUND, Some(&mut ppv));
    assert_eq!(ret, E_NOINTERFACE);
    assert!(ppv.is_none());
}

#[test]
fn dll_get_class_object_finds_every_factory() {
    // Not oracle-derived beyond r15 and r16: every DSOUND_CF entry is found through IUnknown and
    // through IClassFactory.
    for (i, cf) in CLASS_FACTORIES.iter().enumerate() {
        for riid in [&IID_UNKNOWN, &IID_CLASS_FACTORY] {
            let mut ppv = None;
            assert_eq!(dll_get_class_object(&cf.clsid, riid, Some(&mut ppv)), S_OK);
            assert_eq!(factory_index(ppv), Some(i));
        }
    }
}

#[test]
fn recalc_vol_pan_writes_only_first_two_factors() {
    // Not oracle-derived: entries 2 to 7 keep their values, and a silent volume zeroes all eight.
    let mut volpan = VolumePan {
        total_amp_factor: [7; DS_MAX_CHANNELS],
        volume: -600,
        pan: 0,
    };
    recalc_vol_pan(&mut volpan);
    assert!(volpan.total_amp_factor[2..] == [7; 6]);

    let mut volpan = VolumePan {
        total_amp_factor: [7; DS_MAX_CHANNELS],
        volume: DSBVOLUME_MIN,
        pan: 0,
    };
    recalc_vol_pan(&mut volpan);
    assert_eq!(volpan.total_amp_factor, [0; DS_MAX_CHANNELS]);
}

#[test]
fn amp_factor_to_vol_pan_silence_and_one() {
    // Not oracle-derived: a factor of 0 is the -10000 silence value. A factor of 1 gives -9599 from
    // the formula (600 * log2(1 / 65535), truncated).
    let volpan = amp_to_vol_pan(0, 0);
    assert_eq!((volpan.volume, volpan.pan), (-10000, 0));

    let volpan = amp_to_vol_pan(1, 1);
    assert_eq!((volpan.volume, volpan.pan), (-9599, 0));
}

#[test]
fn buf_ptr_diff_equal_and_wrapped_pointers() {
    // Not oracle-derived: equal pointers, a forward distance, and a distance that wraps.
    assert_eq!(buf_ptr_diff(10, 3, 3), 0);
    assert_eq!(buf_ptr_diff(100, 30, 10), 20);
    assert_eq!(buf_ptr_diff(10, 2, 5), 7);
}

#[test]
#[should_panic]
fn buf_ptr_diff_rejects_ptr1_at_buflen() {
    // Not oracle-derived: a pointer must be inside the buffer.
    buf_ptr_diff(10, 10, 0);
}

#[test]
#[should_panic]
fn buf_ptr_diff_rejects_ptr2_at_buflen() {
    // Not oracle-derived: a pointer must be inside the buffer.
    buf_ptr_diff(10, 0, 10);
}

#[test]
fn sample_readers_step_by_channel_count() {
    // Not oracle-derived: the offsets and strides for a two-channel buffer, worked out by hand
    // from the C index expression.
    let mut dst = [0.0f32; 2];
    get8(&[0x10, 0x90, 0x20, 0xa0], &mut dst, 2, 1);
    assert_eq!(dst, [0.125, 0.25]);

    get16(&[0, 0x40, 0, 0, 0, 0xc0, 0, 0], &mut dst, 2, 0);
    assert_eq!(dst, [0.5, -0.5]);

    get32(
        &[0, 0, 0, 0, 0, 0, 0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0xc0],
        &mut dst,
        2,
        1,
    );
    assert_eq!(dst, [0.5, -0.5]);

    getieee32(
        &[0, 0, 0, 0, 0, 0, 0x80, 0x3e, 0, 0, 0, 0, 0, 0, 0, 0xc0],
        &mut dst,
        2,
        1,
    );
    assert_eq!(dst, [0.25, -2.0]);

    get24(&[0, 0, 0x40, 0, 0, 0, 0, 0, 0x20], &mut dst, 2, 0);
    assert_eq!(dst, [0.5, 0.25]);
}

#[test]
fn get24_reads_a_full_top_byte_as_twos_complement() {
    // Not oracle-derived: a top byte of 0xff is the case where the C shift is undefined, so this
    // checks the two's complement reading of the unsigned shifts.
    let mut dst = [0.0f32; 1];
    get24(&[0, 0, 0xff], &mut dst, 1, 0);
    assert_eq!(dst, [-0.0078125]);
}

#[test]
fn vector_nan_results_are_zeroed() {
    // Not oracle-derived: each case has a NaN component, from inf - inf or inf * 0.
    let inf = f32::INFINITY;
    assert_eq!(
        bits3(vector_product(&v(0.0, 0.0, inf), &v(0.0, 1.0, 0.0))),
        [0, 0, 0]
    );
    assert_eq!(
        bits3(vector_product(&v(inf, 0.0, 0.0), &v(1.0, 0.0, 1.0))),
        [0, 0, 0]
    );
    assert_eq!(
        bits3(vector_between_two_points(
            &v(0.0, inf, 0.0),
            &v(0.0, inf, 0.0)
        )),
        [0, 0, 0]
    );
    assert_eq!(
        bits3(vector_between_two_points(
            &v(0.0, 0.0, inf),
            &v(0.0, 0.0, inf)
        )),
        [0, 0, 0]
    );
}

#[test]
fn angle_branches() {
    // Not oracle-derived: the zero-vector branch, an exact cosine of -1, and the two out-of-range
    // cosines that f32 rounding can produce.
    let x = v(1.0, 0.0, 0.0);
    assert_eq!(
        angle_between_vectors_rad(&x, &v(0.0, 0.0, 0.0)).to_bits(),
        0
    );
    assert_eq!(
        angle_between_vectors_rad(&x, &v(-1.0, 0.0, 0.0)).to_bits(),
        0x4049_0fdb
    );

    let a = v(0.1, 0.1, 0.1);
    assert_eq!(angle_between_vectors_rad(&a, &a).to_bits(), 0);

    let a = v(3.0, 3.0, 3.0);
    let b = v(-3.0, -3.0, -3.0);
    assert_eq!(angle_between_vectors_rad(&a, &b).to_bits(), 0x4049_0fdb);
}

#[test]
fn speaker_mask_uses_low_byte_only() {
    // Not oracle-derived: DSSPEAKER_CONFIG keeps the low byte, so the high bits are ignored.
    // DSSPEAKER_DIRECTOUT (0) is not in the switch, so it takes the default arm.
    assert_eq!(speaker_config_to_channel_mask(0x104), 0x3);
    assert_eq!(speaker_config_to_channel_mask(0xffff_ff06), 0x3f);
    assert_eq!(speaker_config_to_channel_mask(0), 0x3);
}

#[test]
fn export_table_lookups() {
    // Not oracle-derived: lookups by name and by ordinal over dsound.spec.
    let create8 = export("DirectSoundCreate8").unwrap();
    assert_eq!(create8.ordinal, Some(11));
    assert!(create8.kind == ExportKind::Stdcall);

    let get_class = export("DllGetClassObject").unwrap();
    assert_eq!(get_class.ordinal, None);
    assert!(get_class.kind == ExportKind::StdcallPrivate);

    assert!(export("Nope").is_none());
    assert!(export("directsoundcreate8").is_none());

    assert!(export_by_ordinal(4).is_none());
    assert_eq!(
        export_by_ordinal(12).unwrap().name,
        "DirectSoundCaptureCreate8"
    );
    assert_eq!(EXPORTS.len(), 14);
}

#[test]
fn dshresult_macros_build_already_initialized() {
    // Not oracle-derived: MAKE_DSHRESULT(130) is the value that DSERR_ALREADYINITIALIZED has in
    // include/dsound.h. The function is called at run time so that its body is covered.
    assert_eq!(make_dshresult(130), 0x8878_0082);
    assert_eq!(make_hresult(1, 0x878, 130), 0x8878_0082);
    assert_eq!(DSERR_ALREADYINITIALIZED, 0x8878_0082);
}
