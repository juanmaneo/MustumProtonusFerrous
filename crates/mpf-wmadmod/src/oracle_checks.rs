//! Oracle-derived test vectors for the ported wmadmod logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`, the
//! class factory, `DllRegisterServer` and `DllUnregisterServer` (`dlls/wmadmod/wmadmod.c`) and
//! the `DllCanUnloadNow` default (`libs/winecrt0/dll_canunload.c`) under
//! `target/canon/wmadmod/` with `gcc -fsanitize=address,undefined -g`, and running the harness.
//! The harness printed 25 rows, and none was excluded because ASan and UBSan reported nothing.
//! The comment above each assertion is the printed row.

use super::*;
use std::cell::RefCell;

const MFT_TOKEN: &str = "mft(WMA,MFTAudioDecoder,WMAudio Decoder MFT,1,[Audio/MSAudio1 Audio/WMAudioV8 Audio/WMAudioV9 Audio/Lossless],[Audio/PCM Audio/Float])";
const DMO_TOKEN: &str = "dmo(WMA,DMOAudioDecoder,WMAudio Decoder DMO,0,[Audio/MSAudio1 Audio/WMAudioV8 Audio/WMAudioV9 Audio/Lossless],[Audio/PCM Audio/Float])";

/// The harness names: `WMA`, `MFTAudioDecoder`, `DMOAudioDecoder`, `Audio`, `PCM`, `Float`,
/// `MSAudio1`, `WMAudioV8`, `WMAudioV9`, `Lossless`. An unknown GUID prints as `?`.
fn guid_name(g: &Guid) -> &'static str {
    const NAMES: [(Guid, &str); 10] = [
        (CLSID_WMA_DEC_MEDIA_OBJECT, "WMA"),
        (MFT_CATEGORY_AUDIO_DECODER, "MFTAudioDecoder"),
        (DMO_CATEGORY_AUDIO_DECODER, "DMOAudioDecoder"),
        (MEDIA_TYPE_AUDIO, "Audio"),
        (SUBTYPE_PCM, "PCM"),
        (SUBTYPE_IEEE_FLOAT, "Float"),
        (SUBTYPE_MSAUDIO1, "MSAudio1"),
        (SUBTYPE_WMAUDIO2, "WMAudioV8"),
        (SUBTYPE_WMAUDIO3, "WMAudioV9"),
        (SUBTYPE_WMAUDIO_LOSSLESS, "Lossless"),
    ];
    NAMES
        .iter()
        .find(|(known, _)| is_equal_guid(known, g))
        .map_or("?", |&(_, name)| name)
}

fn list(types: &[MediaType]) -> String {
    let items: Vec<String> = types
        .iter()
        .map(|t| format!("{}/{}", guid_name(&t.major), guid_name(&t.subtype)))
        .collect();
    format!("[{}]", items.join(" "))
}

/// `(hr, out)` for a `Result<(), HResult>` from a class-object call: `factory` when `*out` is set.
fn outcome(r: Result<(), HResult>) -> (HResult, &'static str) {
    match r {
        Ok(()) => (S_OK, "factory"),
        Err(hr) => (hr, "NULL"),
    }
}

/// Runs `DllRegisterServer` with each step returning the given `HRESULT`, and returns the
/// result and the external calls in order.
fn register(res: HResult, mft: HResult, dmo: HResult) -> (HResult, String) {
    let trace = RefCell::new(Vec::new());
    let hr = dll_register_server(
        || {
            trace.borrow_mut().push("res".to_string());
            res
        },
        |r| {
            trace.borrow_mut().push(format!(
                "mft({},{},{},{},{},{})",
                guid_name(&r.clsid),
                guid_name(&r.category),
                r.name,
                r.flags,
                list(r.inputs),
                list(r.outputs)
            ));
            mft
        },
        |r| {
            trace.borrow_mut().push(format!(
                "dmo({},{},{},{},{},{})",
                guid_name(&r.clsid),
                guid_name(&r.category),
                r.name,
                r.flags,
                list(r.inputs),
                list(r.outputs)
            ));
            dmo
        },
    );
    (hr, trace.into_inner().join(";"))
}

/// Runs `DllUnregisterServer` with each step returning the given `HRESULT`, and returns the
/// result and the external calls in order.
fn unregister(res: HResult, mft: HResult, dmo: HResult) -> (HResult, String) {
    let trace = RefCell::new(Vec::new());
    let hr = dll_unregister_server(
        || {
            trace.borrow_mut().push("unres".to_string());
            res
        },
        |clsid| {
            trace
                .borrow_mut()
                .push(format!("mftU({})", guid_name(clsid)));
            mft
        },
        |clsid, category| {
            trace.borrow_mut().push(format!(
                "dmoU({},{})",
                guid_name(clsid),
                guid_name(category)
            ));
            dmo
        },
    );
    (hr, trace.into_inner().join(";"))
}

#[test]
fn oracle_class_factory_rows() {
    let iid_unknown_near = Guid {
        data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47],
        ..IID_IUNKNOWN
    };
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };

    // row 01 QueryInterface riid=IClassFactory -> hr=00000000 out=factory
    assert_eq!(
        outcome(class_factory_query_interface(&IID_ICLASS_FACTORY)),
        (0x0000_0000, "factory")
    );
    // row 02 QueryInterface riid=IUnknown -> hr=00000000 out=factory
    assert_eq!(
        outcome(class_factory_query_interface(&IID_IUNKNOWN)),
        (0x0000_0000, "factory")
    );
    // row 03 QueryInterface riid=zero -> hr=80004002 out=NULL
    assert_eq!(
        outcome(class_factory_query_interface(&zero)),
        (0x8000_4002, "NULL")
    );
    // row 04 QueryInterface riid=IUnknownNear -> hr=80004002 out=NULL
    assert_eq!(
        outcome(class_factory_query_interface(&iid_unknown_near)),
        (0x8000_4002, "NULL")
    );
    // row 05 AddRef -> 2
    assert_eq!(class_factory_add_ref(), 2);
    // row 06 Release -> 1
    assert_eq!(class_factory_release(), 1);
    // row 07 LockServer dolock=1 -> hr=00000000
    assert_eq!(class_factory_lock_server(true), 0x0000_0000);
}

#[test]
fn oracle_dll_get_class_object_rows() {
    let clsid_near = Guid {
        data1: 0x2eeb_4ade,
        ..CLSID_WMA_DEC_MEDIA_OBJECT
    };
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };

    // row 08 DllGetClassObject clsid=WMA riid=IClassFactory -> hr=00000000 out=factory
    assert_eq!(
        outcome(dll_get_class_object(
            &CLSID_WMA_DEC_MEDIA_OBJECT,
            &IID_ICLASS_FACTORY
        )),
        (0x0000_0000, "factory")
    );
    // row 09 DllGetClassObject clsid=WMA riid=IUnknown -> hr=00000000 out=factory
    assert_eq!(
        outcome(dll_get_class_object(
            &CLSID_WMA_DEC_MEDIA_OBJECT,
            &IID_IUNKNOWN
        )),
        (0x0000_0000, "factory")
    );
    // row 10 DllGetClassObject clsid=WMA riid=zero -> hr=80004002 out=NULL
    assert_eq!(
        outcome(dll_get_class_object(&CLSID_WMA_DEC_MEDIA_OBJECT, &zero)),
        (0x8000_4002, "NULL")
    );
    // row 11 DllGetClassObject clsid=zero riid=IClassFactory -> hr=80040111 out=NULL
    assert_eq!(
        outcome(dll_get_class_object(&zero, &IID_ICLASS_FACTORY)),
        (0x8004_0111, "NULL")
    );
    // row 12 DllGetClassObject clsid=zero riid=zero -> hr=80040111 out=NULL
    assert_eq!(
        outcome(dll_get_class_object(&zero, &zero)),
        (0x8004_0111, "NULL")
    );
    // row 13 DllGetClassObject clsid=CLSIDNear riid=IClassFactory -> hr=80040111 out=NULL
    assert_eq!(
        outcome(dll_get_class_object(&clsid_near, &IID_ICLASS_FACTORY)),
        (0x8004_0111, "NULL")
    );
}

#[test]
fn oracle_dll_can_unload_now_row() {
    // row 14 DllCanUnloadNow -> hr=00000001
    assert_eq!(dll_can_unload_now(), 0x0000_0001);
}

#[test]
fn oracle_dll_register_server_rows() {
    // row 15 DllRegisterServer res=00000000 mft=00000000 dmo=00000000 -> hr=00000000
    let (hr, trace) = register(S_OK, S_OK, S_OK);
    assert_eq!(hr, 0x0000_0000);
    assert_eq!(trace, format!("res;{MFT_TOKEN};{DMO_TOKEN}"));

    // row 16 DllRegisterServer res=80004005 mft=00000000 dmo=00000000 -> hr=80004005 trace=res
    let (hr, trace) = register(E_FAIL, S_OK, S_OK);
    assert_eq!(hr, 0x8000_4005);
    assert_eq!(trace, "res");

    // row 17 DllRegisterServer res=00000000 mft=80004005 dmo=00000000 -> hr=80004005
    let (hr, trace) = register(S_OK, E_FAIL, S_OK);
    assert_eq!(hr, 0x8000_4005);
    assert_eq!(trace, format!("res;{MFT_TOKEN}"));

    // row 18 DllRegisterServer res=00000000 mft=00000001 dmo=00000000 -> hr=00000000
    let (hr, trace) = register(S_OK, S_FALSE, S_OK);
    assert_eq!(hr, 0x0000_0000);
    assert_eq!(trace, format!("res;{MFT_TOKEN};{DMO_TOKEN}"));

    // row 19 DllRegisterServer res=00000000 mft=00000000 dmo=80070005 -> hr=80070005
    let (hr, trace) = register(S_OK, S_OK, E_ACCESSDENIED);
    assert_eq!(hr, 0x8007_0005);
    assert_eq!(trace, format!("res;{MFT_TOKEN};{DMO_TOKEN}"));

    // row 20 DllRegisterServer res=8000FFFF mft=00000000 dmo=00000000 -> hr=8000FFFF trace=res
    let (hr, trace) = register(E_UNEXPECTED, S_OK, S_OK);
    assert_eq!(hr, 0x8000_FFFF);
    assert_eq!(trace, "res");
}

#[test]
fn oracle_dll_unregister_server_rows() {
    // row 21 DllUnregisterServer res=00000000 mft=00000000 dmo=00000000 -> hr=00000000
    //        trace=unres;mftU(WMA);dmoU(WMA,DMOAudioDecoder)
    let (hr, trace) = unregister(S_OK, S_OK, S_OK);
    assert_eq!(hr, 0x0000_0000);
    assert_eq!(trace, "unres;mftU(WMA);dmoU(WMA,DMOAudioDecoder)");

    // row 22 DllUnregisterServer res=80004005 mft=00000000 dmo=00000000 -> hr=80004005 trace=unres
    let (hr, trace) = unregister(E_FAIL, S_OK, S_OK);
    assert_eq!(hr, 0x8000_4005);
    assert_eq!(trace, "unres");

    // row 23 DllUnregisterServer res=00000000 mft=80004005 dmo=00000000 -> hr=80004005
    //        trace=unres;mftU(WMA)
    let (hr, trace) = unregister(S_OK, E_FAIL, S_OK);
    assert_eq!(hr, 0x8000_4005);
    assert_eq!(trace, "unres;mftU(WMA)");

    // row 24 DllUnregisterServer res=00000000 mft=00000001 dmo=00000000 -> hr=00000000
    //        trace=unres;mftU(WMA);dmoU(WMA,DMOAudioDecoder)
    let (hr, trace) = unregister(S_OK, S_FALSE, S_OK);
    assert_eq!(hr, 0x0000_0000);
    assert_eq!(trace, "unres;mftU(WMA);dmoU(WMA,DMOAudioDecoder)");

    // row 25 DllUnregisterServer res=00000000 mft=00000000 dmo=80004001 -> hr=80004001
    //        trace=unres;mftU(WMA);dmoU(WMA,DMOAudioDecoder)
    let (hr, trace) = unregister(S_OK, S_OK, E_NOTIMPL);
    assert_eq!(hr, 0x8000_4001);
    assert_eq!(trace, "unres;mftU(WMA);dmoU(WMA,DMOAudioDecoder)");
}
