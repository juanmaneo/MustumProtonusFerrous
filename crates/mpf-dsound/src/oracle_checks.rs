//! Oracle-derived test vectors for the ported dsound logic.
//!
//! The vectors come from the verbatim Wine C bodies in target/canon/dsound, compiled with
//! gcc -fsanitize=address,undefined -g and run by the harness. The harness prints 51 rows, r01
//! to r51. Row 26 is excluded: UBSan flags the signed left shift in the get24 body
//! (body_mixer.c:87) for a top byte of 0x80 or more. So 50 rows are asserted here.

use super::*;
use std::cell::Cell;

pub(crate) const OBJECT: &str = "object";
pub(crate) const SENTINEL: &str = "sentinel";

pub(crate) const IID_IDIRECTSOUND: Guid = Guid {
    data1: 0x279a_fa83,
    data2: 0x4981,
    data3: 0x11ce,
    data4: [0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60],
};

const ZERO_GUID: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// The result of one create call: the return value, the out pointer after the call (None when
/// the pointer was NULL, Some(None) when it holds NULL), and the number of calls made to the
/// object's Create, Initialize and Release.
pub(crate) struct Outcome {
    pub(crate) ret: Hresult,
    pub(crate) out: Option<Option<&'static str>>,
    pub(crate) creates: usize,
    pub(crate) initializes: usize,
    pub(crate) releases: usize,
}

pub(crate) enum Entry {
    Create,
    Create8,
    CaptureCreate,
    CaptureCreate8,
    FullDuplexCreate,
}

/// Runs one create entry point. `out` is the starting out pointer: None for a NULL pointer, and
/// Some(value) for a pointer that holds value.
/// `create_result` is what the create closure returns, and `init_result` what Initialize returns.
/// The closures are built here, so each one is called by some row.
pub(crate) fn run(
    entry: Entry,
    out: Option<Option<&'static str>>,
    outer: bool,
    create_result: Result<&'static str, Hresult>,
    init_result: Hresult,
) -> Outcome {
    let mut cell = out;
    let creates = Cell::new(0);
    let initializes = Cell::new(0);
    let releases = Cell::new(0);
    let create = || {
        creates.set(creates.get() + 1);
        create_result
    };
    let initialize = |_: &&'static str| {
        initializes.set(initializes.get() + 1);
        init_result
    };
    let release = |_: &'static str| releases.set(releases.get() + 1);
    let ret = match entry {
        Entry::Create => direct_sound_create(cell.as_mut(), outer, create, initialize, release),
        Entry::Create8 => direct_sound_create8(cell.as_mut(), outer, create, initialize, release),
        Entry::CaptureCreate => {
            direct_sound_capture_create(cell.as_mut(), outer, create, initialize, release)
        }
        Entry::CaptureCreate8 => {
            direct_sound_capture_create8(cell.as_mut(), outer, create, initialize, release)
        }
        Entry::FullDuplexCreate => {
            direct_sound_full_duplex_create(cell.as_mut(), outer, create, initialize, release)
        }
    };
    Outcome {
        ret,
        out: cell,
        creates: creates.get(),
        initializes: initializes.get(),
        releases: releases.get(),
    }
}

pub(crate) fn v(x: f32, y: f32, z: f32) -> Vector {
    Vector { x, y, z }
}

pub(crate) fn bits3(a: Vector) -> [u32; 3] {
    [a.x.to_bits(), a.y.to_bits(), a.z.to_bits()]
}

/// DSOUND_RecalcVolPan applied to a fresh DSVOLUMEPAN with the given volume and pan.
pub(crate) fn recalc_from(volume: i32, pan: i32) -> VolumePan {
    let mut volpan = VolumePan {
        total_amp_factor: [0; DS_MAX_CHANNELS],
        volume,
        pan,
    };
    recalc_vol_pan(&mut volpan);
    volpan
}

/// DSOUND_AmpFactorToVolPan applied to a fresh DSVOLUMEPAN with the given first two factors.
pub(crate) fn amp_to_vol_pan(left: u32, right: u32) -> VolumePan {
    let mut volpan = VolumePan {
        total_amp_factor: [left, right, 0, 0, 0, 0, 0, 0],
        volume: 0,
        pan: 0,
    };
    amp_factor_to_vol_pan(&mut volpan);
    volpan
}

/// The index of the DSOUND_CF entry that ppv points to, if it points to one.
pub(crate) fn factory_index(ppv: Option<&'static ClassFactory>) -> Option<usize> {
    ppv.and_then(|cf| CLASS_FACTORIES.iter().position(|x| std::ptr::eq(x, cf)))
}

#[test]
fn oracle_create_rows() {
    // r01 DirectSoundCreate ppDS=NULL outer=0 create=0 init=0 -> ret=0x80070057 out=- init=0 release=0
    let r = run(Entry::Create, None, false, Ok(OBJECT), DS_OK);
    assert_eq!(r.ret, 0x8007_0057);
    assert!(r.out.is_none());
    assert_eq!((r.initializes, r.releases), (0, 0));

    // r02 DirectSoundCreate ppDS=set outer=0 create=0 init=0 -> ret=0 out=OBJ init=1 release=0
    let r = run(Entry::Create, Some(None), false, Ok(OBJECT), DS_OK);
    assert_eq!(r.ret, 0);
    assert_eq!(r.out, Some(Some(OBJECT)));
    assert_eq!((r.initializes, r.releases), (1, 0));

    // r03 DirectSoundCreate ppDS=set outer=0 create=0 init=ALREADY -> ret=0 out=OBJ init=1 release=0
    let r = run(
        Entry::Create,
        Some(None),
        false,
        Ok(OBJECT),
        DSERR_ALREADYINITIALIZED,
    );
    assert_eq!(r.ret, 0);
    assert_eq!(r.out, Some(Some(OBJECT)));
    assert_eq!((r.initializes, r.releases), (1, 0));

    // r04 DirectSoundCreate ppDS=set outer=0 create=0 init=E_FAIL -> ret=0x80004005 out=NULL init=1 release=1
    let r = run(Entry::Create, Some(None), false, Ok(OBJECT), E_FAIL);
    assert_eq!(r.ret, 0x8000_4005);
    assert_eq!(r.out, Some(None));
    assert_eq!((r.initializes, r.releases), (1, 1));

    // r05 DirectSoundCreate8 ppDS=set outer=0 create=E_OUTOFMEMORY init=0 -> ret=0x8007000e out=NULL init=0 release=0
    let r = run(Entry::Create8, Some(None), false, Err(E_OUTOFMEMORY), DS_OK);
    assert_eq!(r.ret, 0x8007_000e);
    assert_eq!(r.out, Some(None));
    assert_eq!((r.initializes, r.releases), (0, 0));
}

#[test]
fn oracle_capture_rows() {
    // r06 DirectSoundCaptureCreate ppDSC=set outer=0 create=0 init=0 -> ret=0 out=OBJ init=1 release=0
    let r = run(Entry::CaptureCreate, Some(None), false, Ok(OBJECT), DS_OK);
    assert_eq!(r.ret, 0);
    assert_eq!(r.out, Some(Some(OBJECT)));
    assert_eq!((r.initializes, r.releases), (1, 0));

    // r07 DirectSoundCaptureCreate ppDSC=set outer=0 create=0 init=ALREADY -> ret=0x88780082 out=NULL init=1 release=1
    let r = run(
        Entry::CaptureCreate,
        Some(None),
        false,
        Ok(OBJECT),
        DSERR_ALREADYINITIALIZED,
    );
    assert_eq!(r.ret, 0x8878_0082);
    assert_eq!(r.out, Some(None));
    assert_eq!((r.initializes, r.releases), (1, 1));

    // r08 DirectSoundCaptureCreate ppDSC=set outer=1 create=0 init=0 -> ret=0x80040110 out=SENT init=0 release=0
    let r = run(
        Entry::CaptureCreate,
        Some(Some(SENTINEL)),
        true,
        Ok(OBJECT),
        DS_OK,
    );
    assert_eq!(r.ret, 0x8004_0110);
    assert_eq!(r.out, Some(Some(SENTINEL)));
    assert_eq!((r.initializes, r.releases), (0, 0));

    // r09 DirectSoundCaptureCreate8 ppDSC8=set outer=0 create=0 init=0 -> ret=0 out=OBJ init=1 release=0
    let r = run(Entry::CaptureCreate8, Some(None), false, Ok(OBJECT), DS_OK);
    assert_eq!(r.ret, 0);
    assert_eq!(r.out, Some(Some(OBJECT)));
    assert_eq!((r.initializes, r.releases), (1, 0));

    // r10 DirectSoundCaptureCreate8 ppDSC8=set outer=0 create=0 init=E_FAIL -> ret=0x80004005 out=NULL init=1 release=1
    let r = run(Entry::CaptureCreate8, Some(None), false, Ok(OBJECT), E_FAIL);
    assert_eq!(r.ret, 0x8000_4005);
    assert_eq!(r.out, Some(None));
    assert_eq!((r.initializes, r.releases), (1, 1));
}

#[test]
fn oracle_full_duplex_rows() {
    // r11 DirectSoundFullDuplexCreate dsfd=set outer=0 create=0 init=0 -> ret=0 out=OBJ init=1 release=0
    let r = run(
        Entry::FullDuplexCreate,
        Some(None),
        false,
        Ok(OBJECT),
        DS_OK,
    );
    assert_eq!(r.ret, 0);
    assert_eq!(r.out, Some(Some(OBJECT)));
    assert_eq!((r.initializes, r.releases), (1, 0));

    // r12 DirectSoundFullDuplexCreate dsfd=set outer=0 create=0 init=E_FAIL -> ret=0x80004005 out=NULL init=1 release=1
    let r = run(
        Entry::FullDuplexCreate,
        Some(None),
        false,
        Ok(OBJECT),
        E_FAIL,
    );
    assert_eq!(r.ret, 0x8000_4005);
    assert_eq!(r.out, Some(None));
    assert_eq!((r.initializes, r.releases), (1, 1));
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // r13 DllGetClassObject ppv=NULL -> ret=0x80070057
    assert_eq!(
        dll_get_class_object(&CLASS_FACTORIES[0].clsid, &IID_UNKNOWN, None),
        0x8007_0057
    );

    // r14 DllGetClassObject riid=IDirectSound clsid=DirectSound -> ret=0x80004002 out=NULL
    let mut ppv = None;
    let ret = dll_get_class_object(&CLASS_FACTORIES[0].clsid, &IID_IDIRECTSOUND, Some(&mut ppv));
    assert_eq!(ret, 0x8000_4002);
    assert!(ppv.is_none());

    // r15 DllGetClassObject riid=IUnknown clsid=DirectSound -> ret=0 out=FACTORY (class index 0)
    let mut ppv = None;
    let ret = dll_get_class_object(&CLASS_FACTORIES[0].clsid, &IID_UNKNOWN, Some(&mut ppv));
    assert_eq!(ret, 0);
    assert_eq!(factory_index(ppv), Some(0));

    // r16 DllGetClassObject riid=IClassFactory clsid=DirectSoundPrivate -> ret=0 out=FACTORY (class index 5)
    let mut ppv = None;
    let ret = dll_get_class_object(
        &CLASS_FACTORIES[5].clsid,
        &IID_CLASS_FACTORY,
        Some(&mut ppv),
    );
    assert_eq!(ret, 0);
    assert_eq!(factory_index(ppv), Some(5));

    // r17 DllGetClassObject riid=IClassFactory clsid=zero(unknown) -> ret=0x80040111 out=NULL
    let mut ppv = None;
    let ret = dll_get_class_object(&ZERO_GUID, &IID_CLASS_FACTORY, Some(&mut ppv));
    assert_eq!(ret, 0x8004_0111);
    assert!(ppv.is_none());
}

#[test]
fn oracle_recalc_vol_pan_rows() {
    // r18 DSOUND_RecalcVolPan vol=-10000 pan=0 -> amp0=0 amp1=0
    let volpan = recalc_from(-10000, 0);
    assert_eq!(volpan.total_amp_factor[0], 0);
    assert_eq!(volpan.total_amp_factor[1], 0);

    // r19 DSOUND_RecalcVolPan vol=-600 pan=300 -> amp0=23170 amp1=32767
    let volpan = recalc_from(-600, 300);
    assert_eq!(volpan.total_amp_factor[0], 23170);
    assert_eq!(volpan.total_amp_factor[1], 32767);

    // r38 DSOUND_RecalcVolPan vol=-600 pan=-300 -> amp0=32767 amp1=23170
    let volpan = recalc_from(-600, -300);
    assert_eq!(volpan.total_amp_factor[0], 32767);
    assert_eq!(volpan.total_amp_factor[1], 23170);
}

#[test]
fn oracle_amp_factor_to_vol_pan_rows() {
    // r20 DSOUND_AmpFactorToVolPan amp0=65535 amp1=32768 -> vol=0 pan=-599
    let volpan = amp_to_vol_pan(65535, 32768);
    assert_eq!((volpan.volume, volpan.pan), (0, -599));

    // r21 DSOUND_AmpFactorToVolPan amp0=0 amp1=65535 -> vol=0 pan=10000
    let volpan = amp_to_vol_pan(0, 65535);
    assert_eq!((volpan.volume, volpan.pan), (0, 10000));

    // r39 DSOUND_AmpFactorToVolPan amp0=4294967295 amp1=0 -> vol=9600 pan=-10000
    let volpan = amp_to_vol_pan(4_294_967_295, 0);
    assert_eq!((volpan.volume, volpan.pan), (9600, -10000));
}

#[test]
fn oracle_buf_ptr_diff_rows() {
    // r22 DSOUND_BufPtrDiff buflen=100 ptr1=10 ptr2=30 -> 80
    assert_eq!(buf_ptr_diff(100, 10, 30), 80);

    // r40 DSOUND_BufPtrDiff buflen=4294967295 ptr1=5 ptr2=10 -> 4294967290
    assert_eq!(buf_ptr_diff(4_294_967_295, 5, 10), 4_294_967_290);
}

#[test]
fn oracle_sample_reader_rows() {
    // r23 get8 bytes=00,80,ff,40 ch=0 -> -1 0 0.9921875 -0.5
    let mut dst = [0.0f32; 4];
    get8(&[0x00, 0x80, 0xff, 0x40], &mut dst, 1, 0);
    assert_eq!(dst, [-1.0, 0.0, 0.9921875, -0.5]);

    // r24 get16 ch=1/2 bytes@2=00,80 @6=ff,7f -> -1 0.999969482
    let mut dst = [0.0f32; 2];
    get16(&[0, 0, 0x00, 0x80, 0, 0, 0xff, 0x7f], &mut dst, 2, 1);
    assert_eq!(dst, [-1.0, 32767.0 / 32768.0]);

    // r25 get24 bytes=12,34,56 -> 0.67346406 (bits 0x3f2c6824)
    let mut dst = [0.0f32; 1];
    get24(&[0x12, 0x34, 0x56], &mut dst, 1, 0);
    assert_eq!(dst[0].to_bits(), 0x3f2c_6824);

    // r26 get24 bytes=00,00,80 is excluded: UBSan flags the signed shift in body_mixer.c:87.

    // r27 get32 INT_MIN,INT_MAX -> -1 1
    let mut dst = [0.0f32; 2];
    get32(&[0, 0, 0, 0x80, 0xff, 0xff, 0xff, 0x7f], &mut dst, 1, 0);
    assert_eq!(dst, [-1.0, 1.0]);

    // r28 getieee32 0x3fc00000 -> 1.5
    let mut dst = [0.0f32; 1];
    getieee32(&[0, 0, 0xc0, 0x3f], &mut dst, 1, 0);
    assert_eq!(dst, [1.5]);
}

#[test]
fn oracle_scalar_and_vector_product_rows() {
    let inf = f32::INFINITY;

    // r29 ScalarProduct (1,2,3).(4,5,6) -> 32 bits=0x42000000
    let c = scalar_product(&v(1.0, 2.0, 3.0), &v(4.0, 5.0, 6.0));
    assert_eq!(c.to_bits(), 0x4200_0000);

    // r30 ScalarProduct (inf,0,0).(0,0,0) -> 0 bits=0x00000000
    let c = scalar_product(&v(inf, 0.0, 0.0), &v(0.0, 0.0, 0.0));
    assert_eq!(c.to_bits(), 0);

    // r31 VectorProduct (1,0,0)x(0,1,0) -> (0,0,1) bits=0x00000000,0x00000000,0x3f800000
    let c = vector_product(&v(1.0, 0.0, 0.0), &v(0.0, 1.0, 0.0));
    assert_eq!(bits3(c), [0, 0, 0x3f80_0000]);

    // r41 VectorProduct (1,0,inf)x(0,1,1) -> (0,0,0)
    let c = vector_product(&v(1.0, 0.0, inf), &v(0.0, 1.0, 1.0));
    assert_eq!(bits3(c), [0, 0, 0]);
}

#[test]
fn oracle_magnitude_and_projection_rows() {
    // r34 VectorMagnitude (3,4,0) -> 5 bits=0x40a00000
    let m = vector_magnitude(&v(3.0, 4.0, 0.0));
    assert_eq!(m.to_bits(), 0x40a0_0000);

    // r33 ProjectVector (1,0,0) on (0,0,0) -> 0 bits=0x00000000
    let p = project_vector(&v(1.0, 0.0, 0.0), &v(0.0, 0.0, 0.0));
    assert_eq!(p.to_bits(), 0);

    // r51 ProjectVector (1,2,3) on (4,5,6) -> 3.64673853 bits=0x4069642a
    let p = project_vector(&v(1.0, 2.0, 3.0), &v(4.0, 5.0, 6.0));
    assert_eq!(p.to_bits(), 0x4069_642a);
}

#[test]
fn oracle_angle_and_point_rows() {
    let inf = f32::INFINITY;

    // r32 AngleBetweenVectorsDeg (1,0,0),(0,1,0) -> 90 bits=0x42b40000
    let a = angle_between_vectors_deg(&v(1.0, 0.0, 0.0), &v(0.0, 1.0, 0.0));
    assert_eq!(a.to_bits(), 0x42b4_0000);

    // r43 AngleBetweenVectorsRad (0.1,0.1,0.1),(0.1,0.1,0.1) -> 0 bits=0x00000000
    let a = v(0.1, 0.1, 0.1);
    assert_eq!(angle_between_vectors_rad(&a, &a).to_bits(), 0);

    // r44 AngleBetweenVectorsRad (0.1,0.1,0.1),(-0.1,-0.1,-0.1) -> 3.14159274 bits=0x40490fdb
    let b = v(-0.1, -0.1, -0.1);
    assert_eq!(angle_between_vectors_rad(&a, &b).to_bits(), 0x4049_0fdb);

    // r45 AngleBetweenVectorsDeg (0,0,0),(1,0,0) -> 0 bits=0x00000000
    let a = angle_between_vectors_deg(&v(0.0, 0.0, 0.0), &v(1.0, 0.0, 0.0));
    assert_eq!(a.to_bits(), 0);

    // r42 VectorBetweenTwoPoints A=(inf,0,0) B=(inf,1,1) -> (0,0,0)
    let d = vector_between_two_points(&v(inf, 0.0, 0.0), &v(inf, 1.0, 1.0));
    assert_eq!(bits3(d), [0, 0, 0]);
}

#[test]
fn oracle_speaker_rows() {
    // r35 speaker_config_to_channel_mask DSSPEAKER_STEREO -> 0x3
    assert_eq!(speaker_config_to_channel_mask(DSSPEAKER_STEREO), 0x3);

    // r36 speaker_config_to_channel_mask 0x00140008 (7.1 surround) -> 0x63f
    assert_eq!(speaker_config_to_channel_mask(0x0014_0008), 0x63f);

    // r37 speaker_config_to_channel_mask DSSPEAKER_SURROUND (unmapped) -> 0x3
    assert_eq!(speaker_config_to_channel_mask(DSSPEAKER_SURROUND), 0x3);

    // r46 speaker_config_to_channel_mask DSSPEAKER_MONO -> 0x1
    assert_eq!(speaker_config_to_channel_mask(DSSPEAKER_MONO), 0x1);

    // r47 speaker_config_to_channel_mask DSSPEAKER_HEADPHONE -> 0x3
    assert_eq!(speaker_config_to_channel_mask(DSSPEAKER_HEADPHONE), 0x3);

    // r48 speaker_config_to_channel_mask DSSPEAKER_QUAD -> 0x33
    assert_eq!(speaker_config_to_channel_mask(DSSPEAKER_QUAD), 0x33);

    // r49 speaker_config_to_channel_mask DSSPEAKER_5POINT1_BACK -> 0x3f
    assert_eq!(speaker_config_to_channel_mask(DSSPEAKER_5POINT1_BACK), 0x3f);

    // r50 speaker_config_to_channel_mask DSSPEAKER_7POINT1_WIDE -> 0xff
    assert_eq!(speaker_config_to_channel_mask(DSSPEAKER_7POINT1_WIDE), 0xff);
}
