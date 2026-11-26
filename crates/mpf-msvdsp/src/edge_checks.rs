//! Edge-case tests for the ported msvdsp logic.
//!
//! These cover the `FAILED` boundary, the `checked` step, the table shapes and contents, the
//! runtime GUID helpers, and the `DllGetClassObject` out-parameter. The expectations here are not
//! oracle-derived; each one says so in a comment.

use super::*;
use crate::guid::*;
use crate::hresult::checked;
use crate::register::*;

#[test]
fn hresult_values_are_pinned() {
    // Not oracle-derived: the values come from include/winerror.h.
    assert_eq!(S_OK, 0x0000_0000);
    assert_eq!(S_FALSE, 0x0000_0001);
    assert_eq!(E_FAIL, 0x8000_4005);
    assert_eq!(E_ACCESSDENIED, 0x8007_0005);
    assert_eq!(E_OUTOFMEMORY, 0x8007_000e);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
}

#[test]
fn failed_tests_the_severity_bit() {
    // Not oracle-derived: FAILED() is true exactly when the sign bit is set (include/winerror.h).
    assert!(!failed(S_OK));
    assert!(!failed(S_FALSE));
    assert!(!failed(0x7fff_ffff));
    assert!(failed(0x8000_0000));
    assert!(failed(u32::MAX));
}

#[test]
fn checked_turns_failed_into_err() {
    // Not oracle-derived: the Result form of FAILED() at the same boundaries.
    assert_eq!(checked(S_OK), Ok(()));
    assert_eq!(checked(S_FALSE), Ok(()));
    assert_eq!(checked(0x7fff_ffff), Ok(()));
    assert_eq!(checked(0x8000_0000), Err(0x8000_0000));
    assert_eq!(checked(E_FAIL), Err(E_FAIL));
}

#[test]
fn color_converter_table_lengths() {
    // Not oracle-derived: counted from the initialiser lists in msvdsp.c.
    assert_eq!(COLOR_CONVERTER_MFT_INPUTS.len(), 20);
    assert_eq!(COLOR_CONVERTER_MFT_OUTPUTS.len(), 16);
    assert_eq!(COLOR_CONVERTER_DMO_INPUTS.len(), 20);
    assert_eq!(COLOR_CONVERTER_DMO_OUTPUTS.len(), 16);
}

#[test]
fn outputs_are_the_first_sixteen_inputs() {
    // Not oracle-derived: msvdsp.c lists each output array as the first 16 input entries.
    assert!(COLOR_CONVERTER_MFT_INPUTS[..16] == COLOR_CONVERTER_MFT_OUTPUTS);
    assert!(COLOR_CONVERTER_DMO_INPUTS[..16] == COLOR_CONVERTER_DMO_OUTPUTS);
}

#[test]
fn major_types_are_video() {
    // Not oracle-derived: MFT entries use MFMediaType_Video and DMO entries use MEDIATYPE_Video.
    assert!(COLOR_CONVERTER_MFT_INPUTS
        .iter()
        .all(|t| t.major_type == MF_MEDIA_TYPE_VIDEO));
    assert!(COLOR_CONVERTER_MFT_OUTPUTS
        .iter()
        .all(|t| t.major_type == MF_MEDIA_TYPE_VIDEO));
    assert!(COLOR_CONVERTER_DMO_INPUTS
        .iter()
        .all(|t| t.major_type == MEDIATYPE_VIDEO));
    assert!(COLOR_CONVERTER_DMO_OUTPUTS
        .iter()
        .all(|t| t.major_type == MEDIATYPE_VIDEO));
}

#[test]
fn rgb_entries_use_format_guids_in_mft_tables_and_subtypes_in_dmo_tables() {
    // Not oracle-derived: msvdsp.c names RGB entries DMOVideoFormat_RGB* in the MFT tables and
    // MEDIASUBTYPE_RGB* in the DMO tables. The indices are the RGB positions in the input list.
    let rgb: [(usize, Guid, Guid); 5] = [
        (5, DMO_VIDEO_FORMAT_RGB32, MEDIASUBTYPE_RGB32),
        (6, DMO_VIDEO_FORMAT_RGB565, MEDIASUBTYPE_RGB565),
        (10, DMO_VIDEO_FORMAT_RGB24, MEDIASUBTYPE_RGB24),
        (11, DMO_VIDEO_FORMAT_RGB555, MEDIASUBTYPE_RGB555),
        (12, DMO_VIDEO_FORMAT_RGB8, MEDIASUBTYPE_RGB8),
    ];
    for &(i, mft, dmo) in &rgb {
        assert!(COLOR_CONVERTER_MFT_INPUTS[i].subtype == mft);
        assert!(COLOR_CONVERTER_MFT_OUTPUTS[i].subtype == mft);
        assert!(COLOR_CONVERTER_DMO_INPUTS[i].subtype == dmo);
        assert!(COLOR_CONVERTER_DMO_OUTPUTS[i].subtype == dmo);
    }
}

#[test]
fn guid_helpers_build_the_base_layouts() {
    // Not oracle-derived: the expected layouts are the MF base GUID (include/mfapi.h:48-51) and
    // the DMO base GUID (include/dmoreg.h, include/uuids.h), written out here.
    let mf_base = Guid {
        data1: 0x7364_6976,
        data2: 0x0000,
        data3: 0x0010,
        data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
    };
    let dmo_base = Guid {
        data1: 22,
        data2: 0x524f,
        data3: 0x11ce,
        data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
    };
    assert!(mf_guid(0x7364_6976) == mf_base);
    assert!(mf_guid(0x3231_564e) == MF_VIDEO_FORMAT_NV12);
    assert!(dmo_guid(22) == dmo_base);
    assert!(dmo_guid(0xe436_eb7e) == MEDIASUBTYPE_RGB32);
}

#[test]
fn get_class_object_leaves_a_none_out_as_none() {
    // Not oracle-derived: the oracle starts from a non-NULL sentinel; this starts the
    // out-parameter at None.
    let mut out: Option<u8> = None;
    let hr = dll_get_class_object(
        &CLSID_C_FRAME_RATE_CONVERT_DMO,
        &CLSID_C_COLOR_CONTROL_DMO,
        &mut out,
    );
    assert_eq!(hr, CLASS_E_CLASSNOTAVAILABLE);
    assert!(out.is_none());
}
