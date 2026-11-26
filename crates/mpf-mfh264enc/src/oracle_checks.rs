//! Oracle-derived test vectors for the ported mfh264enc logic.
//!
//! The vectors are the 25 rows printed by a harness that compiles the verbatim Wine bodies of
//! `dlls/mfh264enc/mfh264enc.c` (lines 34-129) and `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`
//! with `gcc -fsanitize=address,undefined -g`, under `target/canon/mfh264enc/`. The shims there replace the
//! Windows headers and record the OS calls. No row was flagged by ASan or UBSan, so no vector is excluded.

use super::*;

const E_FAIL: Hresult = 0x8000_4005u32 as i32;
const E_ACCESSDENIED: Hresult = 0x8007_0005u32 as i32;
const REGDB_E_CLASSNOTREG: Hresult = 0x8004_0154u32 as i32;

const CLSID_WG: &str = "6c34de69-4670-46cd-8cb4-1f2fa1dffb65";
const CLSID_MFT: &str = "6ca50344-051a-4ded-9779-a43305165e35";
const CATEGORY_ENCODER: &str = "f79eac7d-e545-4387-bdee-d647d7bde42a";
const IID_UNKNOWN_TEXT: &str = "00000000-0000-0000-c000-000000000046";
const IID_CLASS_FACTORY_TEXT: &str = "00000001-0000-0000-c000-000000000046";
const IID_NEAR_TEXT: &str = "00000002-0000-0000-c000-000000000046";
const GUID_NULL_TEXT: &str = "00000000-0000-0000-0000-000000000000";
const MAJOR_VIDEO: &str = "73646976-0000-0010-8000-00aa00389b71";
const SUB_IYUV: &str = "56555949-0000-0010-8000-00aa00389b71";
const SUB_YV12: &str = "32315659-0000-0010-8000-00aa00389b71";
const SUB_NV12: &str = "3231564e-0000-0010-8000-00aa00389b71";
const SUB_YUY2: &str = "32595559-0000-0010-8000-00aa00389b71";
const SUB_H264: &str = "34363248-0000-0010-8000-00aa00389b71";

/// Parses the `XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX` text that the harness prints.
fn guid_of(text: &str) -> Guid {
    let hex = text.replace('-', "");
    let byte = |i: usize| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap();
    Guid {
        data1: u32::from_str_radix(&hex[0..8], 16).unwrap(),
        data2: u16::from_str_radix(&hex[8..12], 16).unwrap(),
        data3: u16::from_str_radix(&hex[12..16], 16).unwrap(),
        data4: [
            byte(8),
            byte(9),
            byte(10),
            byte(11),
            byte(12),
            byte(13),
            byte(14),
            byte(15),
        ],
    }
}

fn info(major: &str, subtype: &str) -> MftRegisterTypeInfo {
    MftRegisterTypeInfo {
        major_type: guid_of(major),
        subtype: guid_of(subtype),
    }
}

/// Calls `CreateInstance` with a fake `CoCreateInstance` that returns `co_ret`. Returns the result and the
/// `(clsid, clsctx, riid)` of each call.
fn create_instance(riid: &Guid, co_ret: Hresult) -> (Hresult, Vec<(Guid, u32, Guid)>) {
    let mut calls = Vec::new();
    let ret = h264_encoder_factory_create_instance(riid, |clsid, clsctx, iid| {
        calls.push((*clsid, clsctx, *iid));
        co_ret
    });
    (ret, calls)
}

/// Calls `DllRegisterServer` with fake OS calls. Returns the result, the `__wine_register_resources` call
/// count, and the `MFTRegister` registrations seen.
fn register(resources_ret: Hresult, mft_ret: Hresult) -> (Hresult, u32, Vec<MftRegistration>) {
    let mut resource_calls = 0;
    let mut seen = Vec::new();
    let ret = dll_register_server(
        || {
            resource_calls += 1;
            resources_ret
        },
        |reg| {
            seen.push(reg.clone());
            mft_ret
        },
    );
    (ret, resource_calls, seen)
}

/// Calls `DllUnregisterServer` with fake OS calls. Returns the result, the `__wine_unregister_resources` call
/// count, and the CLSIDs passed to `MFTUnregister`.
fn unregister(resources_ret: Hresult, mft_ret: Hresult) -> (Hresult, u32, Vec<Guid>) {
    let mut resource_calls = 0;
    let mut seen = Vec::new();
    let ret = dll_unregister_server(
        || {
            resource_calls += 1;
            resources_ret
        },
        |clsid| {
            seen.push(*clsid);
            mft_ret
        },
    );
    (ret, resource_calls, seen)
}

/// The `MFTRegister` arguments that the harness printed for the H.264 encoder.
fn assert_oracle_registration(reg: &MftRegistration) {
    assert_eq!(reg.clsid, guid_of(CLSID_MFT));
    assert_eq!(reg.category, guid_of(CATEGORY_ENCODER));
    assert_eq!(reg.name, "H264 Encoder MFT");
    assert_eq!(reg.flags, 0x1);
    assert_eq!(
        reg.inputs,
        [
            info(MAJOR_VIDEO, SUB_IYUV),
            info(MAJOR_VIDEO, SUB_YV12),
            info(MAJOR_VIDEO, SUB_NV12),
            info(MAJOR_VIDEO, SUB_YUY2),
        ]
    );
    assert_eq!(reg.outputs, [info(MAJOR_VIDEO, SUB_H264)]);
}

#[test]
fn oracle_create_instance_rows() {
    // row 01 CreateInstance outer=NULL riid=IID_IUnknown co_ret=S_OK: ret=0x00000000, one CoCreateInstance
    // call with CLSID_wg_h264_encoder, CLSCTX_INPROC_SERVER (0x1) and riid IID_IUnknown.
    let (ret, calls) = create_instance(&IID_UNKNOWN, S_OK);
    assert_eq!(ret, 0x0000_0000);
    assert_eq!(calls, [(guid_of(CLSID_WG), 0x1, guid_of(IID_UNKNOWN_TEXT))]);

    // row 02 CreateInstance outer=non-NULL riid=IID_IClassFactory co_ret=REGDB_E_CLASSNOTREG: ret=0x80040154.
    // The outer argument is not modelled.
    let (ret, calls) = create_instance(&IID_CLASS_FACTORY, REGDB_E_CLASSNOTREG);
    assert_eq!(ret, REGDB_E_CLASSNOTREG);
    assert_eq!(
        calls,
        [(guid_of(CLSID_WG), 0x1, guid_of(IID_CLASS_FACTORY_TEXT))]
    );

    // row 03 CreateInstance outer=NULL riid=GUID_NULL co_ret=E_NOINTERFACE: ret=0x80004002
    let (ret, calls) = create_instance(&guid_of(GUID_NULL_TEXT), E_NOINTERFACE);
    assert_eq!(ret, E_NOINTERFACE);
    assert_eq!(calls, [(guid_of(CLSID_WG), 0x1, guid_of(GUID_NULL_TEXT))]);
}

#[test]
fn oracle_class_factory_rows() {
    // row 04 QueryInterface riid=IID_IClassFactory: ret=0x00000000 out=iface
    assert_eq!(
        class_factory_query_interface(&IID_CLASS_FACTORY),
        Ok(ClassFactory)
    );

    // row 05 QueryInterface riid=IID_IUnknown: ret=0x00000000 out=iface
    assert_eq!(
        class_factory_query_interface(&IID_UNKNOWN),
        Ok(ClassFactory)
    );

    // row 06 QueryInterface riid=GUID_NULL: ret=0x80004002 out=NULL
    assert_eq!(
        class_factory_query_interface(&guid_of(GUID_NULL_TEXT)),
        Err(E_NOINTERFACE)
    );

    // row 07 QueryInterface riid=00000002-0000-0000-C000-000000000046: ret=0x80004002 out=NULL
    assert_eq!(
        class_factory_query_interface(&guid_of(IID_NEAR_TEXT)),
        Err(E_NOINTERFACE)
    );

    // row 08 AddRef: ret=2
    assert_eq!(class_factory_add_ref(), 2);

    // row 09 Release: ret=1
    assert_eq!(class_factory_release(), 1);

    // row 10 LockServer dolock=FALSE: ret=0x00000000
    assert_eq!(class_factory_lock_server(false), S_OK);

    // row 11 LockServer dolock=TRUE: ret=0x00000000
    assert_eq!(class_factory_lock_server(true), S_OK);
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 12 clsid=CLSID_MSH264EncoderMFT riid=IID_IClassFactory: ret=0x00000000 out=iface
    assert_eq!(
        dll_get_class_object(&guid_of(CLSID_MFT), &IID_CLASS_FACTORY),
        Ok(ClassFactory)
    );

    // row 13 clsid=CLSID_MSH264EncoderMFT riid=IID_IUnknown: ret=0x00000000 out=iface
    assert_eq!(
        dll_get_class_object(&guid_of(CLSID_MFT), &IID_UNKNOWN),
        Ok(ClassFactory)
    );

    // row 14 clsid=CLSID_MSH264EncoderMFT riid=GUID_NULL: ret=0x80004002 out=NULL
    assert_eq!(
        dll_get_class_object(&guid_of(CLSID_MFT), &guid_of(GUID_NULL_TEXT)),
        Err(E_NOINTERFACE)
    );

    // row 15 clsid=CLSID_wg_h264_encoder riid=IID_IClassFactory: ret=0x80040111 out=NULL
    assert_eq!(
        dll_get_class_object(&guid_of(CLSID_WG), &IID_CLASS_FACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_dll_can_unload_now_row() {
    // row 16 DllCanUnloadNow: ret=0x00000001
    assert_eq!(dll_can_unload_now(), S_FALSE);
}

#[test]
fn oracle_dll_register_server_rows() {
    // row 17 res=S_OK mft=S_OK: ret=0x00000000 res_calls=1 mft_calls=1, registration checked below
    let (ret, resource_calls, seen) = register(S_OK, S_OK);
    assert_eq!((ret, resource_calls), (S_OK, 1));
    assert_eq!(seen.len(), 1);
    assert_oracle_registration(&seen[0]);

    // row 18 res=E_FAIL mft=S_OK: ret=0x80004005 res_calls=1 mft_calls=0
    let (ret, resource_calls, seen) = register(E_FAIL, S_OK);
    assert_eq!((ret, resource_calls), (E_FAIL, 1));
    assert!(seen.is_empty());

    // row 19 res=S_OK mft=E_ACCESSDENIED: ret=0x80070005 res_calls=1 mft_calls=1
    let (ret, resource_calls, seen) = register(S_OK, E_ACCESSDENIED);
    assert_eq!((ret, resource_calls), (E_ACCESSDENIED, 1));
    assert_eq!(seen.len(), 1);
    assert_oracle_registration(&seen[0]);

    // row 20 res=S_FALSE mft=S_OK: ret=0x00000000 res_calls=1 mft_calls=1
    let (ret, resource_calls, seen) = register(S_FALSE, S_OK);
    assert_eq!((ret, resource_calls), (S_OK, 1));
    assert_eq!(seen.len(), 1);
    assert_oracle_registration(&seen[0]);

    // row 21 res=S_OK mft=S_FALSE: ret=0x00000000 res_calls=1 mft_calls=1
    let (ret, resource_calls, seen) = register(S_OK, S_FALSE);
    assert_eq!((ret, resource_calls), (S_OK, 1));
    assert_eq!(seen.len(), 1);
    assert_oracle_registration(&seen[0]);
}

#[test]
fn oracle_dll_unregister_server_rows() {
    // row 22 res=S_OK mft=S_OK: ret=0x00000000 unres_calls=1 mft_unregister_calls=1 clsid=CLSID_MSH264EncoderMFT
    let (ret, resource_calls, seen) = unregister(S_OK, S_OK);
    assert_eq!((ret, resource_calls), (S_OK, 1));
    assert_eq!(seen, [guid_of(CLSID_MFT)]);

    // row 23 res=E_FAIL mft=S_OK: ret=0x80004005 unres_calls=1 mft_unregister_calls=0
    let (ret, resource_calls, seen) = unregister(E_FAIL, S_OK);
    assert_eq!((ret, resource_calls), (E_FAIL, 1));
    assert!(seen.is_empty());

    // row 24 res=S_OK mft=E_ACCESSDENIED: ret=0x80070005 unres_calls=1 mft_unregister_calls=1
    let (ret, resource_calls, seen) = unregister(S_OK, E_ACCESSDENIED);
    assert_eq!((ret, resource_calls), (E_ACCESSDENIED, 1));
    assert_eq!(seen, [guid_of(CLSID_MFT)]);

    // row 25 res=S_FALSE mft=S_FALSE: ret=0x00000000 unres_calls=1 mft_unregister_calls=1
    let (ret, resource_calls, seen) = unregister(S_FALSE, S_FALSE);
    assert_eq!((ret, resource_calls), (S_OK, 1));
    assert_eq!(seen, [guid_of(CLSID_MFT)]);
}
