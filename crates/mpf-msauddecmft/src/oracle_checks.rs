//! Oracle-derived test vectors for the ported msauddecmft logic.
//!
//! The vectors are the 25 rows printed by a harness that compiles the verbatim Wine bodies of
//! `dlls/msauddecmft/msauddecmft.c` (lines 38-133) and `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`
//! (lines 35-38) with `gcc -fsanitize=address,undefined -g`, under `target/canon/msauddecmft/`. The shims there
//! replace the Windows headers and record the OS calls. No row was flagged by ASan or UBSan, so no vector is excluded.

use super::*;

const E_FAIL: Hresult = 0x8000_4005u32 as i32;
const E_ACCESSDENIED: Hresult = 0x8007_0005u32 as i32;
const REGDB_E_CLASSNOTREG: Hresult = 0x8004_0154u32 as i32;

const CLSID_WG: &str = "e7889a8a-2083-4844-8370-5ee349b14503";
const CLSID_MFT: &str = "32d186a7-218f-4c75-8876-dd77273a8999";
const CATEGORY_DECODER: &str = "9ea73fb4-ef7a-4559-8d5d-719d8f0426c7";
const IID_UNKNOWN_TEXT: &str = "00000000-0000-0000-c000-000000000046";
const IID_CLASS_FACTORY_TEXT: &str = "00000001-0000-0000-c000-000000000046";
const IID_NEAR_TEXT: &str = "00000002-0000-0000-c000-000000000046";
const GUID_NULL_TEXT: &str = "00000000-0000-0000-0000-000000000000";
const MAJOR_AUDIO: &str = "73647561-0000-0010-8000-00aa00389b71";
const SUB_AAC: &str = "00001610-0000-0010-8000-00aa00389b71";
const SUB_RAW_AAC: &str = "000000ff-0000-0010-8000-00aa00389b71";
const SUB_ADTS: &str = "00001600-0000-0010-8000-00aa00389b71";
const SUB_FLOAT: &str = "00000003-0000-0010-8000-00aa00389b71";
const SUB_PCM: &str = "00000001-0000-0010-8000-00aa00389b71";

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
    let ret = aac_decoder_factory_create_instance(riid, |clsid, clsctx, iid| {
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

/// The `MFTRegister` arguments that the harness printed for the AAC decoder.
fn assert_oracle_registration(reg: &MftRegistration) {
    assert_eq!(reg.clsid, guid_of(CLSID_MFT));
    assert_eq!(reg.category, guid_of(CATEGORY_DECODER));
    assert_eq!(reg.name, "Microsoft AAC Audio Decoder MFT");
    assert_eq!(reg.flags, 0x1);
    assert_eq!(
        reg.inputs,
        [
            info(MAJOR_AUDIO, SUB_AAC),
            info(MAJOR_AUDIO, SUB_RAW_AAC),
            info(MAJOR_AUDIO, SUB_ADTS),
        ]
    );
    assert_eq!(
        reg.outputs,
        [info(MAJOR_AUDIO, SUB_FLOAT), info(MAJOR_AUDIO, SUB_PCM)]
    );
}

#[test]
fn oracle_create_instance_rows() {
    // row 01 CreateInstance outer=NULL riid=IID_IUnknown co_ret=S_OK: ret=0x00000000, one CoCreateInstance
    // call with CLSID_wg_aac_decoder, CLSCTX_INPROC_SERVER (0x1) and riid IID_IUnknown.
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
    // row 12 DllGetClassObject clsid=CLSID_MSAACDecMFT riid=IID_IClassFactory: ret=0x00000000 out=iface
    assert_eq!(
        dll_get_class_object(&guid_of(CLSID_MFT), &IID_CLASS_FACTORY),
        Ok(ClassFactory)
    );

    // row 13 DllGetClassObject clsid=CLSID_MSAACDecMFT riid=IID_IUnknown: ret=0x00000000 out=iface
    assert_eq!(
        dll_get_class_object(&guid_of(CLSID_MFT), &IID_UNKNOWN),
        Ok(ClassFactory)
    );

    // row 14 DllGetClassObject clsid=CLSID_MSAACDecMFT riid=GUID_NULL: ret=0x80004002 out=NULL
    assert_eq!(
        dll_get_class_object(&guid_of(CLSID_MFT), &guid_of(GUID_NULL_TEXT)),
        Err(E_NOINTERFACE)
    );

    // row 15 DllGetClassObject clsid=CLSID_wg_aac_decoder riid=IID_IClassFactory: ret=0x80040111 out=NULL
    assert_eq!(
        dll_get_class_object(&guid_of(CLSID_WG), &IID_CLASS_FACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_dll_can_unload_now_row() {
    // row 16 DllCanUnloadNow: ret=0x00000001
    assert_eq!(dll_can_unload_now(), 0x0000_0001);
}

#[test]
fn oracle_dll_register_server_rows() {
    // row 17 DllRegisterServer res=S_OK mft=S_OK: ret=0x00000000, one resources call, one MFTRegister call
    let (ret, resource_calls, seen) = register(S_OK, S_OK);
    assert_eq!(ret, 0x0000_0000);
    assert_eq!(resource_calls, 1);
    assert_eq!(seen.len(), 1);
    assert_oracle_registration(&seen[0]);

    // row 18 DllRegisterServer res=E_FAIL mft=S_OK: ret=0x80004005, MFTRegister not called
    let (ret, resource_calls, seen) = register(E_FAIL, S_OK);
    assert_eq!(ret, E_FAIL);
    assert_eq!(resource_calls, 1);
    assert!(seen.is_empty());

    // row 19 DllRegisterServer res=S_OK mft=E_ACCESSDENIED: ret=0x80070005
    let (ret, resource_calls, seen) = register(S_OK, E_ACCESSDENIED);
    assert_eq!(ret, E_ACCESSDENIED);
    assert_eq!(resource_calls, 1);
    assert_eq!(seen.len(), 1);
    assert_oracle_registration(&seen[0]);

    // row 20 DllRegisterServer res=S_FALSE mft=S_OK: ret=0x00000000, S_FALSE does not fail
    let (ret, resource_calls, seen) = register(S_FALSE, S_OK);
    assert_eq!(ret, S_OK);
    assert_eq!(resource_calls, 1);
    assert_eq!(seen.len(), 1);
    assert_oracle_registration(&seen[0]);

    // row 21 DllRegisterServer res=S_OK mft=S_FALSE: ret=0x00000000
    let (ret, resource_calls, seen) = register(S_OK, S_FALSE);
    assert_eq!(ret, S_OK);
    assert_eq!(resource_calls, 1);
    assert_eq!(seen.len(), 1);
    assert_oracle_registration(&seen[0]);
}

#[test]
fn oracle_dll_unregister_server_rows() {
    // row 22 DllUnregisterServer res=S_OK mft=S_OK: ret=0x00000000, MFTUnregister gets CLSID_MSAACDecMFT
    let (ret, resource_calls, clsids) = unregister(S_OK, S_OK);
    assert_eq!(ret, S_OK);
    assert_eq!(resource_calls, 1);
    assert_eq!(clsids, [guid_of(CLSID_MFT)]);

    // row 23 DllUnregisterServer res=E_FAIL mft=S_OK: ret=0x80004005, MFTUnregister not called
    let (ret, resource_calls, clsids) = unregister(E_FAIL, S_OK);
    assert_eq!(ret, E_FAIL);
    assert_eq!(resource_calls, 1);
    assert!(clsids.is_empty());

    // row 24 DllUnregisterServer res=S_OK mft=E_ACCESSDENIED: ret=0x80070005
    let (ret, resource_calls, clsids) = unregister(S_OK, E_ACCESSDENIED);
    assert_eq!(ret, E_ACCESSDENIED);
    assert_eq!(resource_calls, 1);
    assert_eq!(clsids, [guid_of(CLSID_MFT)]);

    // row 25 DllUnregisterServer res=S_FALSE mft=S_FALSE: ret=0x00000000
    let (ret, resource_calls, clsids) = unregister(S_FALSE, S_FALSE);
    assert_eq!(ret, S_OK);
    assert_eq!(resource_calls, 1);
    assert_eq!(clsids, [guid_of(CLSID_MFT)]);
}
