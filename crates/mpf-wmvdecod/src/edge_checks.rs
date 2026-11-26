//! Edge-case tests for the ported `wmvdecod` logic.
//!
//! These cover the `FAILED` boundary, the GUID comparison, the class factory dispatch, the
//! registration sequences past their first step, and the constant helpers. Expectations the oracle
//! does not print are marked "not oracle-derived" in a comment.

use super::*;
use crate::oracle_checks::{full_register_calls, full_unregister_calls, mft_call, Call, Recorder};

#[test]
fn failed_boundaries() {
    // Not oracle-derived: `FAILED` is a sign test, so only the sign bit decides.
    assert!(!failed(i32::MAX));
    assert!(!failed(1));
    assert!(!failed(S_OK));
    assert!(failed(-1));
    assert!(failed(i32::MIN));
    assert!(failed(E_FAIL));
    assert!(failed(E_OUTOFMEMORY));
    // Row 18 shows S_FALSE does not fail.
    assert!(!failed(S_FALSE));
}

#[test]
fn hresult_values_match_printed_rows() {
    // Each value is printed as `hr=` or `ret=` in the oracle rows named in the comments.
    assert_eq!(S_OK, 0); // rows 01 and 14
    assert_eq!(S_FALSE, 1); // rows 18 and 25
    assert_eq!(E_FAIL, 0x8000_4005u32 as i32); // rows 17, 20, 22
    assert_eq!(E_NOINTERFACE, 0x8000_4002u32 as i32); // rows 03 and 08
    assert_eq!(E_OUTOFMEMORY, 0x8007_000Eu32 as i32); // row 19
    assert_eq!(REGDB_E_CLASSNOTREG, 0x8004_0154u32 as i32); // row 12
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111u32 as i32); // rows 04 and 05
}

#[test]
fn is_equal_guid_single_field_differences() {
    // Not oracle-derived: each field alone differs, so the comparison must cover every byte.
    let base = CLSID_WMVDECODER_MFT;
    assert!(is_equal_guid(&base, &base));

    let mut other = base;
    other.data1 ^= 1;
    assert!(!is_equal_guid(&base, &other));

    let mut other = base;
    other.data2 ^= 1;
    assert!(!is_equal_guid(&base, &other));

    let mut other = base;
    other.data3 ^= 1;
    assert!(!is_equal_guid(&base, &other));

    let mut other = base;
    other.data4[0] ^= 1;
    assert!(!is_equal_guid(&base, &other));

    let mut other = base;
    other.data4[7] ^= 1;
    assert!(!is_equal_guid(&base, &other));
}

#[test]
fn dll_get_class_object_one_byte_differences() {
    // Not oracle-derived: a one-byte change to either argument must miss the match.
    let mut clsid = CLSID_WMVDECODER_MFT;
    clsid.data4[7] ^= 1;
    assert_eq!(
        dll_get_class_object(&clsid, &IID_ICLASSFACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );

    let mut riid = IID_ICLASSFACTORY;
    riid.data1 = 2;
    assert_eq!(
        dll_get_class_object(&CLSID_WMVDECODER_MFT, &riid),
        Err(E_NOINTERFACE)
    );

    let mut riid = IID_IUNKNOWN;
    riid.data4[7] ^= 1;
    assert_eq!(
        dll_get_class_object(&CLSID_WMVDECODER_MFT, &riid),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn h264_factory_query_interface_accepts_iunknown() {
    // Not oracle-derived: row 07 queries the H264 factory for IClassFactory only.
    assert_eq!(
        factory_query_interface(ClassFactory::H264, &IID_IUNKNOWN),
        Ok(ClassFactory::H264)
    );
    let mut riid = IID_ICLASSFACTORY;
    riid.data4[0] ^= 1;
    assert_eq!(
        factory_query_interface(ClassFactory::H264, &riid),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn factory_lock_server_false_is_s_ok() {
    // Not oracle-derived: row 11 passes TRUE only. The port ignores the flag.
    assert_eq!(factory_lock_server(false), S_OK);
}

#[test]
fn h264_create_instance_uses_inproc_server_context() {
    // Row 13 prints ctx=1 for the H264 factory. This checks the same field through the struct.
    let request = create_instance_request(ClassFactory::H264);
    assert_eq!(request.clsctx, CLSCTX_INPROC_SERVER);
    assert_eq!(CLSCTX_INPROC_SERVER, 1);
    assert_eq!(request.clsid, CLSID_WG_H264_DECODER);
}

#[test]
fn media_type_and_rgb_helpers_build_the_printed_guids() {
    // Rows 15 and 16 print these GUIDs. The constants are built by the same const fns, but these are
    // runtime calls of those fns, so their code is exercised as well.
    assert_eq!(
        media_type_guid(0x7364_6976),
        Guid::new(
            0x7364_6976,
            0x0000,
            0x0010,
            [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
        )
    );
    assert_eq!(media_type_guid(0x3156_4d57), MEDIASUBTYPE_WMV1);
    assert_eq!(media_type_guid(0x3231_564e), MEDIASUBTYPE_NV12);
    assert_eq!(rgb_guid(22), DMOVIDEOFORMAT_RGB32);
    assert_eq!(rgb_guid(0xe436_eb7a), MEDIASUBTYPE_RGB8);
    assert_eq!(
        rgb_guid(0xe436_eb7a),
        Guid::new(
            0xe436_eb7a,
            0x524f,
            0x11ce,
            [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
        )
    );
}

#[test]
fn register_steps_continue_on_s_false() {
    // Not oracle-derived for these steps: row 18 shows S_FALSE continues after the resources step,
    // and the same `FAILED` test applies to each later step.
    let mut r = Recorder::new();
    r.ret_mft_register = S_FALSE;
    assert_eq!(dll_register_server(&mut r), S_OK);
    assert_eq!(r.calls, full_register_calls());

    let mut r = Recorder::new();
    r.ret_dmo_register = S_FALSE;
    assert_eq!(dll_register_server(&mut r), S_OK);
    assert_eq!(r.calls, full_register_calls());
}

#[test]
fn register_mft_failure_stops_before_dmo() {
    // Not oracle-derived: row 19 uses E_OUTOFMEMORY, this uses E_FAIL at the same step.
    let mut r = Recorder::new();
    r.ret_mft_register = E_FAIL;
    assert_eq!(dll_register_server(&mut r), E_FAIL);
    assert_eq!(r.calls, [Call::RegisterResources, mft_call()]);
}

#[test]
fn unregister_steps_continue_on_s_false() {
    // Not oracle-derived: the unregister steps have no S_FALSE row. The same `FAILED` test applies.
    let mut r = Recorder::new();
    r.ret_unregister_resources = S_FALSE;
    assert_eq!(dll_unregister_server(&mut r), S_OK);
    assert_eq!(r.calls, full_unregister_calls());

    let mut r = Recorder::new();
    r.ret_mft_unregister = S_FALSE;
    assert_eq!(dll_unregister_server(&mut r), S_OK);
    assert_eq!(r.calls, full_unregister_calls());

    let mut r = Recorder::new();
    r.ret_dmo_unregister = S_FALSE;
    assert_eq!(dll_unregister_server(&mut r), S_OK);
    assert_eq!(r.calls, full_unregister_calls());
}

#[test]
fn unregister_resources_failure_is_the_only_call() {
    // Not oracle-derived: row 22 covers E_FAIL at this step; this covers a negative value other than it.
    let mut r = Recorder::new();
    r.ret_unregister_resources = i32::MIN;
    assert_eq!(dll_unregister_server(&mut r), i32::MIN);
    assert_eq!(r.calls, [Call::UnregisterResources]);
}
